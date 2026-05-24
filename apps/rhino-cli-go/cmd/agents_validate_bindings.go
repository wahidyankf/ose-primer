package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/agents"
)

var validateBindingsCmd = &cobra.Command{
	Use:   "validate-bindings",
	Short: "Validate vendor binding files and catalog coverage",
	Long: `Validate the generated vendor binding files against their expected content
and assert that the platform-bindings catalog documents every binding
directory present on disk.

This command performs two checks:

- binding-parity: each expected binding file (see ` + "`agents emit-bindings`" + `)
  must exist and match its expected bytes; mismatches are reported as DRIFT.
- catalog-coverage: docs/reference/platform-bindings.md must mention every
  binding directory that exists on disk; gaps are reported as MISSING-CATALOG.`,
	Example: `  # Validate bindings
  rhino-cli agents validate-bindings`,
	SilenceErrors: true,
	RunE:          runValidateBindings,
}

func init() {
	agentsCmd.AddCommand(validateBindingsCmd)
}

func runValidateBindings(cmd *cobra.Command, _ []string) error {
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	out, validationErr := agents.ValidateBindings(repoRoot)
	_, _ = fmt.Fprint(cmd.OutOrStdout(), out)
	return validationErr
}
