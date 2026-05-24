//! Validators for governance-layer conventions.
//!
//! Byte-for-byte port of `apps/rhino-cli-go/internal/repo-governance/`. The
//! primer repo's Go reference ships a single auditor — the vendor-independence
//! scanner — so this module mirrors only that. (Upstream `ose-public` carries
//! additional auditors that are intentionally NOT ported here.)

pub mod vendor_audit;
