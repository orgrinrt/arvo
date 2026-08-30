#!/usr/bin/env bash
# p6: the entailment check on 106 section 3.1's law bullet.
#
# 106 states, as a three-or-more-instances finding and with NO `holds for:`
# line, the only bullet in 3.1 without one:
#
#   "Multiplicative associativity and distributivity hold at F = 0 and fail
#    at F > 0."
#
# and cites 93's F1, 94's probe C part 2, and 97's criterion as support.
#
# This checks the sentence against the committed probe outputs of the two
# files it cites, at F = 0.
set -u
cd "$(dirname "$0")/.." || exit 1

echo "=== what 106 section 3.1 says, verbatim, with its (absent) predicate ==="
sed -n '323,329p' 106_giesen_consolidation_the_strategy_axis.md
echo
echo "-- every other bullet in 3.1 carries a 'holds for:' line; this one does not --"
awk 'NR>=287 && NR<=330' 106_giesen_consolidation_the_strategy_axis.md | grep -c 'holds for'
echo "   (2 of the 4 bullets; the law bullet and the 123-tests bullet have none)"

echo
echo "=== 93's F1 predicate, which 106 drops ==="
grep -A4 '^\*\*F1\. ' 93_orchard_the_strategy_axis_derived_cold.md | head -6

echo
echo "=== 93's P7, at F = 0, SIGNED SATURATING (the counterexample) ==="
grep -A4 -- '--- signed W = 7, domain \[-64, 63\], overflow = saturate ---' \
  93_probes/p7_signedness_breaks_the_congruence.out

echo
echo "=== 97's P2, at F = 0, the same cell, from an independently written model ==="
awk '/^signed   saturate F=0 truncate/{f=1} f&&/^====/{exit} f' \
  97_probes/p2_congruence_predicts_the_laws.out

echo
echo "=== and the hazard INSIDE the region the sentence is right about ==="
echo "-- unsigned saturate F=0: distrib holds, mul_over_sub does not --"
awk '/^unsigned saturate F=0 truncate/{f=1} f&&/^====/{exit} f' \
  97_probes/p2_congruence_predicts_the_laws.out | head -12

echo
echo "=== does 106 carry signedness anywhere in its own prose? ==="
echo -n "occurrences of 'signed' in 106 outside a quoted predicate: "
grep -n 'signed' 106_giesen_consolidation_the_strategy_axis.md | grep -vc 'holds for\|both signedness'
echo "-- the lines --"
grep -n 'signed\|signedness' 106_giesen_consolidation_the_strategy_axis.md

echo
echo "=== the same sentence is what the workspace rule was CORRECTED AWAY FROM during this unit ==="
grep -n 'necessary and it is not sufficient\|necessary and not sufficient' \
  /Users/orgrinrt/Dev/clause-dev/.claude/rules/arvo-always-optimal-internals.md
