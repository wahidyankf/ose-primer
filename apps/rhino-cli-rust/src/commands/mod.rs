// Command registry for rhino-cli. Each submodule backs one command family
// (agents, contracts, docs, doctor, env, git, java, repo-governance,
// spec-coverage, test-coverage, workflows).

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
