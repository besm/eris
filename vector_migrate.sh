#!/usr/bin/env bash
# Migrate vector symbol references from old to new
# Usage: ./vector_migrate.sh <vector_name>
# Example: ./vector_migrate.sh transitive

set -euo pipefail

migrate() {
  local name="$1"
  local file=$(find crates/eris/defs/vectors -name "$name.ron" 2>/dev/null | head -1)

  [[ -z "$file" ]] && echo "Not found: $name.ron" && return 1

  local old=$(rg -oP 'supersedes: Some\("\K[^"]+' "$file") || true
  local new=$(rg -oP '^    symbol: "\K[^"]+' "$file")

  [[ -z "$old" ]] && echo "$name: no supersedes field" && return 1

  local before=$(rg -l -F "$old" crates/eris/defs/entities/ crates/eris/defs/chronos/ 2>/dev/null | wc -l)
  echo "$name: $old → $new ($before files)"

  sd "$old" "$new" crates/eris/defs/entities/*.ron crates/eris/defs/chronos/*.ron 2>/dev/null || true

  local after=$(rg -l -F "$old" crates/eris/defs/entities/ crates/eris/defs/chronos/ 2>/dev/null | wc -l)
  echo "  $before → $after remaining"
}

[[ $# -eq 0 ]] && echo "Usage: $0 <vector_name>" && exit 1

migrate "$1"
