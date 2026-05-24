# Technical Documentation — Two Rhino Versions

> HOW we build it. See [brd.md](./brd.md) for WHY and [prd.md](./prd.md) for WHAT.

## Current State _[Repo-grounded]_

- `apps/rhino-cli/` — Go, `module` via `go.mod`, depends on `libs/golang-commons`
  (`implicitDependencies: ["golang-commons"]`). Cobra command tree rooted at
  `cmd/root.go` (`Use: "rhino-cli"`). Built with `CGO_ENABLED=0 go build -o dist/rhino-cli`.
- Nx targets: `build`, `install`, `run`, `test:quick`, `test:unit`,
  `test:integration`, `typecheck`, `lint`, `spec-coverage`,
  `validate:naming-agents`, `validate:naming-workflows`,
  `validate:repo-governance-vendor-audit`, `validate:cross-vendor-parity`,
  `validate:mermaid`. _[Repo-grounded: `apps/rhino-cli/project.json`]_
- Behavior specs: `specs/apps/rhino/behavior/cli/gherkin/` with domain subdirs
  `agents/ contracts/ docs/ env/ git/ java/ repo-governance/ spec-coverage/
system/ test-coverage/ workflows/`. _[Repo-grounded]_
- Rust is already provisioned: `.github/actions/setup-rust/action.yml` (installs
  toolchain + `cargo-llvm-cov`, caches `~/.cargo` + one app's `target`); `doctor`
  probes `rustc`/`cargo-llvm-cov`; `apps/crud-be-rust-axum/` is a working Rust
  project to copy target idiom from. _[Repo-grounded]_

### Caller inventory (must all be repointed) _[Repo-grounded]_

| Caller                                                   | Reference today                                                                                                                                                                                             |
| -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.husky/pre-commit`                                      | `CGO_ENABLED=0 go run -C apps/rhino-cli main.go git pre-commit`                                                                                                                                             |
| `.husky/pre-push`                                        | `nx run rhino-cli:validate:naming-agents` / `:naming-workflows` / `:mermaid` / `:cross-vendor-parity`                                                                                                       |
| `.github/workflows/pr-quality-gate.yml` (naming job)     | `nx run rhino-cli:validate:naming-agents` + `:validate:naming-workflows` (uses `setup-golang`)                                                                                                              |
| `.github/workflows/pr-validate-links.yml`                | `CGO_ENABLED=0 go run -C apps/rhino-cli main.go docs validate-links` (uses `setup-go`)                                                                                                                      |
| `package.json` scripts                                   | `dev:rhino-cli`, `sync:claude-to-opencode`, `sync:agents`, `sync:skills`, `sync:dry-run`, `validate:sync`, `validate:claude`, `doctor` — all build `rhino-cli` then run `./apps/rhino-cli/dist/rhino-cli …` |
| ~23 `apps/*` + `libs/*` `project.json`                   | `implicitDependencies: ["rhino-cli"]` and `test:quick`/`spec-coverage` calling `go run -C apps/rhino-cli main.go test-coverage validate …` / `spec-coverage validate …`                                     |
| `apps/rhino-cli/scripts/validate-cross-vendor-parity.sh` | Calls `go run main.go repo-governance vendor-audit …` + `npm run sync:claude-to-opencode`                                                                                                                   |
| `infra/dev/rhino-cli/docker-compose.yml`                 | Referenced by `dev:rhino-cli`                                                                                                                                                                               |
| Governance docs + `README.md` + `apps/README.md`         | Textual references naming `rhino-cli` as the canonical CLI                                                                                                                                                  |

> The exact set of `project.json` callers MUST be regenerated at execution time:
> `grep -rln 'rhino-cli' apps libs --include=project.json`. Do not trust a frozen list.

## Target Architecture

Two sibling Nx projects under `apps/`, each a standalone build, both reading the
same `specs/apps/rhino/` contract:

```
apps/
├── rhino-cli-go/      # renamed from rhino-cli; Go; implicitDependencies: [golang-commons]
│   ├── cmd/ internal/ main.go go.mod project.json scripts/
│   └── dist/rhino-cli            # binary basename unchanged (drop-in)
└── rhino-cli-rust/    # new; Rust standalone crate (no cargo workspace)
    ├── Cargo.toml rust-toolchain.toml deny.toml project.json
    ├── src/{main.rs, lib.rs, cli.rs, commands/, internal/}
    ├── scripts/shadow-diff.sh   # parity harness
    └── dist/rhino-cli           # binary basename matches (drop-in)
```

### Naming + tags

- Project names: `rhino-cli-go`, `rhino-cli-rust`. Binary basename stays
  `rhino-cli` in both `dist/` dirs (full path disambiguates; preserves drop-in).
- Tags (mirror the `crud-be-*` four-dimension scheme
  _[Repo-grounded: `nx-targets.md`]_):
  - go: `["type:app", "platform:cli", "lang:golang", "domain:tooling"]`
  - rust: `["type:app", "platform:cli", "lang:rust", "domain:tooling"]`
- New CLI sub-pattern for `apps/README.md`: `rhino-cli-<lang>`.

### Rust crate design (technique from ose-public) _[Web-cited: ose-public `apps/rhino-cli/` — sibling repo, not verifiable in ose-primer.]_

- `edition = 2024`, pinned `rust-toolchain.toml`; `[[bin]] name = "rhino-cli"`,
  `[lib] name = "rhino_cli"`.
- `clap` derive command tree mirroring the Cobra tree; global flags
  (`--verbose`, `--quiet`, `--output`, `--no-color`) validated before subcommand
  dispatch.
- Sealed `OutputFormat` enum (`Text | Json | Markdown`) in `internal/cliout`;
  every command `run()` matches it exhaustively.
- `internal/` modules mirror the Go `internal/` package layout for mechanical
  porting (e.g. `testcoverage/`, `speccoverage/`, `docs/`, `agents/`,
  `repo_governance/`, `doctor/`, `naming/`, `git/`).
- Dev-deps: `cucumber` (BDD), `assert_cmd`, `predicates`, `tempfile`.
- Coverage: `cargo llvm-cov … --fail-under-lines 90` (matches Go's 90% floor).
- Lints: `unsafe_code = "deny"`, clippy pedantic with a documented allow-list.
- Dependency-vetting: `cargo deny check` (add `deny.toml`).

> Pin exact crate versions at execution time by reading ose-public's
> `apps/rhino-cli/Cargo.toml` as the reference and running `cargo update`
>
> - `cargo deny check`. Do NOT copy version numbers blind — verify against the
>   registry at port time. _[Unverified: exact versions — resolve before authoring Cargo.toml]_

### Nx target mapping (Go idiom → Rust idiom) _[Repo-grounded: `apps/crud-be-rust-axum/project.json`]_

| Logical target     | Go (`rhino-cli-go`)                                                                               | Rust (`rhino-cli-rust`)                                                               |
| ------------------ | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `build`            | `CGO_ENABLED=0 go build -o dist/rhino-cli`                                                        | `cargo build --release && cp target/release/rhino-cli dist/rhino-cli`                 |
| `test:unit`        | `go test ./... -count=1`                                                                          | `cargo test --lib`                                                                    |
| `test:quick`       | `go test -coverprofile=cover.out ./... && … test-coverage validate cover.out 90`                  | `cargo llvm-cov --lcov --output-path cover.out --fail-under-lines 90`                 |
| `test:integration` | `go test -tags=integration …`                                                                     | `cargo test --tests` (cucumber-rs integration world)                                  |
| `typecheck`        | `go vet ./...`                                                                                    | `cargo check --all-targets`                                                           |
| `lint`             | `golangci-lint run …`                                                                             | `cargo fmt --check && cargo clippy -- -D warnings`                                    |
| `spec-coverage`    | `… spec-coverage validate specs/apps/rhino/behavior/cli/gherkin apps/rhino-cli-go --shared-steps` | same, target dir `apps/rhino-cli-rust`                                                |
| `validate:*`       | `go run -C apps/rhino-cli-go main.go <ns> <cmd>`                                                  | `cargo run --release -q --manifest-path apps/rhino-cli-rust/Cargo.toml -- <ns> <cmd>` |
| `install`          | `go mod tidy`                                                                                     | `cargo fetch`                                                                         |
| `run`              | `go run main.go`                                                                                  | `cargo run --`                                                                        |

## Parity Model (the "on par" mechanism)

The `crud-be-*` apps prove parity with a **shared E2E HTTP suite** + per-impl
`spec-coverage`. _[Repo-grounded: `apps/crud-be-e2e/`,
`bdd-spec-test-mapping.md`]_ A CLI has no HTTP surface, so the analog is a
**shadow-diff harness**:

1. **Shared specs** — both implementations have a `spec-coverage` target pointed
   at `specs/apps/rhino/behavior/cli/gherkin/` and BDD tests (godog for Go,
   cucumber-rs for Rust) executing those scenarios. Single-sourced contract.
2. **Shadow-diff** — `apps/rhino-cli-rust/scripts/shadow-diff.sh` builds both
   binaries, runs each on a corpus of representative invocations (per command,
   per `--output` format), and asserts byte-identical stdout, stderr, and exit
   code. Color disabled (`--no-color`) to normalize.
3. **Permanent CI parity gate** — a `parity` job in `pr-quality-gate.yml` runs
   the shadow-diff corpus on every PR that touches either CLI or the specs, and
   blocks divergence. This is the CLI analog of the `crud-be-e2e` implicit-dep
   gate.

During the port (Phases 3–8) shadow-diff is the **per-command acceptance gate**;
after cutover it becomes the **permanent anti-drift gate**.

## Design Decisions

1. **Big-bang cutover, not incremental.** _[User decision]_ Go (renamed) stays
   canonical in CI until Rust reaches full parity, then one atomic commit flips
   all callers. Lowest risk of partial/red CI; Rust is exercised by its own
   tests + shadow-diff before it ever fronts a gate. Rejected: ose-public's
   per-command incremental flip (correct for a _replacement_, needless mixed
   state for a _retained twin_).
2. **Keep Go forever as a twin.** _[User decision]_ Unlike ose-public (which
   archived Go), we keep both buildable and tested — the dual-impl demonstration
   is the deliverable.
3. **Binary basename stays `rhino-cli`** in both projects — preserves drop-in
   semantics; the project/dir name carries the language suffix.
4. **Standalone Rust crate (no cargo workspace)** — matches `crud-be-rust-axum`;
   avoids coupling to a workspace manifest. _[Repo-grounded]_
5. **Parity target = ose-primer's current Go surface**, not ose-public's larger
   surface. ose-public is a technique reference only.

## File Impact

**Renamed**

- `apps/rhino-cli/` → `apps/rhino-cli-go/` (via `git mv`, preserves history)
- `infra/dev/rhino-cli/` → `infra/dev/rhino-cli-go/`

**Edited (rename phase)**: `apps/rhino-cli-go/project.json` (name, sourceRoot,
self-referencing command paths), `go.mod` (verify module path), every caller in
the inventory table above, governance docs, `README.md`, `apps/README.md`,
`specs/apps/rhino/README.md` backlinks.

**New**

- `apps/rhino-cli-rust/**` (crate, project.json, src, scripts/shadow-diff.sh)
- `repo-governance/.../rhino-cli-dual-implementation-parity.md` (convention)
- A `parity` job in `.github/workflows/pr-quality-gate.yml`

**Edited (cutover phase)**: every caller flipped `rhino-cli-go` → `rhino-cli-rust`.

## Dependencies & Sequencing

- Phase 1 (rename) must complete and be green before Phase 2 (scaffold).
- Phases 3–8 (port) each depend on Phase 2 and the shadow-diff harness (Phase 3
  introduces it).
- Phase 10 (cutover) depends on ALL commands reaching parity (Phases 3–8) and the
  parity gate (Phase 9).
- No cargo workspace changes; `setup-rust` cache must add `apps/rhino-cli-rust/target`.

## Rollback

- **Pre-cutover**: Rust is unwired; deleting `apps/rhino-cli-rust/` and reverting
  the rename restores the original state. Low risk.
- **Cutover**: a single revert of the cutover commit repoints all callers back to
  `rhino-cli-go` (still present, still green). The parity gate guarantees the two
  were identical at flip time, so revert is behavior-neutral.

## Open Questions

- Exact Rust crate versions — _resolve by reading ose-public `Cargo.toml` +
  `cargo deny check` at port time._ _[Unverified]_
- Whether `infra/dev/` needs a `rhino-cli-rust` dev compose — _decide in Phase 2;
  default: rename Go's, add Rust's only if `dev:` parity is wanted._ _[Unverified]_
