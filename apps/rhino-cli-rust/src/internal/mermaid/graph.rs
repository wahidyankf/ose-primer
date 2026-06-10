//! Rank assignment and width/depth metrics.

use std::collections::HashMap;

use super::types::{Edge, Node};

/// DFS colors for back-edge detection.
const WHITE: u8 = 0;
const GRAY: u8 = 1;
const BLACK: u8 = 2;

/// Detects back edges (edges pointing at a node currently on the DFS stack)
/// via iterative DFS, visiting roots in node-declaration order and children
/// in edge-declaration order (plan DD-14 fix 2). Returns the indices of back
/// edges in the caller's edge slice.
fn find_back_edges(
    nodes: &[Node],
    adj: &HashMap<&str, Vec<(usize, &str)>>,
) -> std::collections::HashSet<usize> {
    let mut color: HashMap<&str, u8> = HashMap::with_capacity(nodes.len());
    for n in nodes {
        color.insert(n.id.as_str(), WHITE);
    }

    let mut back = std::collections::HashSet::new();
    for n in nodes {
        if color.get(n.id.as_str()).copied().unwrap_or(WHITE) != WHITE {
            continue;
        }
        // Stack of (node, next-child cursor) frames.
        let mut stack: Vec<(&str, usize)> = vec![(n.id.as_str(), 0)];
        color.insert(n.id.as_str(), GRAY);
        while let Some(&(cur, cursor)) = stack.last() {
            let children: &[(usize, &str)] = adj.get(cur).map_or(&[], Vec::as_slice);
            if cursor < children.len() {
                if let Some(top) = stack.last_mut() {
                    top.1 += 1;
                }
                let (edge_idx, child) = children[cursor];
                match color.get(child).copied().unwrap_or(WHITE) {
                    GRAY => {
                        // Child is on the active DFS path → cycle-closing edge.
                        back.insert(edge_idx);
                    }
                    WHITE => {
                        color.insert(child, GRAY);
                        stack.push((child, 0));
                    }
                    _ => {} // BLACK: forward/cross edge — keep it.
                }
            } else {
                color.insert(cur, BLACK);
                stack.pop();
            }
        }
    }
    back
}

/// Runs Kahn's BFS longest-path rank assignment on the DAG left after
/// back-edge removal. Returns `nodeID -> rank`.
///
/// Plan DD-14 fix 2: back edges are detected via iterative DFS in
/// node-declaration order and removed before ranking, so cyclic diagrams
/// rank as chains. Previously a cycle emptied Kahn's queue, every node fell
/// back to rank 0, and the bogus span equaled the node count.
fn rank_assign(nodes: &[Node], edges: &[Edge]) -> HashMap<String, i64> {
    if nodes.is_empty() {
        return HashMap::new();
    }

    // Index of valid node IDs.
    let mut node_set: HashMap<&str, bool> = HashMap::with_capacity(nodes.len());
    for n in nodes {
        node_set.insert(n.id.as_str(), true);
    }

    // Adjacency over valid edges only, carrying each edge's index so back
    // edges can be excluded from the ranking pass below.
    let mut adj_idx: HashMap<&str, Vec<(usize, &str)>> = HashMap::with_capacity(nodes.len());
    for n in nodes {
        adj_idx.entry(n.id.as_str()).or_default();
    }
    for (i, e) in edges.iter().enumerate() {
        if node_set.contains_key(e.from.as_str()) && node_set.contains_key(e.to.as_str()) {
            adj_idx
                .entry(e.from.as_str())
                .or_default()
                .push((i, e.to.as_str()));
        }
    }

    let back_edges = find_back_edges(nodes, &adj_idx);

    // Adjacency (out-edges) and in-degree for valid nodes only, skipping the
    // detected back edges.
    let mut adj: HashMap<String, Vec<String>> = HashMap::with_capacity(nodes.len());
    let mut in_degree: HashMap<String, i64> = HashMap::with_capacity(nodes.len());
    for n in nodes {
        adj.entry(n.id.clone()).or_default();
        in_degree.entry(n.id.clone()).or_insert(0);
    }
    for (i, e) in edges.iter().enumerate() {
        if back_edges.contains(&i) {
            continue;
        }
        if node_set.contains_key(e.from.as_str()) && node_set.contains_key(e.to.as_str()) {
            adj.entry(e.from.clone()).or_default().push(e.to.clone());
            *in_degree.entry(e.to.clone()).or_insert(0) += 1;
        }
    }

    let mut rank: HashMap<String, i64> = HashMap::with_capacity(nodes.len());
    let mut visited: HashMap<String, bool> = HashMap::with_capacity(nodes.len());

    // Kahn's BFS: start with all in-degree-0 nodes, in node-slice order.
    let mut queue: Vec<String> = Vec::with_capacity(nodes.len());
    for n in nodes {
        if in_degree.get(&n.id).copied().unwrap_or(0) == 0 {
            queue.push(n.id.clone());
            rank.insert(n.id.clone(), 0);
        }
    }

    let mut head = 0usize;
    while head < queue.len() {
        let cur = queue[head].clone();
        head += 1;
        visited.insert(cur.clone(), true);

        let cur_rank = rank.get(&cur).copied().unwrap_or(0);
        let next_nodes = adj.get(&cur).cloned().unwrap_or_default();
        for next in next_nodes {
            // Longest-path rank: update if going through cur gives a longer path.
            if cur_rank + 1 > rank.get(&next).copied().unwrap_or(0) {
                rank.insert(next.clone(), cur_rank + 1);
            }
            let d = in_degree.entry(next.clone()).or_insert(0);
            *d -= 1;
            if *d == 0 {
                queue.push(next);
            }
        }
    }

    // Safety fallback: rank 0 for any node the BFS did not reach. With back
    // edges removed the remaining graph is a DAG and Kahn visits everything,
    // so this is defensive only.
    for n in nodes {
        if !visited.get(&n.id).copied().unwrap_or(false) {
            rank.insert(n.id.clone(), 0);
        }
    }

    rank
}

/// Maximum number of nodes sharing the same rank. 0 for empty graphs.
pub fn max_width(nodes: &[Node], edges: &[Edge]) -> i64 {
    if nodes.is_empty() {
        return 0;
    }

    let ranks = rank_assign(nodes, edges);
    let mut rank_count: HashMap<i64, i64> = HashMap::new();
    for r in ranks.values() {
        *rank_count.entry(*r).or_insert(0) += 1;
    }

    rank_count.values().copied().max().unwrap_or(0)
}

/// Number of distinct rank values (longest path length + 1). 0 for empty graphs.
pub fn depth(nodes: &[Node], edges: &[Edge]) -> i64 {
    if nodes.is_empty() {
        return 0;
    }

    let ranks = rank_assign(nodes, edges);
    let distinct: std::collections::HashSet<i64> = ranks.values().copied().collect();
    i64::try_from(distinct.len()).unwrap_or(i64::MAX)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    fn n(id: &str) -> Node {
        Node {
            id: id.to_string(),
            label: String::new(),
        }
    }
    fn e(from: &str, to: &str) -> Edge {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    #[test]
    fn empty_graph_is_zero() {
        assert_eq!(max_width(&[], &[]), 0);
        assert_eq!(depth(&[], &[]), 0);
    }

    #[test]
    fn fan_out_width_and_depth() {
        // R -> A,B,C,D : rank0 = {R}, rank1 = {A,B,C,D}
        let nodes = vec![n("R"), n("A"), n("B"), n("C"), n("D")];
        let edges = vec![e("R", "A"), e("R", "B"), e("R", "C"), e("R", "D")];
        assert_eq!(max_width(&nodes, &edges), 4);
        assert_eq!(depth(&nodes, &edges), 2);
    }

    #[test]
    fn sequential_chain_depth() {
        let nodes = vec![n("A"), n("A1"), n("A2"), n("A3")];
        let edges = vec![e("A", "A1"), e("A1", "A2"), e("A2", "A3")];
        assert_eq!(depth(&nodes, &edges), 4);
        assert_eq!(max_width(&nodes, &edges), 1);
    }

    #[test]
    fn cycle_ranks_as_chain_after_back_edge_removal() {
        // Plan DD-14 fix 2: the cyclic diagram `A-->B-->C-->A` must rank as a
        // chain — the back edge (C->A, found via iterative DFS in
        // node-declaration order) is removed, then Kahn longest-path ranking
        // runs on the remaining DAG A->B->C: span 1, depth 3. Today the cycle
        // empties Kahn's queue, every node falls back to rank 0, and the
        // bogus span equals the node count.
        let nodes = vec![n("A"), n("B"), n("C")];
        let edges = vec![e("A", "B"), e("B", "C"), e("C", "A")];
        assert_eq!(
            max_width(&nodes, &edges),
            1,
            "cycle must rank as a chain (span 1)"
        );
        assert_eq!(
            depth(&nodes, &edges),
            3,
            "cycle must rank as a chain (depth 3)"
        );
    }

    #[test]
    fn two_node_cycle_ranks_as_chain() {
        // BEHAVIOR CHANGE (plan DD-14 fix 2): this test previously pinned the
        // OLD cycle fallback (`cycle_nodes_rank_zero`), where the A <-> B
        // cycle emptied Kahn's queue and both nodes fell back to rank 0
        // (width 2, depth 1). The back edge B->A (found via iterative DFS in
        // node-declaration order) is now removed before ranking, so the cycle
        // ranks as the chain A->B: span 1, depth 2.
        let nodes = vec![n("A"), n("B")];
        let edges = vec![e("A", "B"), e("B", "A")];
        assert_eq!(max_width(&nodes, &edges), 1);
        assert_eq!(depth(&nodes, &edges), 2);
    }
}
