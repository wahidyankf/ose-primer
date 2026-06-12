# Business Requirements — lint-safety-parity (ose-primer)

## Business goal

Bring `ose-primer`'s linting strictness and unsafe-Rust posture to **parity** with its sibling
repositories (`ose-public`, `ose-infra`), so the three repos enforce one equal cross-language
quality bar. As the **public template** other teams clone, ose-primer's enforced strictness is
also what every downstream consumer inherits by default.

## Business rationale (WHY)

- ose-primer is the scaffolding template teams fork to build their own Sharia-compliant enterprise
  products. Whatever strictness it ships becomes the floor for every downstream repo. A weaker
  floor here silently weakens every consumer. [Judgment call: template-multiplier effect — a
  template's defaults propagate to all forks.]
- Today ose-primer is **stricter in some dimensions than its siblings and weaker in others**
  (e.g. Rust has no `forbid(unsafe_code)`; Python type-checking is `basic`, not strict). Drift
  between sibling repos makes the parity claim in `AGENTS.md` ("multi-harness, parity-maintained")
  untrue for the lint/safety surface.
- Catching unsafe-code introduction and lint regressions at commit/PR time is cheaper than catching
  them in review or production. [Judgment call: shift-left economics — the standard senior-engineer
  rationale for static gates.]

## Business impact

### Pain points addressed

- **Inconsistent template floor** — forks of ose-primer inherit whichever strictness ose-primer
  happens to enforce; gaps (no `forbid(unsafe_code)`, `pyright basic`) propagate widely.
- **Sibling drift** — the three repos cannot honestly claim parity while their lint gates differ.
- **Undetected unsafe Rust** — without `forbid(unsafe_code)`, a future contributor could add
  `unsafe` to `crud-be-rust-axum` with no automated objection.
- **Missing infra-file linting** — Dockerfiles, shell scripts, and GitHub Actions YAML are
  currently unlinted, so defects in them surface only at runtime/CI-failure time.

### Expected benefits

- One equal, documented strictness standard across all three repos.
- Downstream forks inherit a strong floor by default.
- Unsafe Rust, lint regressions, and infra-file defects fail fast (pre-commit/pre-push + CI).
- A written, discoverable rationale doc explaining each decision for future maintainers and forkers.

## Affected roles

This is a solo-maintainer repo; "roles" denote hats the maintainer wears and the agents that
consume the artifacts. No sign-off ceremonies.

| Role / hat               | Stake in this plan                                                         |
| ------------------------ | -------------------------------------------------------------------------- |
| Maintainer (build hat)   | Runs cleanup, flips gates, owns the green build.                           |
| Maintainer (review hat)  | Relies on the strict gate to catch unsafe/lint regressions automatically.  |
| Downstream forker        | Inherits the strict floor; reads the rationale doc to understand defaults. |
| `repo-setup-manager`     | Phase 0 environment setup + baseline.                                      |
| `swe-rust-dev`           | Executes D1 Rust gate work.                                                |
| `swe-csharp-dev`         | Executes D3 C# gate work.                                                  |
| `plan-execution-checker` | Validates the finished work against acceptance criteria.                   |

## Business-level success metrics

| Metric                                         | Type        | Target / observable check                                                       |
| ---------------------------------------------- | ----------- | ------------------------------------------------------------------------------- |
| Rust crate forbids unsafe                      | Observable  | `crud-be-rust-axum/Cargo.toml` contains `unsafe_code = "forbid"`; build passes. |
| Dimensions executed vs planned                 | Observable  | D1, D3, D4, D6, D7, D8 each have a green gate; D2/D5/D9/D10 correctly skipped.  |
| Strict gate enforced in CI **and** local hooks | Observable  | New gates fail on injected violation in both `pr-quality-gate.yml` and hooks.   |
| Rationale doc present                          | Observable  | `docs/explanation/lint-safety-parity-decisions.md` exists and covers all rows.  |
| Sibling parity honesty                         | Qualitative | Deviation matrix in tech-docs matches sibling plans; M1 deviation recorded.     |

## Business-scope non-goals

- Achieving parity on dimensions ose-primer does not own (F#, IaC) — those are sibling-repo scope.
- Implementing TS DDD import-boundary enforcement (D5) — explicitly deferred to a future plan.
- Re-architecting the demo apps; only their lint posture changes.
- Adding new linters beyond the locked dimension set.

## Business risks and mitigations

| Risk                                                           | Likelihood | Mitigation                                                                                  |
| -------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------- |
| Strict gate breaks first CI run on existing backlog            | Medium     | **Clean-then-gate** rollout: clean violations BEFORE flipping the gate ON.                  |
| `latest-All` C# analysis floods demo code with warnings        | High       | Phase budgets a dedicated C# cleanup backlog before enabling `latest-All`.                  |
| basedpyright strict surfaces many latent Python type errors    | Medium     | Phase budgets Python cleanup before swapping the type-checker in the gate.                  |
| main-to-main delivery bypasses ose-primer Sync Convention (M1) | Accepted   | Invoker explicitly approved; deviation + justification recorded in tech-docs and rationale. |
| Downstream forks surprised by stricter defaults                | Low        | Rationale doc documents each decision and the exemption philosophy.                         |
