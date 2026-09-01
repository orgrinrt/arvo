#!/usr/bin/env bash
# Seat 245. Entailment-check instrument 2, extended.
#
# 244's own anchor_diff.sh pattern (P = numbered .md filenames, rs/toml:line,
# bare rs/toml filenames) does not match a registry row slug at all:
# `ruling::foo`, `proposal::bar`, `question::baz`. Those slugs are exactly what
# a reader follows via `cargo mock query`, and my brief names "row slugs"
# explicitly as an anchor kind. This is the same anchor-diff instrument run
# with a pattern that adds them, to find out whether the gap in 244's own
# instrument concealed any real loss.
#
# THE CASE THAT MUST FAIL: if either side extracts zero slugs, the diff below
# is about the pattern rather than about the documents. Refuse to print if so.
set -u
cd "$(dirname "$0")/.." || exit 1   # the panel directory

SLUG='(ruling|proposal|question|probe|retirement|obligation|law|dimension)::[a-z0-9_]+'

SRC1=241_kiselyov_admission_is_a_resolution_not_a_verdict.md
SRC2=242_what-admits-a-number-system.md
SRC3=243_seat242_the_resolution_has_no_second_arm.md
MINE=244_orchard_consolidation_admission_and_the_number_system.md

anchors() { grep -oE "$SLUG" | sort -u; }

S=$( cat "$SRC1" "$SRC2" "$SRC3" | anchors )
M=$( cat "$MINE" | anchors )   # whole file: this is a check on 244, not a re-run of 244's own accounting

ns=$(printf '%s\n' "$S" | grep -c .)
nm=$(printf '%s\n' "$M" | grep -c .)

printf 'slug anchors in the three sources      : %d\n' "$ns"
printf 'slug anchors in the consolidation       : %d (whole file)\n' "$nm"
if [ "$ns" -eq 0 ] || [ "$nm" -eq 0 ]; then
  echo "CONTROL FAILED: one side extracted zero slugs. Stopping."
  exit 2
fi
echo "control passes: both sides nonempty."

echo
echo "LOST (slugs cited by a source, not cited anywhere in 244):"
comm -23 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | sed 's/^/  /'
printf '  count = %d\n' "$(comm -23 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | grep -c .)"

echo
echo "new (cited by 244, not by any source):"
comm -13 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | sed 's/^/  /'
printf '  count = %d\n' "$(comm -13 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | grep -c .)"

echo
echo "carried:"
comm -12 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | sed 's/^/  /'
printf '  count = %d\n' "$(comm -12 <(printf '%s\n' "$S") <(printf '%s\n' "$M") | grep -c .)"
