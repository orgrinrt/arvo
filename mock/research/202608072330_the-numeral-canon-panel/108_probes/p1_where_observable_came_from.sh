#!/usr/bin/env bash
# 108 p1. Where the word "observable" comes from, and what its qualifier was.
#
# The pair's first component is "an assignment on the axes a consumer can
# observe". Everything turns on that word. This traces it.
#
# Run from the panel root. Excludes nothing because the panel root has no
# target/ tree; the contamination hazard 106/107 record is under
# mock/benches/variants/ and does not apply here.
set -u
cd "$(dirname "$0")/.." || exit 1

echo "### 1. The definition, at its earliest appearance in this panel"
echo
grep -n "An axis is \*\*observable\*\*" 40_leijen_what_the_axes_actually_are.md
echo
echo "### 2. The qualifier 40 attaches to it, in its own section 5.2"
echo
sed -n '/^### 5.2 Headroom is unobservable only because of a convention/,/^### 5.3/p' \
  40_leijen_what_the_axes_actually_are.md
echo
echo "### 3. Who cites 40 in the strategy-axis unit, by count"
echo
for f in 93_*.md 94_*.md 97_*.md 98_*.md 100_*.md 101_*.md 102_*.md 103_*.md 106_*.md 107_*.md; do
  printf '%3s  %s\n' "$(grep -c '`40`' "$f")" "$f"
done
echo
echo "### 4. 97 credits 40 for the definition, explicitly"
echo
grep -n "40:398" 97_dolan_the_strategy_space_attacked.md
echo
echo "### 5. Does the qualifier survive? grep for the convention it depends on"
echo "    (the policy applying at the logical width rather than the container width)"
echo
for f in 97_*.md 98_*.md 100_*.md 101_*.md 102_*.md 103_*.md 106_*.md 107_*.md; do
  c=$(grep -ci "logical width" "$f")
  d=$(grep -ci "convention" "$f")
  printf '%3s logical-width  %3s convention   %s\n' "$c" "$d" "$f"
done
echo
echo "### 6. What 106 attributes the definition to"
echo
grep -n "defines an observable coordinate" 106_giesen_consolidation_the_strategy_axis.md
echo
echo "### 7. 40's own claim about novelty, and 97's honest correction of the record"
echo
grep -n "This is the piece I think is new" 40_leijen_what_the_axes_actually_are.md
grep -n "which is the piece I think is new" 40_leijen_what_the_axes_actually_are.md
