#!/usr/bin/env bash
# App-local wrapper — prefers ~/.cargo/bin/omnisan, then monorepo Cargo fallback.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [[ -n "${OMNISAN_BIN:-}" && -x "${OMNISAN_BIN}" ]]; then
  exec "${OMNISAN_BIN}" "$@"
fi

global_bin="$(type -P omnisan 2>/dev/null || true)"
if [[ -n "$global_bin" ]]; then
  exec "$global_bin" "$@"
fi

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
  echo "omnisan: command not found on PATH and Omium monorepo not detected." >&2
  echo "Install the CLI:" >&2
  echo "  cargo install omnisan" >&2
  exit 1
}

# shellcheck source=omnisan-dispatch.sh
source "$FRAMEWORK_ROOT/scripts/omnisan-dispatch.sh"
omnisan_dispatch "$FRAMEWORK_ROOT" "$@"
