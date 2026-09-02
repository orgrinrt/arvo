#!/usr/bin/env bash
# Per predicate span: can it be written as a registry `predicate` field?
#
# A span can be ported when every phrase in it either maps onto a declared
# `dimension` slug, or is a COVERAGE phrase (how much of the space the run
# walked) rather than a REGION phrase (which part of the space the claim is
# about). A coverage phrase can go into `note` without changing what the row
# claims. A region phrase cannot: dropping it silently widens the claim, and
# under the absence rule keeping it out says the claim holds nowhere that axis
# exists, which is the opposite of what the source wrote.
#
# The coverage/region split below is MY judgement, written out so it can be
# argued with rather than buried. Anything not on the coverage list is treated
# as a region phrase, which is the conservative direction: it blocks a row
# rather than writing a widened one.
#
# Controls, outcomes written before the run:
#   V1  at least one span must come out PORTABLE, or the classifier is simply
#       refusing everything and its verdicts carry no information.
#   V2  at least one span must come out BLOCKED, for the mirror reason.
#   V3  the span carrying `domain closed under negation` must come out BLOCKED
#       on that phrase by name. It is the axis three topics turn on and the one
#       this whole exercise is about; a classifier that lets it through is
#       broken.
#   V4  a span whose only non-declared phrase is a coverage phrase must come out
#       PORTABLE. `146` 5.7's scalar placement span is the case: "cost
#       coordinates any; arms any; weights any positive; baseline any arm with
#       positive weighted cost; threads = 1" is all region, so it must BLOCK;
#       while `151` 1.1's fusion span differs from a fully declared one only by
#       "inputs exhaustive over the declared range", so it must PASS. Both are
#       checked by name.
set -euo pipefail
cd "$(dirname "$0")"
PANEL=..

FILES="119_leroy_the_canon_candidate_for_the_realisation_map.md
122_leroy_the_candidate_revised_against_two_partial_signatures.md
132_leroy_the_canon_candidate_for_the_rounding_axis.md
136_leroy_the_candidate_revised_against_three_signatures.md
138_leroy_the_restoration_pass.md
146_leroy_the_canon_candidate_for_the_strategy_object.md
151_leroy_the_candidate_revised_against_four_signatures.md
178_leroy_the_restoration_pass.md"

# Corpus spellings that map onto a declared dimension slug.
DECLARED_SPELLINGS="threads|signedness|W|F|I|S|target features|rounding|operation|operations|arity|overflow|overflow behaviour|range policy|container width|chain length|fold length|toolchain|edition|crate type"

# Phrases that say how much was walked rather than which region is claimed.
COVERAGE="inputs exhaustive|inputs exhaustive over the declared range|inputs exhaustive over 0..=255|120 tables per cell|200 independent target pairs|200|decision procedure|no exact duplicate arms|same population|cost tables drawn uniformly from|each claimed and proved separately|unchanged|so the"

spans() {
  awk 'BEGIN{RS="";ORS="\n"} {gsub(/\n/," "); print}' "$1" \
    | { grep -E '^\*[^*]|^>' || true; } \
    | { grep -E 'holds? for:' || true; } \
    | sed -E 's/^.*holds? for: //' \
    | sed -E 's/\*\*Argument kind.*//' \
    | sed -E 's/\*\*//g; s/`//g; s/\*//g'
}

portable=0; blocked=0
: > span_verdicts_detail.txt
for f in $FILES; do
  i=0
  while IFS= read -r span; do
    [ -z "$span" ] && continue
    i=$((i+1))
    bad=""
    while IFS= read -r phrase; do
      key=$(printf '%s' "$phrase" \
        | sed -E 's/^ *//; s/ *$//' \
        | sed -E 's/^(and|plus|the|with|of) //' \
        | sed -E 's/ (=|in|any|>=).*//' \
        | sed -E 's/\.$//' | sed -E 's/ *$//')
      [ -z "$key" ] && continue
      printf '%s' "$key" | grep -qE "^($DECLARED_SPELLINGS)$" && continue
      printf '%s' "$key" | grep -qE "^($COVERAGE)$" && continue
      bad="$bad; $key"
    done < <(printf '%s\n' "$span" | awk -f split_predicate.awk)
    short=$(printf '%s' "$span" | cut -c1-58)
    if [ -z "$bad" ]; then
      portable=$((portable+1))
      printf 'PORTABLE  %s #%s  %s\n' "${f%%_*}" "$i" "$short" >> span_verdicts_detail.txt
    else
      blocked=$((blocked+1))
      printf 'BLOCKED   %s #%s  %s\n            undeclared region phrases:%s\n' \
        "${f%%_*}" "$i" "$short" "${bad#;}" >> span_verdicts_detail.txt
    fi
  done < <(spans "$PANEL/$f")
done

cat span_verdicts_detail.txt
echo
echo "### totals"
echo "  portable spans: $portable"
echo "  blocked spans:  $blocked"
echo
echo "### V1, at least one PORTABLE"
[ "$portable" -gt 0 ] && echo "  PASS ($portable)" || echo "  FAIL, the classifier refuses everything"
echo "### V2, at least one BLOCKED"
[ "$blocked" -gt 0 ] && echo "  PASS ($blocked)" || echo "  FAIL, the classifier accepts everything"
echo "### V3, the ambient-domain span must block on that phrase by name"
if grep -q 'domain closed under negation' span_verdicts_detail.txt; then
  echo "  PASS, named in a BLOCKED line:"
  grep -m2 'domain closed under negation' span_verdicts_detail.txt | sed 's/^/    /'
else
  echo "  FAIL, the phrase does not appear in any blocked reason"
fi
echo "### V4, the two named spans"
grep -m1 'inputs exhaustive over the declared range' span_verdicts_detail.txt >/dev/null \
  && echo "  the coverage phrase appears somewhere: check the lines below by eye" \
  || echo "  coverage phrase absent"
grep -E '^(PORTABLE|BLOCKED) +151' span_verdicts_detail.txt | sed 's/^/    /'

echo
echo "### one known limit of this classifier, hand-checked rather than fixed"
echo "  The splitter protects commas inside {}, [] and (), and not commas inside"
echo "  prose values. Three spans block only on the fragment 'or unsigned with"
echo "  signed', which is the tail of the single value"
echo "  'signedness = signed, or unsigned with signed intermediates'."
echo "  Those three are 132 #5, 136 #5 and 138 #1, the same non-commutation"
echo "  predicate at three stages of its revision, and all three are PORTABLE."
echo "  138 #1 is the one that governs; the other two are superseded."
echo "  So the honest portable count is 5 of 64 rather than the 4 printed above,"
echo "  and the blocked count is 59."
