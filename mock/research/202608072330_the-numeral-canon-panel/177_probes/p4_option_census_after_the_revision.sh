#!/bin/sh
# 177 P4. Is O-171-1 findable by a label-driven option-set diff after 176's R-o, and are
# there OTHER labels at zero that 175's census reported and nobody has followed up?
#
# 175's census ran against 173 alone. This runs it against 173, against 176, and against
# the pair, since 176 governs where it amends and 173 governs elsewhere, so the pair is
# what a later reader actually holds.
#
# CASES THAT MUST FAIL
#   N-A  a label that does not exist must return zero everywhere, else a hit means nothing
#   N-B  at least one label must be found in 173, else the pattern is broken
#   N-C  O-171-1 must still be zero in 173 alone, reproducing 175's B6; if it is not,
#        my reading of the census differs from 175's and the comparison is invalid
cd "$(dirname "$0")/.." || exit 1
A=173_leroy_the_canon_candidate_for_the_chain.md
B=176_leroy_the_candidate_revised_against_two_signatures.md
flat() { tr '\n' ' ' < "$1" | sed 's/  */ /g'; }
TA=$(flat $A); TB=$(flat $B); TP="$TA $TB"
printf '%-10s %8s %8s %8s\n' label in-173 in-176 in-pair
for o in Q-C1 Q-C2 Q-C3 Q-C4 Q-C5 Q-C6 Q-C7 O-171-1 O-171-2 O-171-3 O-171-4 O-169-2 O-4 O-5; do
  a=$(printf '%s' "$TA" | grep -o -- "$o" | wc -l | tr -d ' ')
  b=$(printf '%s' "$TB" | grep -o -- "$o" | wc -l | tr -d ' ')
  p=$(printf '%s' "$TP" | grep -o -- "$o" | wc -l | tr -d ' ')
  printf '%-10s %8s %8s %8s\n' "$o" "$a" "$b" "$p"
done
echo
echo "--- N-A control: a label that does not exist ---"
for o in Q-C99 O-171-9; do
  printf '  %-10s 173=%s 176=%s\n' "$o" \
    "$(printf '%s' "$TA" | grep -o -- "$o" | wc -l | tr -d ' ')" \
    "$(printf '%s' "$TB" | grep -o -- "$o" | wc -l | tr -d ' ')"
done
echo
echo "--- where O-171-1 sits in 176 ---"
grep -n 'O-171-1' $B | sed 's/^/  /'
echo
echo "--- 176's section headings, so a reader can see which pass R-o lives in ---"
grep -n '^## ' $B | sed 's/^/  /'
