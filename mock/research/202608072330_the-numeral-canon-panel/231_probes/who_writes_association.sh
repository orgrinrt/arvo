#!/usr/bin/env bash
# How many personas write about the association of a term as a coordinate?
#
# `230` settles axis candidacy partly on a persona count, on the ground that
# `ambient_domain` and `radix` were declared at two. If association is a real
# coordinate it should clear the same bar, in the same currency, counted over
# the whole panel rather than the eight files the ranking used.
#
# TWO THINGS THIS GOT WRONG FIRST TIME, both caught by A1 and kept in
# `who_writes_association_first_attempt.out`:
#
#   1. `quaternion_of_tuesdays` came back at TWO personas, because the corpus now
#      contains `229` and `230`, which both name that control token. Our own two
#      deliverables are the reading, not the corpus, and are excluded here.
#   2. `persona` and `the` were counted as personas. They are filename stems of
#      `NNN_persona_checkpoint_*.md` and `NNN_the_*.md`, not authors.
#
# Controls, outcomes written before the run:
#   A1  a phrase nobody writes must return zero. THE FIRST TWO TOKENS FAILED IT.
#       `phase_of_the_moon` is not a phrase nobody writes: it is this panel's
#       shared name for a non-axis control, and `216_lamport` writes it in its
#       own prose. Both failing runs are kept beside this.
#   A2  `threads` must return many, or the persona extraction is broken.
#   A3  the count must not saturate.
#   A4  the exclusion must actually exclude: `229` and `230` must not appear in
#       the file list this reads.
set -uo pipefail
cd "$(dirname "$0")"
PANEL=..

# The panel's own files, minus the two readings of this question and minus the
# stems that are not authors.
files() { ls "$PANEL"/[0-9]*.md | grep -vE '/(229|230)_'; }
pers() {
  grep -l "$1" $(files) 2>/dev/null | xargs -n1 basename 2>/dev/null \
    | awk -F_ '{print $2}' | grep -vE '^(persona|the|op|checkpoint|dispatcher|catalogue)$' | sort -u
}
count() { pers "$1" | wc -l | tr -d ' '; }

echo "### A4, the exclusion"
n229=$(files | grep -c '/229_' || true); n230=$(files | grep -c '/230_' || true)
echo "  229 files in the read: $n229, 230 files: $n230"
if [ "$n229" = "0" ] && [ "$n230" = "0" ]; then echo "  PASS"; else echo "  FAIL"; fi

echo "### A1, a phrase nobody writes"
n=$(count 'quaternion_of_tuesdays'); echo "  quaternion_of_tuesdays: $n"
if [ "$n" = "0" ]; then echo "  PASS"; else echo "  FAIL, still contaminated"; fi

echo "### A2, a phrase everybody writes"
n=$(count 'threads'); echo "  threads: $n personas"
if [ "$n" -gt 10 ]; then echo "  PASS"; else echo "  FAIL"; fi

echo "### A3, the count must not saturate"
echo "  discharge check: $(count 'discharge check') personas"

echo
echo "### personas writing about the association of a term"
for p in "left-fold" "tree-fold" "balanced tree" "left fold" "parenthesisation"; do
  printf '  %-20s %-3s  %s\n' "$p" "$(count "$p")" "$(pers "$p" | tr '\n' ' ')"
done

echo
echo "### the union, which is the number that answers the question"
{ pers "left-fold"; pers "tree-fold"; pers "balanced tree"; pers "left fold"; pers "parenthesisation"; } \
  | sort -u > assoc_personas.txt
printf '  %s personas: %s\n' "$(wc -l < assoc_personas.txt | tr -d ' ')" "$(tr '\n' ' ' < assoc_personas.txt)"

echo
echo "### the same for leaf aliasing, 230's candidate, in one currency"
{ pers "every leaf occurs at most once"; pers "leaf identification"; pers "distinct occurrences"; } \
  | sort -u > alias_personas.txt
printf '  %s personas: %s\n' "$(wc -l < alias_personas.txt | tr -d ' ')" "$(tr '\n' ' ' < alias_personas.txt)"

echo
echo "### and the bar both are measured against"
echo "  dimension::ambient_domain and dimension::radix were declared on two"
echo "  independent readings each, which is the count 230 uses as the threshold."
