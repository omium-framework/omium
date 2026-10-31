#!/usr/bin/env bash
# Framework repo wrapper — same commands as a globally installed `omnisan`.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=omnisan-dispatch.sh
source "$SCRIPT_DIR/omnisan-dispatch.sh"

find_framework_root() {
  local dir="$1"
  while [[ -n "$dir" && "$dir" != "/" ]]; do
    if [[ -f "$dir/crates/omium-cli/Cargo.toml" ]]; then
      printf '%s\n' "$dir"
      return 0
    fi
    dir="$(dirname "$dir")"
  done
  return 1
}

FRAMEWORK_ROOT="$(find_framework_root "$SCRIPT_DIR")" || {
  echo "omnisan: Omium framework not found (expected crates/omium-cli)." >&2
  echo "Run from the Framework repo or install the CLI: cargo install --path crates/omium-cli --bin omnisan" >&2
  exit 1
}

if [[ -f "$SCRIPT_DIR/config/app.toml" && -f "$SCRIPT_DIR/Cargo.toml" ]]; then
  WORK_DIR="$SCRIPT_DIR"
else
  WORK_DIR="$(pwd)"
fi

cd "$WORK_DIR"

omnisan_dispatch "$FRAMEWORK_ROOT" "$@"
