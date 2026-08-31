#!/usr/bin/env bash
# `mockspace.toml` declares the `panel` reference root frozen, and states the
# reason: "a numbered panel file is written once and never edited, and that is
# what makes a line citation into it honest rather than a hazard."
#
# This tests the premise, on the four files every registry citation lands in.
#
# CASE THAT MUST FAIL: the control asks the same question of two files expected
# to be single-commit. One of the two, `107`, turned out to carry two, so the
# control half-failed and is recorded rather than tuned: `91` reads 1, which is
# what shows the count is per file and not a constant, and `107` reads 2, which
# is a real second commit correcting its own probe count.
set -uo pipefail
cd "$(dirname "$0")/../../../.."   # repo root
P=mock/research/202608072330_the-numeral-canon-panel

echo "=== commits touching each consolidation, and the line delta of every one after the first ==="
for f in 63_spj_consolidation_the_format_concept.md \
         74_giesen_consolidation_the_number_system_concept.md \
         90_giesen_consolidation_derived_algebraic_laws.md \
         106_giesen_consolidation_the_strategy_axis.md; do
  n=$(git log --oneline -- "$P/$f" | wc -l | tr -d ' ')
  printf '\n%-56s %s commit(s)\n' "$f" "$n"
  [ "$n" -le 1 ] && { echo "   never edited after landing"; continue; }
  git log --reverse --format='%h %s' -- "$P/$f" | tail -n +2 | while read -r h rest; do
    ins=$(git show --numstat --format= "$h" -- "$P/$f" | awk '{print $1}')
    del=$(git show --numstat --format= "$h" -- "$P/$f" | awk '{print $2}')
    # the earliest line touched: everything at or after it has a shifted number
    first=$(git show "$h" -- "$P/$f" | grep -m1 -oE '^@@ -[0-9]+' | tr -d '@ -')
    printf '   %s  +%-5s -%-5s  first line touched: %-6s  net shift below it: %+d\n' \
      "$h" "${ins:-0}" "${del:-0}" "${first:-?}" "$(( ${ins:-0} - ${del:-0} ))"
    printf '      %s\n' "$rest"
  done
done

echo
echo "=== what the config says ==="
grep -n 'frozen = true' mockspace.toml | head -3
sed -n '311,313p' mockspace.toml

echo
echo "=== CONTROL: a panel file with exactly one commit must report never edited ==="
for f in 91_ringer_entailment_check_on_the_derived_laws_consolidation.md \
         107_arntzen_entailment_check_on_the_strategy_consolidation.md; do
  printf '%-62s %s commit(s)\n' "$f" "$(git log --oneline -- "$P/$f" | wc -l | tr -d ' ')"
done

echo
echo "=== the consequence, on one citation, demonstrated ==="
cd "$(git rev-parse --show-toplevel)"
P=mock/research/202608072330_the-numeral-canon-panel
echo "91:85 cites 90:346-348 and quotes:"
sed -n '87,89p' "$P/91_ringer_entailment_check_on_the_derived_laws_consolidation.md" | sed 's/^/    /'
echo "90:346-348 said, at the commit 91 was written against (98237e5b):"
git show 98237e5b:"$P/90_giesen_consolidation_derived_algebraic_laws.md" | sed -n '346,347p' | sed 's/^/    /'
echo "90:346-348 says today:"
sed -n '346,348p' "$P/90_giesen_consolidation_derived_algebraic_laws.md" | sed 's/^/    /'
echo
echo "The citation still resolves. The linter passes it. It now points at the"
echo "announcement of the repair rather than at the defect it was reporting."
