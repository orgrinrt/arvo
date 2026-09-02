#!/usr/bin/env bash
# How many questions and obligations anything points at, split by who points.
#
# The split is the whole point and it is the schema's, not a presentation
# choice. `mockspace.toml` on the obligation namespace: "A proposal alone is
# **proposed rather than answered**: anything counting coverage draws that
# line, because reporting a proposal as an answer closes a gap op has never
# seen." So three populations per row and never two:
#
#   answered  a ruling names it. Op said it.
#   proposed  only a proposal names it. The panel would settle it if stamped.
#   open      nothing names it.
#
# A question named by both counts as answered, since op's word outranks the
# paper. None currently is.
#
# Run with `--control` to plant one ruling edge and one proposal edge against
# two questions that have neither, and watch both move out of `open` into the
# right column. Without that, a report of 59 open is indistinguishable from a
# parser that never matched an edge at all. Transcript in `control_runs.txt`.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

edges=$(mktemp); rows=$(mktemp); trap 'rm -f "$edges" "$rows"' EXIT

# Every `answers` and `obligation` edge, as "<referring namespace> <target>".
# `answers` on a ruling and on a proposal mean different things, which is why
# the referring namespace is carried rather than collapsed.
awk '
  /^\[\[/ { ns = substr($0, 3, length($0) - 4); next }
  /^(answers|obligation) = / {
    split($0, f, " = "); field = f[1]
    line = $0; gsub(/^[a-z]+ = \[|\]$/, "", line); gsub(/"/, "", line)
    n = split(line, t, ",")
    for (i = 1; i <= n; i++) { gsub(/^ +| +$/, "", t[i]); if (t[i] != "") print ns, field, t[i] }
  }
' "$reg/ruling.toml" "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" > "$edges"

if [ "${1:-}" = "--control" ]; then
  echo "ruling answers what_a_datum_stands_for" >> "$edges"
  echo "proposal answers does_narrowing_compose" >> "$edges"
  echo "  [control] planted a ruling edge at what_a_datum_stands_for and a proposal edge at does_narrowing_compose"
fi

report() { # report <namespace> <file> <group-field>
  local ns=$1 file=$2 group=$3
  awk -v ns="$ns" -v group="$group" '
    /^\[\[/  { if (id != "") print id "\t" g; id = ""; g = "-"; next }
    /^id = / { gsub(/^id = "|"$/, ""); id = $0 }
    $0 ~ "^" group " = " { gsub("^" group " = \"|\"$", ""); g = $0 }
    END { if (id != "") print id "\t" g }
  ' "$file" > "$rows"

  echo "######## $ns, by $group"
  printf '  %-14s %6s %9s %9s %6s\n' "$group" total answered proposed open
  { awk -F'\t' '{print $2}' "$rows" | sort -u; echo "ALL"; } | while read -r g; do
    t=0; a=0; p=0; o=0
    while IFS=$'\t' read -r id gg; do
      [ "$g" = "ALL" ] || [ "$gg" = "$g" ] || continue
      t=$((t + 1))
      if grep -qE "^ruling [a-z]+ $id\$" "$edges"; then a=$((a + 1))
      elif grep -qE "^proposal [a-z]+ $id\$" "$edges"; then p=$((p + 1))
      else o=$((o + 1)); fi
    done < "$rows"
    printf '  %-14s %6d %9d %9d %6d\n' "$g" "$t" "$a" "$p" "$o"
  done
  echo
}

report question   "$reg/question.toml"   decider
report obligation "$reg/obligation.toml" consumer

echo "######## the questions and obligations nothing points at"
cut -f1 "$reg/question.toml" >/dev/null 2>&1 || true
for f in question obligation; do
  awk '/^id = / { gsub(/^id = "|"$/, ""); print }' "$reg/$f.toml" | while read -r id; do
    grep -qE "^(ruling|proposal) [a-z]+ $id\$" "$edges" || echo "  $f :: $id"
  done
done
