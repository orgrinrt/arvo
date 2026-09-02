#!/usr/bin/env bash
# A predicate is a product of per-axis regions. How many spans state a region
# that is not one?
#
# The notation gives one entry per axis and a lint (`a-predicate-names-an-axis-
# once`) makes that a hard error. So the region a predicate names is the
# CARTESIAN PRODUCT of its per-axis spans: an axis-aligned box. A source span
# saying "P against Q, and R against S" names a UNION of two boxes, and there is
# no way to write it. Splitting it per axis yields the product, which contains
# cells the source does not claim, so the row silently WIDENS.
#
# That is the opposite failure from the absence rule, which narrows. Both are
# silent and this one has no lint.
#
# Method. Search every `holds for:` paragraph in the whole panel, not the eight
# files the ranking was built from, for the constructions that correlate two
# axes inside one value. Text search rather than key splitting, so it is immune
# to the three predicate dialects `230` found.
#
# Controls, outcomes written before the run:
#   R1  the paragraph grab must find `holds for:` paragraphs in many files by
#       many personas, or it is reading one author again.
#   R2  a construction nobody writes must return zero (`phase_of_the_moon`).
#   R3  the known instance must be found by name: `132`'s commutation span,
#       "every deterministic member against saturation, and every translation-
#       equivariant member against wrapping".
#   R4  a plain rectangular span must NOT be reported, or the matcher flags
#       everything: `119 #9` is all `<axis> = <value>` and must not appear.
set -uo pipefail
cd "$(dirname "$0")"
PANEL=..

# Every paragraph containing `holds for:`, flattened one per line, with its file.
paras() {
  for f in "$PANEL"/[0-9]*.md; do
    awk -v F="$(basename "$f")" 'BEGIN{RS="";ORS="\n"} /holds? for:/ {gsub(/\n/," "); print F "\t" $0}' "$f"
  done
}

paras > paras_all.tsv
nfiles=$(cut -f1 paras_all.tsv | sort -u | wc -l | tr -d ' ')
npara=$(wc -l < paras_all.tsv | tr -d ' ')
npers=$(cut -f1 paras_all.tsv | sed -E 's/^[0-9]+_([a-z_]+)_.*/\1/' | sort -u | wc -l | tr -d ' ')
echo "### R1, the corpus this reads"
printf '  files with a `holds for:` paragraph: %s\n  paragraphs: %s\n  distinct persona slugs in those filenames: %s\n' "$nfiles" "$npara" "$npers"
[ "$nfiles" -gt 8 ] && echo "  PASS, wider than the eight the ranking used" || echo "  FAIL"

echo
echo "### R2, a construction nobody writes"
c=$(grep -c 'phase_of_the_moon' paras_all.tsv || true)
[ "$c" = "0" ] && echo "  PASS, 0" || echo "  FAIL, $c"

echo
echo "### spans naming a correlated region across two axes"
# `A against B, and C against D`  |  `X, or Y with Z`  |  `A for B and C for D`
grep -nE 'against [a-z-]+, and .* against |, or [a-z]+ with [a-z]+ |[a-z] for [a-z-]+ and [a-z-]+ for ' paras_all.tsv \
  | sed -E 's/\t/  ::  /' | cut -c1-230 > nonrect_hits.txt
wc -l < nonrect_hits.txt | tr -d ' ' | sed 's/^/  hits: /'
cat nonrect_hits.txt

echo
echo "### R3, the known instance"
grep -q 'every translation-equivariant member against wrapping' nonrect_hits.txt \
  && echo "  PASS, found" || echo "  FAIL, the matcher misses the case this probe was built for"

echo
echo "### R4, a plain rectangular span must not be reported"
if grep -q 'W = 3, F = 0, signedness in {unsigned, signed}, overflow behaviour in {wrapping, saturating}, rounding = truncation, radix = 2, operations in {add, sub, mul}, term shapes = every term at 2 and 3 leaf slots, arity' nonrect_hits.txt; then
  echo "  FAIL, 119 #9 is a product region and was flagged"
else
  echo "  PASS, the all-rectangular span is not reported"
fi
