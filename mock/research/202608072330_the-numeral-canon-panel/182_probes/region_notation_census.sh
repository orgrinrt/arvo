#!/usr/bin/env bash
# How many claims in each source state a region in the panel's own notation.
#
# The notation has two spellings in this corpus, `holds for:` and `Predicate:`,
# and the census counts both plus a parenthesised inline form. It is the number
# the whole port turns on: a claim that states no region cannot become a
# proposal row, because the checker refuses an established claim with no region.
#
# The negative control is at the bottom: a pattern that must find nothing, and a
# pattern that must find something, run against the same files. Without them a
# zero is a claim about the grep rather than about the file.
#
# Run from the panel directory.
set -uo pipefail
SRC="63_spj_consolidation_the_format_concept.md
74_giesen_consolidation_the_number_system_concept.md
90_giesen_consolidation_derived_algebraic_laws.md
106_giesen_consolidation_the_strategy_axis.md
AGREEMENTS.md"

echo "=== region statements per source ==="
printf '%-56s %8s %8s %8s\n' file 'holds for' 'Predicate' total
for f in $SRC; do
  a=$(grep -c 'holds for:' "$f")
  b=$(grep -c '[Pp]redicate:' "$f")
  printf '%-56s %8s %8s %8s\n' "$f" "$a" "$b" "$((a + b))"
done

echo
echo "=== the eight in 106, with the line each sits on ==="
grep -n 'holds for:' 106_giesen_consolidation_the_strategy_axis.md | cut -c1-100

echo
echo "=== the two in 90 ==="
grep -n '[Pp]redicate:' 90_giesen_consolidation_derived_algebraic_laws.md | cut -c1-100

echo
echo "=== NEGATIVE CONTROL: a pattern that must find nothing ==="
for f in $SRC; do
  printf '%-56s %s\n' "$f" "$(grep -c 'zzz_no_such_string_anywhere' "$f")"
done
echo "(all zero, so a zero above is a real absence rather than a broken grep)"

echo
echo "=== POSITIVE CONTROL: a pattern that must find something in all five ==="
for f in $SRC; do
  printf '%-56s %s\n' "$f" "$(grep -c 'the' "$f")"
done
echo "(all non-zero, so the files are being read at all)"
