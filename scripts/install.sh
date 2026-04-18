#!/usr/bin/env bash
set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"
BIN_NAME="nuggies"

echo "[nuggies] Installing to ${PREFIX}/bin"

if [ ! -f "target/release/${BIN_NAME}" ]; then
    echo "[error] Release binary not found. Run build/release.sh first."
    exit 1
fi

install -Dm755 "target/release/${BIN_NAME}" "${PREFIX}/bin/${BIN_NAME}"

echo "[nuggies] Installed successfully"
echo "[nuggies] Run: ${BIN_NAME}"
