# Business Requirements — rhino-cli-rust Strictness Alignment

## Context

`apps/rhino-cli-rust/` is the Rust port of `rhino-cli`. It was ported from Go and carries
structural debt: Go-parity Clippy allows, a `cwd`-based Nx target style, and missing project
targets. The canonical reference is `ose-public/apps/rhino-cli/`, which has been maintained
longer and represents the target quality bar.

## Business Goals

1. **Consistency** — `ose-primer` is a template repo. Divergence from `ose-public` erodes its
   value as a clean starting point for new OSE-style repos.
2. **Quality signal** — Removing Go-parity allows forces idiomatic Rust, reducing maintenance
   surface and improving readability for future contributors.
3. **Operational completeness** — Missing targets (`fmt`, `fmt:check`, `deny:check`, `check:msrv`)
   mean contributors cannot run standard quality checks via Nx, silently skipping security and
   compatibility gates.

## Affected Roles

- **Developers forking ose-primer** — inherit a stricter, more complete setup.
- **CI pipeline** — gains `deny:check` (security advisory scan) and `check:msrv` (minimum
  supported Rust version verification).
- **Code reviewers** — stricter Clippy rules surface potential issues earlier.

## Success Metrics

- `npx nx run rhino-cli-rust:lint` passes with zero warnings [Judgment call — no baseline metric]
- `npx nx run rhino-cli-rust:deny:check` passes with no advisories [Judgment call]
- `npx nx run rhino-cli-rust:check:msrv` passes against `rust-version = "1.88"` [Judgment call]
- `npx nx run rhino-cli-rust:test:quick` passes at ≥90% line coverage [Repo-grounded: project.json `--fail-under-lines 90`]
- Zero diff in `[lints.clippy]` between `apps/rhino-cli-rust/Cargo.toml` and
  `ose-public/apps/rhino-cli/Cargo.toml` (excluding crate-specific justified exceptions) [Judgment call]

## Non-Goals

- Replacing `serde_norway` with `serde_yml` (different API; requires separate investigation)
- Adding `tree-sitter` dependency (unused in ose-primer binary)
- Removing `thiserror` (used in ose-primer code; not in ose-public but valid pattern)
- Implementing `validate:specs-*` targets (requires new binary subcommands; separate plan)
- Removing or changing `[[test]]` harness entries (different test structure; separate concern)

## Business Risks

| Risk                                                                                                                                      | Likelihood | Mitigation                                                                                                                                                                                                 |
| ----------------------------------------------------------------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Removing Go-parity Clippy allows surfaces a large number of violations that cannot be resolved in one session                             | Medium     | Run `cargo clippy` baseline at Phase 3 start; count violations before committing Cargo.toml change; abort and adjust scope if violation count exceeds a manageable threshold (e.g., >50 distinct sites)    |
| `cargo-deny` and `cargo-hack` are not installed in fork environments; installing them adds an implicit onboarding step for template users | Medium     | Phase 0 environment setup checks and installs both tools with `--locked`; document install commands in `tech-docs.md §External Tool Requirements`                                                          |
| Divergence from ose-public may have occurred in areas not captured in the current diff (plan was grounded on a point-in-time read)        | Low        | The plan is explicitly scoped to the seven documented allows and listed `project.json` targets; any additional drift discovered during execution is out of scope and should be tracked in a follow-on plan |
