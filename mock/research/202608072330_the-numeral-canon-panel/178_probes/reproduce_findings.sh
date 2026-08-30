#!/bin/sh
# 178: reruns the instruments behind 177's seven findings and diffs against the
# committed outputs. CONTROL (must fail): p1's output diffed against p3's must
# DIFFER; if it reports REPRODUCES the differ is broken.
cd "$(dirname "$0")/.."
S="${TMPDIR:-/tmp}/178repro.$$"; mkdir -p "$S"
r() { if diff -q "$2" "$3" >/dev/null 2>&1; then echo "$1 : REPRODUCES"; else echo "$1 : DIFFERS"; fi }

python3 177_probes/p1_licence_family_biconditional.py > "$S/a" 2>&1
r "177 p1 (biconditional: (i) holds, (iii) fails; wrap column 1,2,3,1)" "$S/a" 177_probes/p1_output.txt
python3 177_probes/p3_marks_after_the_amendment.py > "$S/b" 2>&1
r "177 p3 (three mark positions, intra-sentential 1 before 2 after)" "$S/b" 177_probes/p3_output.txt
python3 177_probes/p2_anchor_diff_both_ways.py > "$S/c" 2>&1
r "177 p2 (independent accounting reproduction)" "$S/c" 177_probes/p2_output.txt
sh 177_probes/p6_reserved_still_open.sh > "$S/d" 2>&1
r "177 p6 (nothing reserved closed; gate list partial)" "$S/d" 177_probes/p6_output.txt

# F177-5: the two missing discount components, verified at source
c1=$(tr '\n' ' ' < 175_rompf_signature_in_part.md | grep -c "the world had to cooperate twice")
c2=$(tr '\n' ' ' < 175_rompf_signature_in_part.md | grep -c "Nothing establishes it found the only one")
d1=$(tr '\n' ' ' < 176_leroy_the_candidate_revised_against_two_signatures.md | grep -c "the world had to cooperate twice")
d2=$(tr '\n' ' ' < 176_leroy_the_candidate_revised_against_two_signatures.md | grep -c "Nothing establishes it found the only one")
echo "F177-5 components: in 175 = $c1,$c2 (want 1,1); in 176 = $d1,$d2 (want 0,0)"
# F177-7: the stale docstring control, verified at source
if grep -q "112:904-906" 176_probes/anchor_accounting/count_anchors.py; then
  echo "F177-7 stale docstring control : CONFIRMED PRESENT"
else
  echo "F177-7 stale docstring control : NOT FOUND (already repaired?)"
fi
# F177-6: the gate's enumeration, counted
n=$(sed -n '/Canon gate: passed/,/Test gate/p' 176_leroy_the_candidate_revised_against_two_signatures.md | tr '\n' ' ' | grep -o "container premise\|Q65\|X1[ ]*through[ ]*X4\|canon-form\|observability principle\|accuracy target\|carrier ships\|vocabulary calls" | sort -u | wc -l | tr -d ' ')
echo "F177-6 gate enumerates $n of 8 reserved names (want < 8 to confirm partial)"
echo "CONTROL (p1 vs p3, must DIFFER):"
r "  control" "$S/a" "$S/b"
rm -rf "$S"
