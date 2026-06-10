// Command registry for rhino-cli.
//
// Phase 3 adds the `test-coverage` and `spec-coverage` command families. The
// remaining namespaces (agents_*, docs_*, doctor, env_*, git_*, governance_*,
// workflows_*, …) ported from the Go CLI are declared here in later phases.

pub mod agents;
pub mod contracts;
pub mod docs;
pub mod doctor;
pub mod env;
pub mod env_validate;
pub mod git;
pub mod java;
pub mod repo_governance;
pub mod speccoverage;
pub mod testcoverage;
pub mod workflows;
