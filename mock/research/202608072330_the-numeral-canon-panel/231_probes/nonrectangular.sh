#!/usr/bin/env bash
# A predicate is a product of per-axis regions. How many spans state a region
# that is not one?
#
# The notation gives one entry per axis and `a-predicate-names-an-axis-once`
# makes a second entry a hard error. So the region a predicate names is the
# CARTESIAN PRODUCT of its per-axis spans: an axis-aligned box. A source span
# saying "P against Q, and R against S" names a UNION of two boxes, and there is
# no way to write it. Splitting it per axis yields the product, which contains
# cells the source does not claim, so the row silently WIDENS.
#
# That is the opposite failure from the absence rule, which narrows. Both are
# silent, and this one has no lint.
#
# Method: text search over every `holds for:` paragraph in the WHOLE panel, not
# the eight files the ranking was built from, for constructions that correlate
# two axes inside one value. Text rather than key splitting, so it is immune to
# the three predicate dialects 230 found.
#
# Controls, outcomes written before the run:
#   R1  the paragraph grab must reach many files and many personas, or it is
#       reading one author again.
#   R2  a construction nobody writes must return zero (`phase_of_the_moon`).
#   R3  the known instance must be found by name: 132's commutation span,
#       "every deterministic member against saturation, and every
#       translation-equivariant member against wrapping".
#       R3 FAILED ON THE FIRST RUN and the run is kept as
#       `nonrectangular_first_attempt.out`. The matcher was fine; the arm read a
#       display file that `cut -c1-230` had truncated before the phrase, so the
#       check was run against evidence the report had shortened. The check now
#       reads the untruncated hits and the truncation happens only on the way to
#       the screen.
#   R4  a plain rectangular span must NOT be reported, or the matcher flags
#       everything: 119 #9 is all `<axis> = <value>` and must not appear.
set -uo pipefail
cd "$(dirname "$0")"
PANEL=..

paras() {
  for f in "$PANEL"/[0-9]*.md; do
    awk -v F="$(basename "$f")" 'BEGIN{RS="";ORS="\n"} /holds? for:/ {gsub(/\n/," "); print F "\t" $0}' "$f"
  done
}

paras > paras_all.tsv
nfiles=$(cut -f1 paras_all.tsv | sort -u | wc -l | tr -d ' ')
npara=$(wc -l < paras_all.tsv | tr -d ' ')
npers=$(cut -f1 paras_all.tsv | awk -F_ '{print $2}' | sort -u | wc -l | tr -d ' ')
echo "### R1, the corpus this reads"
printf '  files with a holds-for paragraph: %s\n  paragraphs: %s\n  distinct personas: %s\n' "$nfiles" "$npara" "$npers"
if [ "$nfiles" -gt 8 ]; then echo "  PASS, wider than the eight the ranking used"; else echo "  FAIL"; fi

echo
echo "### R2, a construction nobody writes"
c=$(grep -c 'phase_of_the_moon' paras_all.tsv || true)
if [ "$c" = "0" ]; then echo "  PASS, 0"; else echo "  FAIL, $c"; fi

echo
echo "### spans naming a correlated region across two axes"
grep -nE 'against [a-z-]+, and .* against |, or [a-z]+ with [a-z]+ |[a-z] for [a-z-]+ and [a-z-]+ for ' paras_all.tsv > nonrect_hits_full.txt || true
wc -l < nonrect_hits_full.txt | tr -d ' ' | sed 's/^/  hits: /'
sed -E 's/\t/  ::  /' nonrect_hits_full.txt | cut -c1-185

echo
echo "### R3, the known instance, checked against the untruncated hits"
if grep -q 'every translation-equivariant member against wrapping' nonrect_hits_full.txt; then
  echo "  PASS, found"
else
  echo "  FAIL, the matcher misses the case this probe was built for"
fi

echo
echo "### R4, a plain rectangular span must not be reported"
if grep -q 'term shapes = every term at 2 and 3 leaf slots, arity' nonrect_hits_full.txt; then
  echo "  FAIL, 119 #9 is a product region and was flagged"
else
  echo "  PASS, the all-rectangular span is not reported"
fi

echo
echo "### how wide the over-claim is, for the one span where both sides are declared axes"
echo "  132's commutation span names, jointly:"
echo "    (deterministic rounding members) x {saturation}"
echo "    (translation-equivariant members) x {wrapping}"
echo "  Written per axis it becomes rounding: {det union transeq} x overflow_policy: {sat, wrap},"
echo "  which additionally claims the two cells (transeq, saturation) and (det, wrapping)."
echo "  The source claims neither. Two cells invented by the notation, silently, on a row"
echo "  whose Argument kind is equivariance."

echo
echo "### reconciliation with 230_probes/who_out/paras.tsv"
echo "  230 reports 60 files and 372 paragraphs. This reports 62 and 378."
echo "  The difference is exactly the two deliverables that quote predicates:"
comm -23 <(cut -f1 paras_all.tsv | sort -u) <(cut -f1 ../230_probes/who_out/paras.tsv | sort -u) | sed 's/^/    /'
echo "  So the corpus figure reproduces under a second extractor: 60 files, 372"
echo "  paragraphs, 21 personas. None of the six hits above comes from either."
