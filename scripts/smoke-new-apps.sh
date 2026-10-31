#!/usr/bin/env bash
# Generate each app template under target/smoke-* and run `cargo build`.
# Usage: from Framework repo root: ./scripts/smoke-new-apps.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ -x "$ROOT/target/debug/omnisan" ]]; then
  OMNISAN=("$ROOT/target/debug/omnisan")
else
  OMNISAN=(cargo run --manifest-path "$ROOT/Cargo.toml" -p omium-cli --)
fi

SMOKE_ROOT="$ROOT/target/smoke-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$SMOKE_ROOT"
cd "$SMOKE_ROOT"

echo "Smoke workspace: $SMOKE_ROOT"
echo

run_new() {
  local dir="$1"
  local frontend="$2"
  shift 2
  echo "==> omnisan new ($frontend) -> $dir"
  "${OMNISAN[@]}" new "$dir" --frontend "$frontend" --yes --no-interaction "$@"
}

run_build() {
  local dir="$1"
  echo "==> cargo build -> $dir"
  cargo build --manifest-path "$dir/Cargo.toml"
  echo "    OK: $dir"
  echo
}

fail=0

if ! run_new demo-api api; then fail=1; fi
if ! run_build demo-api; then fail=1; fi

if ! run_new demo-standard standard; then fail=1; fi
if ! run_build demo-standard; then fail=1; fi

if ! run_new demo-vue vue; then fail=1; fi
if ! run_build demo-vue; then fail=1; fi

if ! run_new demo-react react; then fail=1; fi
if ! run_build demo-react; then fail=1; fi

# Unicode / spaced folder name (Cargo slug must be ASCII — see generator/naming.rs)
UNICODE_DIR="adsız klasör"
if ! run_new "$UNICODE_DIR" standard; then fail=1; fi
if ! run_build "$UNICODE_DIR"; then fail=1; fi

if [[ "$fail" -ne 0 ]]; then
  echo "Smoke failed (see errors above). Workspace kept at: $SMOKE_ROOT" >&2
  exit 1
fi

echo "All smoke checks passed."
echo "Artifacts: $SMOKE_ROOT (safe to delete)"
