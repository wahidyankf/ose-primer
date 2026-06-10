//! Mermaid flowchart parsing. Mirrors Go `parser.go`.
//!
//! Pure regex/string parsing (no tree-sitter — matching the Go implementation).

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

use super::types::{Direction, Edge, MermaidBlock, Node, ParsedDiagram, Subgraph};

/// Matches a flowchart/graph header line (with optional direction). Mirrors Go `flowchartHeaderRe`.
static FLOWCHART_HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*(flowchart|graph)(\s+(TB|TD|BT|LR|RL))?\s*$").expect("valid header regex")
});

/// Captures optional ID and label of a subgraph declaration. Mirrors Go `subgraphHeaderRe`.
static SUBGRAPH_HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^subgraph(?:\s+([^\s\["]+))?(?:\s*\[\s*"?([^"\]]*)"?\s*\])?\s*$"#)
        .expect("valid subgraph regex")
});

/// Identifies edge lines. Mirrors Go `arrowTokenRe`.
static ARROW_TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"-->|---|-\.->|==>|--o|--x|<-->").expect("valid arrow regex"));

/// Matches a bare word node identifier. Mirrors Go `nodeIDRe`.
static NODE_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\w+)$").expect("valid node id regex"));

/// Edge-label cleanup: replaces `-- text -->` with `-->`. Mirrors Go `linkTextRe`.
static LINK_TEXT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"--[^->\n]+?-->").expect("valid link text regex"));

/// Pipe edge-label cleanup: strips a `|label|` segment that follows an arrow
/// token (`A -->|text| B`) so the target node survives edge splitting (plan
/// DD-14 fix 1). Mirrors the Go `pipeLabelRe` twin.
static PIPE_LABEL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(-->|---|-\.->|==>|--o|--x|<-->)\s*\|[^|]*\|").expect("valid pipe label regex")
});

/// Node shape patterns: order matters (longest/most-specific first). Each captures
/// (nodeID, label). Mirrors Go `nodeShapePatterns`.
static NODE_SHAPE_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        // Double-circle: A(((label)))
        r"^(\w+)\(\(\(([^)]*)\)\)\)",
        // Stadium: A([label])
        r"^(\w+)\(\[([^\]]*)\]\)",
        // Circle: A((label))
        r"^(\w+)\(\(([^)]*)\)\)",
        // Subroutine: A[[label]]
        r"^(\w+)\[\[([^\]]*)\]\]",
        // Cylinder: A[(label)]
        r"^(\w+)\[\(([^)]*)\)\]",
        // Round: A(label)
        r"^(\w+)\(([^)]*)\)",
        // Hexagon: A{{label}}
        r"^(\w+)\{\{([^}]*)\}\}",
        // Diamond: A{label}
        r"^(\w+)\{([^}]*)\}",
        // Asymmetric: A>label]
        r"^(\w+)>([^\]]*)\]",
        // Parallelogram forward: A[/label/]
        r"^(\w+)\[/([^/]*)/\]",
        // Parallelogram back: A[\label\]
        r"^(\w+)\[\\([^\\]*)\\]",
        // Rectangle: A[label]
        r"^(\w+)\[([^\]]*)\]",
        // Modern API: A@{ label: "text" }
        r#"^(\w+)@\{\s*[^}]*label:\s*"([^"]*)"\s*[^}]*\}"#,
    ]
    .iter()
    .map(|p| Regex::new(p).expect("valid node shape regex"))
    .collect()
});

/// An insertion-ordered map mirroring Go's `map[string]string` usage where
/// node IDs are inserted in encounter order and looked up by key. Ordering is
/// preserved so that the `new_keys` diff is deterministic.
#[derive(Default)]
struct NodeMap {
    order: Vec<String>,
    map: HashMap<String, String>,
}

impl NodeMap {
    fn contains(&self, k: &str) -> bool {
        self.map.contains_key(k)
    }

    fn set(&mut self, k: &str, v: String) {
        if !self.map.contains_key(k) {
            self.order.push(k.to_string());
        }
        self.map.insert(k.to_string(), v);
    }

    /// Inserts an empty label only if the key is absent (mirrors Go's
    /// `if _, exists := nodeMap[id]; !exists { nodeMap[id] = "" }`).
    fn set_if_absent(&mut self, k: &str) {
        if !self.map.contains_key(k) {
            self.order.push(k.to_string());
            self.map.insert(k.to_string(), String::new());
        }
    }

    fn get(&self, k: &str) -> Option<&String> {
        self.map.get(k)
    }

    fn keys_snapshot(&self) -> std::collections::HashSet<String> {
        self.map.keys().cloned().collect()
    }
}

/// Parses a `MermaidBlock` into a `ParsedDiagram`. The second return value is
/// the number of flowchart/graph headers found (0 → not a flowchart;
/// >1 → caller emits MultipleDiagrams). Mirrors Go `ParseDiagram`.
pub fn parse_diagram(block: &MermaidBlock) -> (ParsedDiagram, usize) {
    let matches: Vec<regex::Captures> = FLOWCHART_HEADER_RE.captures_iter(&block.source).collect();
    let count = matches.len();
    if count == 0 {
        return (
            ParsedDiagram {
                block: block.clone(),
                direction: Direction::Tb,
                nodes: Vec::new(),
                edges: Vec::new(),
                subgraphs: Vec::new(),
            },
            0,
        );
    }

    // Direction from the first header match (group 3).
    let dir = matches[0]
        .get(3)
        .map_or(Direction::Tb, |m| Direction::parse(m.as_str()));

    let mut node_map = NodeMap::default();
    let mut edges: Vec<Edge> = Vec::new();
    let mut subgraphs: Vec<Subgraph> = Vec::new();
    let mut stack: Vec<Subgraph> = Vec::new();

    for (line_idx, raw) in block.source.split('\n').enumerate() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("subgraph") {
            let (id, label) = parse_subgraph_header(line);
            stack.push(Subgraph {
                id,
                label,
                node_ids: Vec::new(),
                start_line: line_idx + 1,
            });
            continue;
        }
        if line == "end" {
            if let Some(sg) = stack.pop() {
                subgraphs.push(sg);
            }
            continue;
        }

        // Skip the flowchart/graph header lines.
        if FLOWCHART_HEADER_RE.is_match(line) {
            continue;
        }

        let before = node_map.keys_snapshot();
        if ARROW_TOKEN_RE.is_match(line) {
            extract_edge_line(line, &mut node_map, &mut edges);
        } else {
            extract_standalone_node(line, &mut node_map);
        }
        let line_ids = new_keys(&node_map, &before);

        // If a subgraph is open, attribute new IDs as direct children.
        if let Some(top) = stack.last_mut()
            && !line_ids.is_empty()
        {
            for id in dedup_order(&line_ids) {
                if !top.node_ids.contains(&id) {
                    top.node_ids.push(id);
                }
            }
        }
    }

    // Pop unclosed subgraphs so they still surface (reverse stack order, matching Go).
    while let Some(sg) = stack.pop() {
        subgraphs.push(sg);
    }

    // Build ordered node list using deterministic re-scan of the source.
    let seen_order = collect_node_order(&block.source, &node_map);
    let nodes: Vec<Node> = seen_order
        .into_iter()
        .map(|id| Node {
            label: node_map.get(&id).cloned().unwrap_or_default(),
            id,
        })
        .collect();

    (
        ParsedDiagram {
            block: block.clone(),
            direction: dir,
            nodes,
            edges,
            subgraphs,
        },
        count,
    )
}

/// Extracts optional ID and label from a subgraph header. Mirrors Go `parseSubgraphHeader`.
fn parse_subgraph_header(line: &str) -> (String, String) {
    if let Some(m) = SUBGRAPH_HEADER_RE.captures(line) {
        let id = m.get(1).map_or(String::new(), |g| g.as_str().to_string());
        let label = m.get(2).map_or(String::new(), |g| g.as_str().to_string());
        return (id, label);
    }
    // Fallback: strip `subgraph ` prefix and treat the rest as a label.
    let rest = line.strip_prefix("subgraph").unwrap_or(line).trim();
    let rest = rest.trim_matches('"');
    (String::new(), rest.to_string())
}

/// Keys in `node_map` absent from the snapshot. Mirrors Go `newKeys`.
/// Iterates in insertion order for deterministic attribution.
fn new_keys(node_map: &NodeMap, snapshot: &std::collections::HashSet<String>) -> Vec<String> {
    node_map
        .order
        .iter()
        .filter(|k| !snapshot.contains(*k))
        .cloned()
        .collect()
}

/// Dedups IDs preserving first occurrence. Mirrors Go `dedupOrder`.
fn dedup_order(ids: &[String]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for id in ids {
        if seen.insert(id.clone()) {
            out.push(id.clone());
        }
    }
    out
}

/// Node IDs in first-seen order from the source lines. Mirrors Go `collectNodeOrder`.
fn collect_node_order(source: &str, node_map: &NodeMap) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut order = Vec::new();

    for raw in source.split('\n') {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("subgraph") || line == "end" {
            continue;
        }
        if FLOWCHART_HEADER_RE.is_match(line) {
            continue;
        }
        for id in extract_all_node_ids(line) {
            if node_map.contains(&id) && seen.insert(id.clone()) {
                order.push(id);
            }
        }
    }
    // Include any IDs in node_map not yet seen (sorted for determinism; mirrors Go's safety pass).
    let mut remaining: Vec<String> = node_map
        .order
        .iter()
        .filter(|id| !seen.contains(*id))
        .cloned()
        .collect();
    remaining.sort();
    order.extend(remaining);
    order
}

/// Pulls every node ID referenced on a single line. Mirrors Go `extractAllNodeIDs`.
fn extract_all_node_ids(line: &str) -> Vec<String> {
    // Strip `|label|` edge labels first so the target node of `A -->|text| B`
    // is seen by the ordering scan as well (plan DD-14 fix 1).
    let line = PIPE_LABEL_RE.replace_all(line, "$1");
    let mut ids = Vec::new();
    if ARROW_TOKEN_RE.is_match(&line) {
        for seg in ARROW_TOKEN_RE.split(&line) {
            ids.extend(extract_node_ids_from_segment(seg));
        }
    } else {
        ids.extend(extract_node_ids_from_segment(&line));
    }
    ids
}

/// Splits a segment on `&` and extracts all node IDs. Mirrors Go `extractNodeIDsFromSegment`.
fn extract_node_ids_from_segment(seg: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for sub in seg.split('&') {
        let id = extract_node_id_from_segment(sub);
        if !id.is_empty() {
            ids.push(id);
        }
    }
    ids
}

/// Extracts a node ID from a single segment. Mirrors Go `extractNodeIDFromSegment`.
fn extract_node_id_from_segment(seg: &str) -> String {
    let seg = seg.trim();
    if seg.is_empty() {
        return String::new();
    }
    for re in NODE_SHAPE_PATTERNS.iter() {
        if let Some(m) = re.captures(seg)
            && let Some(g) = m.get(1)
        {
            return g.as_str().to_string();
        }
    }
    if let Some(m) = NODE_ID_RE.captures(seg)
        && let Some(g) = m.get(1)
    {
        return g.as_str().to_string();
    }
    String::new()
}

/// Parses a standalone node declaration line, updating `node_map`.
/// Mirrors Go `extractStandaloneNode`.
fn extract_standalone_node(line: &str, node_map: &mut NodeMap) {
    let line = line.trim();
    for re in NODE_SHAPE_PATTERNS.iter() {
        if let Some(m) = re.captures(line) {
            let id = m.get(1).map_or("", |g| g.as_str());
            let label = m.get(2).map_or("", |g| g.as_str());
            node_map.set(id, normalize_label(label));
            return;
        }
    }
    if let Some(m) = NODE_ID_RE.captures(line)
        && let Some(g) = m.get(1)
    {
        node_map.set_if_absent(g.as_str());
    }
}

/// Parses an edge line, updating `node_map` and appending edges. Handles the
/// `&` multi-target operator via Cartesian product. Mirrors Go `extractEdgeLine`.
fn extract_edge_line(line: &str, node_map: &mut NodeMap, edges: &mut Vec<Edge>) {
    // Replace `-- text -->` edge labels with `-->`.
    let line = LINK_TEXT_RE.replace_all(line, "-->");
    // Strip `|label|` segments following arrows BEFORE edge splitting so the
    // pipe-labeled edge keeps its target node (plan DD-14 fix 1).
    let line = PIPE_LABEL_RE.replace_all(&line, "$1");

    // Split on arrow tokens — each part is one node group (possibly &-joined).
    let parts: Vec<&str> = ARROW_TOKEN_RE.split(&line).collect();
    if parts.len() < 2 {
        return;
    }

    let mut groups: Vec<Vec<String>> = Vec::new();
    for part in parts {
        let ids = extract_node_group(part, node_map);
        if !ids.is_empty() {
            groups.push(ids);
        }
    }

    // Cartesian product of consecutive groups.
    let mut i = 0;
    while i + 1 < groups.len() {
        for from in &groups[i] {
            for to in &groups[i + 1] {
                edges.push(Edge {
                    from: from.clone(),
                    to: to.clone(),
                });
            }
        }
        i += 1;
    }
}

/// Splits `part` on `&`, extracts node IDs and updates labels. Mirrors Go `extractNodeGroup`.
fn extract_node_group(part: &str, node_map: &mut NodeMap) -> Vec<String> {
    let mut ids = Vec::new();
    for seg in part.split('&') {
        let seg = seg.trim();
        if seg.is_empty() {
            continue;
        }
        let id = extract_node_id_and_label(seg, node_map);
        if !id.is_empty() {
            ids.push(id);
        }
    }
    ids
}

/// Returns the node ID for a segment, updating the label if present.
/// Mirrors Go `extractNodeIDAndLabel`.
fn extract_node_id_and_label(seg: &str, node_map: &mut NodeMap) -> String {
    for re in NODE_SHAPE_PATTERNS.iter() {
        if let Some(m) = re.captures(seg) {
            let id = m.get(1).map_or("", |g| g.as_str());
            let label = m.get(2).map_or("", |g| g.as_str());
            node_map.set(id, normalize_label(label));
            return id.to_string();
        }
    }
    if let Some(m) = NODE_ID_RE.captures(seg)
        && let Some(g) = m.get(1)
    {
        node_map.set_if_absent(g.as_str());
        return g.as_str().to_string();
    }
    String::new()
}

/// Strips surrounding quotes (single/double) and backtick wrappers. Mirrors Go `normalizeLabel`.
fn normalize_label(s: &str) -> String {
    let s = s.trim();
    let bytes = s.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"')
            || (first == b'\'' && last == b'\'')
            || (first == b'`' && last == b'`')
        {
            return s[1..s.len() - 1].to_string();
        }
    }
    s.to_string()
}

/// Display length of a Mermaid node label. Multi-line variants (`<br/>`, `\n`)
/// are checked per visual line; the longest line length (in runes) is returned.
/// Mirrors Go `EffectiveLabelLen`.
pub fn effective_label_len(label: &str) -> usize {
    if label.is_empty() {
        return 0;
    }
    let normalized = label
        .replace("<br/>", "\n")
        .replace("<BR/>", "\n")
        .replace("<br>", "\n")
        .replace("<BR>", "\n")
        .replace("\\n", "\n"); // Mermaid literal \n in labels
    normalized
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn block(src: &str) -> MermaidBlock {
        MermaidBlock {
            file_path: "f.md".to_string(),
            block_index: 0,
            source: src.to_string(),
            start_line: 1,
        }
    }

    #[test]
    fn non_flowchart_returns_zero_count() {
        let (_d, count) = parse_diagram(&block("sequenceDiagram\n  A->>B: hi"));
        assert_eq!(count, 0);
    }

    #[test]
    fn flowchart_direction_default_tb() {
        let (d, count) = parse_diagram(&block("flowchart\n  A --> B"));
        assert_eq!(count, 1);
        assert_eq!(d.direction, Direction::Tb);
    }

    #[test]
    fn flowchart_direction_lr() {
        let (d, _c) = parse_diagram(&block("flowchart LR\n  A --> B"));
        assert_eq!(d.direction, Direction::Lr);
    }

    #[test]
    fn parses_nodes_and_edges() {
        let (d, _c) = parse_diagram(&block("flowchart TD\n  A[Start] --> B[End]"));
        assert_eq!(d.nodes.len(), 2);
        assert_eq!(d.edges.len(), 1);
        assert_eq!(d.edges[0].from, "A");
        assert_eq!(d.edges[0].to, "B");
        let a = d.nodes.iter().find(|n| n.id == "A").unwrap();
        assert_eq!(a.label, "Start");
    }

    #[test]
    fn multi_target_ampersand_cartesian() {
        let (d, _c) = parse_diagram(&block("flowchart TD\n  A & B --> C & D"));
        // A->C, A->D, B->C, B->D
        assert_eq!(d.edges.len(), 4);
    }

    #[test]
    fn edge_label_stripped() {
        let (d, _c) = parse_diagram(&block("flowchart TD\n  A -- yes --> B"));
        assert_eq!(d.edges.len(), 1);
        assert_eq!(d.edges[0].from, "A");
        assert_eq!(d.edges[0].to, "B");
    }

    #[test]
    fn pipe_labeled_edge_parses_as_edge() {
        // Plan DD-14 fix 1: `A -->|text| B` is standard Mermaid. The `|text|`
        // segment after the arrow must be stripped before edge splitting so
        // the edge survives and the target node B is extracted.
        let (d, _c) = parse_diagram(&block("flowchart TD\n  A -->|text| B"));
        assert_eq!(
            d.edges.len(),
            1,
            "pipe-labeled edge must parse as an edge, got {:?}",
            d.edges
        );
        assert_eq!(d.edges[0].from, "A");
        assert_eq!(d.edges[0].to, "B");
        assert!(
            d.nodes.iter().any(|n| n.id == "B"),
            "target node B must be extracted, got {:?}",
            d.nodes
        );
    }

    #[test]
    fn pipe_labeled_edge_target_ranked_below_source() {
        // Plan DD-14 fix 1: with the pipe-labeled edge extracted, B ranks one
        // level below A — chain of two: span 1, depth 2 (today the dropped
        // edge mis-ranks the diagram).
        use super::super::graph;
        let (d, _c) = parse_diagram(&block("flowchart TD\n  A -->|yes| B"));
        assert_eq!(graph::max_width(&d.nodes, &d.edges), 1);
        assert_eq!(graph::depth(&d.nodes, &d.edges), 2);
    }

    #[test]
    fn double_header_count_two() {
        let (_d, count) = parse_diagram(&block("flowchart TD\n  A --> B\nflowchart LR\n  C --> D"));
        assert_eq!(count, 2);
    }

    #[test]
    fn subgraph_children_counted() {
        let src = "flowchart TD\n  subgraph G [Group]\n    N1\n    N2\n    N3\n  end";
        let (d, _c) = parse_diagram(&block(src));
        assert_eq!(d.subgraphs.len(), 1);
        assert_eq!(d.subgraphs[0].label, "Group");
        assert_eq!(d.subgraphs[0].node_ids.len(), 3);
    }

    #[test]
    fn effective_label_len_handles_breaks() {
        assert_eq!(effective_label_len(""), 0);
        assert_eq!(effective_label_len("hello"), 5);
        assert_eq!(effective_label_len("ab<br/>cdef"), 4);
        assert_eq!(effective_label_len("ab\\ncdef"), 4);
    }

    #[test]
    fn normalize_label_strips_quotes() {
        assert_eq!(normalize_label("\"abc\""), "abc");
        assert_eq!(normalize_label("'abc'"), "abc");
        assert_eq!(normalize_label("`abc`"), "abc");
        assert_eq!(normalize_label("abc"), "abc");
    }

    #[test]
    fn node_shapes_diamond_and_hexagon() {
        let (d, _c) = parse_diagram(&block("flowchart TD\n  A{diamond}\n  B{{hex}}"));
        let a = d.nodes.iter().find(|n| n.id == "A").unwrap();
        let b = d.nodes.iter().find(|n| n.id == "B").unwrap();
        assert_eq!(a.label, "diamond");
        assert_eq!(b.label, "hex");
    }
}
