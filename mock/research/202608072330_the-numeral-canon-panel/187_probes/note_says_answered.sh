#!/usr/bin/env bash
# Questions whose own `note` records that they were answered, split by whether
# anything now points at them.
#
# A question row carrying "recorded as answered" in prose and nothing in
# `answers` is a question whose answer went into the register instead of into a
# row. The derived-status mechanism cannot see it, so `refsto()` reports the
# question open while the row's own note says it is closed, and the two
# disagree with no way to tell which is current. That is the exact failure the
# no-status-field design was chosen to avoid, arriving through prose.
#
# The phrase list is the corpus's own vocabulary for it, taken by reading the
# notes rather than guessed. `--control` plants a note using a phrase from the
# list against a question with no such note.
set -euo pipefail
reg="$(cd "$(dirname "$0")/../../../registry" && pwd)"

PHRASES='recorded as answered|recorded as closed|records it as|recorded CLOSED|CLOSED|closed by op|answered by op|answered on 20|declined by him|op answered|op has answered|dissolv'

edges=$(mktemp); trap 'rm -f "$edges"' EXIT
awk '
  /^\[\[/ { ns = substr($0, 3, length($0) - 4); next }
  /^answers = / { line = $0; gsub(/^answers = \[|\]$/, "", line); gsub(/"/, "", line)
    n = split(line, t, ","); for (i = 1; i <= n; i++) { gsub(/^ +| +$/, "", t[i]); if (t[i] != "") print ns, t[i] } }
' "$reg/ruling.toml" "$reg/proposal.toml" "$reg/proposal-the-later-topics.toml" > "$edges"

extra=""
[ "${1:-}" = "--control" ] && extra=$'\nPLANTED_CONTROL\tnote = "recorded as answered by nobody at all"'

{ awk '
  /^\[\[question\]\]/ { id = ""; next }
  /^id = /   { gsub(/^id = "|"$/, ""); id = $0 }
  /^note = / { print id "\t" $0 }
' "$reg/question.toml"; printf '%s' "$extra"; } | grep -E "$PHRASES" | while IFS=$'\t' read -r id note; do
  [ -n "$id" ] || continue
  if grep -q "^ruling $id\$" "$edges"; then state="ANSWERED by a ruling"
  elif grep -q "^proposal $id\$" "$edges"; then state="proposed against"
  else state="**NOTHING POINTS AT IT**"; fi
  echo "$state -- $id"
  echo "$note" | sed 's/^note = "//; s/"$//' | fold -s -w 96 | sed 's/^/      /' | head -4
  echo
done
