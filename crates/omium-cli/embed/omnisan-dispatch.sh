#!/usr/bin/env bash
# Prefer a globally installed `omnisan` (~/.cargo/bin); otherwise compile via Cargo once per invoke.

omnisan_dispatch() {
  local framework_root="$1"
  shift

  if [[ -n "${OMNISAN_BIN:-}" && -x "${OMNISAN_BIN}" ]]; then
    exec "${OMNISAN_BIN}" "$@"
  fi

  local global_bin
  global_bin="$(type -P omnisan 2>/dev/null || true)"
  if [[ -n "$global_bin" ]]; then
    exec "$global_bin" "$@"
  fi

  exec cargo run --manifest-path "$framework_root/crates/omium-cli/Cargo.toml" --bin omnisan -- "$@"
}
