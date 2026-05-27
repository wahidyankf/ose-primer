# Business Requirements Document — Update Polyglot Toolchain Versions

## Business Goal

Ensure every toolchain version declared in `ose-primer` config files is current, CVE-free, and
more than two months past its release date — and that the `rhino doctor` command accurately
validates those versions on every developer machine.

## Business Rationale

### Pain Points

**Silent Go version check bypass**: Both `rhino doctor` implementations (Go and Rust) reference
`apps/rhino-cli/go.mod` to determine the required Go version. That path does not exist in
`ose-primer` (the file is at `apps/rhino-cli-go/go.mod`). The result is that `rhino doctor`
silently skips the Go version check — developers may be running an unvetted or vulnerable Go
version without any warning.

**Stale toolchain version declarations**: Six config files last updated at project inception
still declare toolchain versions from 2024. In the interim, multiple CVEs have been patched
in Python, .NET, Go, and Rust. A developer following these config files risks running vulnerable
toolchain versions.

**Broken Dart SDK constraint**: The pubspec environment declares `sdk: ^3.11.1` but Dart 3.11.1
was never released — the series goes 3.11.0 → 3.11.3. Any tooling that validates this constraint
against available releases will find it unsatisfiable.

### Business Value

**Security posture**: Updating to CVE-patched toolchain versions eliminates known vulnerabilities
in the declared minimum toolchain for all apps in this repo template. Downstream forks inheriting
from `ose-primer` receive the patched baselines automatically.

**Developer trust in doctor**: A `rhino doctor` that accurately checks all tools — including Go —
gives developers a single reliable command to validate their environment. Fixing the path bug
restores that trust.

**Template correctness**: `ose-primer` is a canonical starting point for OSE-style repositories.
Broken constraints and stale versions are bugs that multiply across every downstream fork.

## Constraints

- All target versions must have a release date on or before 2026-03-27 (two months before
  2026-05-27 plan creation date).
- No target version may carry a known high- or critical-severity CVE.
- Changes are limited to config files and the two doctor source files (path fix only).
  No application logic changes, no new features.
- The Go MSRV directive (`go 1.26.1`) and the Rust MSRV (`rust-version = "1.94.1"`) are
  minimum-version declarations; they do not pin the installed toolchain to an exact version.
  The doctor uses `compareGTE` for both — this is by design.

## Stakeholders

- **Infrastructure maintainer** (sole maintainer): executes the update pass and merges to `main`.
- **Downstream fork authors**: inherit corrected toolchain baselines via `ose-primer`.
- **rhino doctor consumers**: any developer running `npm run doctor` gains accurate Go version
  validation after the path fix.
