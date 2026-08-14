#!/usr/bin/env bash
# p1: the anchor set difference, run independently of 106's own census.
#
# Two things this checks that a count alone cannot:
#   - which unique anchors the consolidation dropped (comm -23)
#   - that 106's own section 15 accounting is excluded, since an author that
#     prints the anchors it dropped makes them present and disables the diff
#
# Run from the panel directory.
set -u
cd "$(dirname "$0")/.." || exit 1

MEMBERS="93_orchard_the_strategy_axis_derived_cold.md \
94_wingo_the_strategy_axis_derived_cold.md \
97_dolan_the_strategy_space_attacked.md \
98_spj_what_the_strategy_axis_settles.md \
100_xu_generating_the_table_attacked.md \
101_wronski_the_cost_coordinates.md \
102_torvalds_does_the_mechanism_serve_the_intents.md \
103_mcsherry_what_the_corpus_can_and_cannot_show.md"

TARGET=106_giesen_consolidation_the_strategy_axis.md

# Section 15 is 106's own anchor accounting. Excluding it is mandatory:
# an author that lists what it dropped makes those anchors present in the
# new text and the diff then returns empty for the wrong reason.
ACC_START=$(grep -n '^## 15\. Anchor accounting' "$TARGET" | cut -d: -f1)
ACC_END=$(grep -n '^## 16\.' "$TARGET" | cut -d: -f1)
echo "106 accounting section: lines ${ACC_START}..$((ACC_END-1)) EXCLUDED from the target set"

sed "${ACC_START},$((ACC_END-1))d" "$TARGET" > /tmp/107_target_no_accounting.md
sed -n "${ACC_START},$((ACC_END-1))p" "$TARGET" > /tmp/107_accounting_only.md

# Broad anchor pattern: anything of the form <token>:<line> or <token>:<line>-<line>.
# Deliberately broader than the brief's suggestion, which misses the dominant
# bare-number form (98:398-402) and the INTENTS.md:56 form.
P='[A-Za-z0-9_./-]+\.(md|rs|py|out|sh|toml|txt|s|json|inc):[0-9]+(-[0-9]+)?|(^|[^A-Za-z0-9_./-])[0-9]{1,3}:[0-9]+(-[0-9]+)?'

extract() { grep -ohE "$P" "$@" | sed 's/^[^A-Za-z0-9_.]*//' | sort -u; }

extract $MEMBERS > /tmp/107_members_anchors.txt
extract /tmp/107_target_no_accounting.md > /tmp/107_target_anchors.txt
extract /tmp/107_accounting_only.md > /tmp/107_accounting_anchors.txt

echo
echo "=== RAW TOTALS (non-unique occurrences) ==="
echo -n "members, occurrences: "; grep -ohE "$P" $MEMBERS | wc -l
echo -n "106 body (no sec15), occurrences: "; grep -ohE "$P" /tmp/107_target_no_accounting.md | wc -l

echo
echo "=== UNIQUE ANCHOR SETS ==="
echo -n "members, unique: "; wc -l < /tmp/107_members_anchors.txt
echo -n "106 body (no sec15), unique: "; wc -l < /tmp/107_target_anchors.txt
echo -n "106 sec15 accounting only, unique: "; wc -l < /tmp/107_accounting_anchors.txt

echo
echo "=== CARRIED: member anchors present in 106 body ==="
comm -12 /tmp/107_members_anchors.txt /tmp/107_target_anchors.txt | tee /tmp/107_carried.txt | wc -l

echo
echo "=== DROPPED: member anchors absent from 106 body ==="
comm -23 /tmp/107_members_anchors.txt /tmp/107_target_anchors.txt | tee /tmp/107_dropped.txt | wc -l

echo
echo "=== DROPPED FROM BODY BUT PRESENT IN SEC 15 ACCOUNTING ==="
echo "(these would have silently passed a diff run without the exclusion)"
comm -12 /tmp/107_dropped.txt /tmp/107_accounting_anchors.txt | wc -l

echo
echo "=== FULL DROPPED LIST ==="
cat /tmp/107_dropped.txt

echo
echo "=== NEW IN 106, not in any member ==="
comm -13 /tmp/107_members_anchors.txt /tmp/107_target_anchors.txt
