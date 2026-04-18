#!/usr/bin/env bash
set -euo pipefail

echo "[nuggies doctor] Running system diagnostics..."

fail=0

check() {
    if command -v "$1" >/dev/null 2>&1; then
        echo "[ok] $1 found"
    else
        echo "[missing] $1 not found"
        fail=1
    fi
}

echo "== Core Tools =="
check cargo
check git
check curl

echo
echo "== Package Managers =="
check flatpak
check snap
check apt
check dnf
check pacman

echo
echo "== Directories =="

for dir in "$HOME/.config/nuggies" "$HOME/.cache/nuggies"; do
    if [ -d "$dir" ]; then
        echo "[ok] $dir exists"
    else
        echo "[missing] $dir"
    fi
done

echo
if [ "$fail" -eq 0 ]; then
    echo "[nuggies doctor] System looks good"
else
    echo "[nuggies doctor] Some components are missing"
fi
