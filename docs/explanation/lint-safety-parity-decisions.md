# Lint and Safety Parity — Design Decisions

Plain-language explanation of every decision in the cross-repo deviation matrix
for the "lint-safety-parity" effort, as executed in `ose-primer`. Follows the
`*-parity-decisions.md` precedents in this directory.

`ose-public` is the **canonical reference** for this parity set. Each dimension
below is marked with whether `ose-primer` acted on it; every divergence is
intentional and recorded, with zero silent deviations. The full plan lives in
[`plans/done/`](../../plans/done/) under the `lint-safety-parity` folder.

This effort strengthens static-analysis gates across six dimensions and wires
each into the same three enforcement surfaces that already gate Prettier and
markdownlint: the local **pre-push hook**, the **CI quality gate** (on pull
requests), and the **Nx target graph**. The locked gating policy is **"error
threshold = fail on warning-and-above"** (`shellcheck --severity=warning`,
`hadolint --failure-threshold warning`), applied with a **clean-then-gate** TDD
rhythm: surface the existing backlog (RED), clean it (GREEN), then enable the
gate (REFACTOR).

---

## D1 — Rust `forbid(unsafe_code)` + full public `[lints]` standard — **executed**

**Decision**: Aligned `apps/crud-be-rust-axum` to `ose-public`'s verbatim
`[lints.rust]` + `[lints.clippy]` standard, including `unsafe_code = "forbid"`,
`pedantic`/`nursery` at warn, and `unwrap_used`/`expect_used`/`panic`/
`undocumented_unsafe_blocks` at deny. The crate's `lint` Nx target was escalated
from `cargo clippy -- -D warnings` to `cargo clippy --all-targets -- -D warnings`
so the warn-level pedantic/nursery groups become deny on the full target set.

**Rationale**: `forbid(unsafe_code)` is a compile-enforced safety guarantee — no
`unsafe` block can ever compile in this crate. The clippy standard matches the
reference repo exactly so the three OSE repos share one Rust quality bar.

**What this required**: cleaning 121 latent production-code violations (casts via
`try_from`, documented helpers, `.expect()` → `anyhow::Context`, mechanical
clippy `--fix` lints). Test-only code carries minimal scoped
`#![allow(clippy::unwrap_used, expect_used, panic, …)]` inside `#[cfg(test)]`
modules and the BDD harness crates so `--all-targets` does not weaken the
crate-wide production denies.

**Safety proof**: a temporary `unsafe { /* noop */ }` was injected and the build
failed with `error: usage of an unsafe block … requested on the command line
with -F unsafe-code`, then removed (RED→GREEN).

---

## D1b — Rust 2024 `env::set_var`/`remove_var` unsafe in tests — **skipped (infra-only)**

**Decision**: No action in `ose-primer`.

**Rationale**: In Rust 2024, `std::env::set_var`/`remove_var` became `unsafe`
(POSIX `setenv` is not thread-safe). The refactor to inject `Config` directly
instead of mutating process-env applies only to the infra repo's
`coralpolyp-be` test module. `ose-primer`'s `crud-be-rust-axum` has **no
handwritten `unsafe`** (verified: `grep -rn "unsafe" apps/crud-be-rust-axum/src`
returns nothing), so `forbid(unsafe_code)` needed no test refactor here.

---

## D2 — F# strict stack — **skipped (reference)**

**Decision**: No action.

**Rationale**: `ose-primer`'s two F# projects already carry the target strict
stack; for this dimension `ose-primer` **is** the reference that `ose-public`
aligns up to. Nothing to change downstream.

---

## D3 — C# strict baseline (`AnalysisLevel=latest-All` + Sonar enforced) — **executed**

**Decision**: Added `<AnalysisLevel>latest-All</AnalysisLevel>` to
`apps/crud-be-csharp-aspnetcore/Directory.Build.props` (joining the existing
`TreatWarningsAsErrors=true` and `EnforceCodeStyleInBuild=true`) and raised
`SonarAnalyzer.CSharp` rule severities to error in the project `.editorconfig`.

**Rationale**: `latest-All` enables the full set of .NET code-quality (CA) rules,
and enforcing Sonar at error severity (combined with treat-warnings-as-errors)
makes the analyzer backlog build-breaking. This is the strictest standard .NET
offers and matches the cross-repo C# bar.

**What this required**: cleaning the `latest-All` + Sonar backlog across `src`
and `tests`, with narrow, justified suppressions only where a rule is genuinely
inapplicable to demo code. A Sonar proof (an unused private field) was injected
to confirm the gate fails, then removed (RED→GREEN).

---

## D4 — Python strict (basedpyright strict + expanded ruff) — **executed**

**Decision**: Swapped the type-checker from `pyright` to `basedpyright` with
`typeCheckingMode = "strict"`, and expanded the ruff `select` to
`E,W,F,B,UP,SIM,I,N,S,RUF,C4,T20,ANN` (excluding the deprecated `ANN101`/`ANN102`),
with `per-file-ignores` exempting tests from `S101` (assert) and `ANN`.

**Rationale**: `basedpyright` is a stricter `pyright` fork; strict mode plus the
broad ruff rule set catches type, security (`S`), and modernization (`UP`) issues
the previous `basic`/narrow config missed.

**What this required**: resolving 1037 strict type errors and 36 ruff violations.
`src` stays fully strict; a `tests`-scoped `executionEnvironments` block relaxes
only the dynamic-JSON `reportUnknown*` family for BDD glue, and a `py.typed`
marker makes the package resolve as typed. The `typecheck` Nx target now invokes
`basedpyright`.

---

## ~~D5~~ — TypeScript DDD import-boundaries — **dropped (deferred)**

**Decision**: Dropped from this effort entirely; deferred to a dedicated future
plan. No `ose-primer` action beyond recording the deferral here.

**Rationale**: TypeScript DDD import-boundary enforcement is too
language-divergent to fold into this cross-language strictness pass, and getting
it right deserves its own plan.

### Exemption philosophy (recorded per the plan)

DDD import-boundary enforcement, when it lands, targets **business-domain
backends only** — the apps that actually model a bounded domain. **Demo,
content, and frontend apps are exempt**: their structure is presentational or
illustrative, not domain-driven, so imposing DDD layering boundaries on them
would add ceremony without protecting a real domain model. This keeps the future
gate proportionate: strict where a domain exists, silent where one does not.

---

## D6 — Dockerfile lint (hadolint) — **executed**

**Decision**: Added a root `.hadolint.yaml` (`failure-threshold: warning`;
`trustedRegistries: docker.io, ghcr.io, mcr.microsoft.com`) and gated all 30
real Dockerfiles. Wired via a new `rhino-cli:lint:dockerfiles` Nx target, a CI
`hadolint` job, and a scoped pre-push hook branch.

**Rationale**: hadolint catches Dockerfile correctness and best-practice issues
(registry trust, shell-form `CMD`, cache bloat). `mcr.microsoft.com` is trusted
because the .NET (C#/F#) images come from the official Microsoft Artifact
Registry.

**Justified ignores** (deliberate for **demo / development / integration**
images, which are not production artifacts):

- **DL3018 / DL3008 / DL3013** — version-pinning of `apk`/`apt`/`pip` packages.
  Pinning is brittle and high-churn for demo images; intentionally unpinned.
- **DL3007** — `latest` tag. Integration-test builder stages intentionally track
  the latest language toolchain.
- **DL3003** — `cd` inside a `RUN`. A few multi-step build `RUN`s use `cd`
  deliberately for a single transient step where a `WORKDIR` would leak.

**Real fixes kept enforced** (so the gate stays meaningful): DL3042
(`pip install --no-cache-dir`) and DL3025 (JSON-array `CMD`). Vendored Dockerfiles
under `deps/`, `_build/`, `.venv/`, `target/`, `archived/`, and `node_modules/`
are excluded from the gate.

---

## D7 — Shell lint (shellcheck) — **executed**

**Decision**: Added a root `.shellcheckrc` (`shell=bash` default for
shebang-less scripts, `external-sources=true`) and gated all 15 shell scripts
(`*.sh` plus the extensionless `.husky/` hooks) at `--severity=warning`. Wired
via `rhino-cli:lint:shell`, a CI `shellcheck` job, and a scoped pre-push branch.

**Rationale**: shellcheck catches portability and correctness defects in shell
scripts. `shellcheck` ships preinstalled on GitHub-hosted runners, so the CI job
needs no install step.

**What this required**: a single real fix — single-quoting the git upstream
refspec (`'@{u}'`) in `.husky/pre-push` so shellcheck no longer reads the braces
as a literal brace expansion (SC1083). No blanket disables.

---

## D8 — GitHub Actions lint (actionlint) — **executed**

**Decision**: Gated `.github/workflows/` with `actionlint`, wired via
`rhino-cli:lint:actions`, a CI `actionlint` job, and a scoped pre-push branch.

**Rationale**: actionlint type-checks `${{ }}` expressions, validates runner
labels and cron syntax, and runs shellcheck over embedded `run:` scripts.

**No `.github/actionlint.yaml`**: the config file is **not needed** — `ose-primer`
uses only GitHub-hosted runners (no self-hosted runner labels to declare) and
actionlint reports no undefined-config-variable errors. An empty config would be
clutter, so none was added (the plan listed it as "optional / if needed").

**What this required**: fixing 5 embedded-shellcheck findings — grouping the
`$GITHUB_OUTPUT` echoes into one redirect block (SC2129), removing dead
`FAILED`/`RESULT`/no-op loop code in the gate-check step (SC2034), and a scoped
`# shellcheck disable=SC2163` on the intentional `export "$line"` KEY=VALUE
injection in the reusable E2E workflow.

---

## D9 — Terraform + Ansible/YAML lint — **skipped (no IaC)**

**Decision**: No action.

**Rationale**: `ose-primer` has no `.tf` files and no Ansible. There is nothing
for `tflint`/`terraform validate`/`ansible-lint`/`yamllint` to gate; this
dimension is infra-only.

---

## D10 — Dead `.golangci.yml` — **skipped (kept)**

**Decision**: Keep `ose-primer`'s `.golangci.yml`.

**Rationale**: The cross-repo decision removes dead `.golangci.yml` files from
repos with no active Go. `ose-primer` has **active Go** (`apps/crud-be-golang-gin`,
`libs/golang-commons`), so its `.golangci.yml` is live and is retained.

---

## M1 — `ose-primer` Sync Convention deviation (main-to-main delivery)

**Status**: **ACCEPTED** — a prominent, deliberate deviation explicitly approved
by the invoker.

**The default**: the `ose-primer` Sync Convention's PR-only Safety Invariant
mandates that all `ose-primer` mutations flow through a **draft PR**.

**The deviation**: this plan was delivered **main-to-main** — commits (plan files
and the gate changes) push directly to `origin main`, bypassing the PR-only
default. **No PR was opened.**

**Justification** (verbatim from the source brief): "invoker explicit
instruction; plan files are low-risk planning docs, not template-content
mutations." The gate changes are config-and-tooling additions validated by local
quality gates before each push.

**Consequence for D8's CI proof**: because `pr-quality-gate.yml` triggers on
`pull_request` only, a probe-branch _push_ triggers no CI under main-to-main
mode. The RED proof for the new CI jobs was therefore demonstrated via the
**identical command the CI job runs** — injecting a `RUN sudo apt-get update`
fixture and confirming `nx run rhino-cli:lint:dockerfiles` fails on DL3004, then
reverting — rather than via a throwaway PR. This deviation is recorded here and
in the plan's `delivery.md`.
