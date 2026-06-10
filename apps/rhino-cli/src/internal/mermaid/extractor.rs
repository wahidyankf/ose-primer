//! Mermaid fenced-code-block extraction.

use super::types::MermaidBlock;

/// Scans markdown content line-by-line and returns all mermaid fenced code
/// blocks. `start_line` is the 1-based line number of the opening fence.
pub fn extract_blocks(file_path: &str, content: &str) -> Vec<MermaidBlock> {
    let mut blocks = Vec::new();
    // Go uses strings.Split(content, "\n"); replicate exactly (no trailing-empty trimming).
    let lines: Vec<&str> = content.split('\n').collect();

    let mut in_block = false;
    let mut source_lines: Vec<&str> = Vec::new();
    let mut block_index = 0usize;
    let mut start_line = 0usize;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !in_block {
            // Accept ```mermaid or ~~~mermaid as opening fences.
            if line.starts_with("```mermaid") || line.starts_with("~~~mermaid") {
                in_block = true;
                source_lines.clear();
                start_line = i + 1; // convert to 1-based
            }
        } else if trimmed == "```" || trimmed == "~~~" {
            // Closing fence.
            blocks.push(MermaidBlock {
                file_path: file_path.to_string(),
                block_index,
                source: source_lines.join("\n"),
                start_line,
            });
            block_index += 1;
            in_block = false;
        } else {
            source_lines.push(line);
        }
    }

    blocks
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn extracts_single_block() {
        let content = "# Title\n```mermaid\nflowchart TD\n  A --> B\n```\n";
        let blocks = extract_blocks("f.md", content);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].block_index, 0);
        assert_eq!(blocks[0].start_line, 2);
        assert_eq!(blocks[0].source, "flowchart TD\n  A --> B");
        assert_eq!(blocks[0].file_path, "f.md");
    }

    #[test]
    fn extracts_multiple_blocks_with_indices() {
        let content = "```mermaid\nflowchart TD\n```\ntext\n```mermaid\ngraph LR\n```\n";
        let blocks = extract_blocks("f.md", content);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0].block_index, 0);
        assert_eq!(blocks[1].block_index, 1);
        assert_eq!(blocks[1].start_line, 5);
    }

    #[test]
    fn accepts_tilde_fences() {
        let content = "~~~mermaid\nflowchart TD\n~~~\n";
        let blocks = extract_blocks("f.md", content);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].source, "flowchart TD");
    }

    #[test]
    fn no_blocks_when_absent() {
        assert!(extract_blocks("f.md", "# Just text\nno mermaid here\n").is_empty());
    }
}
