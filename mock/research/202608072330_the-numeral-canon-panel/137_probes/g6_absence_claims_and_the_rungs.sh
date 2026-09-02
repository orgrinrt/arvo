#!/usr/bin/env bash
# g6: two checks in one probe, both cheap.
#
# (A) ABSENCE CLAIMS. A negative claim about evidence names no place, so a
#     citation checker passes it by construction. Each one owes the search that
#     established it. Every absence claim in 136 is listed and the ones that are
#     checkable are checked.
#
# (B) THE RUNGS. 134 confirmed its own half (129's two). This checks the other
#     half (128's five) and the reconciliation to six, since a merge here would
#     have lost B6, which 132 calls the most consequential of the six.
set -u
cd "$(dirname "$0")/.." || exit 1

echo "############ (A) ABSENCE CLAIMS IN 136 ############"
echo
echo "--- every sentence in 136 making a negative claim about evidence ---"
grep -nE 'nobody|no file|no clause|no signature|never |returns nothing|appears in neither|no probe|unchallenged|no such|defines no' \
  136_leroy_the_candidate_revised_against_three_signatures.md | cut -c1-140 | sed 's/^/  /'

echo
echo "--- CHECKED 1: '`x3` Q5 greps: the sentence appears in neither 125 nor 126' ---"
echo "   the sentence is 5.3's 'no member carries more than one of the first three'"
for f in 125_*.md 126_*.md 131_*.md 132_*.md; do
  n=$(grep -c 'more than one of the first three' "$f")
  printf "     %-52s %s\n" "${f:0:52}" "$n"
done

echo
echo "--- CHECKED 2: '125 defines no T9' ---"
echo -n "     definitions matching **Tn ( in 125 : "; grep -c '^\*\*T[0-9]' 125_*.md
echo "     which labels are defined:"; grep -o '^\*\*T[0-9]b\?' 125_*.md | sort -u | tr '\n' ' '; echo
echo -n "     references to T9 in 125                : "; grep -c '\bT9\b' 125_*.md

echo
echo "--- CHECKED 3: 'no signature reported the four absences' ---"
for f in 133_*.md 134_*.md 135_*.md; do
  n=$(grep -ci 'vacuous\|absent dimension\|no domain dimension' "$f")
  printf "     %-52s %s\n" "${f:0:52}" "$n"
done
echo "     (135 dissents that 5.6/5.7 are OVER-NARROW, the opposite reading;"
echo "      136 4.2 says so and that is checked in the write-up)"

echo
echo "--- CHECKED 4: '135's z1 carries no negative control' ---"
grep -ciE 'control' 135_probes/z1_*.py 2>/dev/null | sed 's/^/     matches for control in z1: /'

echo
echo "--- CHECKED 5: '131 F131-6's vocabulary count is unchallenged by any signature' ---"
echo "     NOTE: F131-6 is the staged-narrowing finding, not the vocabulary one."
grep -n '^\*\*F131-6' 131_*.md | cut -c1-120 | sed 's/^/     /'
grep -n '^\*\*F131-3' 131_*.md | cut -c1-120 | sed 's/^/     /'

echo
echo "############ (B) THE RUNGS ############"
echo
echo "--- 128's five, verbatim from its own section 7 ---"
sed -n '241,248p' 128_*.md | sed 's/^/  /'
echo
echo "--- 129's two, verbatim from its own section ---"
sed -n '188,195p' 129_*.md | sed 's/^/  /'
echo
echo "--- B6 must be in 126 PHASE ONE to be blind. Is it? ---"
echo "  126's phase-one answer, stated up front:"
sed -n '24,26p' 126_*.md | sed 's/^/    /'
echo "  and 125's section 8 heading:"
grep -n '^## 8\.' 125_*.md | sed 's/^/    /'
echo
echo "--- does 133 (Knuth resumed) confirm B1..B6 as his phase-one content? ---"
sed -n '162,165p' 133_*.md | sed 's/^/  /'
