#!/usr/bin/env bash
# G. Is `255`'s R5 a sentence that survives, or a census with an expiry?
#
# R5 is `the_primitive_surface_is_cut_by_kind_and_the_demand_rows_are_a_sample`,
# proposed at `255` section 4.2 as `rung = "ratified"`. Its `promotion` field
# ends: "Region: the five rows as they stand at `b34d7a3c`. A sixth consumer
# request lands in one of the kinds or names a kind this row lacks, and either
# is a finding rather than a refutation."
#
# The `ruling` namespace declares no `predicate` field, so that region is prose
# inside a field meant for the promotion argument. This arm asks the only
# question that decides whether that matters: does the namespace R5 quantifies
# over move?
#
# Controls, written before the run:
#   G1  `b34d7a3c` must resolve in this repository, or the region names nothing.
#   G2  the reader must count the same number of rows the engine does at HEAD.
#   G3  the walk must produce more than one distinct count over the file's
#       history, or the namespace has never moved and the question is idle.
set -uo pipefail
cd "$(dirname "$0")"
ROOT=$(cd ../../../.. && pwd)
cd "$ROOT"

echo "### G1, the commit R5 names"
git cat-file -t b34d7a3c 2>&1 | sed 's/^/  type: /'
git log -1 --format='  %h %ad %s' --date=short b34d7a3c 2>&1
git cat-file -t b34d7a3c >/dev/null 2>&1 && echo "  PASS, it resolves" || echo "  FAIL, it does not"
echo

echo "### G2, obligation rows at HEAD"
mine=$(grep -c '^\[\[obligation\]\]' mock/registry/obligation.toml)
eng=$( (cargo mock query 'obligation.count()' 2>/dev/null || true) | grep -oE '^[0-9]+$' | tail -1 )
echo "  grep $mine, engine ${eng:-?}"
[ "$mine" = "${eng:-x}" ] && echo "  PASS" || echo "  FAIL, the reader and the engine disagree"
echo

echo "### the obligation namespace's row count at every commit that touched it"
prev=""
distinct=0
for c in $(git log --reverse --format=%h -- mock/registry/obligation.toml); do
  n=$(git show "$c:mock/registry/obligation.toml" 2>/dev/null | grep -c '^\[\[obligation\]\]')
  [ "$n" != "$prev" ] && distinct=$((distinct+1))
  prev=$n
  printf '  %-9s %-3s %s\n' "$c" "$n" "$(git log -1 --format='%ad %s' --date=short "$c")"
done
echo
echo "### G3, distinct counts seen"
echo "  $distinct"
[ "$distinct" -gt 1 ] && echo "  PASS, the namespace R5 quantifies over has moved" \
  || echo "  FAIL, it has never moved, so the expiry is hypothetical"
echo

echo "### and whether it has moved since b34d7a3c on this branch"
git diff --stat b34d7a3c..HEAD -- mock/registry/obligation.toml | sed 's/^/  /'
echo "  (empty means unchanged on this branch, which is a fact about this branch"
echo "   rather than about the claim's durability)"
