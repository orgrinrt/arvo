#!/usr/bin/env bash
# Which axes do the five later topics actually predicate over, and how many of
# them does `mock/registry/dimension.toml` declare?
#
# The question is not decorative. A predicate entry naming an undeclared axis is
# refused by `mock/checks/tests/every_predicate_names_a_declared_axis.rs`, so an
# axis the corpus uses and the registry does not declare names a region that
# cannot be written into the canon at all.
#
# Method. A predicate in this corpus is an emphasis run inside one paragraph, so
# the paragraph is the delimiter: the span runs from `holds for:` to the end of
# its paragraph, minus the trailing `**Argument kind...**` tag. Split on `;`,
# take the key side of each phrase.
#
# THE FIRST VERSION OF THIS SCRIPT WAS WRONG AND ITS OWN CONTROL SAID SO. It cut
# the span at the end of the flattened file rather than at the end of a
# paragraph, so every span swallowed the prose after it and the key list came
# back full of sentences. C2 failed, and that is how it was caught, not by
# reading the code. The output is kept beside this as
# `axis_census_first_attempt.out`. Four extractors in this panel have now been
# defeated by the span boundary (`136` x4, `151` v2, `138` y1, and this one),
# which is worth more than the census itself: the boundary is where this class
# lives, every time.
#
# Controls, each with the outcome required of it written before the run:
#   C1  `threads` and `signedness` MUST come out as bare declared keys.
#   C2  `radix` MUST appear as a bare key and MUST read undeclared. No dimension
#       row mentions it, and it is short enough that an over-capturing span
#       hides it inside a longer string, which is what caught version one.
#   C3  every governing file's span count is printed BEFORE any aggregation, so
#       a file the pattern does not match is visible rather than absorbed.
#   C4  `domain` MUST appear spelled in full. `136`'s x4 parsed it as `doma` by
#       matching the `in` inside the word.
#   C5  no key may exceed 40 characters. A key that long is a swallowed
#       sentence, which is the defect version one had and the control it lacked.
set -euo pipefail
cd "$(dirname "$0")"
PANEL=..
REG=../../../registry

FILES="119_leroy_the_canon_candidate_for_the_realisation_map.md
122_leroy_the_candidate_revised_against_two_partial_signatures.md
132_leroy_the_canon_candidate_for_the_rounding_axis.md
136_leroy_the_candidate_revised_against_three_signatures.md
138_leroy_the_restoration_pass.md
146_leroy_the_canon_candidate_for_the_strategy_object.md
151_leroy_the_candidate_revised_against_four_signatures.md
161_leroy_the_canon_candidate_for_the_primitive.md
164_leroy_the_candidate_revised_against_two_signatures.md
173_leroy_the_canon_candidate_for_the_chain.md
176_leroy_the_candidate_revised_against_two_signatures.md
178_leroy_the_restoration_pass.md"

DECLARED=$(grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//')
echo "### declared axes ($(printf '%s\n' "$DECLARED" | wc -l | tr -d ' '))"
printf '%s\n' "$DECLARED" | tr '\n' ' '; echo; echo

spans() {
  awk 'BEGIN{RS="";ORS="\n"} {gsub(/\n/," "); print}' "$1" \
    | { grep -E '^\*[^*]|^>' || true; } \
    | { grep -E 'holds? for:' || true; } \
    | sed -E 's/^.*holds? for: //' \
    | sed -E 's/\*\*Argument kind.*//' \
    | sed -E 's/\*\*//g; s/`//g; s/\*//g'
}

echo "### C3, predicate spans per governing file, printed before aggregation"
total=0
for f in $FILES; do
  n=$(spans "$PANEL/$f" | { grep -c . || true; })
  total=$((total + n))
  printf '  %-70s %s\n' "$f" "$n"
done
echo "  total spans: $total"
echo

for f in $FILES; do spans "$PANEL/$f"; done \
  | awk -f split_predicate.awk \
  | sed -E 's/^ *//; s/ *$//' \
  | sed -E 's/^(and|plus|the|with|of) //' \
  | sed -E 's/ (=|in|any|>=).*//' \
  | sed -E 's/\.$//' \
  | sed -E 's/ *$//' \
  | grep -v '^$' \
  | sort | uniq -c | sort -rn > keys.txt

echo "### keys the corpus predicates over, by occurrence, against the declared set"
declared_n=0; undeclared_n=0
while read -r n key; do
  slug=$(printf '%s' "$key" | tr 'A-Z ' 'a-z_' | tr -d '()')
  mark="UNDECLARED"
  for d in $DECLARED; do
    case "$slug" in "$d"|"${d}s") mark="declared" ;; esac
  done
  case "$slug" in
    w|width) mark="declared as total_width" ;;
    f) mark="declared as fraction_width" ;;
    i) mark="declared as integer_width" ;;
    s) mark="declared as strategy" ;;
    overflow|overflow_behaviour|range_policy) mark="declared as overflow_policy" ;;
    operations) mark="declared as operation" ;;
    fold_length) mark="declared as chain_length" ;;
    container_width|accumulator_width) mark="declared as container" ;;
    debug-assertions|opt_level|toolchain|edition|crate_type) mark="declared as build_profile, loosely" ;;
  esac
  case "$mark" in UNDECLARED) undeclared_n=$((undeclared_n+1));; *) declared_n=$((declared_n+1));; esac
  printf '  %4s  %-40s %s\n' "$n" "$key" "$mark"
done < keys.txt
echo
echo "  distinct keys $((declared_n + undeclared_n)), of which declared $declared_n and undeclared $undeclared_n"

echo
echo "### C1, two axes that must read as bare declared keys"
for k in threads signedness; do
  if grep -qE "^ +[0-9]+ $k\$" keys.txt; then echo "  $k: PASS"; else echo "  $k: FAIL, extractor is broken"; fi
done
echo "### C2, one key that must be present and must read undeclared"
if grep -qE "^ +[0-9]+ radix\$" keys.txt; then
  echo "  radix: PASS, present as a bare key and no dimension row declares it"
else
  echo "  radix: FAIL, the span is over-capturing and hiding short keys"
fi
echo "### C4, the key three earlier extractors mangled"
if grep -qE "^ +[0-9]+ domain\$" keys.txt; then echo "  domain: PASS, spelled in full"; else echo "  domain: FAIL"; fi
echo "### C5, no key may carry a sentence-ending period, which is the over-capture tell"
bad=$(grep -cE '\. ' keys.txt || true)
if [ "$bad" = "0" ]; then
  echo "  0 keys carry a sentence break: PASS"
else
  echo "  $bad keys carry a sentence break, the span is swallowing prose: FAIL"
  grep -E '\. ' keys.txt | sed 's/^/    /'
fi

echo
echo "### the phrases the corpus writes into a predicate that are not <key> <op> <value>"
echo "### (not an extractor defect: these are the corpus's own entries, and none of"
echo "###  them can become a registry predicate entry, which needs a declared slug)"
awk '{n=$1;$1="";sub(/^ /,"");if(length($0)>34)printf "  %4s  %s\n",n,$0}' keys.txt
