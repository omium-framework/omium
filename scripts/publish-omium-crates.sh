#!/usr/bin/env bash
# Publish Omium framework crates to crates.io in dependency order.
# Prereqs: cargo login, version bumps for any changed crate, `cargo publish --dry-run`.
#
# Usage:
#   ./scripts/publish-omium-crates.sh              # all framework crates + omnisan
#   ./scripts/publish-omium-crates.sh omium-cli   # one or more crates (omnisan last if listed)
#   ALLOW_DIRTY=1 ./scripts/publish-omium-crates.sh omium-cli
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

ALLOW_DIRTY="${ALLOW_DIRTY:-}"
EXTRA=()
if [[ "$ALLOW_DIRTY" == "1" ]]; then
  EXTRA+=(--allow-dirty)
fi

ORDER=(omium-core omium-view omium-http omium-db omium-auth omium-inertia omium-cli)

publish_framework() {
  local crate="$1"
  echo "==> Publishing ${crate} …"
  cargo publish -p "$crate" "${EXTRA[@]}"
}

publish_omnisan() {
  echo "==> Publishing omnisan (Cargo/omnisan) …"
  (
    cd "$ROOT/../Cargo/omnisan"
    cargo publish "${EXTRA[@]}"
  )
}

if [[ $# -gt 0 ]]; then
  for arg in "$@"; do
    if [[ "$arg" == "omnisan" ]]; then
      publish_omnisan
      continue
    fi
    found=0
    for crate in "${ORDER[@]}"; do
      if [[ "$crate" == "$arg" ]]; then
        found=1
        publish_framework "$crate"
        break
      fi
    done
    if [[ "$found" -eq 0 ]]; then
      echo "Unknown crate: $arg (known: ${ORDER[*]} omnisan)" >&2
      exit 1
    fi
  done
  echo "Done."
  exit 0
fi

for crate in "${ORDER[@]}"; do
  publish_framework "$crate"
done
publish_omnisan

echo "Done. Verify: cargo install omnisan && omnisan --version"
