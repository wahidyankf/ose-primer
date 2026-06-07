#!/usr/bin/env bash
#
# shadow-diff.sh — byte-parity harness for the Go → Rust rhino-cli port.
#
# Builds BOTH binaries (Go via its Nx dist, Rust via `cargo build --release`),
# then runs a corpus of representative invocations for each requested command
# in every output format (text/json/markdown) with --no-color. For each case it
# diffs stdout, stderr, and exit code between the two binaries and fails the
# whole run if ANY difference remains.
#
# Timestamps in JSON output are wall-clock dependent, so both streams are passed
# through a normaliser that masks the `"timestamp": "..."` field before diffing.
#
# Usage:
#   shadow-diff.sh [--help] [COMMAND ...]
#
#   COMMAND  One or more of: test-coverage spec-coverage crud-spec-coverage …
#            (default: all commands)
#
# Exit status: 0 when every case is byte-identical, 1 on any divergence,
#              2 on setup/usage errors.

set -euo pipefail

print_help() {
  cat <<'EOF'
shadow-diff.sh — byte-parity harness for the Go → Rust rhino-cli port

USAGE:
  shadow-diff.sh [--help] [COMMAND ...]

COMMANDS:
  test-coverage    Diff the `test-coverage validate|diff|merge` corpus
  spec-coverage    Diff the `spec-coverage validate` corpus
  crud-spec-coverage
                   Diff `spec-coverage validate` for EVERY crud backend and
                   frontend app, using the exact args each app's project.json
                   `spec-coverage` target uses. Guards against per-language
                   step-extraction divergence (the class of bug that the
                   shared-steps corpus above does not exercise — see NOTE).
  docs             Diff the `docs validate-links|validate-mermaid|validate-heading-hierarchy` corpus
  agents           Diff the `agents sync|validate-claude|validate-sync|validate-naming` corpus
  repo-governance  Diff the `repo-governance vendor-audit|gherkin-keyword-cardinality` corpus
  workflows        Diff the `workflows validate-naming` corpus
  git              Diff the `git pre-commit` corpus (error path only — see NOTE)
  contracts        Diff the `contracts java-clean-imports|dart-scaffold` corpus
  java             Diff the `java validate-annotations` corpus
  env              Diff the `env init|backup|restore` corpus (synthetic repos)
  doctor           Diff the `doctor` corpus (same-machine tool probes)

  With no COMMAND, every command's corpus runs.

NOTE on git pre-commit:
  The pre-commit orchestrator shells out to docker/nx/npx/npm/git on its
  success path, mutating the working tree and producing environment-dependent
  output — it cannot be diffed deterministically. The corpus therefore runs
  ONLY the "outside a git repository" failure path (from a fresh temp dir with
  no .git), whose output and exit code are deterministic and byte-identical.

NOTE on contracts / java fixtures:
  Both `contracts` subcommands WRITE files (cleaned Java sources / generated
  Dart scaffold) and `java validate-annotations` is read-only. To compare the
  two binaries on identical inputs, each contracts case builds TWO sibling
  fixture trees (one per binary) and diffs both the streams AND the resulting
  on-disk files. All fixtures live under the OS temp dir and never touch the
  real repo. JSON `timestamp` fields are normalised before comparison.

NOTE on agents sync:
  The `agents sync` corpus always passes `--dry-run` so the real `.opencode/`
  tree is never mutated by the harness. Byte-parity of the ACTUAL generated
  tree is verified separately (run the Rust binary's `agents sync` without
  --dry-run from the repo root and confirm `git status --short .opencode/` is
  empty). The text/markdown `Duration:` line and JSON `duration_ms`/`timestamp`
  fields are wall-clock dependent, so all are normalised before comparison.

NOTE on env corpus:
  The env commands MUTATE the filesystem (copy .env files into/out of a backup
  dir). To diff the two binaries on identical inputs, each env case seeds TWO
  byte-identical synthetic fixtures (a fake git repo + an external backup dir)
  inside the OS temp dir — one per binary — runs each binary against its own
  fixture, then diffs BOTH the streams AND the resulting on-disk trees. The real
  repo's .env files are NEVER touched. Because both fixtures use the same
  relative layout, the JSON `dir` field is normalised to mask the per-binary
  temp path before comparison.

NOTE on doctor corpus:
  doctor probes the host's installed tool versions, which are environment-
  specific BUT identical across the two binaries on the same machine — so a
  direct same-machine diff IS valid. The JSON `timestamp`/`duration_ms` fields
  and the text/markdown `Duration:`/`**Generated**:` lines are wall-clock
  dependent and are normalised before comparison.

NOTE on crud-spec-coverage corpus:
  The original `spec-coverage` corpus only validates the rhino CLI's own gherkin
  tree against the Go app, which exercises a narrow slice of the per-language
  step extractors. The polyglot crud backends/frontends (Kotlin, Java, Python,
  Elixir, F#, C#, Clojure, Dart, TS, Rust, Go) each carry step definitions whose
  regex/Cucumber-expression shapes stress different extractor code paths. This
  corpus runs `spec-coverage validate` for EVERY crud app — with the precise
  args its project.json target uses — and diffs Go vs Rust. It is the permanent
  regression guard for per-language extraction divergence (e.g. RE2-vs-Rust
  literal-brace handling in Kotlin marker patterns). Apps absent from the
  checkout are skipped with a logged notice rather than failing the run.

NOTE on docs text/markdown output:
  The Go `docs validate-mermaid` (and validate-links) TEXT formatter groups
  findings into a Go map and ranges it, so the file ordering is
  NON-DETERMINISTIC across runs whenever 2+ files have findings — the Go binary
  cannot even match itself. The Rust port emits sorted (deterministic) output.
  The docs corpus therefore restricts text finding-cases to a SINGLE file
  (where ordering is trivially identical) and exercises all multi-file finding
  cases via JSON or markdown: the mermaid JSON and MARKDOWN formatters iterate
  the violations/warnings slices directly (discovery order), so both ARE
  deterministic and match byte-for-byte. The live tree carries mermaid
  findings at default thresholds (cleaned in a later phase), so repo-wide
  default-scan cases — including -q/-v, whose suppression only applies when
  there are NO findings — must avoid TEXT until that cleanup lands; -q/-v
  text parity is covered by fixture-confined scans instead.
  validate-heading-hierarchy is exempt from this restriction: BOTH binaries
  emit findings in discovery order from a lexically sorted walk (Go
  filepath.WalkDir / Rust WalkDir.sort_by_file_name), so multi-file
  text/markdown finding cases are deterministic and diffable.

NOTE on mermaid fixtures:
  The pipe-labeled-edge and cyclic-diagram parser fixtures live under
  docs/.shadow-mermaid-fixtures/ (the repo-wide walk only skips the named
  noise dirs, not hidden dirs, so both the default scan and a positional scan
  see them). Both fixtures are CLEAN at default thresholds (chain span 1,
  depth 2/3), so they add no findings to the default scan — the fixture-aware
  default-scan cases still run JSON/markdown only because the LIVE tree
  carries findings (see NOTE on docs text/markdown output). The computed
  span/depth is surfaced via the threshold device --max-width 0 --max-depth 1
  (the integration-test trick), which turns the rank observation into a
  complex_diagram warning carrying the values. The tree is created after the
  single-file cases and deleted before the links fixtures are seeded, so no
  other docs case ever sees it.

NOTE on heading-hierarchy fixtures:
  validate-heading-hierarchy file selection is allowlist default-deny — only
  prose trees (docs/, repo-governance/, specs/, plans/ minus plans/done/,
  root-level *.md, apps|libs/<name>/README.md, apps|libs/<name>/docs/**) are
  ever scanned. A repo-root dot-dir like the links corpus' fixture tree would
  be default-denied and could never produce findings, so the heading fixtures
  live under docs/.shadow-heading-fixtures/ (the docs/ prefix is allowlisted;
  the repo-wide walk only skips the named noise dirs, not hidden dirs). The
  tree is created after the links fixtures are removed and deleted before the
  corpus returns, so no other docs case ever sees it.

BEHAVIOUR:
  Builds the Go binary (Nx dist) and the Rust binary (cargo --release), then
  for each corpus invocation compares stdout, stderr, and exit code. JSON
  timestamps are normalised before comparison. Exits non-zero on ANY diff.
EOF
}

# --- Resolve repo root (apps/rhino-cli-rust/scripts/ → up 3) ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

GO_DIR="${REPO_ROOT}/apps/rhino-cli-go"
RS_DIR="${REPO_ROOT}/apps/rhino-cli-rust"
GO_BIN="${GO_DIR}/dist/rhino-cli"
RS_BIN="${RS_DIR}/target/release/rhino-cli"

# --- Parse args ---
COMMANDS=()
for arg in "$@"; do
  case "${arg}" in
    --help | -h)
      print_help
      exit 0
      ;;
    test-coverage | spec-coverage | crud-spec-coverage | docs | agents | repo-governance | workflows | git | contracts | java | env | doctor)
      COMMANDS+=("${arg}")
      ;;
    *)
      echo "shadow-diff: unknown argument '${arg}'" >&2
      echo "Run 'shadow-diff.sh --help' for usage." >&2
      exit 2
      ;;
  esac
done
if [[ ${#COMMANDS[@]} -eq 0 ]]; then
  COMMANDS=(test-coverage spec-coverage crud-spec-coverage docs agents repo-governance workflows git contracts java env doctor)
fi

# --- Build both binaries ---
echo "shadow-diff: building Go binary…" >&2
( cd "${REPO_ROOT}" && CGO_ENABLED=0 go build -C "${GO_DIR}" -o dist/rhino-cli ) \
  || { echo "shadow-diff: Go build failed" >&2; exit 2; }
echo "shadow-diff: building Rust binary…" >&2
( cd "${RS_DIR}" && cargo build --release --quiet ) \
  || { echo "shadow-diff: Rust build failed" >&2; exit 2; }

[[ -x "${GO_BIN}" ]] || { echo "shadow-diff: Go binary missing at ${GO_BIN}" >&2; exit 2; }
[[ -x "${RS_BIN}" ]] || { echo "shadow-diff: Rust binary missing at ${RS_BIN}" >&2; exit 2; }

TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

# Mask wall-clock-dependent fields (JSON timestamp/duration_ms plus the text and
# markdown Duration lines emitted by the agents reporters) so the two runs can
# be compared. The Go `time.Duration` `%v` text rendering varies per run.
normalise() {
  sed -E \
    -e 's/"timestamp": "[^"]*"/"timestamp": "<TS>"/' \
    -e 's/"duration_ms": [0-9]+/"duration_ms": <D>/' \
    -e 's/^Duration: .*/Duration: <D>/' \
    -e 's/^- \*\*Duration\*\*: .*/- **Duration**: <D>/' \
    -e 's/^\*\*Generated\*\*: .*/**Generated**: <TS>/'
}

FAIL=0
CASE_NO=0

# run_case <label> <args...>
# Runs both binaries with identical args (from REPO_ROOT) and diffs the streams.
run_case() {
  local label="$1"; shift
  CASE_NO=$((CASE_NO + 1))

  # The binaries legitimately exit non-zero (failure/error cases), so disable
  # errexit around the invocations and capture the codes explicitly.
  set +e
  ( cd "${REPO_ROOT}" && "${GO_BIN}" "$@" ) > "${TMP}/go.out" 2> "${TMP}/go.err"
  local go_exit=$?
  ( cd "${REPO_ROOT}" && "${RS_BIN}" "$@" ) > "${TMP}/rs.out" 2> "${TMP}/rs.err"
  local rs_exit=$?
  set -e

  normalise < "${TMP}/go.out" > "${TMP}/go.out.n"
  normalise < "${TMP}/rs.out" > "${TMP}/rs.out.n"
  normalise < "${TMP}/go.err" > "${TMP}/go.err.n"
  normalise < "${TMP}/rs.err" > "${TMP}/rs.err.n"

  local diverged=0
  if ! diff -q "${TMP}/go.out.n" "${TMP}/rs.out.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stdout DIVERGED (args: $*)" >&2
    diff -u "${TMP}/go.out.n" "${TMP}/rs.out.n" | sed 's/^/    /' >&2 || true
  fi
  if ! diff -q "${TMP}/go.err.n" "${TMP}/rs.err.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stderr DIVERGED (args: $*)" >&2
    diff -u "${TMP}/go.err.n" "${TMP}/rs.err.n" | sed 's/^/    /' >&2 || true
  fi
  if [[ "${go_exit}" != "${rs_exit}" ]]; then
    diverged=1
    echo "✗ [${label}] exit code DIVERGED (go=${go_exit} rs=${rs_exit}, args: $*)" >&2
  fi

  if [[ "${diverged}" -eq 0 ]]; then
    echo "✓ [${label}] (exit ${go_exit})"
  else
    FAIL=1
  fi
}

# --- Corpus inputs (real repo artefacts) ---
GO_COVER="apps/rhino-cli-go/cover.out"
LCOV_A="apps/crud-be-rust-axum/coverage/lcov.info"
LCOV_B="apps/crud-be-ts-effect/coverage/lcov.info"
LCOV_C="apps/crud-fe-ts-nextjs/coverage/lcov.info"
JACOCO="apps/crud-be-java-springboot/target/site/jacoco/jacoco.xml"
GHERKIN="specs/apps/rhino/behavior/cli/gherkin"
GHERKIN_TC="specs/apps/rhino/behavior/cli/gherkin/test-coverage"

corpus_test_coverage() {
  echo "── test-coverage corpus ──" >&2
  # validate: each format × format-detector
  for fmt in text json markdown; do
    run_case "tc validate go ${fmt}"     test-coverage validate "${GO_COVER}" 85 -o "${fmt}" --no-color
    run_case "tc validate lcov ${fmt}"   test-coverage validate "${LCOV_A}"  85 -o "${fmt}" --no-color
  done
  # JaCoCo only if the report exists (Java build is optional in some checkouts)
  if [[ -f "${REPO_ROOT}/${JACOCO}" ]]; then
    for fmt in text json markdown; do
      run_case "tc validate jacoco ${fmt}" test-coverage validate "${JACOCO}" 85 -o "${fmt}" --no-color
    done
  fi
  # validate: pass/fail/edge thresholds
  run_case "tc validate pass-high"  test-coverage validate "${GO_COVER}" 50 --no-color
  run_case "tc validate fail"       test-coverage validate "${GO_COVER}" 99 --no-color
  run_case "tc validate fail json"  test-coverage validate "${GO_COVER}" 99 -o json --no-color
  run_case "tc validate fail md"    test-coverage validate "${GO_COVER}" 99 -o markdown --no-color
  # validate: per-file
  run_case "tc validate per-file"        test-coverage validate "${LCOV_A}" 85 --per-file --no-color
  run_case "tc validate per-file json"   test-coverage validate "${LCOV_A}" 85 --per-file -o json --no-color
  run_case "tc validate per-file md"     test-coverage validate "${LCOV_A}" 85 --per-file -o markdown --no-color
  run_case "tc validate per-file below"  test-coverage validate "${LCOV_A}" 85 --per-file --below-threshold 90 --no-color
  # validate: exclude
  run_case "tc validate exclude"  test-coverage validate "${LCOV_A}" 85 --exclude "*.test.ts" --per-file --no-color
  # validate: error cases
  run_case "tc validate missing"          test-coverage validate "does/not/exist.out" 85 --no-color
  run_case "tc validate bad-threshold"    test-coverage validate "${GO_COVER}" not-a-number --no-color

  # diff: no-change path (HEAD...HEAD) and bad base
  for fmt in text json markdown; do
    run_case "tc diff head ${fmt}"  test-coverage diff "${GO_COVER}" --base HEAD -o "${fmt}" --no-color
  done
  run_case "tc diff bad-base"  test-coverage diff "${GO_COVER}" --base nonexistent-ref-xyz --no-color
  run_case "tc diff threshold-met"  test-coverage diff "${GO_COVER}" --base HEAD --threshold 50 --no-color

  # merge: two LCOV files, each format
  for fmt in text json markdown; do
    run_case "tc merge lcov ${fmt}"  test-coverage merge "${LCOV_A}" "${LCOV_B}" -o "${fmt}" --no-color
  done
  run_case "tc merge out-file"   test-coverage merge "${LCOV_A}" "${LCOV_B}" --out-file "${TMP_OUT_REL}" --no-color
  run_case "tc merge validate-pass"  test-coverage merge "${LCOV_A}" "${LCOV_B}" --validate 1 --no-color
  run_case "tc merge validate-fail"  test-coverage merge "${LCOV_A}" "${LCOV_B}" --validate 99 --no-color
  run_case "tc merge parse-error"    test-coverage merge "nope1.info" "nope2.info" --no-color
}

corpus_spec_coverage() {
  echo "── spec-coverage corpus ──" >&2
  # shared-steps against the real gherkin tree + go app (all covered → success)
  for fmt in text json markdown; do
    run_case "sc shared ${fmt}"  spec-coverage validate "${GHERKIN}" apps/rhino-cli-go --shared-steps -o "${fmt}" --no-color
  done
  run_case "sc shared quiet"  spec-coverage validate "${GHERKIN}" apps/rhino-cli-go --shared-steps -q --no-color
  # one-to-one mode against a dir with no test files → file gaps (failure path)
  for fmt in text json markdown; do
    run_case "sc gaps ${fmt}"  spec-coverage validate "${GHERKIN_TC}" apps/rhino-cli-go/scripts -o "${fmt}" --no-color
  done
  run_case "sc gaps quiet"  spec-coverage validate "${GHERKIN_TC}" apps/rhino-cli-go/scripts -q --no-color
  # exclude-dir
  run_case "sc shared exclude-dir"  spec-coverage validate "${GHERKIN}" apps/rhino-cli-go --shared-steps --exclude-dir system --no-color
}

# Per-language crud spec-coverage parity. Each entry mirrors the exact args the
# app's project.json `spec-coverage` target invokes. Only the trailing
# positional gherkin-dir / app-dir and the per-app flag set vary, so the corpus
# is data-driven: "<app-dir>|<gherkin-dir>|<extra-flags>". Apps missing from the
# checkout are skipped (logged) rather than failing the run.
corpus_crud_spec_coverage() {
  echo "── crud-spec-coverage corpus ──" >&2
  local be="specs/apps/crud/behavior/be/gherkin"
  local web="specs/apps/crud/behavior/web/gherkin"

  # All backends: --shared-steps --exclude-dir test-support against the BE tree.
  local backends=(
    crud-be-golang-gin
    crud-be-rust-axum
    crud-be-java-springboot
    crud-be-java-vertx
    crud-be-python-fastapi
    crud-be-clojure-pedestal
    crud-be-csharp-aspnetcore
    crud-be-ts-effect
    crud-be-kotlin-ktor
    crud-be-elixir-phoenix
    crud-be-fsharp-giraffe
  )
  local app
  for app in "${backends[@]}"; do
    if [[ ! -d "${REPO_ROOT}/apps/${app}" ]]; then
      echo "shadow-diff: apps/${app} absent — skipping" >&2
      continue
    fi
    run_case "crud-sc ${app}" \
      spec-coverage validate --shared-steps --exclude-dir test-support \
      "${be}" "apps/${app}" --no-color
  done

  # TS frontends: --shared-steps --exclude-dir test-support against the WEB tree.
  local ts_frontends=(crud-fe-ts-nextjs crud-fe-ts-tanstack-start)
  for app in "${ts_frontends[@]}"; do
    if [[ ! -d "${REPO_ROOT}/apps/${app}" ]]; then
      echo "shadow-diff: apps/${app} absent — skipping" >&2
      continue
    fi
    run_case "crud-sc ${app}" \
      spec-coverage validate --shared-steps --exclude-dir test-support \
      "${web}" "apps/${app}" --no-color
  done

  # Dart frontend: --shared-steps (NO --exclude-dir) against the WEB tree.
  if [[ -d "${REPO_ROOT}/apps/crud-fe-dart-flutterweb" ]]; then
    run_case "crud-sc crud-fe-dart-flutterweb" \
      spec-coverage validate --shared-steps \
      "${web}" apps/crud-fe-dart-flutterweb --no-color
  else
    echo "shadow-diff: apps/crud-fe-dart-flutterweb absent — skipping" >&2
  fi
}

corpus_docs() {
  echo "── docs corpus ──" >&2

  # --- validate-links: full default corpus, every format. The link reporter ---
  # --- sorts categories (fixed order), files (alphabetical), and lines, so ----
  # --- output is deterministic even with findings (counts/TS normalised). -----
  for fmt in text json markdown; do
    run_case "links default ${fmt}"  docs validate-links -o "${fmt}" --no-color
  done
  run_case "links quiet"    docs validate-links -q --no-color
  run_case "links verbose"  docs validate-links -v --no-color
  # staged-only on a clean tree → no staged files → success, identical output.
  run_case "links staged-only"  docs validate-links --staged-only --no-color

  # --- validate-mermaid: full default corpus. The live tree has findings in --
  # --- many files at default thresholds (cleaned in a later phase), so every --
  # --- repo-wide / multi-finding-file case runs JSON or markdown — never ------
  # --- text (see NOTE on docs text/markdown output). --------------------------
  for fmt in json markdown; do
    run_case "mermaid default ${fmt}"  docs validate-mermaid -o "${fmt}" --no-color
  done
  # -q/-v over the repo-wide scan: findings exist, so -q does NOT suppress and
  # the text path would hit Go map ordering — exercise the flags via JSON; the
  # text -q/-v paths are covered by the fixture-confined cases further down.
  run_case "mermaid quiet json"    docs validate-mermaid -q -o json --no-color
  run_case "mermaid verbose json"  docs validate-mermaid -v -o json --no-color
  # Scoped to individual real directories. docs/ has findings in 2+ files →
  # markdown/JSON only; repo-governance/ has findings in exactly one file and
  # .claude/ has none, so their text scans stay deterministic.
  run_case "mermaid docs dir md"     docs validate-mermaid docs -o markdown --no-color
  run_case "mermaid docs dir json"   docs validate-mermaid docs -o json --no-color
  run_case "mermaid repo-gov dir"    docs validate-mermaid repo-governance --no-color
  run_case "mermaid .claude dir"     docs validate-mermaid .claude --no-color
  run_case "mermaid multi dir json"  docs validate-mermaid docs repo-governance .claude -o json --no-color
  # staged-only on a clean tree → no staged files → deterministic empty scan.
  # changed-only falls back to the repo-wide default scan (no upstream delta)
  # → findings in many files → JSON only.
  run_case "mermaid staged-only"        docs validate-mermaid --staged-only --no-color
  run_case "mermaid changed-only json"  docs validate-mermaid --changed-only -o json --no-color

  # --- Finding cases: JSON only across the whole corpus (deterministic slice ---
  # --- order). These exercise every violation/warning kind on real files. -----
  for thr in 1 2 3; do
    run_case "mermaid max-width ${thr} json"        docs validate-mermaid docs --max-width "${thr}" -o json --no-color
  done
  run_case "mermaid max-label-len 10 json"     docs validate-mermaid docs --max-label-len 10 -o json --no-color
  run_case "mermaid max-label-len 20 json"     docs validate-mermaid docs --max-label-len 20 -o json --no-color
  run_case "mermaid max-subgraph-nodes 1 json" docs validate-mermaid docs --max-subgraph-nodes 1 -o json --no-color
  run_case "mermaid max-subgraph-nodes 3 json" docs validate-mermaid docs --max-subgraph-nodes 3 -o json --no-color
  run_case "mermaid both-exceeded json"        docs validate-mermaid docs --max-width 1 --max-depth 1 -o json --no-color

  # --- Single-file text/markdown finding cases (deterministic: one file). -----
  # Resolve a single real markdown file that contains a width violation at
  # --max-width 1 by asking the Go binary (the parity oracle) for the first
  # filePath in its JSON output, then converting to a repo-relative path.
  # Disable errexit/pipefail around this resolution: `grep | head` legitimately
  # closes the pipe early, which would otherwise surface as SIGPIPE (exit 141).
  local one_file go_json
  set +e +o pipefail
  go_json="$( cd "${REPO_ROOT}" && "${GO_BIN}" docs validate-mermaid docs --max-width 1 -o json --no-color 2>/dev/null )"
  one_file="$( printf '%s' "${go_json}" | grep -o '"filePath": "[^"]*"' | head -1 | sed 's/"filePath": "//; s/"$//' )"
  set -e -o pipefail
  if [[ -n "${one_file}" ]]; then
    local rel_file="${one_file#"${REPO_ROOT}"/}"
    run_case "mermaid one-file width text"  docs validate-mermaid "${rel_file}" --max-width 1 --no-color
    run_case "mermaid one-file width md"    docs validate-mermaid "${rel_file}" --max-width 1 -o markdown --no-color
    run_case "mermaid one-file width json"  docs validate-mermaid "${rel_file}" --max-width 1 -o json --no-color
    run_case "mermaid one-file label text"  docs validate-mermaid "${rel_file}" --max-label-len 5 --no-color
    run_case "mermaid one-file subgraph text"  docs validate-mermaid "${rel_file}" --max-subgraph-nodes 1 --no-color
  else
    echo "shadow-diff: no single-file mermaid finding found in corpus; skipping single-file text cases" >&2
  fi

  # --- Mermaid parser fixtures: pipe-labeled edge + cyclic diagram (see NOTE ---
  # --- on mermaid fixtures). Both are CLEAN at default thresholds; the ---------
  # --- --max-width 0 --max-depth 1 device surfaces the computed span/depth ----
  # --- in a complex_diagram warning, proving edge parsing and back-edge -------
  # --- removal rank identically in both binaries. ------------------------------
  local mf_dir="docs/.shadow-mermaid-fixtures"
  local mf_abs="${REPO_ROOT}/${mf_dir}"
  rm -rf "${mf_abs}"
  mkdir -p "${mf_abs}"
  printf '# Pipe Fixture\n\n```mermaid\nflowchart TD\n    A -->|text| B\n```\n' > "${mf_abs}/pipe.md"
  printf '# Cycle Fixture\n\n```mermaid\nflowchart TD\n    A-->B-->C-->A\n```\n' > "${mf_abs}/cycle.md"

  # Pipe-labeled edge: chain A->B → span 1, depth 2. A parser that drops the
  # |text| edge (and node B) would report different values. Single file →
  # text/markdown ordering is trivially deterministic.
  for fmt in text json markdown; do
    run_case "mermaid fixture pipe ${fmt}" \
      docs validate-mermaid "${mf_dir}/pipe.md" --max-width 0 --max-depth 1 -o "${fmt}" --no-color
  done
  # Cycle A->B->C->A: after back-edge removal the chain ranks span 1, depth 3.
  # The old fallback ranked all nodes 0 (span 3).
  for fmt in text json markdown; do
    run_case "mermaid fixture cycle ${fmt}" \
      docs validate-mermaid "${mf_dir}/cycle.md" --max-width 0 --max-depth 1 -o "${fmt}" --no-color
  done
  # Both fixture files have findings → multi-file case is JSON-only (see NOTE
  # on docs text/markdown output).
  run_case "mermaid fixture dir json" \
    docs validate-mermaid "${mf_dir}" --max-width 0 --max-depth 1 -o json --no-color

  # -q/-v TEXT parity, fixture-confined so output stays deterministic: the
  # fixture dir is clean at default thresholds (quiet → empty, verbose →
  # summary only), and the single-file threshold runs print findings under
  # both flags (one file → trivially ordered).
  run_case "mermaid fixture quiet"    docs validate-mermaid "${mf_dir}" -q --no-color
  run_case "mermaid fixture verbose"  docs validate-mermaid "${mf_dir}" -v --no-color
  run_case "mermaid fixture pipe quiet" \
    docs validate-mermaid "${mf_dir}/pipe.md" --max-width 0 --max-depth 1 -q --no-color
  run_case "mermaid fixture pipe verbose" \
    docs validate-mermaid "${mf_dir}/pipe.md" --max-width 0 --max-depth 1 -v --no-color

  # Repo-wide default scan with the fixtures present: live findings span many
  # files → JSON/markdown only; the file/block counts prove the default walk
  # picked the fixture tree up identically in both binaries.
  run_case "mermaid default fixtures md"    docs validate-mermaid -o markdown --no-color
  run_case "mermaid default fixtures json"  docs validate-mermaid -o json --no-color
  # Repo-wide default scan with the threshold device: every live diagram's
  # span/depth is computed and compared. Findings span many files → JSON only
  # (deterministic slice order).
  run_case "mermaid default thresholds json" \
    docs validate-mermaid --max-width 0 --max-depth 1 -o json --no-color

  # --exclude that CHANGES the result: excluding the fixture tree drops both
  # fixture files from the default scan (file/block counts shrink vs the
  # "default fixtures" cases above). Trailing-slash, slash-less, and repeated
  # forms; JSON counts prove the filter took effect identically. Repo-wide →
  # JSON/markdown only (live findings, see NOTE).
  run_case "mermaid exclude fixture md"     docs validate-mermaid --exclude "${mf_dir}/" -o markdown --no-color
  run_case "mermaid exclude fixture json"   docs validate-mermaid --exclude "${mf_dir}/" -o json --no-color
  run_case "mermaid exclude noslash json"   docs validate-mermaid --exclude "${mf_dir}" -o json --no-color
  run_case "mermaid exclude repeat json"    docs validate-mermaid --exclude "${mf_dir}/" --exclude repo-governance/ -o json --no-color
  # Positional dir + file-prefix --exclude: only the pipe fixture remains (one
  # finding file → text is deterministic too).
  run_case "mermaid dir+exclude text" \
    docs validate-mermaid "${mf_dir}" --exclude "${mf_dir}/cycle.md" --max-width 0 --max-depth 1 --no-color
  run_case "mermaid dir+exclude json" \
    docs validate-mermaid "${mf_dir}" --exclude "${mf_dir}/cycle.md" --max-width 0 --max-depth 1 -o json --no-color

  rm -rf "${mf_abs}"

  # --- Anchor + --exclude fixtures: synthetic tree UNDER the repo root (the ---
  # --- repo-wide walk starts at the git root, so out-of-repo temp files are ---
  # --- never scanned). All findings live in ONE file and ONE category --------
  # --- (broken-anchor), so text/markdown ordering is deterministic (see NOTE).
  # --- Link URLs deliberately avoid the skip-list substrings (link/target). ---
  local lf_dir=".shadow-links-fixtures"
  local lf_abs="${REPO_ROOT}/${lf_dir}"
  rm -rf "${lf_abs}"
  mkdir -p "${lf_abs}"
  printf '# Dest\n\n## Real Section\n\nBody.\n' > "${lf_abs}/dest.md"
  printf '# Source\n\n## Local Heading\n\n[ok](./dest.md#real-section)\n\n[bad](./dest.md#missing-section)\n\n[self ok](#local-heading)\n\n[self bad](#nonexistent-heading)\n' > "${lf_abs}/source.md"

  # Anchor findings: a broken #anchor to an existing file plus a same-file
  # pure-anchor broken link (2 findings), alongside a valid cross-file #anchor
  # and a valid same-file pure anchor (0 findings from those two).
  for fmt in text json markdown; do
    run_case "links anchors ${fmt}"  docs validate-links -o "${fmt}" --no-color
  done
  run_case "links anchors quiet"    docs validate-links -q --no-color
  run_case "links anchors verbose"  docs validate-links -v --no-color

  # --exclude that CHANGES the result: excluding the fixture tree removes both
  # fixture findings (broken count drops by 2 vs the anchors cases above). The
  # repeatable form and a non-fixture exclusion are diffed via JSON so
  # total_files/total_links/broken_count prove the filter took effect
  # identically in both binaries.
  run_case "links exclude fixture"       docs validate-links --exclude "${lf_dir}/" --no-color
  run_case "links exclude fixture json"  docs validate-links --exclude "${lf_dir}/" -o json --no-color
  run_case "links exclude repeat json"   docs validate-links --exclude "${lf_dir}/" --exclude docs/ -o json --no-color
  run_case "links exclude other json"    docs validate-links --exclude docs/ -o json --no-color

  rm -rf "${lf_abs}"

  # --- validate-heading-hierarchy: full default corpus (real prose trees), ----
  # --- every format + verbosity. Findings come out in discovery order from a --
  # --- lexically sorted walk in BOTH binaries, so output is deterministic ----
  # --- even with findings (see NOTE on heading-hierarchy fixtures). -----------
  for fmt in text json markdown; do
    run_case "headings default ${fmt}"  docs validate-heading-hierarchy -o "${fmt}" --no-color
  done
  run_case "headings quiet"    docs validate-heading-hierarchy -q --no-color
  run_case "headings verbose"  docs validate-heading-hierarchy -v --no-color
  # staged-only on a clean tree → no staged files → success, identical output.
  run_case "headings staged-only"  docs validate-heading-hierarchy --staged-only --no-color

  # --- Heading fixtures: synthetic tree UNDER docs/ (a repo-root dot-dir is ---
  # --- default-denied by the prose allowlist and would yield zero findings). --
  # --- One deterministic finding per kind plus a fence-clean control file. ----
  local hf_dir="docs/.shadow-heading-fixtures"
  local hf_abs="${REPO_ROOT}/${hf_dir}"
  rm -rf "${hf_abs}"
  mkdir -p "${hf_abs}/nested"
  printf '# Clean Doc\n\n```bash\n# fenced pseudo h1\n### fenced pseudo h3\n```\n\n## Real Section\n' > "${hf_abs}/aaa-clean.md"
  printf '# First Title\n\nbody\n\n# Second Title\n' > "${hf_abs}/dup-h1.md"
  printf '## Only A Section\n\nbody\n' > "${hf_abs}/missing-h1.md"
  printf '# Nested Doc\n\n#### Deep Jump\n' > "${hf_abs}/nested/deep.md"
  printf '# Title\n\n### Jumped Here\n' > "${hf_abs}/skip-level.md"

  # Full scan with fixture findings present: duplicate-h1, missing-h1, and
  # skipped-level all fire; the fenced control file stays clean. Multi-file
  # text/markdown IS deterministic for this validator (see NOTE).
  for fmt in text json markdown; do
    run_case "headings fixtures ${fmt}"  docs validate-heading-hierarchy -o "${fmt}" --no-color
  done
  run_case "headings fixtures quiet"    docs validate-heading-hierarchy -q --no-color
  run_case "headings fixtures verbose"  docs validate-heading-hierarchy -v --no-color

  # Positional-path variants: directory walk, single file, multi-arg mix, and
  # a default-denied tree (allowlist filters every candidate → success).
  for fmt in text json markdown; do
    run_case "headings dir ${fmt}"  docs validate-heading-hierarchy "${hf_dir}" -o "${fmt}" --no-color
  done
  run_case "headings one-file dup text"   docs validate-heading-hierarchy "${hf_dir}/dup-h1.md" --no-color
  run_case "headings one-file dup md"     docs validate-heading-hierarchy "${hf_dir}/dup-h1.md" -o markdown --no-color
  run_case "headings one-file dup json"   docs validate-heading-hierarchy "${hf_dir}/dup-h1.md" -o json --no-color
  run_case "headings one-file clean"      docs validate-heading-hierarchy "${hf_dir}/aaa-clean.md" --no-color
  run_case "headings multi-path json"     docs validate-heading-hierarchy "${hf_dir}/missing-h1.md" "${hf_dir}/nested" -o json --no-color
  run_case "headings denied tree"         docs validate-heading-hierarchy .claude --no-color
  run_case "headings denied tree json"    docs validate-heading-hierarchy .claude -o json --no-color

  # --exclude that CHANGES the result: excluding the fixture tree must restore
  # the pre-fixture default output (total_findings drops by the 4 fixture
  # findings); JSON total_findings proves the filter took effect identically
  # in both binaries. Repeated and slash-less forms too.
  run_case "headings exclude fixture"       docs validate-heading-hierarchy --exclude "${hf_dir}/" --no-color
  run_case "headings exclude fixture json"  docs validate-heading-hierarchy --exclude "${hf_dir}/" -o json --no-color
  run_case "headings exclude noslash json"  docs validate-heading-hierarchy --exclude "${hf_dir}" -o json --no-color
  run_case "headings exclude repeat json"   docs validate-heading-hierarchy --exclude "${hf_dir}/" --exclude repo-governance/ -o json --no-color
  run_case "headings exclude docs json"     docs validate-heading-hierarchy --exclude docs/ -o json --no-color
  run_case "headings dir+exclude json"      docs validate-heading-hierarchy "${hf_dir}" --exclude "${hf_dir}/nested/" -o json --no-color

  rm -rf "${hf_abs}"
}

corpus_agents() {
  echo "── agents corpus ──" >&2

  # --- agents sync: ALWAYS --dry-run so the real .opencode/ tree is untouched.
  # --- Every format + the flag matrix. (Skills are never copied, so
  # --- --skills-only yields an empty result; --agents-only mirrors the default
  # --- on this repo since there are no skill copies.) -------------------------
  for fmt in text json markdown; do
    run_case "agents sync dry-run ${fmt}"  agents sync --dry-run -o "${fmt}" --no-color
  done
  run_case "agents sync dry-run quiet"        agents sync --dry-run -q --no-color
  run_case "agents sync dry-run verbose"      agents sync --dry-run -v --no-color
  run_case "agents sync dry-run agents-only"  agents sync --dry-run --agents-only --no-color
  run_case "agents sync dry-run skills-only"  agents sync --dry-run --skills-only --no-color
  run_case "agents sync dry-run skills-only json"  agents sync --dry-run --skills-only -o json --no-color
  # Flag conflict (error path: both --agents-only and --skills-only).
  run_case "agents sync dry-run conflict"  agents sync --dry-run --agents-only --skills-only --no-color

  # --- agents validate-claude: every format + the flag matrix (real tree). ----
  for fmt in text json markdown; do
    run_case "agents validate-claude ${fmt}"  agents validate-claude -o "${fmt}" --no-color
  done
  run_case "agents validate-claude quiet"        agents validate-claude -q --no-color
  run_case "agents validate-claude verbose"      agents validate-claude -v --no-color
  run_case "agents validate-claude verbose json" agents validate-claude -v -o json --no-color
  run_case "agents validate-claude agents-only"  agents validate-claude --agents-only --no-color
  run_case "agents validate-claude skills-only"  agents validate-claude --skills-only --no-color
  run_case "agents validate-claude agents-only json" agents validate-claude --agents-only -o json --no-color
  # Flag conflict (error path).
  run_case "agents validate-claude conflict"  agents validate-claude --agents-only --skills-only --no-color

  # --- agents validate-sync: every format + verbosity (real tree). ------------
  for fmt in text json markdown; do
    run_case "agents validate-sync ${fmt}"  agents validate-sync -o "${fmt}" --no-color
  done
  run_case "agents validate-sync quiet"        agents validate-sync -q --no-color
  run_case "agents validate-sync verbose"      agents validate-sync -v --no-color
  run_case "agents validate-sync verbose md"   agents validate-sync -v -o markdown --no-color

  # --- agents validate-naming: every format + verbosity (real tree). ----------
  for fmt in text json markdown; do
    run_case "agents validate-naming ${fmt}"  agents validate-naming -o "${fmt}" --no-color
  done
  run_case "agents validate-naming quiet"    agents validate-naming -q --no-color
  run_case "agents validate-naming verbose"  agents validate-naming -v --no-color

  # --- agents emit-bindings: ALWAYS --dry-run so the real .amazonq/ tree is
  # --- untouched. The emitter is deterministic from AGENTS.md, so dry-run output
  # --- is byte-identical across both implementations. ------------------------
  run_case "agents emit-bindings dry-run"  agents emit-bindings --dry-run --no-color

  # --- agents validate-bindings: read-only deterministic guard over the real
  # --- tree (no network, no agent). Passes when the committed bridge matches a
  # --- regenerate from AGENTS.md and the catalog covers every binding dir. ----
  run_case "agents validate-bindings"  agents validate-bindings --no-color
}

corpus_repo_governance() {
  echo "── repo-governance corpus ──" >&2

  # --- Clean real-repo targets: the live governance tree and the root
  # --- instruction surfaces are vendor-neutral, so every format yields the
  # --- PASSED output. These exercise the directory-walk and single-file paths
  # --- against real artefacts. -----------------------------------------------
  for fmt in text json markdown; do
    run_case "vendor-audit repo-governance ${fmt}"  repo-governance vendor-audit repo-governance/ -o "${fmt}" --no-color
  done
  run_case "vendor-audit default"        repo-governance vendor-audit --no-color
  run_case "vendor-audit AGENTS.md"      repo-governance vendor-audit AGENTS.md --no-color
  run_case "vendor-audit CLAUDE.md"      repo-governance vendor-audit CLAUDE.md --no-color
  run_case "vendor-audit AGENTS.md json" repo-governance vendor-audit AGENTS.md -o json --no-color
  run_case "vendor-audit docs dir"       repo-governance vendor-audit docs/ --no-color
  run_case "vendor-audit nonexistent"    repo-governance vendor-audit does/not/exist --no-color

  # --- Synthetic finding cases: write fixtures UNDER the repo root (the path
  # --- argument is joined under the git root by the binary, so out-of-repo
  # --- temp files never resolve). The fixtures exercise every exemption region
  # --- and the failure/error path in all three formats. The directory walks
  # --- multiple files to confirm lexical ordering parity. ---------------------
  local fix_dir=".shadow-vendor-fixtures"
  local fix_abs="${REPO_ROOT}/${fix_dir}"
  rm -rf "${fix_abs}"
  mkdir -p "${fix_abs}/nested"
  printf '# Doc\n\nWe use Claude Code daily.\nThe .opencode/ path matters.\n' > "${fix_abs}/aaa.md"
  printf '# Other\n\nWe rely on Skills and OpenAI here.\n' > "${fix_abs}/bbb.md"
  printf '# Nested\n\nGemini and Sonnet and Opus appear.\n' > "${fix_abs}/nested/ccc.md"
  # Exemption coverage: fence, frontmatter, inline code, link, PB heading — clean.
  printf -- '---\ntitle: Claude Code\n---\n\n```\nClaude Code\n```\n\nUse `Claude Code` inline.\n\n[x](https://e.com/Claude-Code)\n\n## Platform Binding Examples\n\nClaude Code is fine.\n' > "${fix_abs}/clean.md"

  for fmt in text json markdown; do
    run_case "vendor-audit fixture file ${fmt}"  repo-governance vendor-audit "${fix_dir}/aaa.md" -o "${fmt}" --no-color
    run_case "vendor-audit fixture dir ${fmt}"   repo-governance vendor-audit "${fix_dir}" -o "${fmt}" --no-color
  done
  run_case "vendor-audit fixture clean"  repo-governance vendor-audit "${fix_dir}/clean.md" --no-color

  rm -rf "${fix_abs}"

  # --- gherkin-keyword-cardinality: default repo scan + error path. Both
  # --- binaries emit findings sorted by (path, line) from a lexical walk, so
  # --- the default scan is deterministic and diffable even while the live tree
  # --- carries offenders (the retrofit phases clean them later). --------------
  run_case "gherkin-cardinality default"      repo-governance gherkin-keyword-cardinality --no-color
  run_case "gherkin-cardinality nonexistent"  repo-governance gherkin-keyword-cardinality does/not/exist --no-color

  # --- Synthetic gherkin fixtures UNDER the repo root (the path argument is
  # --- joined under the git root by both binaries). One violating + one
  # --- conforming feature file, each across every format, plus the directory
  # --- walk over both. --------------------------------------------------------
  local gkc_dir=".shadow-gherkin-fixtures"
  local gkc_abs="${REPO_ROOT}/${gkc_dir}"
  rm -rf "${gkc_abs}"
  mkdir -p "${gkc_abs}"
  printf 'Feature: Fixture\n\n  Scenario: Double when offender\n    Given a start\n    When the first action runs\n    When the second action runs\n    Then the outcome is checked\n' > "${gkc_abs}/violating.feature"
  printf 'Feature: Fixture\n\n  Scenario: Conforming chained scenario\n    Given a start\n    And another precondition\n    When the action runs\n    Then the outcome is checked\n' > "${gkc_abs}/conforming.feature"

  for fmt in text json markdown; do
    run_case "gherkin-cardinality violating ${fmt}"   repo-governance gherkin-keyword-cardinality "${gkc_dir}/violating.feature" -o "${fmt}" --no-color
    run_case "gherkin-cardinality conforming ${fmt}"  repo-governance gherkin-keyword-cardinality "${gkc_dir}/conforming.feature" -o "${fmt}" --no-color
  done
  run_case "gherkin-cardinality fixture dir"  repo-governance gherkin-keyword-cardinality "${gkc_dir}" --no-color

  rm -rf "${gkc_abs}"
}

corpus_workflows() {
  echo "── workflows corpus ──" >&2

  # --- Live governance tree: every workflow obeys the naming rule, so all
  # --- formats + verbosity yield the PASSED output deterministically. ---------
  for fmt in text json markdown; do
    run_case "validate-naming ${fmt}"  workflows validate-naming -o "${fmt}" --no-color
  done
  run_case "validate-naming quiet"    workflows validate-naming -q --no-color
  run_case "validate-naming verbose"  workflows validate-naming -v --no-color
  run_case "validate-naming verbose json"  workflows validate-naming -v -o json --no-color
}

# run_case_in_dir <label> <workdir> <args...>
# Like run_case, but runs both binaries from <workdir> instead of REPO_ROOT.
# Used by the git pre-commit error path (no .git in <workdir>).
run_case_in_dir() {
  local label="$1"; shift
  local workdir="$1"; shift
  CASE_NO=$((CASE_NO + 1))

  set +e
  ( cd "${workdir}" && "${GO_BIN}" "$@" ) > "${TMP}/go.out" 2> "${TMP}/go.err"
  local go_exit=$?
  ( cd "${workdir}" && "${RS_BIN}" "$@" ) > "${TMP}/rs.out" 2> "${TMP}/rs.err"
  local rs_exit=$?
  set -e

  normalise < "${TMP}/go.out" > "${TMP}/go.out.n"
  normalise < "${TMP}/rs.out" > "${TMP}/rs.out.n"
  normalise < "${TMP}/go.err" > "${TMP}/go.err.n"
  normalise < "${TMP}/rs.err" > "${TMP}/rs.err.n"

  local diverged=0
  if ! diff -q "${TMP}/go.out.n" "${TMP}/rs.out.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stdout DIVERGED (args: $*)" >&2
    diff -u "${TMP}/go.out.n" "${TMP}/rs.out.n" | sed 's/^/    /' >&2 || true
  fi
  if ! diff -q "${TMP}/go.err.n" "${TMP}/rs.err.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stderr DIVERGED (args: $*)" >&2
    diff -u "${TMP}/go.err.n" "${TMP}/rs.err.n" | sed 's/^/    /' >&2 || true
  fi
  if [[ "${go_exit}" != "${rs_exit}" ]]; then
    diverged=1
    echo "✗ [${label}] exit code DIVERGED (go=${go_exit} rs=${rs_exit}, args: $*)" >&2
  fi

  if [[ "${diverged}" -eq 0 ]]; then
    echo "✓ [${label}] (exit ${go_exit})"
  else
    FAIL=1
  fi
}

# run_pair_case <label> <go_dir> <rs_dir> <subcmd...>
# Runs the Go binary against <go_dir> and the Rust binary against <rs_dir> (their
# LAST positional arg respectively), diffing both the streams AND the on-disk
# trees the commands produce. The two fixture dirs must start byte-identical.
run_pair_case() {
  local label="$1"; shift
  local go_dir="$1"; shift
  local rs_dir="$1"; shift
  CASE_NO=$((CASE_NO + 1))

  set +e
  ( cd "${REPO_ROOT}" && "${GO_BIN}" "$@" "${go_dir}" ) > "${TMP}/go.out" 2> "${TMP}/go.err"
  local go_exit=$?
  ( cd "${REPO_ROOT}" && "${RS_BIN}" "$@" "${rs_dir}" ) > "${TMP}/rs.out" 2> "${TMP}/rs.err"
  local rs_exit=$?
  set -e

  normalise < "${TMP}/go.out" > "${TMP}/go.out.n"
  normalise < "${TMP}/rs.out" > "${TMP}/rs.out.n"

  local diverged=0
  if ! diff -q "${TMP}/go.out.n" "${TMP}/rs.out.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stdout DIVERGED (args: $* <dir>)" >&2
    diff -u "${TMP}/go.out.n" "${TMP}/rs.out.n" | sed 's/^/    /' >&2 || true
  fi
  if ! diff -q "${TMP}/go.err" "${TMP}/rs.err" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stderr DIVERGED (args: $* <dir>)" >&2
    diff -u "${TMP}/go.err" "${TMP}/rs.err" | sed 's/^/    /' >&2 || true
  fi
  if [[ "${go_exit}" != "${rs_exit}" ]]; then
    diverged=1
    echo "✗ [${label}] exit code DIVERGED (go=${go_exit} rs=${rs_exit})" >&2
  fi
  # Compare the resulting on-disk trees (the whole point of these commands).
  if ! diff -rq "${go_dir}" "${rs_dir}" > /dev/null; then
    diverged=1
    echo "✗ [${label}] GENERATED FILES DIVERGED" >&2
    diff -ru "${go_dir}" "${rs_dir}" | sed 's/^/    /' >&2 || true
  fi

  if [[ "${diverged}" -eq 0 ]]; then
    echo "✓ [${label}] (exit ${go_exit}, files identical)"
  else
    FAIL=1
  fi
}

corpus_git() {
  echo "── git corpus ──" >&2
  # Pre-commit: ONLY the deterministic error path. A fresh temp dir with no .git
  # makes findGitRoot fail before any external-tool step runs. Both binaries
  # print the same cobra usage block + error to stderr and exit 1.
  local nogit
  nogit="$(mktemp -d)"
  for fmt in text json markdown; do
    run_case_in_dir "git pre-commit no-repo ${fmt}" "${nogit}" git pre-commit -o "${fmt}" --no-color
  done
  run_case_in_dir "git pre-commit no-repo quiet"   "${nogit}" git pre-commit -q --no-color
  run_case_in_dir "git pre-commit no-repo verbose" "${nogit}" git pre-commit -v --no-color
  rm -rf "${nogit}"
}

# Populate a generated-contracts Java fixture at $1 (idempotent fresh tree).
seed_java_fixture() {
  local d="$1"
  mkdir -p "${d}"
  printf 'package com.foo;\nimport java.util.List;\nimport java.util.Map;\nimport com.foo.Helper;\nimport java.util.List;\n\nclass Foo { List x; Helper h; }\n' > "${d}/Foo.java"
  printf 'package com.bar;\nimport java.util.Set;\n\nclass Bar { Set s; }\n' > "${d}/Bar.java"
  mkdir -p "${d}/sub"
  printf 'package com.sub;\nimport java.time.Instant;\n\nclass Sub { Instant i; }\n' > "${d}/sub/Sub.java"
}

# Populate a generated-contracts Dart fixture at $1 with model files.
seed_dart_fixture() {
  local d="$1"
  mkdir -p "${d}/lib/model"
  printf '// user model\n'    > "${d}/lib/model/user.dart"
  printf '// account model\n' > "${d}/lib/model/account.dart"
  printf '// order model\n'   > "${d}/lib/model/order.dart"
}

corpus_contracts() {
  echo "── contracts corpus ──" >&2

  # --- java-clean-imports: every format × the flag matrix. Each case rebuilds
  # --- BOTH fixture trees so the in-place rewrite is compared file-for-file. ---
  for fmt in text json markdown; do
    for flags in "" "-v" "-q"; do
      local gd rd
      gd="$(mktemp -d)"; rd="$(mktemp -d)"
      seed_java_fixture "${gd}"; seed_java_fixture "${rd}"
      # shellcheck disable=SC2086
      run_pair_case "java-clean ${fmt} '${flags}'" "${gd}" "${rd}" contracts java-clean-imports -o "${fmt}" ${flags} --no-color
      rm -rf "${gd}" "${rd}"
    done
  done
  # Empty directory (no .java files → no modifications).
  local egd erd
  egd="$(mktemp -d)"; erd="$(mktemp -d)"
  run_pair_case "java-clean empty" "${egd}" "${erd}" contracts java-clean-imports --no-color
  rm -rf "${egd}" "${erd}"

  # --- dart-scaffold: with models and without models, every format × verbosity.
  for fmt in text json markdown; do
    for flags in "" "-v" "-q"; do
      local gd rd
      gd="$(mktemp -d)"; rd="$(mktemp -d)"
      seed_dart_fixture "${gd}"; seed_dart_fixture "${rd}"
      # shellcheck disable=SC2086
      run_pair_case "dart-scaffold models ${fmt} '${flags}'" "${gd}" "${rd}" contracts dart-scaffold -o "${fmt}" ${flags} --no-color
      rm -rf "${gd}" "${rd}"
    done
  done
  # No model files (barrel built with no part directives).
  for fmt in text json markdown; do
    local gd rd
    gd="$(mktemp -d)"; rd="$(mktemp -d)"
    run_pair_case "dart-scaffold no-models ${fmt}" "${gd}" "${rd}" contracts dart-scaffold -o "${fmt}" --no-color
    rm -rf "${gd}" "${rd}"
  done
  # Overwrite existing scaffold.
  local ogd ord
  ogd="$(mktemp -d)"; ord="$(mktemp -d)"
  for d in "${ogd}" "${ord}"; do
    mkdir -p "${d}/lib/model"
    printf 'name: old\n' > "${d}/pubspec.yaml"
    printf '// stale barrel\n' > "${d}/lib/crud_contracts.dart"
    printf '// user\n' > "${d}/lib/model/user.dart"
  done
  run_pair_case "dart-scaffold overwrite" "${ogd}" "${ord}" contracts dart-scaffold --no-color
  rm -rf "${ogd}" "${ord}"
}

corpus_java() {
  echo "── java corpus ──" >&2

  # validate-annotations is read-only, so a single shared fixture under TMP is
  # diffed across formats. The tree mixes all three states: valid, missing
  # package-info, and present-but-missing-annotation.
  local src="${TMP}/java-src"
  mkdir -p "${src}/com/a" "${src}/com/b" "${src}/com/c" "${src}/com/d"
  printf 'package com.a;\n' > "${src}/com/a/A.java"
  printf '@NullMarked\npackage com.a;\n' > "${src}/com/a/package-info.java"
  printf 'package com.b;\n' > "${src}/com/b/B.java"            # missing package-info
  printf 'package com.c;\n' > "${src}/com/c/C.java"
  printf 'package com.c;\n' > "${src}/com/c/package-info.java" # missing annotation
  printf 'package com.d;\n' > "${src}/com/d/D.java"
  printf '@NullMarked\npackage com.d;\n' > "${src}/com/d/package-info.java"

  for fmt in text json markdown; do
    run_case "java validate-annotations mixed ${fmt}" java validate-annotations "${src}" -o "${fmt}" --no-color
  done
  run_case "java validate-annotations mixed quiet"   java validate-annotations "${src}" -q --no-color
  run_case "java validate-annotations mixed verbose" java validate-annotations "${src}" -v --no-color

  # All-valid tree → success, zero violations.
  local ok="${TMP}/java-ok"
  mkdir -p "${ok}/com/a" "${ok}/com/b"
  printf 'package com.a;\n' > "${ok}/com/a/A.java"
  printf '@NullMarked\npackage com.a;\n' > "${ok}/com/a/package-info.java"
  printf 'package com.b;\n' > "${ok}/com/b/B.java"
  printf '@NullMarked\npackage com.b;\n' > "${ok}/com/b/package-info.java"
  for fmt in text json markdown; do
    run_case "java validate-annotations all-valid ${fmt}" java validate-annotations "${ok}" -o "${fmt}" --no-color
  done
  run_case "java validate-annotations all-valid quiet" java validate-annotations "${ok}" -q --no-color

  # Custom annotation via --annotation flag (NonNull).
  local nn="${TMP}/java-nn"
  mkdir -p "${nn}/com/a"
  printf 'package com.a;\n' > "${nn}/com/a/A.java"
  printf '@NonNull\npackage com.a;\n' > "${nn}/com/a/package-info.java"
  for fmt in text json markdown; do
    run_case "java validate-annotations custom ${fmt}" java validate-annotations "${nn}" --annotation NonNull -o "${fmt}" --no-color
  done

  # Empty tree → zero packages, zero violations, success.
  local empty="${TMP}/java-empty"
  mkdir -p "${empty}"
  run_case "java validate-annotations empty" java validate-annotations "${empty}" --no-color
}

# run_env_pair_case <label> <go_repo> <rs_repo> <go_dir> <rs_dir> <args...>
# Runs the Go binary from <go_repo> and the Rust binary from <rs_repo> with the
# trailing args, then diffs the streams AND the resulting on-disk trees of BOTH
# the repo and the backup dir. If a trailing arg equals the literal token
# @GO_DIR@ / @RS_DIR@, it is substituted with the per-binary backup-dir path so
# each binary writes to (or reads from) its own fixture. JSON `dir` fields and
# the temp paths in error text are masked before comparison. `PWD` is set so the
# Go-parity `getwd()` in the Rust binary uses the same logical path the Go
# binary's `os.Getwd()` sees (matters for the inside-repo rejection).
run_env_pair_case() {
  local label="$1"; shift
  local go_repo="$1"; shift
  local rs_repo="$1"; shift
  local go_dir="$1"; shift
  local rs_dir="$1"; shift
  CASE_NO=$((CASE_NO + 1))

  # Build per-binary arg arrays, substituting the @GO_DIR@/@RS_DIR@ placeholders.
  local go_args=() rs_args=() a
  for a in "$@"; do
    case "${a}" in
      @DIR@) go_args+=("${go_dir}"); rs_args+=("${rs_dir}") ;;
      *)     go_args+=("${a}");      rs_args+=("${a}") ;;
    esac
  done

  set +e
  ( cd "${go_repo}" && PWD="${go_repo}" "${GO_BIN}" "${go_args[@]}" ) > "${TMP}/go.out" 2> "${TMP}/go.err"
  local go_exit=$?
  ( cd "${rs_repo}" && PWD="${rs_repo}" "${RS_BIN}" "${rs_args[@]}" ) > "${TMP}/rs.out" 2> "${TMP}/rs.err"
  local rs_exit=$?
  set -e

  # Mask per-binary temp paths (repo + backup dir) so the JSON `dir`/error text
  # compares equal, then apply the standard timestamp/duration normaliser.
  env_mask() {
    sed -E -e "s#${go_repo}#<REPO>#g" -e "s#${rs_repo}#<REPO>#g" \
           -e "s#${go_dir}#<DIR>#g" -e "s#${rs_dir}#<DIR>#g"
  }
  env_mask < "${TMP}/go.out" | normalise > "${TMP}/go.out.n"
  env_mask < "${TMP}/rs.out" | normalise > "${TMP}/rs.out.n"
  env_mask < "${TMP}/go.err" | normalise > "${TMP}/go.err.n"
  env_mask < "${TMP}/rs.err" | normalise > "${TMP}/rs.err.n"

  local diverged=0
  if ! diff -q "${TMP}/go.out.n" "${TMP}/rs.out.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stdout DIVERGED (args: $*)" >&2
    diff -u "${TMP}/go.out.n" "${TMP}/rs.out.n" | sed 's/^/    /' >&2 || true
  fi
  if ! diff -q "${TMP}/go.err.n" "${TMP}/rs.err.n" > /dev/null; then
    diverged=1
    echo "✗ [${label}] stderr DIVERGED (args: $*)" >&2
    diff -u "${TMP}/go.err.n" "${TMP}/rs.err.n" | sed 's/^/    /' >&2 || true
  fi
  if [[ "${go_exit}" != "${rs_exit}" ]]; then
    diverged=1
    echo "✗ [${label}] exit code DIVERGED (go=${go_exit} rs=${rs_exit}, args: $*)" >&2
  fi
  # Compare the resulting repo + backup trees (relative listings).
  if ! diff -q <(cd "${go_repo}" && find . -type f | sort) <(cd "${rs_repo}" && find . -type f | sort) > /dev/null; then
    diverged=1
    echo "✗ [${label}] REPO TREE DIVERGED" >&2
  fi
  # The backup dir may not exist (rejection / missing-dir cases); the cd error
  # is suppressed and an absent dir yields an empty (matching) listing.
  local go_bk_list rs_bk_list
  go_bk_list="$( (cd "${go_dir}" 2>/dev/null && find . -type f 2>/dev/null | sort) || true )"
  rs_bk_list="$( (cd "${rs_dir}" 2>/dev/null && find . -type f 2>/dev/null | sort) || true )"
  if [[ "${go_bk_list}" != "${rs_bk_list}" ]]; then
    diverged=1
    echo "✗ [${label}] BACKUP TREE DIVERGED" >&2
  fi

  if [[ "${diverged}" -eq 0 ]]; then
    echo "✓ [${label}] (exit ${go_exit}, trees identical)"
  else
    FAIL=1
  fi
}

# Seed a synthetic git repo with assorted .env files at $1 (fresh tree).
seed_env_repo() {
  local d="$1"
  mkdir -p "${d}/.git" "${d}/apps/web/node_modules" "${d}/node_modules" "${d}/.claude"
  printf 'ROOT=1\n'   > "${d}/.env"
  printf 'WEB=1\n'    > "${d}/apps/web/.env.local"
  printf 'IGNORED=1\n'> "${d}/node_modules/.env"
  printf 'IGNORED=1\n'> "${d}/apps/web/node_modules/.env"
  printf '{}\n'       > "${d}/.claude/settings.local.json"
}

# Seed a backup dir at $1 holding previously backed-up .env files (for restore).
seed_env_backup() {
  local d="$1"
  mkdir -p "${d}/apps/web"
  printf 'ROOT=1\n' > "${d}/.env"
  printf 'WEB=1\n'  > "${d}/apps/web/.env.local"
  printf '# doc\n'  > "${d}/README.md"
}

corpus_env() {
  echo "── env corpus ──" >&2

  # --- env init: walk infra/dev/ for .env.example. Fresh / skip / force / zero.
  for state in fresh skip force; do
    for fmt in text; do
      local gr rr; gr="$(mktemp -d)"; rr="$(mktemp -d)"
      for d in "${gr}" "${rr}"; do
        mkdir -p "${d}/.git" "${d}/infra/dev/svc-a" "${d}/infra/dev/svc-b"
        printf 'A=1\n' > "${d}/infra/dev/svc-a/.env.example"
        printf 'B=1\n' > "${d}/infra/dev/svc-b/.env.example"
        if [[ "${state}" != "fresh" ]]; then
          printf 'OLD=1\n' > "${d}/infra/dev/svc-a/.env"
        fi
      done
      local flag=""
      [[ "${state}" == "force" ]] && flag="--force"
      # shellcheck disable=SC2086
      run_env_pair_case "env init ${state} ${fmt}" "${gr}" "${rr}" "${gr}" "${rr}" \
        env init ${flag} -o "${fmt}" --no-color
      rm -rf "${gr}" "${rr}"
    done
  done
  # init with zero examples.
  local gz rz; gz="$(mktemp -d)"; rz="$(mktemp -d)"
  for d in "${gz}" "${rz}"; do mkdir -p "${d}/.git" "${d}/infra/dev"; done
  run_env_pair_case "env init zero" "${gz}" "${rz}" "${gz}" "${rz}" env init --no-color
  rm -rf "${gz}" "${rz}"

  # --- env backup: every format, force (deterministic — non-TTY stdin forces).
  for fmt in text json markdown; do
    local gr rr gb rb; gr="$(mktemp -d)"; rr="$(mktemp -d)"; gb="$(mktemp -d)"; rb="$(mktemp -d)"
    seed_env_repo "${gr}"; seed_env_repo "${rr}"
    run_env_pair_case "env backup ${fmt}" "${gr}" "${rr}" "${gb}" "${rb}" \
      env backup --dir @DIR@ --force -o "${fmt}" --no-color
    rm -rf "${gr}" "${rr}" "${gb}" "${rb}"
  done
  # backup --worktree-aware (repo basename namespacing).
  for fmt in text json; do
    local gp rp gb rb; gp="$(mktemp -d)"; rp="$(mktemp -d)"; gb="$(mktemp -d)"; rb="$(mktemp -d)"
    seed_env_repo "${gp}/proj"; seed_env_repo "${rp}/proj"
    run_env_pair_case "env backup worktree-aware ${fmt}" "${gp}/proj" "${rp}/proj" "${gb}" "${rb}" \
      env backup --dir @DIR@ --worktree-aware --force -o "${fmt}" --no-color
    rm -rf "${gp}" "${rp}" "${gb}" "${rb}"
  done
  # backup --include-config (picks up .claude/settings.local.json).
  for fmt in text json; do
    local gr rr gb rb; gr="$(mktemp -d)"; rr="$(mktemp -d)"; gb="$(mktemp -d)"; rb="$(mktemp -d)"
    seed_env_repo "${gr}"; seed_env_repo "${rr}"
    run_env_pair_case "env backup include-config ${fmt}" "${gr}" "${rr}" "${gb}" "${rb}" \
      env backup --dir @DIR@ --include-config --force -o "${fmt}" --no-color
    rm -rf "${gr}" "${rr}" "${gb}" "${rb}"
  done
  # backup --dir inside the repo → deterministic rejection error.
  local gri rri; gri="$(mktemp -d)"; rri="$(mktemp -d)"
  seed_env_repo "${gri}"; seed_env_repo "${rri}"
  run_env_pair_case "env backup inside-repo reject" "${gri}" "${rri}" "${gri}/inside" "${rri}/inside" \
    env backup --dir @DIR@ --no-color
  rm -rf "${gri}" "${rri}"

  # --- env restore: every format, force, + missing-dir error path.
  for fmt in text json markdown; do
    local gr rr gb rb; gr="$(mktemp -d)"; rr="$(mktemp -d)"; gb="$(mktemp -d)"; rb="$(mktemp -d)"
    mkdir -p "${gr}/.git" "${rr}/.git"
    seed_env_backup "${gb}"; seed_env_backup "${rb}"
    run_env_pair_case "env restore ${fmt}" "${gr}" "${rr}" "${gb}" "${rb}" \
      env restore --dir @DIR@ --force -o "${fmt}" --no-color
    rm -rf "${gr}" "${rr}" "${gb}" "${rb}"
  done
  # restore missing dir → error.
  local grm rrm; grm="$(mktemp -d)"; rrm="$(mktemp -d)"
  mkdir -p "${grm}/.git" "${rrm}/.git"
  run_env_pair_case "env restore missing-dir" "${grm}" "${rrm}" "/nonexistent-sd-xyz" "/nonexistent-sd-xyz" \
    env restore --dir @DIR@ --no-color
  rm -rf "${grm}" "${rrm}"
}

corpus_doctor() {
  echo "── doctor corpus ──" >&2
  # doctor probes the host's tools; both binaries see the same versions on this
  # machine, so a direct same-machine diff (with TS/duration masked) is valid.
  for fmt in text json markdown; do
    run_case "doctor ${fmt}"           doctor -o "${fmt}" --no-color
    run_case "doctor minimal ${fmt}"   doctor --scope minimal -o "${fmt}" --no-color
  done
  run_case "doctor quiet"   doctor -q --no-color
  run_case "doctor verbose" doctor -v --no-color
  run_case "doctor fix dry-run" doctor --fix --dry-run --no-color
}

# A repo-relative temp output path for `merge --out-file` (lives under TMP).
TMP_OUT_REL="$(python3 -c "import os,sys; print(os.path.relpath('${TMP}/merged.info', '${REPO_ROOT}'))" 2>/dev/null || echo "${TMP}/merged.info")"

for cmd in "${COMMANDS[@]}"; do
  case "${cmd}" in
    test-coverage) corpus_test_coverage ;;
    spec-coverage) corpus_spec_coverage ;;
    crud-spec-coverage) corpus_crud_spec_coverage ;;
    docs) corpus_docs ;;
    agents) corpus_agents ;;
    repo-governance) corpus_repo_governance ;;
    workflows) corpus_workflows ;;
    git) corpus_git ;;
    contracts) corpus_contracts ;;
    java) corpus_java ;;
    env) corpus_env ;;
    doctor) corpus_doctor ;;
  esac
done

echo "──────────────────────────────────────────" >&2
if [[ "${FAIL}" -eq 0 ]]; then
  echo "Shadow diff PASS — ${CASE_NO} cases byte-identical."
  exit 0
else
  echo "Shadow diff FAIL — see divergences above (${CASE_NO} cases run)." >&2
  exit 1
fi
