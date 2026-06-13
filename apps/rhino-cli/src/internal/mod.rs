// Internal support modules for rhino-cli. Each submodule holds the per-domain
// logic (agents, docs, doctor, env, git, java, mermaid, naming,
// repo-governance, spec-coverage, test-coverage, …) that backs a command
// namespace, kept separate from the thin `commands` adapters.

pub mod agents;
pub mod allowlist;
pub mod bcregistry;
pub mod cliout;
pub mod contracts;
pub mod docs;
pub mod doctor;
pub mod envbackup;
pub mod envvalidate;
pub mod git;
pub mod glossary;
pub mod java;
pub mod mermaid;
pub mod naming;
pub mod repo_governance;
pub mod severity;
pub mod speccoverage;
pub mod specs;
pub mod testcoverage;
