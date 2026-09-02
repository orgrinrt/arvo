#!/usr/bin/env bash
# p2b: p2 undercounted. Members label findings (F1, W9, D3, T2, X4, M5...), and
# a reference of the form `93`'s F9 IS a locatable anchor even though it names no
# section. Recount admitting finding IDs, section numbers and probe filenames as
# locators. A reference followed by none of those points at a whole file.
set -u
cd "$(dirname "$0")/.." || exit 1
T=106_giesen_consolidation_the_strategy_axis.md
A=$(grep -n '^## 15\. Anchor accounting' "$T" | cut -d: -f1)
B=$(grep -n '^## 16\.' "$T" | cut -d: -f1)
sed "${A},$((B-1))d" "$T" > /tmp/107_t.md

# A locator is: "section N", "§", a finding id (capital letter + digit(s)),
# a probe path, or "sections".
LOC='(section|sections|§|[A-Z][0-9]{1,2}\b|_probes/|\bQ[0-9]+)'

echo "member ref = a backticked bare number naming a panel file"
echo
printf "%-6s %-8s %-12s %-10s\n" file refs locatable pct
tot_all=0; loc_all=0
for n in 93 94 97 98 100 101 102 103 25 40 22 35 36 37 38 39 83 85 87 88 92 96 99 104 105; do
  tot=$(grep -oE "\`$n\`" /tmp/107_t.md | wc -l | tr -d ' ')
  [ "$tot" -eq 0 ] && continue
  # look at the 25 chars following each occurrence
  loc=$(grep -oE "\`$n\`('s)?[^\`]{0,25}" /tmp/107_t.md | grep -cE "^\`$n\`('s)?[ ,]*$LOC")
  tot_all=$((tot_all+tot)); loc_all=$((loc_all+loc))
  printf "%-6s %-8s %-12s %-10s\n" "$n" "$tot" "$loc" "$((loc*100/tot))%"
done
echo
echo "TOTAL refs=$tot_all locatable=$loc_all  -> $((loc_all*100/tot_all))%"
echo
echo "=== for contrast, the same measure over the eight members themselves ==="
for m in 93_orchard*.md 94_wingo*.md 97_dolan*.md 98_spj*.md 100_xu*.md 101_wronski*.md 102_torvalds*.md 103_mcsherry*.md; do
  n=$(grep -ohE '[A-Za-z0-9_./-]+\.(md|rs|py|out|sh|toml|txt|s|json|inc):[0-9]+(-[0-9]+)?|(^|[^A-Za-z0-9_./-])[0-9]{1,3}:[0-9]+(-[0-9]+)?' "$m" | wc -l | tr -d ' ')
  printf "  %-50s line-anchors=%s\n" "$m" "$n"
done
echo
echo -n "106 whole file, line-anchors: "
grep -ohE '[A-Za-z0-9_./-]+\.(md|rs|py|out|sh|toml|txt|s|json|inc):[0-9]+(-[0-9]+)?|(^|[^A-Za-z0-9_./-])[0-9]{1,3}:[0-9]+(-[0-9]+)?' "$T" | wc -l
