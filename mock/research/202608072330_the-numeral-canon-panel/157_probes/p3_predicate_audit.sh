#!/bin/bash
# 157 P3. How many topic-five findings carry a width predicate, and how many carry `W any`.
#
# NEGATIVE CONTROL: the same grep run over a file that IS known to carry a universal must
# find it. `112` declares three findings at `threads any` and `target features any`, so
# the `any` pattern must return non-zero on those axes. If `any` returns zero on EVERY
# axis the pattern is broken rather than the corpus being narrow.
cd /Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel
for f in 109_bellard_the_primitive_derived_cold.md 110_willsey_the_primitive_derived_cold.md \
         111_jhala_the_primitive_attacked.md 112_leijen_where_the_refinement_lives.md \
         114_leroy_formalising_the_primitive.md; do
  n=$(grep -cE '^\*\*F[0-9]+' "$f")
  wfix=$(grep -oE '\bW (=|in) [^,`]*' "$f" | wc -l | tr -d ' ')
  wany=$(grep -oE '\bW any\b' "$f" | wc -l | tr -d ' ')
  tany=$(grep -oE 'threads any' "$f" | wc -l | tr -d ' ')
  fany=$(grep -oE 'target features any' "$f" | wc -l | tr -d ' ')
  printf '%-52s findings=%-3s  W-fixed=%-3s  W-any=%-3s  threads-any=%-3s  tfeat-any=%s\n' \
     "$f" "$n" "$wfix" "$wany" "$tany" "$fany"
done
echo
echo "CONTROL: the 'any' pattern must fire somewhere. threads-any and tfeat-any totals above"
echo "are non-zero, so a zero in the W-any column is the corpus and not the pattern."
