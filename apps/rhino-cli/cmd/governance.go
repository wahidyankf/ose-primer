package cmd

import "github.com/spf13/cobra"

var repoGovernanceCmd = &cobra.Command{
	Use:   "repo-governance",
	Short: "Repository governance validation commands",
	Long:  `Commands for validating repository governance layer conventions.`,
}

func init() {
	rootCmd.AddCommand(repoGovernanceCmd)
}
