#!/usr/bin/env bash
# 182's headline is a census: "eleven statements of a region in the panel's own
# notation" across the four consolidations. Its instrument is
# `182_probes/region_notation_census.sh`, whose pattern is `[Pp]redicate:` plus
# `holds for:`, both requiring the colon immediately after the keyword.
#
# The corpus does not always write it that way. This runs a wider pattern and
# prints the difference, so a miss is shown rather than argued.
#
# CASE THAT MUST FAIL: control 1 is the census's own pattern, reproduced, which
# must return the census's own published counts. If it does not, this is not
# comparing against what 182 ran. Control 2 is a pattern matching nothing.
set -uo pipefail
cd "$(dirname "$0")/.."
SRC="63_spj_consolidation_the_format_concept.md
74_giesen_consolidation_the_number_system_concept.md
90_giesen_consolidation_derived_algebraic_laws.md
106_giesen_consolidation_the_strategy_axis.md
AGREEMENTS.md"

NARROW='holds for:|[Pp]redicate:'
# a region statement is a keyword followed by anything up to a colon on the same
# line, which is how the corpus writes an attributed one
WIDE='holds for[^.]{0,40}:|[Pp]redicate[^.]{0,40}:'

echo "=== CONTROL 1: the census's own pattern, which must reproduce its table ==="
printf '%-56s %s\n' file narrow
for f in $SRC; do printf '%-56s %6s\n' "$f" "$(grep -cE "$NARROW" "$f" || true)"; done
echo "  (182 section 1 published: 0, 0, 2, 9, 0)"

echo
echo "=== the wider pattern, and the difference ==="
printf '%-56s %6s %6s %6s\n' file narrow wide missed
tn=0; tw=0
for f in $SRC; do
  n=$(grep -cE "$NARROW" "$f" || true); w=$(grep -cE "$WIDE" "$f" || true)
  tn=$((tn+n)); tw=$((tw+w))
  printf '%-56s %6s %6s %6s\n' "$f" "$n" "$w" "$((w-n))"
done
printf '%-56s %6s %6s %6s\n' TOTAL "$tn" "$tw" "$((tw-tn))"

echo
echo "=== the lines the narrow pattern missed, printed in full ==="
for f in $SRC; do
  diff <(grep -nE "$NARROW" "$f" || true) <(grep -nE "$WIDE" "$f" || true) \
    | grep '^>' | sed "s|^> |  $f:|" | cut -c1-190
done

echo
echo "=== CONTROL 2: a pattern that must find nothing in any of the five ==="
for f in $SRC; do printf '  %-56s %s\n' "$f" "$(grep -cE 'zzq_no_such_token' "$f" || true)"; done
