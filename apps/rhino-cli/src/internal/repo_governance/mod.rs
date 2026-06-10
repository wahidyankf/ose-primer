//! Validators for governance-layer conventions.
//!
//! The primer repo's Go reference ships two auditors — the vendor-independence scanner and
//! the Gherkin step-keyword cardinality audit — and this module mirrors both. (Upstream
//! `ose-public` carries additional auditors that are intentionally NOT ported here.)

pub mod gherkin_keyword_cardinality;
pub mod vendor_audit;
