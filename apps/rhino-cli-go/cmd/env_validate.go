package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/wahidyankf/ose-public/apps/rhino-cli/internal/envvalidate"
)

var envValidateCmd = &cobra.Command{
	Use:   "validate",
	Short: "Validate .env.example declarations against source reads",
	Long: `Compare each app's declared env vars (infra/dev/<app>/.env.example)
against what its source code actually reads, using line-oriented regex
extractors per language. Exits non-zero when drift is detected.`,
	SilenceErrors: true,
	SilenceUsage:  false,
	RunE:          runEnvValidate,
}

func init() {
	envCmd.AddCommand(envValidateCmd)
}

func runEnvValidate(cmd *cobra.Command, _ []string) error {
	repoRoot, err := findGitRoot()
	if err != nil {
		return fmt.Errorf("failed to find git repository root: %w", err)
	}

	outputFmt, _ := cmd.Flags().GetString("output")

	var surfaces []*envvalidate.SurfaceResult
	for _, surface := range envvalidate.Surfaces {
		sr, validateErr := envvalidate.ValidateSurface(repoRoot, surface)
		if validateErr != nil {
			return fmt.Errorf("env validate failed for %s: %w", surface.App, validateErr)
		}
		surfaces = append(surfaces, sr)
	}

	result := &envvalidate.ValidateResult{Surfaces: surfaces}

	var out string
	if outputFmt == "json" {
		out, err = envvalidate.FormatJSON(result)
		if err != nil {
			return fmt.Errorf("failed to format JSON: %w", err)
		}
	} else {
		out = envvalidate.FormatText(result)
	}

	if _, printErr := fmt.Fprint(cmd.OutOrStdout(), out); printErr != nil {
		return fmt.Errorf("failed to write output: %w", printErr)
	}

	if !result.IsOK() {
		return fmt.Errorf("env validate found violations")
	}
	return nil
}
