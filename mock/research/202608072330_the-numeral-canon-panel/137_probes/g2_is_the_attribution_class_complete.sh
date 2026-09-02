#!/usr/bin/env bash
# g2: the class-fix question. 136 0.2 says x3 Q4d finds the wrong attribution
# in "five places across four files" and names them. This checks the LIST for
# completeness rather than the count, since a class fix is judged by whether it
# reached every instance.
set -u
cd "$(dirname "$0")/.." || exit 1

echo "=== the five 136 names, each opened ==="
for loc in 131:48 131:52 132:43 134:13 135:10; do
  f=${loc%%:*}; n=${loc##*:}
  fn=$(ls ${f}_*.md 2>/dev/null | head -1)
  printf "%-8s %s\n" "$loc" "$(sed -n "${n}p" "$fn" | cut -c1-95)"
done

echo
echo "=== is 130 in that list? what does 130 carry? ==="
sed -n '11,15p' 130_dolan_reply_one_axis_two_keyings.md
echo
echo "-- 130's commit time against 131's, i.e. which came first --"
git -C /Users/orgrinrt/Dev/clause-dev/arvo log --format='%ad %s' --date=format:'%H:%M' -- \
  mock/research/202608072330_the-numeral-canon-panel/130_dolan_reply_one_axis_two_keyings.md \
  mock/research/202608072330_the-numeral-canon-panel/131_leroy_formalising_the_rounding_axis.md | tail -4

echo
echo "=== what 133 ACTUALLY named, against 136's summary of it ==="
echo "-- 136 0.2 says: '133 names 131 and 132' --"
sed -n '36,37p' 136_leroy_the_candidate_revised_against_three_signatures.md
echo "-- 133 D1, verbatim --"
sed -n '197,200p' 133_knuth_signature_in_part_with_two_corrections.md

echo
echo "=== 136's two counts of the same class, side by side ==="
echo "-- section 0.2 --"; sed -n '36,38p' 136_leroy_the_candidate_revised_against_three_signatures.md | tr '\n' ' '; echo
echo "-- section 10 --"; sed -n '490,491p' 136_leroy_the_candidate_revised_against_three_signatures.md

echo
echo "=== files in THIS topic carrying the 123 figure at all, chronologically ==="
for f in 125 126 127 128 129 130 131 132 133 134 135 136; do
  fn=$(ls ${f}_*.md 2>/dev/null | head -1); [ -z "$fn" ] && continue
  n=$(grep -c '123 across 13' "$fn")
  [ "$n" -gt 0 ] && printf "  %-4s %s occurrence(s)\n" "$f" "$n"
done
