#!/usr/bin/env bash
# Seat 256. The brief said the sitting on admission (241, 242, 243) had no consolidation
# and that it was to be numbered after 243. This prints what the tree at origin/dev holds
# instead, and re-runs the two premise checks 244 section 0 and 245 already ran, with
# both controls, so the third refutation is measured rather than inherited.
#
# The case that must fail: section 2's pattern is shown firing on a planted line before
# it is run over the sources, so a zero over 242 and 243 is a zero about the files and not
# about the pattern.
set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory

echo "== 1. the files after 243, and the origin/dev commit that added each"
for n in 244 245 246 247 248 249 250 251 252 253 254 255; do
  f=$(ls "${n}"_*.md 2>/dev/null | head -1)
  if [ -z "$f" ]; then echo "$n  MISSING"; continue; fi
  printf '%s  %s  %s\n' "$n" "$(git log --diff-filter=A --format='%h %ad' --date=short -1 origin/dev -- "$f")" "$f"
done

echo
echo "== 2. does 242 or 243 say its reading must not be merged with 241's"
PAT='merg|combin|synthes|kept apart|read as one|must not be (read|joined)|do not (merge|combine)'
echo "negative control, a planted line the pattern must catch:"
echo "  their two readings must not be merged" | grep -ciE "$PAT"
echo "hits over the two files, each hit printed so the reader sees what it is:"
for f in 242_what-admits-a-number-system.md 243_seat242_the_resolution_has_no_second_arm.md; do
  printf "  %s  %s\n" "$(grep -ciE "$PAT" "$f")" "$f"
  grep -niE "$PAT" "$f" | sed "s/^/      /"
done
echo "positive control, words known to be in each file:"
printf '  resolution in 243: %s\n' "$(grep -c 'resolution' 243_seat242_the_resolution_has_no_second_arm.md)"
printf '  tier in 242: %s\n' "$(grep -c 'tier' 242_what-admits-a-number-system.md)"

echo
echo "== 3. the tier count: where 244 filed it, and where 246 retired that filing"
grep -n '^### C2\.' 244_orchard_consolidation_admission_and_the_number_system.md
grep -n '^### 5\.4' 246_kiselyov_the_two_promotions_and_what_they_second.md
echo "the only tier-count phrase in 242 itself:"
grep -n -iE 'tier count|three tiers|five tiers|number of tiers' 242_what-admits-a-number-system.md

echo
echo "== 4. how many briefs have carried the premise, by the files that quote it"
grep -l 'must NOT be merged\|must not be merged' 244_*.md 245_*.md
