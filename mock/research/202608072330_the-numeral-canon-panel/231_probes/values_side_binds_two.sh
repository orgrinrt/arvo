#!/usr/bin/env bash
# The corpus routes around the one-entry-per-axis rule by writing a second
# region into the values side, and every shipped guard is blind to it.
#
# `a-predicate-names-an-axis-once` is a HARD_ERROR and its own doc says why: two
# entries for one axis is "two regions with nothing saying which governs". It
# reads slugs. `every-predicate-names-a-declared-axis` reads slugs and says so
# in terms: "The values side is not checked, and deliberately."
#
# So an entry naming ONE axis and carrying TWO regions inside its value passes
# both. This counts them in the committed canon.
#
# `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`'s own note
# already reports this class at six and says "the shipped arm checks the slug
# side only and sees none of them". This names which, and what shape each takes.
#
# Controls, outcomes written before the run:
#   V1  the total entry count must be 527, so this is reading the same
#       population every other instrument here reads.
#   V2  a construction nobody writes must return zero.
#   V3  a plainly single-region entry must NOT be reported, or the matcher flags
#       everything: `threads: threads = 1` must not appear.
#   V4  the known instance must be found by name: the value
#       `signed, or unsigned with signed intermediates`, which 230 argues is not
#       portable and which somebody ported anyway.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry
FILES="$REG/proposal.toml $REG/proposal-the-later-topics.toml $REG/law.toml $REG/law-the-later-topics.toml"

grep -hoE '"[a-z_]+: [^"]*"' $FILES > all_entries.txt
n=$(wc -l < all_entries.txt | tr -d ' ')
echo "### V1, the population"
echo "  entries: $n"
if [ "$n" = "527" ]; then echo "  PASS"; else echo "  FAIL, not the 527 every other reader here sees"; fi

echo
echo "### V2, a construction nobody writes"
c=$(grep -c 'quaternion_of_tuesdays' all_entries.txt || true)
if [ "$c" = "0" ]; then echo "  PASS, 0"; else echo "  FAIL"; fi

echo
echo "### entries whose values side carries a second region"
# `, or `        a disjunction
# ` against `    a paired quantification
# ` for the `    a region attached to a named sub-case
# `, and ` + a second `=` or `in `   two regions joined
# THE FIRST VERSION required a second `=`, `in ` or `{` anywhere in the value,
# to hold down false positives, and that dropped the one case this probe was
# built for: `signed, or unsigned with signed intermediates` carries none of
# the three. V4 caught it; the failing run is values_side_binds_two_first_attempt.out.
grep -nE ", or | against | for the [a-z]" all_entries.txt > values_two_regions.txt || true
grep -nE ", and " all_entries.txt | grep -E "(=|in |[{])" >> values_two_regions.txt || true
sort -u -o values_two_regions.txt values_two_regions.txt
wc -l < values_two_regions.txt | tr -d ' ' | sed 's/^/  hits: /'
cut -c1-175 values_two_regions.txt

echo
echo "### by the axis whose slug they sit on"
sed -E 's/^[0-9]+:"([a-z_]+): .*/\1/' values_two_regions.txt | sort | uniq -c | sort -rn | sed 's/^/  /'

echo
echo "### V3, a single-region entry must not be reported"
if grep -q '"threads: threads = 1"' values_two_regions.txt; then
  echo "  FAIL, the matcher flags a plain entry"
else
  echo "  PASS"
fi

echo
echo "### V4, the known instance"
if grep -q 'or unsigned with signed intermediates' values_two_regions.txt; then
  echo "  PASS, found, and it is in the committed canon rather than only in the panel"
  grep -n 'or unsigned with signed intermediates' $FILES | cut -c1-120
else
  echo "  FAIL"
fi

echo
echo "### what each shape is, stated rather than counted"
echo "  , or         a UNION over two axes. The product a per-axis reading gives is wider."
echo "  against      a PAIRING of two axes. Same."
echo "  for the      a region attached to a sub-case that is not an axis at all,"
echo "               usually which run it came from, which is coverage in the region slot."
echo "  , and        two regions on ONE axis inside one entry, which is exactly what"
echo "               a-predicate-names-an-axis-once forbids across two entries."

echo
echo "### where the known instance landed"
echo "  law::quantise_then_reduce_commutes, in its \`fails\` field."
echo "  230 argues from 132/136/138 that the span is NOT portable, because the"
echo "  value is a disjunction over the declared signedness and the intermediate's"
echo "  and no axis exists for the second. It is right. It was ported anyway, and"
echo "  the row is in the canon now, and nothing can see it: the two shipped"
echo "  predicate lints read the slug side, and the slug is a declared axis."
