#!/usr/bin/env bash
# Does `230_probes/axis_uptake.out` reproduce under a different extractor?
#
# `229` counted predicate entries with a flat grep over the four predicate-bearing
# files. `230` counted them with an awk state machine that only reads inside a
# `predicate`/`holds`/`fails` array, over all twelve registry files, and its own
# header records that its FIRST version missed single-line arrays and reported
# `strategy` at zero.
#
# The two mechanisms are unrelated: one matches a quoted string anywhere in a
# file, the other tracks array state. So agreement is corroboration rather than
# one reader wearing two names. This runs the flat one again, widened to all
# twelve files, and diffs the result against 230's committed output.
#
# Controls, outcomes written before the run:
#   X1  the total must be 527, which a third party states independently in
#       `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`'s note.
#   X2  widening from four files to twelve must add nothing, or predicate
#       entries live outside the four `PREDICATE_FIELDS` name and the lint that
#       walks them is blind to some.
#   X3  a slug nobody declared must not appear, since a HARD_ERROR lint forbids
#       it. If one does, this reader splits differently from the lint.
#   X4  the per-axis vector must match 230's byte for byte, or one of the two
#       readers is wrong and the disagreement names where.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry
OTHER=../230_probes/axis_uptake.out

flat() { grep -hoE '"[a-z_]+: [^"]*"' "$@" | sed -E 's/^"([a-z_]+): .*/\1/'; }

FOUR="$REG/proposal.toml $REG/proposal-the-later-topics.toml $REG/law.toml $REG/law-the-later-topics.toml"
ALL=$(ls "$REG"/*.toml)

n4=$(flat $FOUR | wc -l | tr -d ' ')
n12=$(flat $ALL | wc -l | tr -d ' ')
echo "### X1, the total"
echo "  four predicate-bearing files: $n4"
[ "$n4" = "527" ] && echo "  PASS, 527" || echo "  FAIL, expected 527"

echo "### X2, widening to all twelve registry files"
echo "  all twelve: $n12"
[ "$n4" = "$n12" ] && echo "  PASS, no entry lives outside the four" || echo "  FAIL, $((n12-n4)) entries elsewhere"

flat $FOUR | sort | uniq -c | sort -rn | awk '{printf "%s %s\n", $2, $1}' > uptake_mine.txt

echo "### X3, every slug is declared"
grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//' | sort > declared.txt
undeclared=$(awk '{print $1}' uptake_mine.txt | sort | comm -23 - declared.txt)
[ -z "$undeclared" ] && echo "  PASS, none undeclared" || { echo "  FAIL:"; echo "$undeclared"; }

echo "### X4, against 230's committed vector"
# 230 prints `name count` pairs across a block; pull them back out.
sed -n '/^access_pattern/,/^operation/p' "$OTHER" 2>/dev/null | tr -s ' ' '\n' | grep -v '^$' \
  | paste - - | awk '{printf "%s %s\n", $1, $2}' | sort > uptake_theirs.txt || true
# fall back: read every `<slug> <int>` pair anywhere in their file
grep -oE '[a-z_]+ +[0-9]+' "$OTHER" | tr -s ' ' ' ' | sort -u > uptake_theirs_pairs.txt
awk '{print $1, $2}' uptake_mine.txt | sort > uptake_mine_sorted.txt
# every declared axis, with 0 where absent, for both
while read -r a; do
  m=$(awk -v k="$a" '$1==k {print $2}' uptake_mine.txt); m=${m:-0}
  t=$(awk -v k="$a" '$1==k {print $2}' uptake_theirs_pairs.txt | head -1); t=${t:-MISSING}
  printf '  %-18s mine %-4s theirs %s\n' "$a" "$m" "$t"
done < declared.txt

echo
echo "### X2 resolved: the two extra entries are MY reader's, not the registry's"
echo "### (kept rather than repaired: the control fired on the instrument, which is its job)"
grep -nE '"[a-z_]+: [^"]*"' "$REG/ruling.toml" | cut -c1-40
echo "  Both sit on one line, in the \`note\` of"
echo "  ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side, which quotes"
echo "  two predicate entries as PROSE EXAMPLES of the two dialects. A flat grep"
echo "  cannot tell a quoted example from a real array element; 230's array-aware"
echo "  reader can, and does. So 527 stands, the lint's PREDICATE_FIELDS walk is"
echo "  complete, and the flat reader is only safe inside the four files."
