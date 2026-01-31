#!/usr/bin/env bash
set -euo pipefail

echo "Building..."
cargo build --release -p eris-cli -p eris-mcp

echo "Updating tracked files..."
./target/release/eris sql --update
./target/release/eris nix --update

echo ""
if [ -n "$(git status --porcelain)" ]; then
    echo "Build successful. Uncommitted changes detected - please commit."
else
    echo "Build successful. Working tree clean."
fi
