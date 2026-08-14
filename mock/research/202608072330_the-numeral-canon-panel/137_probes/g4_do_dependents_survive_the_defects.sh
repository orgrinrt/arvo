#!/usr/bin/env bash
# g4: 136 section 9 records four defects in its own probes. The check is not
# whether they are recorded but whether any CLAIM still rests on a result a
# defective run produced. Traced per defect, with the half-even one first
# because it is the one that wrongly reported a mode carrying exact composition.
set -u
cd "$(dirname "$0")/.." || exit 1

echo "=== DEFECT 1. x1 part B first run reported half_even carrying exact composition ==="
echo "-- what the corrected committed output says --"
sed -n '30,42p' 136_probes/x1_output.txt
echo
echo "-- does any sentence in 136 give half-even exact composition? --"
grep -n -i 'half.even' 136_leroy_the_candidate_revised_against_three_signatures.md | sed 's/^/   /'
echo
echo "-- and what 132 5.3 said, which the defect would have contradicted --"
grep -n 'nearest members make a staged narrowing' 132_leroy_the_canon_candidate_for_the_rounding_axis.md
echo "-- and 125's own P4 count, the third witness --"
grep -n 'half_up 500, half_even 500' 125_knuth_rounding_cold_derivation.md

echo
echo "=== DEFECT 2. x2's two controls could not fire. Do the replacements fire? ==="
grep -n -i 'control' 136_probes/x2_output.txt | sed 's/^/   /'

echo
echo "=== DEFECT 3/4. x4 reported a confident zero, and missed one predicate ==="
echo "-- the corrected x4's predicate count and absences --"
grep -n -iE 'predicates|absent|domain' 136_probes/x4_output.txt | head -18 | sed 's/^/   /'

echo
echo "=== does 136 cite x2 P3/P4 for claims that are now load-bearing? ==="
grep -n 'x2. P[0-9]\|x2` P[0-9]' 136_leroy_the_candidate_revised_against_three_signatures.md | sed 's/^/   /'
echo
echo "-- x2 P4, the condition that bounds 135's widening --"
grep -n -A6 'P4' 136_probes/x2_output.txt | head -20 | sed 's/^/   /'
