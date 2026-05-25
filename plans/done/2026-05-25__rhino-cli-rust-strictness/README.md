# rhino-cli-rust Strictness Alignment

**Status**: In Progress
**Created**: 2026-05-25
**Priority**: HIGH
**Scope**: Align `apps/rhino-cli-rust/` structure, lint rules, and project targets to match `ose-public`'s `apps/rhino-cli/`

## Summary

`apps/rhino-cli-rust/` was ported from Go and retains Go-parity Clippy allows and a `cwd`-based
`project.json` target style that diverges from the canonical pattern in `ose-public/apps/rhino-cli/`.
This plan aligns the Rust app to ose-public's structure and strictness:

1. **project.json structural alignment** — restructure all Nx targets to use `--manifest-path`
   from the workspace root (matching ose-public), add missing targets (`fmt`, `fmt:check`,
   `deny:check`, `check:msrv`), fix `lint` to use a two-command sequential array, and correct
   output paths for `build` and `test:quick`.

2. **Cargo.toml lint alignment** — remove the seven Go-parity `allow` overrides from
   `[lints.clippy]` that are absent from ose-public, bringing the effective lint configuration
   to feature-parity.

3. **Clippy violation remediation** — fix all violations surfaced after removing the allows.

4. **`.gitignore` addition** — add project-level `.gitignore` matching ose-public (`target/`,
   `dist/`, `lcov.info`, `lcov_spec.info`, `*.profraw`).

## Documents

| Document                     | Purpose                                      |
| ---------------------------- | -------------------------------------------- |
| [brd.md](brd.md)             | Business rationale and goals                 |
| [prd.md](prd.md)             | Requirements and Gherkin acceptance criteria |
| [tech-docs.md](tech-docs.md) | Technical design and change specifications   |
| [delivery.md](delivery.md)   | Phased delivery checklist                    |

## Worktree

Execution runs directly on `main` per user directive (no worktree isolation required for
single-app changes). Working directory: `/Users/wkf/ose-projects/ose-primer`.

Provisioning command (if worktree isolation is later needed):

```bash
claude --worktree rhino-cli-rust-strictness
```

Expected path: `worktrees/rhino-cli-rust-strictness/`
