//! Rank assignment and width/depth metrics. Mirrors Go `graph.go`.

use std::collections::HashMap;

use super::types::{Edge, Node};

/// Runs Kahn's BFS longest-path rank assignment. Returns `nodeID -> rank`.
/// Nodes in cycles get rank 0 (fallback). Mirrors Go `rankAssign`.
fn rank_assign(nodes: &[Node], edges: &[Edge]) -> HashMap<String, i64> {
    if nodes.is_empty() {
        return HashMap::new();
    }

    // Index of valid node IDs.
    let mut node_set: HashMap<&str, bool> = HashMap::with_capacity(nodes.len());
    for n in nodes {
        node_set.insert(n.id.as_str(), true);
    }

    // Adjacency (out-edges) and in-degree for valid nodes only.
    let mut adj: HashMap<String, Vec<String>> = HashMap::with_capacity(nodes.len());
    let mut in_degree: HashMap<String, i64> = HashMap::with_capacity(nodes.len());
    for n in nodes {
        adj.entry(n.id.clone()).or_default();
        in_degree.entry(n.id.clone()).or_insert(0);
    }
    for e in edges {
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

    // Cycle fallback: rank 0 for unvisited nodes.
    for n in nodes {
        if !visited.get(&n.id).copied().unwrap_or(false) {
            rank.insert(n.id.clone(), 0);
        }
    }

    rank
}

/// Maximum number of nodes sharing the same rank. 0 for empty graphs.
/// Mirrors Go `MaxWidth`.
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
/// Mirrors Go `Depth`.
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
    fn cycle_nodes_rank_zero() {
        // A <-> B cycle: both unvisited, rank 0, width 2, depth 1.
        let nodes = vec![n("A"), n("B")];
        let edges = vec![e("A", "B"), e("B", "A")];
        assert_eq!(max_width(&nodes, &edges), 2);
        assert_eq!(depth(&nodes, &edges), 1);
    }
}
