#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/../.." && pwd)"
cd "$PROJECT_ROOT"
if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi
if [[ ! -x ./target/release/aegisphrase ]]; then
  echo "[*] Release binary not found. Building with cargo..."
  cargo build --release
fi
if [[ "$#" -eq 0 ]]; then
  ./target/release/aegisphrase --help
else
  ./target/release/aegisphrase "$@"
fi
