#!/usr/bin/env bash
set -euo pipefail

echo "[nuggies] Building project (debug mode)"

if ! command -v cargo >/dev/null 2>&1; then
    echo "[error] Rust toolchain not found. Install Rust first."
    exit 1
fi

cargo build

echo "[nuggies] Build complete"
echo "[nuggies] Binary: target/debug/nuggies"
