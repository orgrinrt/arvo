#!/usr/bin/env bash
# Per declared axis: how many predicate entries in the registry actually use it.
#
# Why. `dimension.toml`'s header says the set moves so that a region the corpus
# states can be written into the canon at all, and names a ranking of which
# axes block the most spans. Declaring is half of that act; the other half is
# porting the blocked spans onto the new axis. This measures whether the second
# half happened, and it is also the instrument for a harder question: under the
# absence rule a claim that does not name an axis holds nowhere that axis
# exists, so an axis with zero uptake is not merely unused.
#
# Method. A predicate entry is an array element `"<slug>: <values>"` in a
# predicate-carrying field, which is the shape
# `mock/lints/every_predicate_names_a_declared_axis.rs` splits on. The slug is
# the first thing on the line and a colon follows it, so the prefix grep is
# exact. U3 below checks that reading against the total.
#
# Controls, outcomes written before the run:
#   U1  an axis in constant use must come back high. `total_width` and
#       `threads`; a zero on either means the grep is wrong rather than that
#       the corpus stopped stating widths.
#   U2  an axis nobody declared must be absent from the table and unused.
#   U3  the per-axis sum must not exceed the registry's entry count, since each
#       entry names one axis. Equality additionally means every entry's slug is
#       declared, which is what the HARD_ERROR lint enforces.
#   U4  the table must carry one line per declared axis. Added after the first
#       run of this script printed eleven of twenty-two and stopped, because
#       under `pipefail` a grep matching nothing fails its pipeline and `set -e`
#       ended the loop at the first zero. U1 through U3 all passed on that run.
#       None of them counted rows, which is why none of them could see it.
set -uo pipefail
cd "$(dirname "$0")"
REG=../../../registry
count() { { grep -rhoE "^ *\"$1: " "$REG"/*.toml || true; } | wc -l | tr -d ' '; }

entries=$({ grep -rhoE '^ *"[a-z_]+: ' "$REG"/*.toml || true; } | wc -l | tr -d ' ')
declared=$(grep -c '^id = ' "$REG/dimension.toml")
echo "### predicate entries in the registry: $entries"
echo "### declared axes: $declared"
echo

total=0; rows=0
printf '%-20s %s\n' axis entries
while read -r a; do
  n=$(count "$a"); total=$((total + n)); rows=$((rows + 1))
  printf '%-20s %s\n' "$a" "$n"
done < <(grep '^id = ' "$REG/dimension.toml" | sed 's/id = "//; s/"//')
echo "---"
printf '%-20s %s\n' "(sum)" "$total"

echo
echo "### U1, two axes in constant use must be high"
for a in total_width threads; do
  n=$(count "$a")
  [ "$n" -ge 20 ] && echo "  $a: PASS ($n)" || echo "  $a: FAIL ($n), the grep is wrong"
done
echo "### U2, an axis nobody declared is absent and unused"
if grep -q '^id = "phase_of_the_moon"' "$REG/dimension.toml"; then echo "  FAIL, it is declared"
elif [ "$(count phase_of_the_moon)" = 0 ]; then echo "  PASS"
else echo "  FAIL, used"; fi
echo "### U3, the per-axis sum against the entry count"
if [ "$total" -eq "$entries" ]; then echo "  PASS, $total = $entries, so every entry names a declared axis"
elif [ "$total" -lt "$entries" ]; then echo "  PASS with a residue, $total < $entries: $((entries-total)) entries name an undeclared slug"
else echo "  FAIL, $total > $entries, an entry is counted twice"; fi
echo "### U4, one table row per declared axis"
[ "$rows" -eq "$declared" ] && echo "  PASS, $rows of $declared" || echo "  FAIL, $rows of $declared"
