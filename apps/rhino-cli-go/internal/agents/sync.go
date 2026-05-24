// Package agents provides agent configuration management across Claude Code and OpenCode,
// including format validation, sync orchestration, and conversion.
package agents

import (
	"time"
)

// SyncAll performs the complete sync operation. Skills are no longer
// copied: OpenCode reads .claude/skills/<name>/SKILL.md natively per
// opencode.ai/docs/skills/. SkillsOnly is now a no-op flag (kept for
// CLI back-compat; documented in agents_sync.go long-help).
func SyncAll(opts SyncOptions) (*SyncResult, error) {
	startTime := time.Now()
	result := &SyncResult{
		FailedFiles: []string{},
	}

	// Sync agents (unless skills-only — which now produces an empty result)
	if !opts.SkillsOnly {
		agentsConverted, agentsFailed, agentFailedFiles, err := ConvertAllAgents(opts.RepoRoot, opts.DryRun)
		if err != nil {
			return nil, err
		}
		result.AgentsConverted = agentsConverted
		result.AgentsFailed = agentsFailed
		result.FailedFiles = append(result.FailedFiles, agentFailedFiles...)
	}

	result.Duration = time.Since(startTime)

	return result, nil
}
