package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/agents"
)

var emitBindingsDryRun bool

var emitBindingsCmd = &cobra.Command{
	Use:   "emit-bindings",
	Short: "Generate vendor binding files (e.g. .amazonq)",
	Long: `Generate the vendor binding files that bridge external AI coding agents
to the canonical AGENTS.md instructions.

The generated files are listed by ` + "`agents validate-bindings`" + ` and asserted
byte-identical against the expected content. Use --dry-run to preview the
paths that would be written without touching the filesystem.`,
	Example: `  # Write the binding files
  rhino-cli agents emit-bindings

  # Preview without writing
  rhino-cli agents emit-bindings --dry-run`,
	SilenceErrors: true,
	RunE:          runEmitBindings,
}

func init() {
	emitBindingsCmd.Flags().BoolVar(&emitBindingsDryRun, "dry-run", false, "Preview the files that would be written without writing them")
	agentsCmd.AddCommand(emitBindingsCmd)
}

func runEmitBindings(cmd *cobra.Command, _ []string) error {
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	out, err := agents.EmitBindings(repoRoot, emitBindingsDryRun)
	if err != nil {
		return err
	}

	_, _ = fmt.Fprint(cmd.OutOrStdout(), out)
	return nil
}
