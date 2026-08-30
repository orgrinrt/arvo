#!/usr/bin/env bash
# Which proposal rows point at which question, printed per question.
#
# The deliverable's per-question table is a claim about this file's output, so
# the table is generated from it rather than typed. A hand-typed mapping of
# twenty-two edges across two files is a list somebody will have to re-derive.
#
# The case that must fail: --control names a question that has an edge and
# asserts the row list is empty. If the script cannot tell an edged question
# from an unedged one, both assertions pass and the script proves nothing.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

edges=$(mktemp); trap 'rm -f "$edges"' EXIT
awk '
  /^id = / { v = $0; gsub(/^id = "|"$/, "", v); row = v }
  /^answers = / {
    line = $0; gsub(/^answers = \[|\]$/, "", line); gsub(/"/, "", line)
    n = split(line, t, ",")
    for (i = 1; i <= n; i++) { gsub(/^ +| +$/, "", t[i]); if (t[i] != "") print t[i] "\t" row }
  }
' "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" | sort > "$edges"

if [ "${1:-}" = "--control" ]; then
  a=$(awk -F'\t' '$1 == "adaptation_in_identity_or_realisation"' "$edges" | wc -l | tr -d ' ')
  b=$(awk -F'\t' '$1 == "what_a_datum_stands_for"' "$edges" | wc -l | tr -d ' ')
  echo "  [control] adaptation_in_identity_or_realisation has $a row(s), must be > 0"
  echo "  [control] what_a_datum_stands_for has $b row(s), must be 0"
  [ "$a" -gt 0 ] || { echo "  CONTROL FAILED: an edged question read as empty"; exit 1; }
  [ "$b" -eq 0 ] || { echo "  CONTROL FAILED: an unedged question read as edged"; exit 1; }
  echo "  [control] both fired the right way"
  exit 0
fi

awk '/^\[\[/ { id = "" } /^id = / { if (id == "") { v = $0; gsub(/^id = "|"$/, "", v); print v } }' \
  "$reg/question.toml" | while read -r q; do
  rows=$(awk -F'\t' -v q="$q" '$1 == q { print "      " $2 }' "$edges")
  [ -n "$rows" ] || continue
  echo "$q"
  echo "$rows"
done
