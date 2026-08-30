#!/usr/bin/env bash
# Checks the one claim that 94 of the 176 retirement rows rest on: that DROPLIST.md
# section 6 is the verbatim extract it says it is, taken from the predecessor panel's
# twelfth consolidation at the moment that panel closed.
#
# Run from mock/research/. Passes silently, prints both hashes, and fails loudly.
#
# The control is the second hash: if the two ranges ever produce the same hash by
# extracting nothing, both are empty and the line counts below catch it.
set -euo pipefail

A=$(mktemp); B=$(mktemp)
trap 'rm -f "$A" "$B"' EXIT

awk '/^## 6\. The droplist, cumulative/,/^## 7\. Reversals/' \
  202608072330_the-numeral-canon-panel/DROPLIST.md > "$A"
awk '/^## 6\. The droplist, cumulative/,/^## 7\. Reversals/' \
  202607301300_formalization-spec-panel/OLD_124_consolidation_twelve.md > "$B"

la=$(wc -l < "$A"); lb=$(wc -l < "$B")
echo "lines: extract=$la source=$lb"
[ "$la" -gt 100 ] || { echo "FAIL: extract is empty or truncated"; exit 1; }
[ "$lb" -gt 100 ] || { echo "FAIL: source range is empty or truncated"; exit 1; }

ha=$(md5 -q "$A" 2>/dev/null || md5sum "$A" | cut -d' ' -f1)
hb=$(md5 -q "$B" 2>/dev/null || md5sum "$B" | cut -d' ' -f1)
echo "extract: $ha"
echo "source:  $hb"
[ "$ha" = "$hb" ] || { echo "FAIL: DROPLIST section 6 is not verbatim"; diff "$B" "$A"; exit 1; }

ea=$(grep -c '^\*\*' "$A")
echo "bold-opening paragraphs: $ea (94 entries plus 2 preamble paragraphs)"
echo "PASS"
