#!/usr/bin/env bash
set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"
BIN_PATH="${PREFIX}/bin/nuggies"

echo "[nuggies] Uninstalling from ${BIN_PATH}"

if [ -f "${BIN_PATH}" ]; then
    rm -f "${BIN_PATH}"
    echo "[nuggies] Removed binary"
else
    echo "[warn] Binary not found"
fi

echo "[nuggies] Uninstall complete"
