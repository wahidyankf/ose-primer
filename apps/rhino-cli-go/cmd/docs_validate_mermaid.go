package cmd

import (
	"fmt"
	"math"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/docs"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/mermaid"
)

var (
	validateMermaidStagedOnly       bool
	validateMermaidChangedOnly      bool
	validateMermaidExclude          []string
	validateMermaidMaxLabelLen      int
	validateMermaidMaxWidth         int
	validateMermaidMaxDepth         int
	validateMermaidMaxSubgraphNodes int
)

// docsValidateMermaidFn and readFileFn are declared in testable.go for dependency injection.

var validateMermaidCmd = &cobra.Command{
	Use:   "validate-mermaid",
	Short: "Validate Mermaid flowchart diagrams in markdown files",
	Long: `Scan markdown files and validate Mermaid flowchart diagrams for structural issues.

Three rules are enforced on flowchart and graph blocks:
  1. Node label length must not exceed --max-label-len (default 30)
  2. Max parallel nodes at one rank must not exceed --max-width (default 4)
     Exception: when BOTH span > max-width AND depth > max-depth, emits a
     warning instead of an error (both-exceeded path).
  3. Each mermaid code block must contain exactly one diagram

Non-flowchart Mermaid types (sequenceDiagram, classDiagram, gantt, etc.) are
silently ignored. This command is read-only — it never modifies any file.`,
	Example: `  # Validate all markdown files in default directories
  rhino-cli docs validate-mermaid

  # Validate specific files or directories
  rhino-cli docs validate-mermaid docs/ repo-governance/

  # Only validate files staged in git (pre-commit use)
  rhino-cli docs validate-mermaid --staged-only

  # Only validate files changed since upstream (pre-push use)
  rhino-cli docs validate-mermaid --changed-only

  # Output as JSON
  rhino-cli docs validate-mermaid -o json

  # Set custom thresholds
  rhino-cli docs validate-mermaid --max-label-len 20 --max-width 4`,
	SilenceErrors: true,
	RunE:          runValidateMermaid,
}

func init() {
	docsCmd.AddCommand(validateMermaidCmd)
	validateMermaidCmd.Flags().BoolVar(&validateMermaidStagedOnly, "staged-only", false,
		"only validate staged files (pre-commit use)")
	validateMermaidCmd.Flags().BoolVar(&validateMermaidChangedOnly, "changed-only", false,
		"only validate files changed since upstream (pre-push use)")
	validateMermaidCmd.Flags().StringArrayVar(&validateMermaidExclude, "exclude", nil,
		"path prefixes to exclude from validation (repeatable)")
	validateMermaidCmd.Flags().IntVar(&validateMermaidMaxLabelLen, "max-label-len", 30,
		"max characters in a node label (default 30 ~ Mermaid wrappingWidth:200px at 16px font)")
	validateMermaidCmd.Flags().IntVar(&validateMermaidMaxWidth, "max-width", 4,
		"max nodes at the same rank")
	validateMermaidCmd.Flags().IntVar(&validateMermaidMaxDepth, "max-depth", 0,
		"depth threshold for the both-exceeded warning: when span>max-width AND depth>max-depth, emit warning not error")
	validateMermaidCmd.Flags().IntVar(&validateMermaidMaxSubgraphNodes, "max-subgraph-nodes", 6,
		"max direct child nodes per subgraph; emits a subgraph_density warning when exceeded")
}

func runValidateMermaid(cmd *cobra.Command, args []string) error {
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	// Resolve file list.
	var mdFiles []string
	switch {
	case validateMermaidStagedOnly:
		mdFiles, err = getMermaidStagedFilesFn(repoRoot)
		if err != nil {
			return fmt.Errorf("failed to get staged files: %w", err)
		}
	case validateMermaidChangedOnly:
		mdFiles, err = getMermaidChangedFilesFn(repoRoot)
		if err != nil {
			return fmt.Errorf("failed to get changed files: %w", err)
		}
	case len(args) > 0:
		mdFiles, err = collectMDFiles(repoRoot, args)
		if err != nil {
			return fmt.Errorf("failed to collect files: %w", err)
		}
	default:
		mdFiles, err = collectMDDefaultDirs(repoRoot)
		if err != nil {
			return fmt.Errorf("failed to collect default files: %w", err)
		}
	}

	// Apply `--exclude` prefix filtering to the collected set (plan DD-2).
	mdFiles = filterMermaidExcluded(repoRoot, mdFiles, validateMermaidExclude)

	// Extract and validate blocks.
	var allBlocks []mermaid.MermaidBlock
	fileSet := make(map[string]bool)
	for _, f := range mdFiles {
		content, readErr := readFileFn(f)
		if readErr != nil {
			continue
		}
		blocks := mermaid.ExtractBlocks(f, string(content))
		allBlocks = append(allBlocks, blocks...)
		if len(blocks) > 0 {
			fileSet[f] = true
		}
	}

	if validateMermaidMaxDepth == 0 {
		validateMermaidMaxDepth = math.MaxInt
	}
	opts := mermaid.ValidateOptions{
		MaxLabelLen:      validateMermaidMaxLabelLen,
		MaxWidth:         validateMermaidMaxWidth,
		MaxDepth:         validateMermaidMaxDepth,
		MaxSubgraphNodes: validateMermaidMaxSubgraphNodes,
	}
	result := docsValidateMermaidFn(allBlocks, opts)
	result.FilesScanned = len(fileSet)

	if err := writeFormatted(cmd, output, verbose, quiet, outputFuncs{
		text:     func(v, q bool) string { return mermaid.FormatText(result, v, q) },
		json:     func() (string, error) { return mermaid.FormatJSON(result) },
		markdown: func() string { return mermaid.FormatMarkdown(result) },
	}); err != nil {
		return err
	}

	if len(result.Violations) > 0 {
		return fmt.Errorf("found %d violation(s)", len(result.Violations))
	}
	return nil
}

// getMermaidStagedFiles returns *.md files staged in git.
func getMermaidStagedFiles(repoRoot string) ([]string, error) {
	out, err := exec.Command("git", "-C", repoRoot, "diff", "--cached", "--name-only", "--diff-filter=ACMR").Output()
	if err != nil {
		return nil, err
	}
	return filterMDPaths(repoRoot, strings.Split(strings.TrimSpace(string(out)), "\n")), nil
}

// getMermaidChangedFiles returns *.md files changed since upstream (@{u}..HEAD).
func getMermaidChangedFiles(repoRoot string) ([]string, error) {
	out, err := exec.Command("git", "-C", repoRoot, "diff", "--name-only", "@{u}..HEAD").Output()
	if err != nil {
		// No upstream: fall back to default scan.
		return collectMDDefaultDirs(repoRoot)
	}
	files := filterMDPaths(repoRoot, strings.Split(strings.TrimSpace(string(out)), "\n"))
	if len(files) == 0 {
		return collectMDDefaultDirs(repoRoot)
	}
	return files, nil
}

// filterMDPaths converts relative paths to absolute and keeps only *.md files.
func filterMDPaths(repoRoot string, relPaths []string) []string {
	var out []string
	for _, p := range relPaths {
		if p == "" {
			continue
		}
		if !strings.HasSuffix(p, ".md") {
			continue
		}
		abs := filepath.Join(repoRoot, p)
		out = append(out, abs)
	}
	return out
}

// collectMDFiles walks given paths (files or directories) and collects *.md
// files. Delegates to the links scanner's walker (docs.GetAllMarkdownFiles) —
// the single noise-skipping walk definition per CLI (plan DD-3). A file path
// yields itself at depth 0 (never filtered), matching the previous
// per-command walker byte-for-byte. Mirrors Rust `collect_md_files`.
func collectMDFiles(repoRoot string, paths []string) ([]string, error) {
	var files []string
	for _, p := range paths {
		abs := p
		if !filepath.IsAbs(p) {
			abs = filepath.Join(repoRoot, p)
		}
		walked, err := docs.GetAllMarkdownFiles(abs)
		if err != nil {
			return nil, err
		}
		files = append(files, walked...)
	}
	return files, nil
}

// collectMDDefaultDirs scans the whole repository for *.md files (plan DD-3):
// a repo-wide walk skipping the standardized noise-skip set by directory
// name, replacing the historical four-dir default
// (docs/repo-governance/.claude/plans) plus root glob. Delegates to
// docs.GetAllMarkdownFiles, the one walker per CLI. Mirrors Rust
// `collect_md_default_dirs`.
func collectMDDefaultDirs(repoRoot string) ([]string, error) {
	return docs.GetAllMarkdownFiles(repoRoot)
}

// filterMermaidExcluded applies `--exclude` prefix semantics to the collected
// mermaid file list (plan DD-2): drops files whose repo-root-relative path
// starts with any excluded prefix (raw or trailing-slash-cleaned). Delegates
// to the links walker's FilterSkipPaths so both gates share one prefix
// implementation per CLI. Mirrors Rust `filter_mermaid_excluded`.
func filterMermaidExcluded(repoRoot string, files []string, exclude []string) []string {
	return docs.FilterSkipPaths(files, repoRoot, exclude)
}
