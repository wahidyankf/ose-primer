//! Mermaid validation data types. Mirrors Go `types.go`.

/// Layout direction of a Mermaid flowchart. Mirrors Go `Direction`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Tb,
    Td,
    Bt,
    Lr,
    Rl,
}

impl Direction {
    /// Parses a direction token; defaults to `Tb` for unknown/empty.
    pub fn parse(s: &str) -> Self {
        match s.trim() {
            "TD" => Direction::Td,
            "BT" => Direction::Bt,
            "LR" => Direction::Lr,
            "RL" => Direction::Rl,
            _ => Direction::Tb,
        }
    }
}

/// Category of a blocking rule violation. Mirrors Go `ViolationKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    LabelTooLong,
    WidthExceeded,
    MultipleDiagrams,
}

impl ViolationKind {
    /// Wire string (used in text/json/markdown output). Mirrors Go constants.
    pub fn as_str(self) -> &'static str {
        match self {
            ViolationKind::LabelTooLong => "label_too_long",
            ViolationKind::WidthExceeded => "width_exceeded",
            ViolationKind::MultipleDiagrams => "multiple_diagrams",
        }
    }
}

/// Category of a non-blocking warning. Mirrors Go `WarningKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningKind {
    ComplexDiagram,
    SubgraphDense,
}

impl WarningKind {
    pub fn as_str(self) -> &'static str {
        match self {
            WarningKind::ComplexDiagram => "complex_diagram",
            WarningKind::SubgraphDense => "subgraph_density",
        }
    }
}

/// Raw source of a single ```` ```mermaid ```` fenced code block. Mirrors Go `MermaidBlock`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MermaidBlock {
    pub file_path: String,
    pub block_index: usize,
    pub source: String,
    /// 1-based line number of the opening fence.
    pub start_line: usize,
}

/// A flowchart node with an ID and optional label. Mirrors Go `Node`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub label: String,
}

/// A directed connection between two nodes. Mirrors Go `Edge`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
}

/// A Mermaid `subgraph ... end` block. `node_ids` holds direct children only.
/// `start_line` is 1-indexed within the parent block. Mirrors Go `Subgraph`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subgraph {
    pub id: String,
    pub label: String,
    pub node_ids: Vec<String>,
    pub start_line: usize,
}

/// Result of parsing a single `MermaidBlock`. Mirrors Go `ParsedDiagram`.
#[derive(Debug, Clone)]
pub struct ParsedDiagram {
    pub block: MermaidBlock,
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub subgraphs: Vec<Subgraph>,
}

/// A non-blocking finding (exit 0). Mirrors Go `Warning`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    pub kind: WarningKind,
    pub file_path: String,
    pub block_index: usize,
    pub start_line: usize,

    // complex_diagram fields.
    pub actual_width: i64,
    pub actual_depth: i64,
    pub max_width: i64,
    pub max_depth: i64,

    // subgraph_density fields.
    pub subgraph_label: String,
    pub subgraph_node_count: usize,
    pub max_subgraph_nodes: i64,
}

/// A blocking rule violation (non-zero exit). Mirrors Go `Violation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub kind: ViolationKind,
    pub file_path: String,
    pub block_index: usize,
    pub start_line: usize,
    pub node_id: String,
    pub label_text: String,
    pub label_len: usize,
    pub max_label_len: i64,
    pub actual_width: i64,
    pub max_width: i64,
}

/// Aggregate findings of a validation run. Mirrors Go `ValidationResult`.
#[derive(Debug, Clone, Default)]
pub struct ValidationResult {
    pub files_scanned: usize,
    pub blocks_scanned: usize,
    pub violations: Vec<Violation>,
    pub warnings: Vec<Warning>,
}
