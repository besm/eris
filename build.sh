#!/usr/bin/env bash
set -euo pipefail

echo "Building..."
cargo build --release -p eris-cli -p eris-mcp

echo "Installing to ~/bin..."
cp target/release/eris ~/bin/eris
cp target/release/eris-mcp ~/bin/eris-mcp

echo "Updating tracked files..."
~/bin/eris sql --update
~/bin/eris nix --update

echo ""
if [ -n "$(git status --porcelain)" ]; then
    echo "Build successful. Uncommitted changes detected - please commit."
else
    echo "Build successful. Working tree clean."
fi
