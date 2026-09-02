#!/usr/bin/env bash
# g7: three residual checks.
#  (1) 136 section 7 restates five clauses. Section 5 diagnoses four predicates
#      vacuous. Which vacuous clause gets no replacement, and what does
#      section 1 say about it?
#  (2) does 7.2's predicate match what x1 actually swept?
#  (3) x1 A3's refutation: monotonicity implies locality. Is it in the output?
set -u
cd "$(dirname "$0")/.." || exit 1
T=136_leroy_the_candidate_revised_against_three_signatures.md

echo "=== (1) which 132 clauses does 136 section 7 restate? ==="
grep -n '^### 7\.' "$T" | sed 's/^/  /'
echo
echo "  section 5's four vacuous verdicts: 5.4 non-commutation, 5.6 variance,"
echo "  5.7 keying, 5.8 entropy."
echo
echo "  -- 5.7 is restated by no 7.x. What does section 1 say about 5.7? --"
grep -n "5.7" "$T" | sed 's/^/  /'

echo
echo "=== (2) 7.2's predicate against x1's actual sweep ==="
echo "  -- 136 7.2 says: --"
sed -n '349,353p' "$T" | sed 's/^/    /'
echo "  -- x1's committed header/params: --"
grep -nE 'W *= *9|F *= *4|4->2->0|4-to-2-to-0|signed' 136_probes/x1_output.txt | head -8 | sed 's/^/    /'

echo
echo "=== (3) x1 A3, the refuted prediction that widened the result ==="
grep -n -B2 -A6 'A3' 136_probes/x1_output.txt | sed 's/^/  /'
