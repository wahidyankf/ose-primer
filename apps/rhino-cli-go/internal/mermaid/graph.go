package mermaid

// DFS colors for back-edge detection.
const (
	dfsWhite = 0
	dfsGray  = 1
	dfsBlack = 2
)

// indexedEdge carries an edge's index in the caller's edge slice alongside
// its target, so detected back edges can be excluded from ranking.
type indexedEdge struct {
	idx int
	to  string
}

// findBackEdges detects back edges (edges pointing at a node currently on the
// DFS stack) via iterative DFS, visiting roots in node-declaration order and
// children in edge-declaration order (plan DD-14 fix 2). Returns the indices
// of back edges in the caller's edge slice. Mirrors Rust `find_back_edges`.
func findBackEdges(nodes []Node, adj map[string][]indexedEdge) map[int]bool {
	color := make(map[string]int, len(nodes))
	for _, n := range nodes {
		color[n.ID] = dfsWhite
	}

	back := make(map[int]bool)
	type frame struct {
		node   string
		cursor int
	}
	for _, n := range nodes {
		if color[n.ID] != dfsWhite {
			continue
		}
		// Stack of (node, next-child cursor) frames.
		stack := []frame{{node: n.ID, cursor: 0}}
		color[n.ID] = dfsGray
		for len(stack) > 0 {
			top := &stack[len(stack)-1]
			children := adj[top.node]
			if top.cursor < len(children) {
				child := children[top.cursor]
				top.cursor++
				switch color[child.to] {
				case dfsGray:
					// Child is on the active DFS path → cycle-closing edge.
					back[child.idx] = true
				case dfsWhite:
					color[child.to] = dfsGray
					stack = append(stack, frame{node: child.to, cursor: 0})
				default:
					// dfsBlack: forward/cross edge — keep it.
				}
			} else {
				color[top.node] = dfsBlack
				stack = stack[:len(stack)-1]
			}
		}
	}
	return back
}

// rankAssign runs Kahn's BFS longest-path rank assignment on the DAG left
// after back-edge removal. Returns map[nodeID]rank.
//
// Plan DD-14 fix 2: back edges are detected via iterative DFS in
// node-declaration order and removed before ranking, so cyclic diagrams rank
// as chains. Previously a cycle emptied Kahn's queue, every node fell back to
// rank 0, and the bogus span equaled the node count. Mirrors Rust `rank_assign`.
func rankAssign(nodes []Node, edges []Edge) map[string]int {
	if len(nodes) == 0 {
		return map[string]int{}
	}

	// Build index of node IDs to detect which IDs are valid.
	nodeSet := make(map[string]bool, len(nodes))
	for _, n := range nodes {
		nodeSet[n.ID] = true
	}

	// Adjacency over valid edges only, carrying each edge's index so back
	// edges can be excluded from the ranking pass below.
	adjIdx := make(map[string][]indexedEdge, len(nodes))
	for _, n := range nodes {
		adjIdx[n.ID] = nil
	}
	for i, e := range edges {
		if nodeSet[e.From] && nodeSet[e.To] {
			adjIdx[e.From] = append(adjIdx[e.From], indexedEdge{idx: i, to: e.To})
		}
	}

	backEdges := findBackEdges(nodes, adjIdx)

	// Adjacency (out-edges) and in-degree for valid nodes only, skipping the
	// detected back edges.
	adj := make(map[string][]string, len(nodes))
	inDegree := make(map[string]int, len(nodes))
	for _, n := range nodes {
		adj[n.ID] = nil
		inDegree[n.ID] = 0
	}
	for i, e := range edges {
		if backEdges[i] {
			continue
		}
		if nodeSet[e.From] && nodeSet[e.To] {
			adj[e.From] = append(adj[e.From], e.To)
			inDegree[e.To]++
		}
	}

	rank := make(map[string]int, len(nodes))
	visited := make(map[string]bool, len(nodes))

	// Kahn's BFS: start with all nodes that have in-degree 0.
	queue := make([]string, 0, len(nodes))
	for _, n := range nodes {
		if inDegree[n.ID] == 0 {
			queue = append(queue, n.ID)
			rank[n.ID] = 0
		}
	}

	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		visited[cur] = true

		for _, next := range adj[cur] {
			// Longest-path rank: update if going through cur gives a longer path.
			if rank[cur]+1 > rank[next] {
				rank[next] = rank[cur] + 1
			}
			inDegree[next]--
			if inDegree[next] == 0 {
				queue = append(queue, next)
			}
		}
	}

	// Safety fallback: rank 0 for any node the BFS did not reach. With back
	// edges removed the remaining graph is a DAG and Kahn visits everything,
	// so this is defensive only.
	for _, n := range nodes {
		if !visited[n.ID] {
			rank[n.ID] = 0
		}
	}

	return rank
}

// MaxWidth returns the maximum number of nodes sharing the same rank.
// Returns 0 for empty graphs.
func MaxWidth(nodes []Node, edges []Edge) int {
	if len(nodes) == 0 {
		return 0
	}

	ranks := rankAssign(nodes, edges)

	// Count nodes per rank.
	rankCount := make(map[int]int)
	for _, r := range ranks {
		rankCount[r]++
	}

	maxW := 0
	for _, cnt := range rankCount {
		if cnt > maxW {
			maxW = cnt
		}
	}
	return maxW
}

// Depth returns the number of distinct rank values (longest path length + 1).
// Returns 0 for empty graphs.
func Depth(nodes []Node, edges []Edge) int {
	if len(nodes) == 0 {
		return 0
	}

	ranks := rankAssign(nodes, edges)

	distinct := make(map[int]bool)
	for _, r := range ranks {
		distinct[r] = true
	}
	return len(distinct)
}
