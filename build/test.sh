#!/usr/bin/env bash
set -euo pipefail

echo "[nuggies] Running tests"

cargo test --all --verbose

echo "[nuggies] Tests complete"
