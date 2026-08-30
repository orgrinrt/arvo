#!/usr/bin/env bash
# The 32 questions with decider = "panel" that nothing points at.
#
# This is the worklist for 189. It is the same population coverage.sh counts in
# its "panel / open" cell, printed by name instead of counted, so the two can be
# cross-checked: the line count here must equal that cell.
#
# The case that must fail: run with --control to plant a proposal edge at one
# of the listed ids and confirm it drops out of the list. Without that, an
# empty or short list is indistinguishable from an awk rule that never matched.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"
edges=$(mktemp); trap 'rm -f "$edges"' EXIT

awk '
  /^(answers) = / {
    line = $0; gsub(/^[a-z]+ = \[|\]$/, "", line); gsub(/"/, "", line)
    n = split(line, t, ",")
    for (i = 1; i <= n; i++) { gsub(/^ +| +$/, "", t[i]); if (t[i] != "") print t[i] }
  }
' "$reg/ruling.toml" "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" > "$edges"

if [ "${1:-}" = "--control" ]; then
  echo "which_width_coordinates_a_consumer_writes" >> "$edges"
  echo "  [control] planted an edge at which_width_coordinates_a_consumer_writes; it must vanish below"
fi

awk '
  /^\[\[/  { if (id != "") print id "\t" d "\t" k; id = ""; d = "-"; k = "-"; next }
  /^id = / { v = $0; gsub(/^id = "|"$/, "", v); id = v }
  /^decider = / { v = $0; gsub(/^decider = "|"$/, "", v); d = v }
  /^key = / { v = $0; gsub(/^key = "|"$/, "", v); k = v }
  END { if (id != "") print id "\t" d "\t" k }
' "$reg/question.toml" | while IFS=$'\t' read -r id d k; do
  [ "$d" = "panel" ] || continue
  grep -qxF "$id" "$edges" && continue
  printf '%-8s %s\n' "$k" "$id"
done
