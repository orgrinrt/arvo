#!/usr/bin/env bash
# Where "2106 occurrences of the word consumer across 436 files" came from.
#
# `184` offers that figure as the size of the corpus it did not sweep, and it
# has travelled since, including into a coordinator state file. It reproduces
# at neither of the two scopes anybody has since measured. This finds the one
# it does reproduce at, because a figure whose scope is unknown cannot be
# argued with and this one is load-bearing: it is the stated reason the panel's
# own corpus is where the rest of the demand side is.
#
# Measured at the commit `184` landed, not at HEAD, because the panel has grown
# since and a figure is a fact about a moment.
#
# Control: a scope that must differ (`*.md` alone) is printed beside the one
# that matches, so a reader can see the match is a scope and not a coincidence
# of any count being near.
set -euo pipefail
root="$(cd "$(dirname "$0")" && pwd)"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
cd "$root"

H=$(git log --format=%h --diff-filter=A --all -- '*184_dispatcher_note_the_demand_side.md' | tail -1)
P=mock/research/202608072330_the-numeral-canon-panel
echo "184 landed at $H"
echo
printf '%-46s %-9s %s\n' SCOPE OCCURRENCES FILES
for pat in "$P/*.md" "$P/"; do
  c=$(git grep -oh 'consumer' "$H" -- "$pat" 2>/dev/null | wc -l | tr -d ' ')
  f=$(git grep -l  'consumer' "$H" -- "$pat" 2>/dev/null | wc -l | tr -d ' ')
  printf '%-46s %-9s %s\n' "$pat" "$c" "$f"
done
echo
echo "reported by 184:                               2106      436"
echo
echo "file types carrying the hits at that commit, all-types scope:"
git grep -l 'consumer' "$H" -- "$P/" 2>/dev/null | sed 's/.*\.//' | sort | uniq -c | sort -rn | sed 's/^/  /'
