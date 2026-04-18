#!/usr/bin/env bash
set -euo pipefail

echo "[nuggies] Building release binary"

if ! command -v cargo >/dev/null 2>&1; then
    echo "[error] Rust toolchain not found."
    exit 1
fi

cargo build --release

echo "[nuggies] Release build complete"
echo "[nuggies] Binary: target/release/nuggies"
