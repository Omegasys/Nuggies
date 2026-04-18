#!/usr/bin/env bash
set -euo pipefail

echo "[nuggies] Bootstrapping development environment"

# Detect distro (very basic)
if [ -f /etc/os-release ]; then
    . /etc/os-release
    echo "[nuggies] Detected distro: $NAME"
fi

# Install Rust if missing
if ! command -v cargo >/dev/null 2>&1; then
    echo "[nuggies] Rust not found, installing via rustup"

    if command -v curl >/dev/null 2>&1; then
        curl https://sh.rustup.rs -sSf | sh -s -- -y
        source "$HOME/.cargo/env"
    else
        echo "[error] curl required to install Rust"
        exit 1
    fi
fi

# Create config dirs
mkdir -p "${HOME}/.config/nuggies"
mkdir -p "${HOME}/.cache/nuggies"

echo "[nuggies] Bootstrap complete"
echo "[nuggies] Next step: run build/build.sh"
