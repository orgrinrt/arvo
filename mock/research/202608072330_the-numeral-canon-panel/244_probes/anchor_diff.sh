#!/usr/bin/env bash
# Seat 244. The anchor-set difference between this consolidation's three sources
# and the consolidation. A rising total is not the check; the set difference is.
#
# Section 7 of the consolidation, which is where this output lands, is cut before
# the count so its own listing cannot make a dropped anchor present.
#
# The case that must fail: if the extraction returns nothing from the sources,
# the difference is empty for the wrong reason. Section 1 prints both cardinals
# and refuses to report a difference if either side is empty.
set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory

P='[0-9]+_[a-z_]+\.md|[a-z_]+\.(rs|toml):[0-9]+|[A-Za-z0-9_]+\.(rs|toml)'

SRC1=241_kiselyov_admission_is_a_resolution_not_a_verdict.md
SRC2=242_what-admits-a-number-system.md
SRC3=243_seat242_the_resolution_has_no_second_arm.md
MINE=244_orchard_consolidation_admission_and_the_number_system.md

# Everything except section 7, which is the accounting itself.
mine_body() { awk '/^## 7\. Accounting/{skip=1} /^## 8\. Coverage/{skip=0} !skip' "$MINE"; }

anchors() { grep -oE "$P" | sort -u; }

S=$( { cat "$SRC1" "$SRC2" "$SRC3"; } | anchors )
M=$( mine_body | anchors )

ns=$(printf '%s\n' "$S" | grep -c . )
nm=$(printf '%s\n' "$M" | grep -c . )

echo "sources: $SRC1"
echo "         $SRC2"
echo "         $SRC3"
echo
printf 'anchors in the three sources : %d\n' "$ns"
printf 'anchors in the consolidation : %d (section 7 excluded)\n' "$nm"
if [ "$ns" -eq 0 ] || [ "$nm" -eq 0 ]; then
  echo "CONTROL FAILED: one side extracted nothing, so a difference would be about the pattern."
  exit 2
fi
echo "control passes: both sides are nonempty, so the difference below is real."

echo
echo "carried (in both):"
comm -12 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | sed 's/^/  /'

echo
echo "LOST (in the sources, not in the consolidation):"
comm -23 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | sed 's/^/  /'
printf '  count = %d\n' "$(comm -23 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | grep -c .)"

echo
echo "new (in the consolidation, not in the sources):"
comm -13 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | sed 's/^/  /'
printf '  count = %d\n' "$(comm -13 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | grep -c .)"

echo
echo "NOTE on the pattern, which is my brief's and which I did not change:"
echo "  [0-9]+_[a-z_]+\\.md does not match 242_what-admits-a-number-system.md,"
echo "  because that filename uses hyphens. So the sources' own citations of 242"
echo "  by filename are invisible to this instrument in both directions, which is"
echo "  symmetric and does not bias the difference."
