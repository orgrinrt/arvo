#!/usr/bin/env bash
# Per declared axis: how many predicate entries in the registry use it, and how
# many of the declared axes each predicate field names.
#
# Why. `dimension.toml`'s header says the set moves so a region the corpus
# states can be written into the canon at all, and names a ranking of which
# undeclared axes block the most spans. Declaring is half that act; porting the
# blocked spans onto the new axis is the other half. This measures the second
# half. It is also the instrument for the harder question, because the absence
# rule is not a tidiness convention: `dimension::threads` says in the registry's
# own words that "an absent `threads` means the claim holds in no situation
# where threads exist at all", and `dimension::access_pattern` refuses a
# per-axis fourth state in as many words. So a field naming k of the declared
# axes claims to hold nowhere any of the other 22 - k exists.
#
# Entries come from `entries.sh`, which carries the reason this script does not
# read them itself: the obvious reader cannot see an array written on one line,
# and the first version of this script and of the distribution below both got
# the count wrong in the same way, so their agreement was worthless.
#
# Controls, outcomes written before the run:
#   U1  an axis in constant use must come back high. `total_width` and
#       `threads`; a zero on either means the reader is broken rather than that
#       the corpus stopped stating widths.
#   U2  an axis nobody declared must be absent from the table and unused.
#   U3  the table must carry one line per declared axis. Added after a run
#       printed eleven of twenty-two and stopped: under `pipefail` a grep
#       matching nothing fails its pipeline and `set -e` ended the loop at the
#       first zero. Every other arm passed on that run, because none counted
#       rows.
#   U4  the per-axis sum must equal the entry total, since each entry names one
#       axis. Equality also means every slug is declared, which is what the
#       HARD_ERROR lint enforces independently.
#   U5  no field may name more axes than are declared, and at least one must
#       name more than one, or the field grouping is degenerate.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry
declared=$(grep -c '^id = ' "$REG/dimension.toml")
bash ./entries.sh > uptake_entries.tsv
entries=$(grep -c . uptake_entries.tsv)

echo "### predicate entries: $entries    declared axes: $declared"
echo "### predicate fields:  $(cut -f1,2 uptake_entries.tsv | sort -u | grep -c .)"
echo
total=0; rows=0
printf '%-20s %s\n' axis entries
while read -r a; do
  n=$(awk -F'\t' -v a="$a" '$3==a{n++} END{print n+0}' uptake_entries.tsv)
  total=$((total + n)); rows=$((rows + 1))
  printf '%-20s %s\n' "$a" "$n"
done < <(grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//')
echo "---"; printf '%-20s %s\n' "(sum)" "$total"

echo
echo "### axes named per predicate field, and what each field is therefore silent on"
cut -f1,2 uptake_entries.tsv | sort | uniq -c \
  | awk -v d="$declared" '{c[$1]++} END{for(k in c) printf "%4s axes named  %4s fields  (silent on %s)\n", k, c[k], d-k}' \
  | sort -n
cut -f1,2 uptake_entries.tsv | sort | uniq -c \
  | awk -v d="$declared" '{s+=$1; n++} END{printf "### mean %.1f of %s named, so %.1f silent on average\n", s/n, d, d-s/n}'
mx=$(cut -f1,2 uptake_entries.tsv | sort | uniq -c | awk 'BEGIN{m=0} $1>m{m=$1} END{print m}')
echo "### the richest field names $mx of $declared"

echo
echo "### U1, two axes in constant use must be high"
for a in total_width threads; do
  n=$(awk -F'\t' -v a="$a" '$3==a{n++} END{print n+0}' uptake_entries.tsv)
  [ "$n" -ge 20 ] && echo "  $a: PASS ($n)" || echo "  $a: FAIL ($n)"
done
echo "### U2, an axis nobody declared is absent and unused"
if grep -q '^id = "phase_of_the_moon"' "$REG/dimension.toml"; then echo "  FAIL, it is declared"
elif ! grep -q "	phase_of_the_moon$" uptake_entries.tsv; then echo "  PASS"
else echo "  FAIL, used"; fi
echo "### U3, one table row per declared axis"
[ "$rows" -eq "$declared" ] && echo "  PASS, $rows of $declared" || echo "  FAIL, $rows of $declared"
echo "### U4, the per-axis sum against the entry total"
[ "$total" -eq "$entries" ] && echo "  PASS, $total = $entries, so every entry names a declared axis" \
  || echo "  FAIL, $total against $entries"
echo "### U5, the field grouping is not degenerate"
if [ "$mx" -le "$declared" ] && [ "$mx" -gt 1 ]; then echo "  PASS, max $mx"; else echo "  FAIL, max $mx"; fi
echo "### U6, the total against a count somebody else arrived at independently."
echo "###     ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side, which"
echo "###     is ratified, says in its note: 139 of 527 entries drop the axis word"
echo "###     from the values side entirely. That 527 comes from a different reader"
echo "###     written by a different seat, so agreeing with it is a check rather"
echo "###     than a restatement. The first version of this script said 517."
if grep -q "of 527 entries" "$REG/ruling.toml"; then
  [ "$entries" = 527 ] && echo "  PASS, $entries matches the ratified note's 527" \
                       || echo "  FAIL, $entries against the note's 527"
else
  echo "  INCONCLUSIVE, the note no longer carries a count to check against"
fi
echo "### U7, at least one entry must come from a single-line array, or this"
echo "###     reader is the old one wearing a new name."
sl=$({ grep -rhoE '^(predicate|holds|fails) = \[".*\]$' "$REG"/*.toml || true; } | { grep -oE "\"[a-z_]+: " || true; } | wc -l | tr -d " ")
[ "${sl:-0}" -gt 0 ] && echo "  PASS, $sl entries live on an array open-bracket line" \
                     || echo "  FAIL, none, so the blind spot is untested"
