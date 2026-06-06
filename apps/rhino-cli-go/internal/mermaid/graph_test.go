package mermaid

import (
	"testing"
)

func makeNodes(ids ...string) []Node {
	nodes := make([]Node, len(ids))
	for i, id := range ids {
		nodes[i] = Node{ID: id}
	}
	return nodes
}

func makeEdges(pairs ...string) []Edge {
	if len(pairs)%2 != 0 {
		panic("makeEdges: odd number of args")
	}
	edges := make([]Edge, len(pairs)/2)
	for i := 0; i < len(pairs); i += 2 {
		edges[i/2] = Edge{From: pairs[i], To: pairs[i+1]}
	}
	return edges
}

func TestMaxWidth(t *testing.T) {
	tests := []struct {
		name  string
		nodes []Node
		edges []Edge
		want  int
	}{
		{
			name:  "empty graph",
			nodes: nil,
			edges: nil,
			want:  0,
		},
		{
			name:  "single node no edges",
			nodes: makeNodes("A"),
			edges: nil,
			want:  1,
		},
		{
			name:  "linear chain A->B->C",
			nodes: makeNodes("A", "B", "C"),
			edges: makeEdges("A", "B", "B", "C"),
			want:  1,
		},
		{
			name:  "long sequential chain 10 nodes",
			nodes: makeNodes("A", "B", "C", "D", "E", "F", "G", "H", "I", "J"),
			edges: makeEdges(
				"A", "B", "B", "C", "C", "D", "D", "E",
				"E", "F", "F", "G", "G", "H", "H", "I", "I", "J",
			),
			want: 1,
		},
		{
			name:  "fan-out A->B A->C A->D",
			nodes: makeNodes("A", "B", "C", "D"),
			edges: makeEdges("A", "B", "A", "C", "A", "D"),
			want:  3,
		},
		{
			name:  "fan-out A->B A->C A->D A->E",
			nodes: makeNodes("A", "B", "C", "D", "E"),
			edges: makeEdges("A", "B", "A", "C", "A", "D", "A", "E"),
			want:  4,
		},
		{
			name:  "diamond A->B A->C B->D C->D",
			nodes: makeNodes("A", "B", "C", "D"),
			edges: makeEdges("A", "B", "A", "C", "B", "D", "C", "D"),
			want:  2,
		},
		{
			name:  "two disconnected chains A->B and C->D",
			nodes: makeNodes("A", "B", "C", "D"),
			edges: makeEdges("A", "B", "C", "D"),
			want:  2, // rank 0: A, C (count=2); rank 1: B, D (count=2)
		},
		{
			// BEHAVIOR CHANGE (plan DD-14 fix 2): this entry previously
			// pinned the OLD cycle fallback (Go twin of Rust's
			// `cycle_nodes_rank_zero`), where the A <-> B cycle emptied
			// Kahn's queue and both nodes fell back to rank 0 (width 2,
			// depth 1). The back edge B->A (found via iterative DFS in
			// node-declaration order) is now removed before ranking, so the
			// cycle ranks as the chain A->B: span 1, depth 2 (mirrors the
			// Rust twin's updated `two_node_cycle_ranks_as_chain`).
			name:  "two node cycle ranks as chain",
			nodes: makeNodes("A", "B"),
			edges: makeEdges("A", "B", "B", "A"),
			want:  1,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := MaxWidth(tt.nodes, tt.edges)
			if got != tt.want {
				t.Errorf("MaxWidth = %d, want %d", got, tt.want)
			}
		})
	}
}

func TestDepth(t *testing.T) {
	tests := []struct {
		name  string
		nodes []Node
		edges []Edge
		want  int
	}{
		{
			name:  "empty graph",
			nodes: nil,
			edges: nil,
			want:  0,
		},
		{
			name:  "single node no edges",
			nodes: makeNodes("A"),
			edges: nil,
			want:  1,
		},
		{
			name:  "linear chain A->B->C",
			nodes: makeNodes("A", "B", "C"),
			edges: makeEdges("A", "B", "B", "C"),
			want:  3,
		},
		{
			name:  "fan-out A->B A->C A->D",
			nodes: makeNodes("A", "B", "C", "D"),
			edges: makeEdges("A", "B", "A", "C", "A", "D"),
			want:  2,
		},
		{
			name:  "diamond A->B A->C B->D C->D",
			nodes: makeNodes("A", "B", "C", "D"),
			edges: makeEdges("A", "B", "A", "C", "B", "D", "C", "D"),
			want:  3,
		},
		{
			// BEHAVIOR CHANGE (plan DD-14 fix 2): this entry previously
			// pinned the OLD cycle fallback (Go twin of Rust's
			// `cycle_nodes_rank_zero`), where the A <-> B cycle emptied
			// Kahn's queue and both nodes fell back to rank 0 (width 2,
			// depth 1). The back edge B->A (found via iterative DFS in
			// node-declaration order) is now removed before ranking, so the
			// cycle ranks as the chain A->B: span 1, depth 2 (mirrors the
			// Rust twin's updated `two_node_cycle_ranks_as_chain`).
			name:  "two node cycle ranks as chain",
			nodes: makeNodes("A", "B"),
			edges: makeEdges("A", "B", "B", "A"),
			want:  2,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := Depth(tt.nodes, tt.edges)
			if got != tt.want {
				t.Errorf("Depth = %d, want %d", got, tt.want)
			}
		})
	}
}

// ---------------------------------------------------------------------------
// Phase 3 TDD RED (plan DD-14 fix 2): cyclic-diagram ranking.
//
// Mirrors the Rust twin's canonical spec in
// `apps/rhino-cli-rust/src/internal/mermaid/graph.rs`
// (cycle_ranks_as_chain_after_back_edge_removal) — fixtures identical.
// ---------------------------------------------------------------------------

func TestCycleRanksAsChainAfterBackEdgeRemoval(t *testing.T) {
	// Plan DD-14 fix 2: the cyclic diagram `A-->B-->C-->A` must rank as a
	// chain — the back edge (C->A, found via iterative DFS in
	// node-declaration order) is removed, then Kahn longest-path ranking
	// runs on the remaining DAG A->B->C: span 1, depth 3. Today the cycle
	// empties Kahn's queue, every node falls back to rank 0, and the
	// bogus span equals the node count.
	nodes := makeNodes("A", "B", "C")
	edges := makeEdges("A", "B", "B", "C", "C", "A")
	if got := MaxWidth(nodes, edges); got != 1 {
		t.Errorf("MaxWidth = %d, want 1 (cycle must rank as a chain)", got)
	}
	if got := Depth(nodes, edges); got != 3 {
		t.Errorf("Depth = %d, want 3 (cycle must rank as a chain)", got)
	}
}
