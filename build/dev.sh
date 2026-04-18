#!/usr/bin/env bash
set -euo pipefail

echo "[nuggies] Starting development mode"

export RUST_LOG=${RUST_LOG:-debug}
export NUGGIES_DEBUG=1

cargo run -- "$@"
