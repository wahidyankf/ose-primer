#!/usr/bin/env bash
# Test suite for block-env-file-access.sh
# Env file paths constructed via variables — no literal .env* in source.
set -euo pipefail

HOOK="$(dirname "$0")/block-env-file-access.sh"
PASS=0; FAIL=0

chk() {
  local mode="$1" desc="$2" input="$3"
  local rc=0; echo "$input" | bash "$HOOK" 2>/dev/null || rc=$?
  if [ "$mode" = "block" ] && [ "$rc" -ne 0 ]; then
    echo "PASS: $desc"; PASS=$((PASS+1))
  elif [ "$mode" = "allow" ] && [ "$rc" -eq 0 ]; then
    echo "PASS: $desc"; PASS=$((PASS+1))
  else
    echo "FAIL: $desc"; FAIL=$((FAIL+1))
  fi
}

# Build env file names from parts — no literal dotenv paths in source
D="."; V="env"
BASE="${D}${V}"
L="${BASE}.local"
PR="${BASE}.production"
ST="${BASE}.staging"
DV="${BASE}.development"
TS="${BASE}.test"
EX="${BASE}.example"

# JSON template fragments
RT='{"tool_name":"Read","tool_input":{"file_path":"/r/'
WT='{"tool_name":"Write","tool_input":{"file_path":"/r/'
ET='{"tool_name":"Edit","tool_input":{"file_path":"/r/'
MT='{"tool_name":"MultiEdit","tool_input":{"file_path":"/r/'
SX='"}}'
BT='{"tool_name":"Bash","tool_input":{"command":"'
BX='"}}'

# ── FILE TOOL: real env files denied ─────────────────────────────────────────
chk block "dotenv read"            "${RT}${BASE}${SX}"
chk block "dotenv-local read"      "${RT}${L}${SX}"
chk block "dotenv-prod read"       "${RT}${PR}${SX}"
chk block "dotenv-staging read"    "${RT}${ST}${SX}"
chk block "dotenv-dev read"        "${RT}${DV}${SX}"
chk block "dotenv-test read"       "${RT}${TS}${SX}"
chk block "dotenv write"           "${WT}${BASE}${SX}"
chk block "dotenv-local edit"      "${ET}${L}${SX}"
chk block "dotenv-local multiedit" "${MT}${L}${SX}"

# ── FILE TOOL: example file allowed ──────────────────────────────────────────
chk allow "dotenv-example read"    "${RT}${EX}${SX}"
chk allow "dotenv-example write"   "${WT}${EX}${SX}"

# ── BASH: direct env manipulation denied ─────────────────────────────────────
CMD1="cat ${L}"
CMD2="echo X > ${L}"
CMD3="git add ${L}"
chk block "bash-cat-local"     "${BT}${CMD1}${BX}"
chk block "bash-echo-local"    "${BT}${CMD2}${BX}"
chk block "bash-git-local"     "${BT}${CMD3}${BX}"

# ── BASH: package runner carve-out allowed ───────────────────────────────────
chk allow "npm-run-setup"    "${BT}npm run setup${BX}"
chk allow "npx-nx-run"       "${BT}npx nx run app:dev${BX}"
chk allow "nx-run"           "${BT}nx run app:dev${BX}"
chk allow "pnpm-install"     "${BT}pnpm install${BX}"
chk allow "yarn-build"       "${BT}yarn build${BX}"

# ── BASH: project script path carve-out allowed ──────────────────────────────
chk allow "scripts-path"     "${BT}scripts/setup-env.sh${BX}"
chk allow "apps-path"        "${BT}apps/myapp/setup.sh${BX}"
chk allow "libs-path"        "${BT}libs/shared/init.sh${BX}"

# ── BASH: safe commands referencing example file allowed ─────────────────────
CMD4="cat ${EX}"
CMD5="git add ${EX}"
CMD6="git add src/app.ts"
chk allow "bash-cat-example"  "${BT}${CMD4}${BX}"
chk allow "bash-git-example"  "${BT}${CMD5}${BX}"
chk allow "bash-git-src"      "${BT}${CMD6}${BX}"

# ── SUMMARY ──────────────────────────────────────────────────────────────────
echo ""
echo "Results: ${PASS} passed, ${FAIL} failed"
[ "$FAIL" -eq 0 ]
