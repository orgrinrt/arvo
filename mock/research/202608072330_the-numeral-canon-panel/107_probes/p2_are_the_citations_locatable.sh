#!/usr/bin/env bash
# p2: 106 says it converted line anchors to heading anchors. This measures
# whether the citations are actually locatable, i.e. whether a bare `NN`
# reference carries a section with it, or whether it points at a whole file.
#
# A `93` alone points at 1182 lines. A `93` section 4.2 is a heading anchor and
# is what how-to-run-a-panel.md prefers. The two are not the same claim.
set -u
cd "$(dirname "$0")/.." || exit 1
T=106_giesen_consolidation_the_strategy_axis.md

# Strip section 15 (the accounting) so the instrument is not disabled by it.
A=$(grep -n '^## 15\. Anchor accounting' "$T" | cut -d: -f1)
B=$(grep -n '^## 16\.' "$T" | cut -d: -f1)
sed "${A},$((B-1))d" "$T" > /tmp/107_t.md

echo "=== every backticked member reference in 106, with the 60 chars after it ==="
grep -oE '`(9[3-8]|10[0-5]|2[0-9]|3[0-9]|4[0-9]|8[0-9]|7[0-9])`[^\n]{0,55}' /tmp/107_t.md > /tmp/107_refs.txt
wc -l < /tmp/107_refs.txt

echo
echo "=== WITH a section/heading qualifier following (locatable) ==="
grep -cE '`[0-9]+`'"'"'?s? (section|sections|§|F[0-9]|its section)' /tmp/107_refs.txt

echo
echo "=== breakdown: which qualifier word follows a member reference ==="
sed -E 's/^`[0-9]+`(.?s)? ?//' /tmp/107_refs.txt | awk '{print $1}' | sort | uniq -c | sort -rn | head -25

echo
echo "=== per-member: total refs vs refs carrying a section qualifier ==="
for n in 93 94 97 98 100 101 102 103 25 40 22 35 87 88 96 99; do
  tot=$(grep -oE "\`$n\`" /tmp/107_t.md | wc -l | tr -d ' ')
  qual=$(grep -oE "\`$n\`('s)? ?(section|sections|§)" /tmp/107_t.md | wc -l | tr -d ' ')
  [ "$tot" -gt 0 ] && printf "  %-5s refs=%-4s with-section=%-4s\n" "$n" "$tot" "$qual"
done
