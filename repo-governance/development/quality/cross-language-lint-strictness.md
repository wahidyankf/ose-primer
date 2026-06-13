# Cross-Language Lint Strictness

This repository enforces a **uniform strictness threshold across every language
and artifact type it ships**: a linter finding at the **warning-and-above** level
fails the build, in both CI and local git hooks. This page is the canonical list
of the cross-language lint gates and the policy that binds them.

## Policy

- **Threshold**: every gate fails on a finding of severity **warning or above**.
  This matches how Prettier and markdownlint are already gated — there is no
  "advisory" tier that prints findings without blocking.
- **Two enforcement points**: every gate runs in CI (`.github/workflows/pr-quality-gate.yml`)
  **and** in the local Husky hooks (`.husky/pre-commit`). CI is the hard gate;
  the local hook gives fast feedback and degrades gracefully (skips with a hint)
  when the tool is not yet installed, so a fresh checkout can still commit before
  `npm run doctor -- --fix` runs.
- **Toolchain convergence**: every gate's binary is registered in the
  `rhino-cli doctor` converger, so `npm run doctor -- --fix` installs it.
- **Clean-then-gate**: a gate is wired ON only after its existing violation
  backlog is cleaned, so the first CI/hook run never breaks on pre-existing
  findings.
- **Documented waivers only**: a rule is suppressed only where applying it would
  reduce clarity or reproducibility for no real safety gain, and every waiver is
  documented inline at the point of suppression (config comment or inline
  `disable`/`nowarn`), never silently.

## Gated Standards

| Artifact            | Tool         | Threshold / config                                   | CI job       |
| ------------------- | ------------ | ---------------------------------------------------- | ------------ |
| Shell scripts       | `shellcheck` | `--severity=warning`; root `.shellcheckrc`           | `shellcheck` |
| Dockerfiles         | `hadolint`   | `--failure-threshold warning`; root `.hadolint.yaml` | `hadolint`   |
| GitHub Actions YAML | `actionlint` | non-zero on any finding (embeds shellcheck)          | `actionlint` |
| Markdown            | markdownlint | see [markdown.md](./markdown.md)                     | `markdown`   |
| Formatting          | Prettier     | `prettier --check`                                   | `format`     |

The `shellcheck`, `hadolint`, and `actionlint` jobs are **always-run** (their artifacts
are not Nx-tagged projects, so they are not gated by language detection). These three
tool-named CI jobs run unconditionally on every PR and on every push to `main`.

## CI Job Naming

Cross-language lint jobs are named after the tool they run, not after the artifact type.
The canonical CI job names are:

| Tool         | CI job key   |
| ------------ | ------------ |
| `shellcheck` | `shellcheck` |
| `hadolint`   | `hadolint`   |
| `actionlint` | `actionlint` |

This naming rule is an invariant in the CI/toolchain parity checklist (Invariant A).

## Configuration Files

- `.shellcheckrc` — `shell=bash`, `external-sources=true`; no repo-wide disables.
- `.hadolint.yaml` — `failure-threshold: warning`; `trustedRegistries`
  (`docker.io`, `ghcr.io`); `ignored: [DL3008, DL3018]`
  (OS-package version-pinning is brittle — reproducibility comes from the pinned
  base-image tag, not per-package pins).

## Principles Implemented/Respected

- **[Automation Over Manual](../../principles/software-engineering/automation-over-manual.md)**:
  Every gate runs in CI and in the local hook — no manual lint sweep is ever required.

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**:
  Every gate, its threshold, its config file, and its CI job name are declared in this
  document. No undocumented lint pass exists.

## Rationale and History

The strictness set was equalized across the three sibling repositories
(ose-public, ose-primer, ose-infra) in the 2026-06-12 `lint-safety-parity` effort.
The full decision log — including which rules are fixed vs. waived and why — lives
in [Lint & Safety Parity — Decisions](../../../docs/explanation/lint-safety-parity-decisions.md).

**See also**: [markdown.md](./markdown.md),
[repository-validation.md](./repository-validation.md).
