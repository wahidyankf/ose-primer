#!/usr/bin/env bash
set -euo pipefail
INPUT=$(cat)
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)
case "$TOOL_NAME" in
  Read | Write | Edit | MultiEdit)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
    if [ -z "$FILE_PATH" ]; then exit 0; fi
    base=$(basename "$FILE_PATH")
    if [[ "$base" == .env* ]] && [[ "$base" != ".env.example" ]]; then
      echo "BLOCKED: real .env* access denied." >&2
      exit 2
    fi ;;
  Bash)
    COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)
    if [ -z "$COMMAND" ]; then exit 0; fi
    if echo "$COMMAND" | grep -qE '^\s*(npm|npx|nx|pnpm|yarn)\s'; then exit 0; fi
    if echo "$COMMAND" | grep -qE '^\s*(apps|libs|scripts)/'; then exit 0; fi
    # ALLOW: course/teaching fixtures under an app's published content tree, e.g.
    # apps/ayokoding-www/content/**/kata.env — curriculum material, never real
    # secrets. The char before `.env` must be neither `/` nor `.`, so dotfile
    # `.env` / `.env.local` stay denied even under content/.
    # See guard-env-file-access §9 content-fixture exclusion.
    if echo "$COMMAND" | grep -qE 'apps/[^/[:space:]]+/content/[^[:space:]]*[^/.[:space:]]\.env([[:space:]]|$)'; then exit 0; fi
    stripped=$(echo "$COMMAND" | sed 's/\.env\.example//g')
    if echo "$stripped" | grep -qE '\.env'; then
      echo "BLOCKED: real .env* manipulation denied." >&2
      exit 2
    fi ;;
esac
exit 0
