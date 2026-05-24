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
#   COMMAND  One or more of: test-coverage spec-coverage
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
  docs             Diff the `docs validate-links|validate-mermaid` corpus

  With no COMMAND, every command's corpus runs.

NOTE on docs text/markdown output:
  The Go `docs validate-mermaid` (and validate-links) text/markdown formatters
  group findings into a Go map and range it, so the file ordering is
  NON-DETERMINISTIC across runs whenever 2+ files have findings — the Go binary
  cannot even match itself. The Rust port emits sorted (deterministic) output.
  The docs corpus therefore restricts text/markdown finding-cases to a SINGLE
  file (where ordering is trivially identical) and exercises all multi-file
  finding cases via JSON, whose slice ordering IS deterministic and matches
  byte-for-byte. Full-corpus default runs (zero findings) are deterministic.

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
    test-coverage | spec-coverage | docs)
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
  COMMANDS=(test-coverage spec-coverage docs)
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

# Mask wall-clock-dependent JSON timestamps so the two runs can be compared.
normalise() {
  sed -E 's/"timestamp": "[^"]*"/"timestamp": "<TS>"/; s/"duration_ms": [0-9]+/"duration_ms": <D>/'
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

corpus_docs() {
  echo "── docs corpus ──" >&2

  # --- validate-links: full default corpus, every format (zero findings → ---
  # --- deterministic; counts/timestamps normalised). -------------------------
  for fmt in text json markdown; do
    run_case "links default ${fmt}"  docs validate-links -o "${fmt}" --no-color
  done
  run_case "links quiet"    docs validate-links -q --no-color
  run_case "links verbose"  docs validate-links -v --no-color
  # staged-only on a clean tree → no staged files → success, identical output.
  run_case "links staged-only"  docs validate-links --staged-only --no-color

  # --- validate-mermaid: full default corpus, every format (zero findings). ---
  for fmt in text json markdown; do
    run_case "mermaid default ${fmt}"  docs validate-mermaid -o "${fmt}" --no-color
  done
  run_case "mermaid quiet"    docs validate-mermaid -q --no-color
  run_case "mermaid verbose"  docs validate-mermaid -v --no-color
  # Scoped to individual real directories (still zero findings on this corpus).
  run_case "mermaid docs dir"        docs validate-mermaid docs --no-color
  run_case "mermaid docs dir json"   docs validate-mermaid docs -o json --no-color
  run_case "mermaid repo-gov dir"    docs validate-mermaid repo-governance --no-color
  run_case "mermaid .claude dir"     docs validate-mermaid .claude --no-color
  run_case "mermaid multi dir json"  docs validate-mermaid docs repo-governance .claude -o json --no-color
  # staged-only / changed-only on a clean tree.
  run_case "mermaid staged-only"   docs validate-mermaid --staged-only --no-color
  run_case "mermaid changed-only"  docs validate-mermaid --changed-only --no-color

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
}

# A repo-relative temp output path for `merge --out-file` (lives under TMP).
TMP_OUT_REL="$(python3 -c "import os,sys; print(os.path.relpath('${TMP}/merged.info', '${REPO_ROOT}'))" 2>/dev/null || echo "${TMP}/merged.info")"

for cmd in "${COMMANDS[@]}"; do
  case "${cmd}" in
    test-coverage) corpus_test_coverage ;;
    spec-coverage) corpus_spec_coverage ;;
    docs) corpus_docs ;;
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
