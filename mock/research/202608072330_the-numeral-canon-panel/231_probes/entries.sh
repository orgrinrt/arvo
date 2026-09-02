#!/usr/bin/env bash
# One line per predicate entry: <file> <tab> <field start line> <tab> <slug>.
#
# Shared by `axis_uptake.sh` and `silence_width.sh`, which each read the
# registry's predicate entries and each got the count wrong the same way the
# first time. Both matched an entry by its own line, `^ *"<slug>: `, which
# cannot see an array written on one line: `predicate = ["a: x", "b: y"]`. Three
# such arrays exist, holding ten entries, one of which is the registry's only
# `strategy` entry, so the first reading of this reported that axis at zero.
#
# The two readers agreeing at 517 was not evidence, because they shared the
# blind spot. What caught it was opening the four fields the distribution
# reported at zero axes, which is the arm neither script had.
#
# Controls, outcomes written before the run:
#   E1  the total must be 527, which is the count
#       `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` states
#       in its own note: "139 of 527 entries drop the axis word from the values
#       side entirely". That number was arrived at by somebody else's reader,
#       so agreeing with it is an independent check rather than a restatement.
#   E2  at least one entry must come from a single-line array, or this reader is
#       the old one wearing a new name.
#   E3  every slug emitted must be a declared axis, which is what the
#       HARD_ERROR lint `every-predicate-names-a-declared-axis` enforces, so a
#       disagreement means this reader is splitting differently from the lint.
set -uo pipefail
cd "$(dirname "$0")"
REG=${REG:-../../../registry}

emit() {
  for f in "$REG"/*.toml; do
    awk -v F="$(basename "$f")" '
      function scan(s, start,   i, n, rest, slug) {
        rest = s
        while (match(rest, /"[a-z_]+:/)) {
          slug = substr(rest, RSTART+1, RLENGTH-2)
          print F "\t" start "\t" slug
          rest = substr(rest, RSTART+RLENGTH)
        }
      }
      /^(predicate|holds|fails) = \[.*\]$/ { scan($0, NR); next }
      /^(predicate|holds|fails) = \[/      { inarr=1; start=NR; next }
      inarr && /^\]/                       { inarr=0; next }
      inarr                                { scan($0, start) }
    ' "$f"
  done
}
emit
