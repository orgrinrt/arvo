#!/usr/bin/env bash
# If exactly one undeclared phrase were declared as a `dimension` row, how many
# more predicate spans could be written into the registry?
#
# `span_verdicts.sh` says 4 of 64 spans are portable and names what blocks the
# other 60. That is the diagnosis. This is the lever: it ranks the undeclared
# phrases by how many spans each is the SOLE blocker of, and separately by how
# many spans it appears in at all. The first number is what one new dimension
# row buys immediately; the second is what it buys once its co-blockers are
# declared too.
#
# Controls:
#   U1  at least one phrase must be a sole blocker somewhere AND appear more
#       widely. If no phrase does, the two columns are one quantity printed
#       twice.
#   U2  the sole-blocker total must not exceed the blocked-span count, since a
#       span has at most one sole blocker.
#       THE FIRST VERSION OF U2 COULD NOT FAIL. Its total was accumulated inside
#       a pipeline, so the loop ran in a subshell and the variable came back
#       zero whatever the data said; it printed `PASS, 0 <= 60` and would have
#       printed that against any input at all. The total is now computed from a
#       file written by the loop rather than from a variable that does not
#       survive it, and `u2_negative_control.out` beside this records the run
#       where the arm is fed a deliberately impossible total and fails.
#   U3  a phrase nobody wrote must produce no row: checked with
#       `phase_of_the_moon`, the same non-axis the committed check's own control
#       uses.
set -euo pipefail
cd "$(dirname "$0")"

[ -f span_verdicts_detail.txt ] || { echo "run span_verdicts.sh first"; exit 1; }

# One line per blocked span, carrying its semicolon-separated blocker list.
grep -A1 '^BLOCKED' span_verdicts_detail.txt \
  | grep 'undeclared region phrases:' \
  | sed 's/.*undeclared region phrases: //' > blockers.txt

blocked_total=$(grep -c '^BLOCKED' span_verdicts_detail.txt)
echo "### blocked spans: $blocked_total"
echo

{
  awk -F'; ' 'NF==1 {print}' blockers.txt | sed 's/^ *//; s/ *$//' | sort | uniq -c \
    | while read -r n p; do printf 'SOLE\t%s\t%s\n' "$n" "$p"; done
  tr ';' '\n' < blockers.txt | sed 's/^ *//; s/ *$//' | grep -v '^$' | sort | uniq -c \
    | while read -r n p; do printf 'ANY\t%s\t%s\n' "$n" "$p"; done
} > tally.txt

# Build the table into a file. Accumulating a total inside a pipeline puts the
# loop in a subshell and the total comes back zero however wrong the data is.
: > table.txt
while IFS= read -r p; do
  s=$(awk -F'\t' -v p="$p" '$1=="SOLE" && $3==p {print $2}' tally.txt); s=${s:-0}
  a=$(awk -F'\t' -v p="$p" '$1=="ANY"  && $3==p {print $2}' tally.txt); a=${a:-0}
  printf '%s\t%s\t%s\n' "$s" "$a" "$p" >> table.txt
done <<< "$(awk -F'\t' '{print $3}' tally.txt | sort -u)"

sole_sum=$(awk -F'\t' '{n+=$1} END{print n+0}' table.txt)

echo "### per undeclared phrase: spans it is the SOLE blocker of, and spans it appears in"
printf '  %-6s %-6s %s\n' sole any phrase
sort -rn -k2 -t"$(printf '\t')" table.txt | while IFS=$'\t' read -r s a p; do
  printf '  %-6s %-6s %s\n' "$s" "$a" "$p"
done

echo
echo "### the same, grouped by the axis a phrase is a spelling of"
echo "### (the corpus writes one axis several ways; the registry would declare one row)"
awk -F'\t' '{
  p = $3
  fam = "(ungrouped) " p
  if (p ~ /domain|one-signed|residue system/)            fam = "the ambient domain"
  else if (p ~ /accumulator width/)                      fam = "the accumulator width"
  else if (p ~ /cost coordinates|arms|selector|weight grid|baseline|weights|reference point|augmentation|cost tables/) fam = "the cost-model population"
  else if (p ~ /assignment|observation set/)             fam = "assignment and observation sets"
  else if (p ~ /term shapes|declarations|structure constants|restrictions|construction|declared grid step|^term$|placements|chains of depth|carriers/) fam = "the term and declaration shape"
  else if (p ~ /F_exact|F_intermediate|F_final|staged narrowing/) fam = "the staged-narrowing widths"
  else if (p ~ /coupling|element count|fraction|keying|decorrelation|input shape|threshold|stochastic|positions 0 to/) fam = "the stochastic-coupling parameters"
  else if (p ~ /edition|feature gates|no_std|float types|crate type|recursion limit|toolchain/) fam = "the compilation environment"
  else if (p ~ /radix/)                                  fam = "radix"
  else if (p ~ /discharge check|equality read|overflow limit|conforming pair|arm set|pi|ops|operations \{|\{m|\{add/) fam = "the operation and check shape"
  a[fam] += $2; s[fam] += $1
} END { for (f in a) printf "%s\t%s\t%s\n", s[f], a[f], f }' table.txt \
  | sort -rn -k2 -t"$(printf '\t')" \
  | while IFS=$'\t' read -r s a f; do printf '  %-6s %-6s %s\n' "$s" "$a" "$f"; done

echo
echo "### U1, at least one phrase must be a sole blocker somewhere AND appear more widely"
both=$(awk -F'\t' '$1>0 && $2>$1' table.txt | wc -l | tr -d ' ')
if [ "$both" -gt 0 ]; then
  echo "  PASS, $both such phrases:"
  awk -F'\t' '$1>0 && $2>$1 {printf "    sole %s, appears %s: %s\n", $1, $2, $3}' table.txt
else
  echo "  FAIL, the two columns carry the same quantity"
fi

echo "### U2, sole-blocker total must not exceed the blocked-span count"
if [ "${U2_FORCE_TOTAL:-}" != "" ]; then
  sole_sum="$U2_FORCE_TOTAL"
  echo "  (negative control: total forced to $sole_sum)"
fi
if [ "$sole_sum" -le "$blocked_total" ]; then
  echo "  PASS, $sole_sum <= $blocked_total"
else
  echo "  FAIL, $sole_sum > $blocked_total"
fi

echo "### U3, a phrase nobody wrote must produce no row"
if grep -q 'phase_of_the_moon' tally.txt; then echo "  FAIL"; else echo "  PASS, phase_of_the_moon absent"; fi
