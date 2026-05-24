//! Broken-link categorization. Mirrors Go `links_categorizer.go`.

/// Categorizes a broken link by pattern. Checks patterns in order
/// (most specific first), matching the Go `CategorizeBrokenLink`.
pub fn categorize_broken_link(link: &str) -> String {
    // workflows/ paths (but not repo-governance/workflows/)
    if link.contains("workflows/") && !link.contains("repo-governance/workflows/") {
        return "workflows/ paths".to_string();
    }

    // vision/ paths (but not repo-governance/vision/)
    if link.contains("vision/") && !link.contains("repo-governance/vision/") {
        return "vision/ paths".to_string();
    }

    // conventions README
    if link.contains("conventions/README.md") {
        return "conventions README".to_string();
    }

    // Missing files
    if link == "CODE_OF_CONDUCT.md" || link == "CHANGELOG.md" {
        return "Missing files".to_string();
    }

    // Default category
    "General/other paths".to_string()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn workflows_paths() {
        assert_eq!(
            categorize_broken_link("workflows/foo.md"),
            "workflows/ paths"
        );
        assert_eq!(
            categorize_broken_link("a/workflows/b.md"),
            "workflows/ paths"
        );
    }

    #[test]
    fn repo_governance_workflows_is_general() {
        assert_eq!(
            categorize_broken_link("repo-governance/workflows/b.md"),
            "General/other paths"
        );
    }

    #[test]
    fn vision_paths() {
        assert_eq!(categorize_broken_link("vision/x.md"), "vision/ paths");
        assert_eq!(
            categorize_broken_link("repo-governance/vision/x.md"),
            "General/other paths"
        );
    }

    #[test]
    fn conventions_readme() {
        assert_eq!(
            categorize_broken_link("a/conventions/README.md"),
            "conventions README"
        );
    }

    #[test]
    fn missing_files() {
        assert_eq!(
            categorize_broken_link("CODE_OF_CONDUCT.md"),
            "Missing files"
        );
        assert_eq!(categorize_broken_link("CHANGELOG.md"), "Missing files");
    }

    #[test]
    fn default_general() {
        assert_eq!(
            categorize_broken_link("./does-not-exist.md"),
            "General/other paths"
        );
    }
}
