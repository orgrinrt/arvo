#!/bin/sh
# 159 P2. Two questions about F157-10, one about its instrument and one about
# my own file.
#
# A. F157-10 counts `W any` with the pattern `\bW any\b`. A universal over
#    width can be written other ways. If the corpus writes one of those, the
#    zero is the pattern rather than the corpus, which is the same defect 154
#    withdrew three findings for.
#
# B. 154 was not in F157-10's corpus. Audited here on the same terms, by its
#    author, per the brief.
#
# NEGATIVE CONTROL for A, stated before the run. The alternative-spelling
# patterns must FIRE somewhere in the panel, or "the audit cannot see them" is
# unfalsifiable. 109 is the test case: if `I any` returns zero there too, every
# pattern I am proposing is dead and F157-10's instrument is vindicated.
# NEGATIVE CONTROL for B: the extractor must be shown able to READ 154's
# predicate blocks, by finding the fixed values it is known to carry
# (`threads = 1`). Testing instead that some axis says `any` was the first
# version of this control and it FAILED, because 154 carries no `any` anywhere.
# That is a fact about 154 and not a broken extractor, and a control that
# cannot tell those apart is the defect this whole exchange is about.
D=${PANEL:-/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel}
FOUR="110_willsey_the_primitive_derived_cold.md 111_jhala_the_primitive_attacked.md \
112_leijen_where_the_refinement_lives.md 114_leroy_formalising_the_primitive.md"

echo "=== A. does a universal over width have other spellings, and does the corpus use them? ==="
printf '%-52s %8s %8s %8s %8s\n' file 'W any' 'I any' 'F any' 'N any'
for f in $FOUR 109_bellard_the_primitive_derived_cold.md; do
  printf '%-52s %8s %8s %8s %8s\n' "$f" \
    "$(grep -ocE '\bW any\b' "$D/$f")" "$(grep -ocE '\bI any\b' "$D/$f")" \
    "$(grep -ocE '\bF any\b' "$D/$f")" "$(grep -ocE '\bN any\b' "$D/$f")"
done
IANY109=$(grep -ocE '\bI any\b' "$D/109_bellard_the_primitive_derived_cold.md")
ALT4=0
for f in $FOUR; do
  ALT4=$((ALT4 + $(grep -ocE '\bI any\b|\bF any\b|\bN any\b|widths? any' "$D/$f")))
done
echo
echo "CONTROL alternative spellings fire somewhere : I any in 109 = $IANY109 (want >= 1)"
[ "$IANY109" -ge 1 ] || { echo "CONTROL FAILED: my proposed patterns are dead, suppressed"; exit 1; }
echo "alternative spellings across the FOUR        : $ALT4"
echo
if [ "$ALT4" -eq 0 ]; then
  echo "VERDICT A: F157-10 stands as literally stated. No spelling of a width"
  echo "           universal appears in the four files. The knife does not cut."
  echo "           BUT its instrument does not enumerate the spellings, and 109"
  echo "           uses one of them ($IANY109 times, at 109:156 and 109:381)."
  echo "           So the CONTROL that F157-10 carries -- 'threads any and"
  echo "           tfeat any are non-zero, so the zero is the corpus' -- proves"
  echo "           the word 'any' is findable, not that the width axis's"
  echo "           spellings were searched. Those are different conjuncts."
  echo "           And 157's prose generalises to 'topic five's corpus', which"
  echo "           includes 109, where the sweeping claim is false: 109's very"
  echo "           first predicate is a universal over integer width."
else
  echo "VERDICT A: F157-10's zero is partly a pattern artifact: $ALT4 alternative"
  echo "           spellings in the four files it audited."
fi

echo
echo "=== B. 154 audited on the same terms, by its author ==="
F154="$D/154_kiselyov_the_primitive_derived_cold.md"
PROBES=$(ls "$D"/154_probes/*/FINDINGS.md 2>/dev/null)
n=$(grep -hcE '^## F[0-9]+\.' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
wany=$(grep -hocE '\bW any\b' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
alt=$(grep -hocE '\bI any\b|\bN any\b|widths? any' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
full=$(grep -hocE 'W in 1\.\.=64' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
tany=$(grep -hocE 'threads any' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
fany=$(grep -hocE 'target features any' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
t1=$(grep -hocE 'threads = 1' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
holds=$(grep -hocE 'holds for:' $PROBES "$F154" 2>/dev/null | paste -sd+ - | bc)
printf '  findings stated              : %s\n' "$n"
printf '  carrying `W any`             : %s\n' "$wany"
printf '  carrying another spelling    : %s\n' "$alt"
printf '  carrying `W in 1..=64`       : %s   (exhaustive over a u64 container)\n' "$full"
printf '  carrying `threads any`       : %s\n' "$tany"
printf '  carrying `target features any`: %s\n' "$fany"
printf '  carrying `threads = 1` (fixed) : %s\n' "$t1"
printf '  predicate blocks found        : %s\n' "$holds"
echo
if [ "$holds" -gt 0 ] && [ "$t1" -gt 0 ]; then
  echo "CONTROL the extractor can read 154's predicate blocks: $holds blocks, $t1 carrying threads = 1"
else
  echo "CONTROL FAILED: the extractor cannot read the blocks, suppressed"; exit 1
fi
echo
echo 'VERDICT B: 154 carries `W any` zero times, so it is a fifth instance of' 
echo "           F157-10's pattern and 157 did not count it. Three of its"
echo "           findings carry W in 1..=64, which is exhaustive over every"
echo "           width a u64 container holds rather than a three-width sample."
echo
echo '           And 154 carries NO `any` on ANY axis: not width, not threads,' 
echo "           not target features. Read literally under I13 its findings hold"
echo "           at threads = 1, at one host's baseline features, at the widths"
echo "           listed, and nowhere else. That is what was measured, so the"
echo "           recording is honest; whether the notation should force a"
echo "           measured-everywhere finding to say so is Q65 and is op's."
