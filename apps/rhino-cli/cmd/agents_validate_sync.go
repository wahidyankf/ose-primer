package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/agents"
)

var validateSyncCmd = &cobra.Command{
	Use:   "validate-sync",
	Short: "Validate that .claude/ and .opencode/ are in sync",
	Long: `Validate that .claude/ and .opencode/agents/ configurations are
semantically equivalent. The legacy singular .opencode/agent/ path is
flagged as drift if it reappears.

This command performs the following validations:

Agents:
- Stale-dir check: Asserts legacy singular .opencode/agent/ does not exist
- Count check: OpenCode plural directory contains every Claude agent (⊆)
- Equivalence check: Validates each agent is semantically equivalent:
  * Description matches exactly
  * Model is correctly converted (sonnet/opus/empty → opencode-go/minimax-m2.7, haiku → opencode-go/glm-5)
  * Tools are correctly mapped (array → boolean map, lowercase)
  * Skills array matches exactly
  * Body content is identical

Skills:
- No-mirror check: Asserts no rhino-cli-managed skill copies exist under
  .opencode/skill/ or .opencode/skills/ (OpenCode reads .claude/skills/
  natively per opencode.ai/docs/skills/).`,
	Example: `  # Validate sync
  rhino-cli agents validate-sync

  # Output as JSON
  rhino-cli agents validate-sync -o json

  # Verbose mode (show all checks)
  rhino-cli agents validate-sync -v

  # Quiet mode (show only summary)
  rhino-cli agents validate-sync -q`,
	SilenceErrors: true,
	RunE:          runValidateSync,
}

func init() {
	agentsCmd.AddCommand(validateSyncCmd)
}

func runValidateSync(cmd *cobra.Command, args []string) error {
	// Find git repository root
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	// Perform validation
	result, err := agentsValidateSyncFn(repoRoot)
	if err != nil {
		return fmt.Errorf("validation failed: %w", err)
	}

	// Format and print output
	if err := writeFormatted(cmd, output, verbose, quiet, outputFuncs{
		text:     func(v, q bool) string { return agents.FormatValidationText(result, v, q) },
		json:     func() (string, error) { return agents.FormatValidationJSON(result) },
		markdown: func() string { return agents.FormatValidationMarkdown(result, verbose) },
	}); err != nil {
		return err
	}

	// Return error if validation failed
	if result.FailedChecks > 0 {
		return fmt.Errorf("validation failed: %d checks failed", result.FailedChecks)
	}

	return nil
}
