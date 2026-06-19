# Business Requirements — Primer Polyglot Demo-App CI Restoration

## Business Goal

Make the `ose-primer` polyglot `crud-*` demo apps build, lint, test, and coverage-check **green from a
clean checkout** in the per-language PR quality gate, so the template's headline feature — a working
multi-language showcase generated from a single OpenAPI contract — is actually verifiable in CI rather
than silently broken.

## Business Impact

`ose-primer` is the downstream public template that packages the scaffolding layer (governance, agents,
CI harness, and the polyglot demo apps) for teams building their own products. The demo apps are the
template's proof that the contract-first, codegen-driven approach works across languages. Today:

- **The showcase is broken on a clean checkout.** Anyone who clones the template and runs the gate (or
  whose CI builds the apps fresh) gets red across Dart/Rust/Go. That directly undermines the template's
  credibility for its primary audience.
- **CI was dishonest, now honest.** Before the matrix fix (`9ede6a70e` [Repo-grounded]) the per-language
  jobs never built the demo apps, so the gate was green-by-omission. The fix made CI honest; this plan
  makes the apps actually pass, closing the gap between "CI green" and "showcase works."
- **A real security exposure was carried in the demo apps.** The SQLite `NU1903` CVE (CVE-2025-6965
  [Web-cited: GitHub Advisory Database, GHSA-2m69-gcr7-jv3q, https://github.com/advisories/GHSA-2m69-gcr7-jv3q,
  accessed 2026-06-19, excerpt: "There exists a vulnerability in SQLite versions before 3.50.2 where
  the number of aggregate terms could exceed the number of columns available. This could lead to a
  memory corruption issue."]) was pinned to a vulnerable transitive version. Already remediated by
  commit `c82c66c6f` [Repo-grounded]; the rest of this plan ensures the apps that ship the fix
  actually build.

## Affected Roles

Solo-maintainer repo collaborating with AI agents — "roles" are hats the maintainer wears plus consumers:

- **Maintainer (template-owner hat)** — wants the polyglot showcase demonstrably working for adopters.
- **Template adopters** — clone `ose-primer`; expect the demo apps to build and the gate to pass.
- **`pr-quality-gate` workflow** — now correctly runs the demo apps; needs them to be fixable to go green.
- **`rhino-cli`** — the shared codegen/specs tool whose dormant `specs scaffold` commands are implicated.

## Business-Level Success Metrics

- **Observable check**: a clean checkout (no `generated-contracts/`, no nx cache) running each
  per-language gate passes — verified by `rm -rf **/generated-contracts && nx run-many … --skip-nx-cache`
  per language exiting 0.
- **Observable check**: the `ose-primer` `PR - Quality Gate` workflow concludes `success` on a commit that
  makes all demo apps affected (e.g., a `rhino-cli` change), with no per-language job red.
- **Observable check**: no NuGet `NU1903`/audit-as-error in the .NET apps (already met by `c82c66c6f`).
- _Judgment call_: the Elixir gate's CI "errors on dependencies" does not recur across 3 consecutive runs
  (if it was transient) or is root-caused to a specific dependency (if real).

## Business-Scope Non-Goals

- This plan does NOT redesign the codegen architecture or the contract pipeline; it restores fresh-checkout
  correctness with the smallest responsible change per language.
- This plan does NOT change the `web-research-maker → web-researcher` rename or the role additions.
- This plan does NOT add new demo apps or languages.

## Business Risks and Mitigations

| Risk                                                                                                                                                  | Likelihood                  | Impact                                                  | Mitigation                                                                                                                                                                                                                                                                        |
| ----------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Activating `rhino-cli specs scaffold dart` diverges the cross-repo byte-identical `rhino-cli` mirror, breaking the harness-parity invariant + checker | _Judgment call_: medium     | Parity gate fails across repos; mirror invariant erodes | Prefer an app-level codegen fix (full-package dart generation) over changing shared tooling; if tooling must change, make it runtime-conditional (no-op where no contract source) so source stays byte-identical, and update the harness-compatibility checker in the same change |
| Go's `oapi-codegen` has no OpenAPI 3.1 support; swapping generators changes the generated Go types                                                    | _Judgment call_: medium     | Go demo app types churn; downstream code may need edits | Evaluate a 3.1-capable generator vs a 3.0 downconversion step for the Go types target only; keep the public type names stable                                                                                                                                                     |
| Fresh-codegen fixes pass locally but not in CI (toolchain/version drift)                                                                              | _Judgment call_: medium     | Extra CI round-trips                                    | Reproduce each fix with `--skip-nx-cache` and a cleaned tree before pushing; treat local green as necessary-not-sufficient                                                                                                                                                        |
| Elixir failure is a real dependency incompatibility, not a flake                                                                                      | _Judgment call_: low-medium | Larger elixir fix than expected                         | Reproduce with a clean `_build`/`deps` (`mix deps.clean --all && mix deps.get && mix compile`) before assuming transient                                                                                                                                                          |
