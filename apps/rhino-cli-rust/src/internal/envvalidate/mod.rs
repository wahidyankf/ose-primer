//! Env variable drift guard subsystem (env validate).
//!
//! Port of `apps/rhino-cli-go/internal/envvalidate/`. Validates each app's
//! declared env vars (infra/dev/<app>/.env.example) against what its source
//! code actually reads, using line-oriented regex extractors per language.

pub mod contract;
pub mod extractor;
pub mod reporter;
pub mod types;
pub mod validator;

pub use contract::{GLOBAL_ALLOWLIST, SURFACES};
pub use reporter::{format_json, format_text};
pub use types::{SurfaceResult, ValidateResult};
pub use validator::{extract_read_keys, parse_declared, validate_surface};
