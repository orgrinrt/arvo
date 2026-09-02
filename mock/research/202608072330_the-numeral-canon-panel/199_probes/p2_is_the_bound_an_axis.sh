#!/usr/bin/env nutshell
# `196` section 2.3 is right that `191` searched for `trip count`, the vocabulary
# of its own hypothesis, and that a result written over `operation` and
# `chain_length` was invisible to it. That is conceded.
#
# So this asks the same question the way `196` says it should be asked: **by the
# declared axis, not by a phrase.** Is there a declared dimension expressing
# whether a bound on the count is available at compile time?
#
# The question is not rhetorical and it has three possible answers, all reachable:
#   1. Yes, and `191` missed it twice. Then `191` section 2.3 is dead outright.
#   2. `chain_length` covers it, so the axis exists under another name.
#   3. No dimension expresses it, in which case the corpus's results about folds
#      are all at a static bound and none of them varied the thing.
#
# ARM 1 lists every declared dimension with its grammar, so a reader can see the
# admissible values rather than take a verdict.
# ARM 2 takes every predicate entry in the registry on `chain_length` and `arity`
# and prints the distinct values written, which is what decides between 2 and 3:
# if every value is a number or a set of numbers, the notation has no way to say
# "not known until run time".
# ARM 3 searches the dimension namespace for the concept under every word I can
# think of that is not my own hypothesis's, since that is the failure being
# corrected.
#
# CONTROLS, three.
#   POSITIVE-A `chain_length` must be found by ARM 3's search on "fold", because
#     its keywords carry it. If ARM 3 finds nothing at all the search is broken.
#   POSITIVE-B ARM 2 must report values for `chain_length`, because `196` cites
#     four rows carrying one. Zero would mean the extractor missed the field.
#   NEGATIVE a dimension id that does not exist must report absent.
set -euo pipefail
root="$PWD"
while [ "$root" != "/" ] && [ ! -f "$root/mockspace.toml" ]; do root="$(dirname "$root")"; done
reg="$root/mock/registry"
echo "### registry: $reg"
echo

echo "######## ARM 1. every declared dimension and the values its grammar admits"
awk '
  /^\[\[dimension\]\]/ { id=""; g="" }
  /^id = /      { l=$0; gsub(/^id = "|"$/, "", l); id=l }
  /^grammar = / { l=$0; gsub(/^grammar = "|"$/, "", l); g=l;
                  printf "  %-18s %s\n", id, substr(g,1,96) }
' "$reg/dimension.toml"
echo "  -- count: $(grep -c '^\[\[dimension\]\]' "$reg/dimension.toml")"
echo

echo "######## ARM 2. every value written on chain_length and arity, across all namespaces"
for ax in chain_length arity accumulator_width; do
  echo "  --- $ax"
  { grep -rhoE "\"$ax: [^\"]*\"" "$reg" || true; } \
    | sed 's/^"//; s/"$//' | sort -u | sed 's/^/      /'
  n=$({ grep -rhoE "\"$ax: [^\"]*\"" "$reg" || true; } | grep -c . || true)
  echo "      (occurrences: $n)"
done
echo

echo "######## ARM 3. the concept searched for under words that are not 191's"
for w in "compile time" "static" "runtime" "dynamic" "bound" "capacity" "count" "unbounded" "fold" "known at" "trip"; do
  hits=$({ grep -in -- "$w" "$reg/dimension.toml" || true; } | wc -l | tr -d ' ')
  printf "  %-14s dimension.toml lines: %s\n" "$w" "$hits"
  [ "$hits" -eq 0 ] || { grep -in -- "$w" "$reg/dimension.toml" || true; } \
    | cut -c1-150 | sed 's/^/        /'
done
echo

echo "######## CONTROL NEGATIVE: a dimension that does not exist"
n=$({ grep -c 'id = "no_such_dimension_at_all"' "$reg/dimension.toml" || true; })
echo "  no_such_dimension_at_all: $n  (must be 0)"
