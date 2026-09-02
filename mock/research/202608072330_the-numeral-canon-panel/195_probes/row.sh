#!/usr/bin/env bash
# Prints one registry row whole, given its namespace and slug.
#
# Reading an edge means opening both rows it joins, and `cargo mock query`
# truncates every column to the terminal width and hides the internal fields,
# so neither `asks` nor `key` survives it. This reads the TOML instead.
#
# usage: row.sh <namespace> <slug>
#
# The control is in `control_runs.txt`: a slug that is not there prints nothing
# and exits 1, so an empty read is distinguishable from a row with empty fields.
#
# The first version of this script printed nothing for a row that was there.
# Its table-header rule ran after the namespace-match rule and reset the buffer
# before it could flush, so every read came back empty and looked exactly like
# a missing slug. Caught by running it against a row known to exist, which is
# why the positive case is in the control transcript beside the negative one.
set -euo pipefail

ns=$1
slug=$2
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

found=$(awk -v ns="[[$ns]]" -v want="id = \"$slug\"" '
  /^\[\[/ {
    if (inrow && hit) printf "%s", buf
    inrow = ($0 == ns); buf = $0 "\n"; hit = 0; next
  }
  inrow   { buf = buf $0 "\n"; if ($0 == want) hit = 1 }
  END     { if (inrow && hit) printf "%s", buf }
' "$reg"/*.toml)

if [ -z "$found" ]; then
  echo "no row $ns::$slug" >&2
  exit 1
fi
printf '%s\n' "$found"
