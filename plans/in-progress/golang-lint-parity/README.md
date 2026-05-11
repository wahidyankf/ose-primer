# golang-lint-parity

Bring `ose-primer` Go lint strictness to full parity with `ose-public`.

## Context

`ose-primer` and `ose-public` share the same `.golangci.yml` base structure, but `ose-primer` is
missing five linters and their settings blocks that `ose-public` has enabled. One existing code
violation is also known: a type assertion on an error value in `diff.go` that `errorlint` will
flag once added.

## Scope

Single repo: `ose-primer`. Two files changed:

- `.golangci.yml` — add 5 linters + 4 settings blocks
- `apps/rhino-cli/internal/testcoverage/diff.go` — fix type assertion → `errors.As`

## Business Rationale

`ose-primer` is the upstream template for all OSE-style repos. Weaker lint in the template means
downstream adopters inherit a less strict baseline. Parity with `ose-public` closes that gap and
prevents lint regressions from propagating to new repos.

**Affected roles**: Downstream repos adopting `ose-primer` as a template; contributors working on
`ose-primer`'s own Go packages (`rhino-cli`, `golang-commons`).

**Non-goals**: Not modifying `ose-public` config; not adding linters beyond the five in scope; not
fixing lint issues in any repo other than `ose-primer`.

## Product Requirements

### User Stories

- As a template adopter, I want `ose-primer`'s Go lint config to match `ose-public` so that my
  project inherits the same strictness baseline without manual config surgery.
- As a contributor, I want `golangci-lint` to flag all error-handling violations so that
  error-wrapping discipline is enforced consistently across the codebase.

### Acceptance Criteria

```gherkin
Feature: Go lint parity with ose-public

  Scenario: All five linters enabled
    Given the .golangci.yml at repo root
    When read
    Then linters gochecksumtype, errorlint, iotamixing, godot, and revive are listed under enable

  Scenario: Settings blocks present for all new linters
    Given the .golangci.yml at repo root
    When read
    Then settings blocks exist for errorlint, gochecksumtype, godot, and revive
    And each block matches the ose-public configuration exactly

  Scenario: No errorlint violation in diff.go
    Given apps/rhino-cli/internal/testcoverage/diff.go
    When read
    Then err.(*exec.ExitError) type assertion is replaced with errors.As pattern

  Scenario: Lint targets pass with zero findings
    Given golangci-lint v2.10.1 installed
    When npx nx run rhino-cli:lint runs
    And npx nx run golang-commons:lint runs
    Then both complete with exit code 0
```

## Technical Approach

### Config delta (`.golangci.yml` additions)

Add under `linters.enable` (after existing `exhaustive` entry, before `unparam`):

```yaml
# Error-handling discipline
- errorlint # forces errors.Is/errors.As; flags non-%w fmt.Errorf
# Const-block hygiene
- iotamixing # forbids mixing iota with non-iota constants in same block
# Documentation style
- godot # exported doc comments must end with a period
- revive # exported rule: every exported symbol must have a doc comment
# Exhaustiveness (sealed interfaces)
- gochecksumtype # exhaustive type switches over sealed interfaces with //sumtype:decl
```

Add under `linters.settings` (after existing `exhaustive` block):

```yaml
errorlint:
  errorf: true
  errorf-multi: true
  asserts: true
  comparison: true

gochecksumtype:
  default-signifies-exhaustive: false

godot:
  scope: declarations
  capital: false
  period: true

revive:
  enable-all-rules: false
  rules:
    - name: exported
      severity: error
      disabled: false
      arguments:
        - "disableStutteringCheck"
    - name: package-comments
      severity: error
      disabled: false
```

### Code fix (`diff.go`)

Replace the type assertion pattern with `errors.As`:

**Before** (`diff.go:135-138` — the two lines being replaced):

```go
if exitErr, ok := err.(*exec.ExitError); ok {
    return "", fmt.Errorf("git diff failed: %s", strings.TrimSpace(string(exitErr.Stderr)))
}
```

**After** (replacing only those lines; the comment on the preceding line is already present and
stays untouched):

```go
var exitErr *exec.ExitError
if errors.As(err, &exitErr) {
    return "", fmt.Errorf("git diff failed: %s", strings.TrimSpace(string(exitErr.Stderr)))
}
```

Also add `"errors"` to the import block if not already present.

## Worktree

```
worktrees/golang-lint-parity/
```

Provision:

```bash
claude --worktree golang-lint-parity
```

After entering the worktree, initialize the toolchain:

```bash
npm install && npm run doctor -- --fix
```

## Delivery Checklist

### Phase 1: Update lint config

- [ ] Add `errorlint`, `iotamixing`, `godot`, `revive`, `gochecksumtype` to `linters.enable` in
      `.golangci.yml` (after existing `exhaustive` entry, before `unparam`)
  - Acceptance: `linters.enable` list matches the ordering described in Technical Approach
- [ ] Add `errorlint`, `gochecksumtype`, `godot`, `revive` settings blocks under `linters.settings`
      in `.golangci.yml` (after existing `exhaustive` block)
  - Acceptance: diff of `.golangci.yml` settings matches the delta described in Technical Approach;
    the file still passes `golangci-lint config verify` if run
- [ ] Verify no unintended changes outside the two modified sections

### Phase 2: Fix errorlint violation

- [ ] Replace `err.(*exec.ExitError)` type assertion with `errors.As` pattern in
      `apps/rhino-cli/internal/testcoverage/diff.go` (see Technical Approach for Before/After)
  - Acceptance: file compiles — `cd apps/rhino-cli && CGO_ENABLED=0 go vet ./...` exits 0
- [ ] Add `"errors"` to the import block in `diff.go` if not already present
  - Acceptance: `cd apps/rhino-cli && CGO_ENABLED=0 go vet ./...` exits 0

### Phase 3: Local quality gate

- [ ] Install golangci-lint if not present:

  ```bash
  go install github.com/golangci/golangci-lint/v2/cmd/golangci-lint@v2.10.1
  ```

- [ ] Run lint for both Go projects:

  ```bash
  npx nx run rhino-cli:lint
  npx nx run golang-commons:lint
  ```

  Acceptance: both commands exit 0; zero lint violations reported

- [ ] If new violations found, fix them and re-run until clean
- [ ] Run typecheck to confirm no regressions:

  ```bash
  npx nx run rhino-cli:typecheck
  ```

  Acceptance: exits 0

### Phase 4: Full quality gate

- [ ] Run the full Go quality gate:

  ```bash
  npx nx run-many -t typecheck lint test:quick spec-coverage --projects=tag:lang:golang
  ```

  Acceptance: all targets pass

- [ ] Fix ALL failures found — including any pre-existing issues, not just ones introduced here
- [ ] Commit changes thematically with Conventional Commits:
  - Commit 1 (config): `chore(lint): add five strict Go linters matching ose-public`
  - Commit 2 (code): `fix(rhino-cli): use errors.As instead of type assertion in diff.go`
  - Push directly to `origin main` per Trunk Based Development convention

### Phase 5: CI verification

- [ ] After push, monitor GitHub Actions workflow for the commit:
  - Go quality gate job (`golang` job in `pr-quality-gate.yml`) must pass
  - If CI fails, fix immediately and push again before declaring done

### Phase 6: Plan archival

- [ ] After CI is green, archive this plan:

  ```bash
  git mv plans/in-progress/golang-lint-parity plans/done/2026-05-11__golang-lint-parity
  ```

  Replace `2026-05-11` with the actual completion date.

- [ ] Update `plans/done/README.md` — add an entry for `golang-lint-parity` with completion date
      and brief summary.
  - Acceptance: `plans/done/README.md` contains a row for `golang-lint-parity`
- [ ] Commit archival:

  ```bash
  git commit -m "chore(plans): move golang-lint-parity to done"
  ```

  Acceptance: `plans/in-progress/golang-lint-parity/` no longer exists; folder appears under
  `plans/done/` with completion-date prefix.

## Quality Gates

- `npx nx run rhino-cli:lint` → exit 0
- `npx nx run golang-commons:lint` → exit 0
- `npx nx run rhino-cli:typecheck` → exit 0
- `npx nx run rhino-cli:test:quick` → exit 0
- GitHub Actions Go job → green

## Verification

No UI or API changes. Verification is lint + test only:

```bash
npx nx run-many -t typecheck lint test:quick spec-coverage --projects=tag:lang:golang
```
