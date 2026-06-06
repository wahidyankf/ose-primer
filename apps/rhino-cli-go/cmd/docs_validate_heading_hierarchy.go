package cmd

import (
	"fmt"
	"path/filepath"
	"strings"

	"github.com/spf13/cobra"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/docs"
)

var (
	validateHeadingHierarchyStagedOnly bool
	validateHeadingHierarchyExclude    []string
)

var validateHeadingHierarchyCmd = &cobra.Command{
	Use:   "validate-heading-hierarchy",
	Short: "Validate markdown heading hierarchy in prose files",
	Long: `Scan prose markdown files for heading hierarchy violations.

Three finding kinds are reported:
  1. missing-h1   — the file has zero H1 headings (reported at line 1)
  2. duplicate-h1 — every H1 after the first in a file
  3. skipped-level — a heading level jumps by more than one (e.g. H1 → H3)

File selection is allowlist default-deny: only prose trees (docs/,
repo-governance/, specs/, plans/ minus plans/done/, root-level *.md,
apps|libs/<name>/README.md, and apps|libs/<name>/docs/**) are ever scanned,
so prompt/skill artifacts (.claude/**, .opencode/**, deep apps/ and libs/
internals) can never trip a finding. Headings inside code fences are ignored.`,
	Example: `  # Validate heading hierarchy in all prose markdown files
  rhino-cli docs validate-heading-hierarchy

  # Validate specific files or directories (prose allowlist still applies)
  rhino-cli docs validate-heading-hierarchy docs/ repo-governance/

  # Validate only staged files (useful in pre-commit hooks)
  rhino-cli docs validate-heading-hierarchy --staged-only

  # Exclude a tree on top of the prose allowlist
  rhino-cli docs validate-heading-hierarchy --exclude docs

  # Output as JSON
  rhino-cli docs validate-heading-hierarchy -o json`,
	SilenceErrors: true, // We handle error messages ourselves
	RunE:          runValidateHeadingHierarchy,
}

func init() {
	docsCmd.AddCommand(validateHeadingHierarchyCmd)
	validateHeadingHierarchyCmd.Flags().BoolVar(&validateHeadingHierarchyStagedOnly, "staged-only", false,
		"only validate staged files")
	validateHeadingHierarchyCmd.Flags().StringArrayVar(&validateHeadingHierarchyExclude, "exclude", nil,
		"path prefixes to exclude from validation (repeatable)")
}

func runValidateHeadingHierarchy(cmd *cobra.Command, args []string) error {
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	// Resolve the candidate paths. The prose allowlist is applied inside the
	// validator's file selection (plan DD-7), so staged and positional inputs
	// can never trip a finding on default-denied trees.
	paths := args
	if validateHeadingHierarchyStagedOnly {
		staged, stagedErr := getMermaidStagedFilesFn(repoRoot)
		if stagedErr != nil {
			return fmt.Errorf("failed to get staged files: %w", stagedErr)
		}
		rels := make([]string, 0, len(staged))
		for _, p := range staged {
			rel, relErr := filepath.Rel(repoRoot, p)
			if relErr != nil || rel == ".." || strings.HasPrefix(rel, "../") {
				continue
			}
			rels = append(rels, rel)
		}
		if len(rels) == 0 {
			// Nothing staged: report success without falling back to a full scan.
			return writeHeadingReport(cmd, nil)
		}
		paths = rels
	}

	opts := docs.HeadingScanOptions{
		Root:    repoRoot,
		Paths:   paths,
		Exclude: validateHeadingHierarchyExclude,
	}
	findings, err := docsValidateHeadingHierarchyFn(opts)
	if err != nil {
		return fmt.Errorf("validation failed: %w", err)
	}

	if err := writeHeadingReport(cmd, findings); err != nil {
		return err
	}

	// Return error if findings found (Cobra will set exit code 1)
	if len(findings) > 0 {
		if !quiet && output == "text" {
			_, _ = fmt.Fprintf(cmd.OutOrStderr(), "\n❌ Found %d heading hierarchy finding(s)\n", len(findings))
		}
		return fmt.Errorf("found %d heading hierarchy finding(s)", len(findings))
	}

	return nil
}

// writeHeadingReport formats and prints the heading-hierarchy report in the
// requested global output format.
func writeHeadingReport(cmd *cobra.Command, findings []docs.HeadingFinding) error {
	return writeFormatted(cmd, output, verbose, quiet, outputFuncs{
		text:     func(_, q bool) string { return docs.FormatHeadingText(findings, q) },
		json:     func() (string, error) { return docs.FormatHeadingJSON(findings) },
		markdown: func() string { return docs.FormatHeadingMarkdown(findings) },
	})
}
