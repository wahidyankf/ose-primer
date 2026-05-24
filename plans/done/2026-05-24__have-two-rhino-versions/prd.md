# Product Requirements — Two Rhino Versions

> WHAT we build. See [brd.md](./brd.md) for WHY and
> [tech-docs.md](./tech-docs.md) for HOW.

## Product Overview

`rhino-cli` becomes a **dual-implementation CLI**: `rhino-cli-go` (Go) and
`rhino-cli-rust` (Rust), each a complete implementation of the command surface
defined by [`specs/apps/rhino/`](../../../specs/apps/rhino/README.md). After
cutover, every automated invocation point (CI workflows, husky hooks,
`package.json` scripts, the ~23 dependent projects' `test:quick`/`spec-coverage`
targets) calls **`rhino-cli-rust`**; `rhino-cli-go` remains as the parity twin.

### Command surface to reach parity on _[Repo-grounded: `apps/rhino-cli/cmd/`]_

| Namespace         | Subcommands                                                   |
| ----------------- | ------------------------------------------------------------- |
| `agents`          | `sync`, `validate-naming`, `validate-claude`, `validate-sync` |
| `contracts`       | `java-clean-imports`, `dart-scaffold`                         |
| `docs`            | `validate-links`, `validate-mermaid`                          |
| `env`             | `init`, `backup`, `restore`                                   |
| `git`             | `pre-commit`                                                  |
| `java`            | `validate-annotations`                                        |
| `repo-governance` | `vendor-audit`                                                |
| `spec-coverage`   | `validate`                                                    |
| `test-coverage`   | `validate`, `merge`, `diff`                                   |
| `workflows`       | `validate-naming`                                             |
| (root)            | `doctor`                                                      |

> The executor MUST enumerate the live surface at execution time
> (`ls apps/rhino-cli-go/cmd/*.go | grep -v _test.go`) and treat that as
> authoritative — the table above is a planning snapshot.

## Personas

- **Maintainer / AI agent** — runs `npm run sync:claude-to-opencode`,
  `npm run doctor`, commits (pre-commit hook), pushes (pre-push hook). Wants
  these to keep working unchanged after cutover.
- **CLI contributor** — edits a validator. Must update both implementations and
  see the parity gate confirm identical behavior.
- **Fork owner** — copies this repo, wants a clear example of two CLIs kept on par.

## User Stories

1. As a maintainer, I want the Rust CLI to be a drop-in replacement so that all
   my existing scripts/hooks/CI keep working after the cutover.
2. As a contributor, I want a parity gate that fails when Go and Rust outputs
   diverge so that the twins cannot silently drift.
3. As a contributor, I want both implementations to consume the same
   `specs/apps/rhino/` Gherkin so that the behavior contract is single-sourced.
4. As a fork owner, I want both CLIs documented in the apps catalog and a
   governance convention explaining the parity model.
5. As a maintainer, I want the Go CLI to remain fully buildable and tested so
   that it is a real twin, not dead code.

## Acceptance Criteria (Gherkin)

### Rename keeps everything green

```gherkin
Given the Go CLI is renamed from rhino-cli to rhino-cli-go
And every caller reference is repointed to rhino-cli-go
When I run "npx nx affected -t typecheck lint test:quick spec-coverage" across the repo
Then all targets pass
And "npm run sync:claude-to-opencode" succeeds
And the pre-commit and pre-push hooks run without error
```

### Rust scaffold builds before any command is ported

```gherkin
Given a new apps/rhino-cli-rust crate with the full Nx target set
When I run "npx nx run rhino-cli-rust:build", ":typecheck", ":lint", ":test:unit"
Then each exits 0
And no existing caller yet depends on rhino-cli-rust
```

### Per-command parity (repeated for every ported command)

```gherkin
Given a command has been ported to rhino-cli-rust
When the shadow-diff harness runs both binaries on the same inputs
Then stdout is byte-identical
And stderr is byte-identical
And the exit code is identical
And this holds for text, json, and markdown output formats where supported
```

### Both implementations cover the shared specs

```gherkin
Given specs/apps/rhino/behavior/cli/gherkin holds the behavior contract
When I run "npx nx run rhino-cli-go:spec-coverage"
And I run "npx nx run rhino-cli-rust:spec-coverage"
Then both report full coverage of the same scenarios
```

### Big-bang cutover flips CI to Rust

```gherkin
Given all commands have reached shadow-diff parity
When the cutover commit repoints every caller from rhino-cli-go to rhino-cli-rust
Then CI runs the Rust binary for naming, links, coverage, and spec-coverage gates
And the ~23 dependent projects' test:quick and spec-coverage targets pass via Rust
And rhino-cli-go still builds and passes its own test:quick and spec-coverage
```

### Permanent drift protection

```gherkin
Given both implementations exist after cutover
When any PR changes either CLI
Then a CI parity job builds both and runs shadow-diff
And the PR is blocked if Go and Rust outputs differ
```

## Product Scope

**In scope**

- Two CLI projects under `apps/`, identical command surface, shared specs.
- Shadow-diff parity harness + permanent CI parity gate.
- Apps catalog + governance convention documenting the model.

**Out of scope**

- New commands, flags, or output formats not already in the Go CLI.
- Deleting/archiving Go.
- Behavior-contract changes in `specs/apps/rhino/`.

## Product Risks

| Risk                                                          | Mitigation                                                                   |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| A caller is missed at cutover → broken hook/CI.               | Grep-based enumeration + atomic cutover commit + full affected run pre-push. |
| Subtle output difference (ordering, trailing newline, color). | Shadow-diff requires byte-identical output; `--no-color` normalized.         |
| Rust spec-coverage diverges from Go's notion of coverage.     | Both run the same `spec-coverage validate` semantics against the same specs. |
