#!/usr/bin/env bash
# Pre-commit guard: reject commits that stage real .env* files (not .env.example).
# Called from .husky/pre-commit before the rhino-cli pre-commit check.

set -euo pipefail

staged=$(git diff --cached --name-only 2>/dev/null | grep -E '(^|/)\.env[^/]*$' | grep -v '\.env\.example$' || true)

if [ -n "$staged" ]; then
  echo "ERROR: Staged real .env* file(s) detected. Remove from staging before committing." >&2
  while IFS= read -r f; do
    echo "  - $f" >&2
  done <<<"$staged"
  echo "Hint: Use '.env.example' for templates. Keep real env vars out of git history." >&2
  exit 1
fi

exit 0
