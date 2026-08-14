#!/usr/bin/env bash
# p3: the set difference. 106 section 15 line 1228 says "The census and the set
# difference are 106_probes/p6_anchor_census.sh". That script computes two counts
# and one grep. It runs no set difference: there is no comm, no diff, no join in
# it. Line 1263 then correctly assigns the set difference to the check after it.
# This is that.
#
# Same pattern and same exclusion as p6, so the two are directly comparable.
set -u
cd "$(dirname "$0")/.." || exit 1
P='[A-Za-z0-9_./-]+\.(rs|py|sh|toml|md|json|s|out|csv|tmpl)'
M="93_orchard*.md 94_wingo*.md 97_dolan*.md 98_spj*.md 100_xu*.md 101_wronski*.md 102_torvalds*.md 103_mcsherry*.md"

cat $M | grep -oE "$P" | sort -u > /tmp/107_union.txt
sed '/^## 15. Anchor accounting/,/^## 16./d' 106_giesen_consolidation_the_strategy_axis.md \
  | grep -oE "$P" | sort -u > /tmp/107_mine.txt
sed -n '/^## 15. Anchor accounting/,/^## 16./p' 106_giesen_consolidation_the_strategy_axis.md \
  | grep -oE "$P" | sort -u > /tmp/107_acc.txt

echo "union=$(wc -l < /tmp/107_union.txt|tr -d ' ')  carried=$(wc -l < /tmp/107_mine.txt|tr -d ' ')  dropped=$(comm -23 /tmp/107_union.txt /tmp/107_mine.txt|wc -l|tr -d ' ')"
echo
echo "=== anchors 106 dropped that its OWN section 15 then names (would defeat a diff run without exclusion) ==="
comm -23 /tmp/107_union.txt /tmp/107_mine.txt | comm -12 - /tmp/107_acc.txt
echo
echo "=== DROPPED, by class ==="
comm -23 /tmp/107_union.txt /tmp/107_mine.txt > /tmp/107_dropped_paths.txt
echo "-- superseded tier (.tmpl): correct to drop, must NOT be restored --"
grep '\.tmpl' /tmp/107_dropped_paths.txt
echo
echo "-- workspace rules dropped --"
grep -E '^[a-z][a-z0-9-]+\.md$' /tmp/107_dropped_paths.txt
echo
echo "-- live bench tree dropped --"
grep -E 'benches|variants/|bench-harness' /tmp/107_dropped_paths.txt
echo
echo "-- probe anchors dropped: count only, listed in full below --"
grep -c '_probes/' /tmp/107_dropped_paths.txt
echo
echo "-- everything else dropped --"
grep -vE '_probes/|\.tmpl|benches|variants/|bench-harness|^[a-z][a-z0-9-]+\.md$' /tmp/107_dropped_paths.txt
echo
echo "=== FULL DROPPED PROBE LIST ==="
grep '_probes/' /tmp/107_dropped_paths.txt
echo
echo "=== which member probe dirs are represented in what 106 carried ==="
grep -oE '[0-9]+_probes' /tmp/107_mine.txt | sort | uniq -c
echo
echo "=== which member probe dirs the union has ==="
grep -oE '[0-9]+_probes' /tmp/107_union.txt | sort | uniq -c
