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

## Affected Roles

- **Infrastructure maintainer** (sole maintainer): executes the update pass and merges to `main`.
- **Downstream fork authors**: inherit corrected toolchain baselines via `ose-primer`.
- **rhino doctor consumers**: any developer running `npm run doctor` gains accurate Go version
  validation after the path fix.

## Business-Level Success Metrics

_Judgment call: no pre-existing baseline metrics exist; these are observable facts derivable
from repo state after the plan completes._

- All six toolchain config files declare the safe target version (verifiable via `grep` against
  each config file — see delivery.md Phase 2 acceptance criteria).
- Both doctor implementations reference `apps/rhino-cli-go/go.mod` (verifiable via
  `grep -rn "rhino-cli/go.mod" apps/rhino-cli-go/ apps/rhino-cli-rust/` returning zero matches).
- All quality gates pass after the changes: typecheck, lint, test:quick, lint:md,
  validate:harness-bindings, validate:config all exit 0.

## Non-Goals

At the business level, this maintenance pass explicitly does **not**:

- Guarantee that downstream forks of `ose-primer` have applied these updates — each fork
  must propagate changes independently.
- Address application-level dependency CVEs (npm packages, Go module deps, Dart pub packages)
  — those are covered by separate dependency-audit plans.
- Rebuild or re-release the `rhino-cli` binaries — the path fix applies to source only;
  consumers of prebuilt binaries require a separate release workflow.
- Eliminate all possible toolchain vulnerabilities — only the six config-file-pinned tools
  are in scope; tools managed by `noReq` or `compareGTE` without config pins are out of scope.

## Business Risks

- **CI regression in downstream forks**: A version bump (especially Rust MSRV 1.80 → 1.94.1)
  may break downstream forks whose code relies on pre-1.94 semantics. _Mitigation_: the MSRV
  change applies to `ose-primer` only; downstream forks must opt in by rebasing.
- **Dart pubspec resolution surprise**: Lowering the Dart SDK minimum from `^3.11.1` to
  `^3.11.0` could allow `pub get` to select 3.11.0 on a machine that has only 3.11.0 installed.
  3.11.0 and 3.11.3 are both stable; no behavior regression is anticipated. _Mitigation_:
  the quality gate (`npx nx affected -t lint`) will catch any Dart analysis failures.
- **Flutter floor tightening breaks developer machines**: Raising the Flutter minimum from
  `>=3.41.0` to `>=3.41.4` will fail doctor for any developer still running 3.41.0–3.41.3.
  _Mitigation_: `rhino doctor` will surface the mismatch with a clear message; the fix is
  `flutter upgrade`.
