//! Mermaid rule validation. Mirrors Go `validator.go`.

use std::collections::HashMap;

use super::graph::{depth, max_width};
use super::parser::{effective_label_len, parse_diagram};
use super::types::{
    Direction, MermaidBlock, ValidationResult, Violation, ViolationKind, Warning, WarningKind,
};

/// Validation thresholds. Mirrors Go `ValidateOptions`.
#[derive(Debug, Clone, Copy)]
pub struct ValidateOptions {
    pub max_label_len: i64,
    pub max_width: i64,
    pub max_depth: i64,
    pub max_subgraph_nodes: i64,
}

impl Default for ValidateOptions {
    /// Standard validation thresholds. Mirrors Go `DefaultValidateOptions`
    /// (MaxDepth = math.MaxInt).
    fn default() -> Self {
        Self {
            max_label_len: 30,
            max_width: 4,
            max_depth: i64::MAX,
            max_subgraph_nodes: 6,
        }
    }
}

/// Validates a slice of `MermaidBlock`s against the given options. Applies four
/// rules (three blocking violations, two warnings). Mirrors Go `ValidateBlocks`.
///
/// Note: `files_scanned` is set by the caller in the Go command handler (from
/// the count of files that produced blocks); `ValidateBlocks` itself sets it to
/// the count of distinct file paths among `blocks`, matching the Go function.
pub fn validate_blocks(blocks: &[MermaidBlock], opts: ValidateOptions) -> ValidationResult {
    let mut files_seen: HashMap<&str, bool> = HashMap::new();
    let mut violations: Vec<Violation> = Vec::new();
    let mut warnings: Vec<Warning> = Vec::new();

    for block in blocks {
        files_seen.insert(block.file_path.as_str(), true);
        validate_block(block, opts, &mut violations, &mut warnings);
    }

    ValidationResult {
        files_scanned: files_seen.len(),
        blocks_scanned: blocks.len(),
        violations,
        warnings,
    }
}

/// Validates a single block, appending any findings. Mirrors the per-block body
/// of Go `ValidateBlocks`.
fn validate_block(
    block: &MermaidBlock,
    opts: ValidateOptions,
    violations: &mut Vec<Violation>,
    warnings: &mut Vec<Warning>,
) {
    let (diagram, count) = parse_diagram(block);

    // Rule 3: multiple diagrams in one block.
    if count > 1 {
        violations.push(base_violation(ViolationKind::MultipleDiagrams, block));
    }

    // Non-flowchart: skip Rule 1 and Rule 2.
    if count == 0 {
        return;
    }

    // Rule 1: label length.
    for node in &diagram.nodes {
        let label_len = effective_label_len(&node.label);
        if i64::try_from(label_len).unwrap_or(i64::MAX) > opts.max_label_len {
            let mut v = base_violation(ViolationKind::LabelTooLong, block);
            v.node_id.clone_from(&node.id);
            v.label_text.clone_from(&node.label);
            v.label_len = label_len;
            v.max_label_len = opts.max_label_len;
            violations.push(v);
        }
    }

    // Rule 2: width/depth — direction-aware.
    let span = max_width(&diagram.nodes, &diagram.edges);
    let dep = depth(&diagram.nodes, &diagram.edges);
    let (horizontal, vertical) = match diagram.direction {
        Direction::Lr | Direction::Rl => (dep, span),
        Direction::Tb | Direction::Td | Direction::Bt => (span, dep),
    };

    if horizontal > opts.max_width && vertical > opts.max_depth {
        // Both exceeded → warning only.
        let mut w = base_warning(WarningKind::ComplexDiagram, block);
        w.start_line = block.start_line;
        w.actual_width = horizontal;
        w.actual_depth = vertical;
        w.max_width = opts.max_width;
        w.max_depth = opts.max_depth;
        warnings.push(w);
    } else if horizontal > opts.max_width {
        // Width exceeded alone → violation.
        let mut v = base_violation(ViolationKind::WidthExceeded, block);
        v.actual_width = horizontal;
        v.max_width = opts.max_width;
        violations.push(v);
    }
    // Depth exceeded alone → no output.

    // Rule 4: subgraph density (warning only).
    if opts.max_subgraph_nodes > 0 {
        for sg in &diagram.subgraphs {
            let n = i64::try_from(sg.node_ids.len()).unwrap_or(i64::MAX);
            if n > opts.max_subgraph_nodes {
                let mut w = base_warning(WarningKind::SubgraphDense, block);
                w.start_line = block.start_line + sg.start_line;
                w.subgraph_label.clone_from(&sg.label);
                w.subgraph_node_count = sg.node_ids.len();
                w.max_subgraph_nodes = opts.max_subgraph_nodes;
                warnings.push(w);
            }
        }
    }
}

/// Constructs a violation with file/block/line fields filled and all
/// kind-specific fields zeroed (to be overridden by the caller).
fn base_violation(kind: ViolationKind, block: &MermaidBlock) -> Violation {
    Violation {
        kind,
        file_path: block.file_path.clone(),
        block_index: block.block_index,
        start_line: block.start_line,
        node_id: String::new(),
        label_text: String::new(),
        label_len: 0,
        max_label_len: 0,
        actual_width: 0,
        max_width: 0,
    }
}

/// Constructs a warning with file/block/line fields filled and all
/// kind-specific fields zeroed (to be overridden by the caller).
fn base_warning(kind: WarningKind, block: &MermaidBlock) -> Warning {
    Warning {
        kind,
        file_path: block.file_path.clone(),
        block_index: block.block_index,
        start_line: block.start_line,
        actual_width: 0,
        actual_depth: 0,
        max_width: 0,
        max_depth: 0,
        subgraph_label: String::new(),
        subgraph_node_count: 0,
        max_subgraph_nodes: 0,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::internal::mermaid::extractor::extract_blocks;

    fn blocks(md: &str) -> Vec<MermaidBlock> {
        extract_blocks("f.md", md)
    }

    #[test]
    fn no_violations_for_short_labels() {
        let r = validate_blocks(
            &blocks("```mermaid\nflowchart TD\n  A[ok] --> B[fine]\n```\n"),
            ValidateOptions::default(),
        );
        assert!(r.violations.is_empty());
        assert!(r.warnings.is_empty());
        assert_eq!(r.blocks_scanned, 1);
        assert_eq!(r.files_scanned, 1);
    }

    #[test]
    fn label_too_long_violation() {
        let md = "```mermaid\nflowchart TD\n  A[This label is definitely longer than thirty characters]\n```\n";
        let r = validate_blocks(&blocks(md), ValidateOptions::default());
        assert_eq!(r.violations.len(), 1);
        assert_eq!(r.violations[0].kind, ViolationKind::LabelTooLong);
        assert_eq!(r.violations[0].node_id, "A");
    }

    #[test]
    fn width_exceeded_violation() {
        // 5 parallel nodes at rank 1, default max-width 4 → 5 > 4.
        let md = "```mermaid\nflowchart TD\n  R --> A\n  R --> B\n  R --> C\n  R --> D\n  R --> E\n```\n";
        let r = validate_blocks(&blocks(md), ValidateOptions::default());
        assert_eq!(r.violations.len(), 1);
        assert_eq!(r.violations[0].kind, ViolationKind::WidthExceeded);
        assert_eq!(r.violations[0].actual_width, 5);
    }

    #[test]
    fn both_exceeded_warning_not_violation() {
        // 5 wide AND deep, with max_depth lowered so depth also exceeds.
        let md = "```mermaid\nflowchart TD\n  R --> A\n  R --> B\n  R --> C\n  R --> D\n  R --> E\n  A --> A1\n  A1 --> A2\n```\n";
        let opts = ValidateOptions {
            max_depth: 2,
            ..ValidateOptions::default()
        };
        let r = validate_blocks(&blocks(md), opts);
        assert!(r.violations.is_empty());
        assert_eq!(r.warnings.len(), 1);
        assert_eq!(r.warnings[0].kind, WarningKind::ComplexDiagram);
    }

    #[test]
    fn multiple_diagrams_violation() {
        let md = "```mermaid\nflowchart TD\n  A --> B\nflowchart LR\n  C --> D\n```\n";
        let r = validate_blocks(&blocks(md), ValidateOptions::default());
        assert!(
            r.violations
                .iter()
                .any(|v| v.kind == ViolationKind::MultipleDiagrams)
        );
    }

    #[test]
    fn subgraph_density_warning() {
        let md = "```mermaid\nflowchart TD\n  subgraph G [Group]\n    N1\n    N2\n    N3\n    N4\n    N5\n    N6\n    N7\n  end\n```\n";
        let r = validate_blocks(&blocks(md), ValidateOptions::default());
        assert!(
            r.warnings
                .iter()
                .any(|w| w.kind == WarningKind::SubgraphDense && w.subgraph_node_count == 7)
        );
    }

    #[test]
    fn non_flowchart_ignored() {
        let md = "```mermaid\nsequenceDiagram\n  A->>B: hi\n```\n";
        let r = validate_blocks(&blocks(md), ValidateOptions::default());
        assert!(r.violations.is_empty());
        assert!(r.warnings.is_empty());
        assert_eq!(r.blocks_scanned, 1);
    }
}
