//! Agent configuration management: Claude ↔ OpenCode conversion, sync
//! orchestration, and the validation suites (`validate-claude`,
//! `validate-sync`, `validate-naming`).

pub mod agent_validator;
pub mod bindings;
pub mod claude_validator;
pub mod converter;
pub mod frontmatter;
pub mod naming;
pub mod reporter;
pub mod skill_validator;
pub mod sync;
pub mod sync_validator;
pub mod types;
pub mod yaml_formatting;
