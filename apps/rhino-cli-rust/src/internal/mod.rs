// Internal support modules for rhino-cli.
//
// Phase 3 adds the test-coverage and spec-coverage subsystems plus the minimal
// git-root helper they depend on. Later phases add the remaining per-domain
// helpers (agents, docs, doctor, …) that back each command namespace.

pub mod agents;
pub mod cliout;
pub mod contracts;
pub mod docs;
pub mod git;
pub mod java;
pub mod mermaid;
pub mod naming;
pub mod repo_governance;
pub mod speccoverage;
pub mod testcoverage;
