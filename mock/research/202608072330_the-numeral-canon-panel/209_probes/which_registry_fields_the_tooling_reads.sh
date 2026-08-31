#!/usr/bin/env bash
# Which question-namespace fields any shipped instrument reads, before and after.
#
# The claim it establishes: at the commit this dispatch started from, `decider`,
# `bound`, `unblocks` and `answers` were named by no check, no lint and no tool,
# so no roster of what op owes could be built from them and none was.
#
# **BEFORE is measured against that commit rather than by excluding paths.** The
# two artifacts this dispatch added both read those fields, so a walk of the
# working tree measures the repair and not what it repaired, and an exclusion
# list is a thing to keep in step that nobody will. `git grep <rev>` needs
# neither.
#
# The control is the second block. A grep returning zero for every field is a
# broken grep rather than a finding, so fields known to be read are searched by
# the same instrument, over the same trees, at the same two revisions, in the
# same run. `rung` is read by `awaiting-a-ruling`; `keywords` and `provenance`
# by the checks.
#
# Run from anywhere.

set -uo pipefail
cd "$(git -C "$(dirname "$0")" rev-parse --show-toplevel)" || exit 1

BASE=791a2d5a   # the merge this seat branched from
TREES=(mock/checks mock/lints mock/tools)

# A field is named in code as a string literal, so that is what is searched.
# The bare word matches prose: `bound` alone scores 9 at the base purely from
# "bounded" and "boundary" in comments, which is a measurement of English.
at() { git grep -oh -e "\"$2\"" "$1" -- "${TREES[@]}" 2>/dev/null | wc -l | tr -d ' '; }

row() { printf '%-14s %-8s %s\n' "$1" "$(at $BASE "$1")" "$(at HEAD "$1")"; }

printf 'trees: %s\nbase:  %s\n\n' "${TREES[*]}" "$BASE"
printf '%-14s %-8s %s\n' FIELD BEFORE AFTER
for f in decider bound unblocks answers; do row "$f"; done
echo
echo "--- control: fields already read at the base ---"
for f in rung keywords provenance; do row "$f"; done
