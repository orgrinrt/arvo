#!/bin/sh
# 177 P6. Are the reserved questions still open after the revision?
# Each is searched for in 173 section 6 (what only op decides) and in 176, and any
# sentence in 176 that could read as a closure is printed for reading rather than counted.
#
# CASES THAT MUST FAIL
#   Q-A  a reserved item must be FOUND in 173 section 6, else the pattern is broken
#   Q-B  a string that is not a reserved item must return zero
cd "$(dirname "$0")/.." || exit 1
A=173_leroy_the_canon_candidate_for_the_chain.md
B=176_leroy_the_candidate_revised_against_two_signatures.md
echo "=== 173 section 6, what only op decides ==="
sed -n '/^## 6\. What only op decides/,/^## 7\./p' $A | sed 's/^/  /'
echo
echo "=== 176's own statement of what stays reserved ==="
sed -n '/^## 0\. The two gates/,/^## 1\./p' $B | sed 's/^/  /'
echo
echo "=== does 176 contain any verb of closure applied to a reserved item? ==="
for k in 'container premise' 'Q65' 'X1' 'X-A' 'X-B' 'X-C' 'X-D' 'X-E' 'X-F' '156 item 2' 'observability principle'; do
  n=$(grep -c -- "$k" $B)
  printf '  %-24s occurrences in 176: %s\n' "$k" "$n"
done
echo
echo "  lines in 176 mentioning a reserved item, printed for reading:"
grep -nE 'container premise|Q65|156 item 2|observability principle|X-[A-F]' $B | cut -c1-165 | sed 's/^/    /'
echo
echo "=== CONTROLS ==="
printf '  Q-A "Q65" present in 173: %s  (must be > 0)\n' "$(grep -c 'Q65' $A)"
printf '  Q-B "Q99-not-real" in 176: %s  (must be 0)\n' "$(grep -c 'Q99-not-real' $B)"
