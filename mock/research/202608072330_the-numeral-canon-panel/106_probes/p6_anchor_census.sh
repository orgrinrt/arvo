#!/bin/sh
# The anchor census 106 section 15 reports, and the set difference the check
# after it should run. Run from the panel directory.
#
# Tier rule: for a canon candidate, panel-internal and probe anchors count and
# must survive; anchors into a nuked or superseded tier do not and must NOT be
# restored. The four *.md.tmpl anchors are the superseded root design templates,
# both of which now carry a superseded banner and one of which names a forbidden
# feature. Their absence from 106 is correct.
P='[A-Za-z0-9_./-]+\.(rs|py|sh|toml|md|json|s|out|csv|tmpl)'
M="93_orchard*.md 94_wingo*.md 97_dolan*.md 98_spj*.md 100_xu*.md 101_wronski*.md 102_torvalds*.md 103_mcsherry*.md"

cat $M | grep -oE "$P" | sort -u > /tmp/106_union.txt
# exclude 106's own accounting section, or naming a dropped anchor makes it present
sed '/^## 15. Anchor accounting/,/^## 16./d' 106_giesen_consolidation_the_strategy_axis.md \
  | grep -oE "$P" | sort -u > /tmp/106_mine.txt

echo "union across the eight members : $(wc -l < /tmp/106_union.txt | tr -d ' ')"
echo "  of which probe files          : $(grep -c '_probes/' /tmp/106_union.txt)"
echo "  of which live bench tree      : $(grep -cE 'benches|variants/' /tmp/106_union.txt)"
echo "  of which workspace rules      : $(grep -cE '^[a-z-]+\.md$' /tmp/106_union.txt)"
echo "  of which SUPERSEDED tier      : $(grep -c '\.tmpl' /tmp/106_union.txt)"
echo
echo "carried by 106 (excl. its accounting section) : $(wc -l < /tmp/106_mine.txt | tr -d ' ')"
echo "  superseded-tier anchors carried             : $(grep -c '\.tmpl' /tmp/106_mine.txt)   <- must be 0"
echo
echo "--- superseded-tier anchors, dropped on purpose ---"
grep '\.tmpl' /tmp/106_union.txt
