#!/usr/bin/env bash
# 107 section 4.4: `106` cites 97's criterion by its score and never states it.
# 106's repair pass fixed four other things and left this one, and its section 17
# ("what the check found that is not repaired here") does not list it either.
# 182 read only 106. So the criterion is in no row.
#
# This checks all three links and then asks whether the claim is writable today.
#
# CASE THAT MUST FAIL: control 1 runs the same absence search for a claim from
# the same file that DID reach a row. If that also reads absent, the search is
# not finding claims that are there.
set -uo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
PANEL="$ROOT/mock/research/202608072330_the-numeral-canon-panel"
D="$ROOT/mock/registry/dimension.toml"
REG="$ROOT/mock/registry"

C='ordered nesting|identity of exact|respects every|quotient of exact|iff it is an identity'

echo "### the criterion, at its source"
sed -n '706,709p' "$PANEL/97_dolan_the_strategy_space_attacked.md" | sed 's/^/  /'
echo
echo "  and its own region, at 97:786-791:"
sed -n '784,791p' "$PANEL/97_dolan_the_strategy_space_attacked.md" | sed 's/^/  /'

echo
echo "### link 1: does 106 state it?"
printf '  content hits in 106 : %s\n' "$(grep -icE "$C" "$PANEL/106_giesen_consolidation_the_strategy_axis.md" || true)"
printf '  score hits in 106   : %s\n' "$(grep -c '552 cells' "$PANEL/106_giesen_consolidation_the_strategy_axis.md" || true)"
echo "  (the score is carried and the content is not, which is 107 section 4.4)"

echo
echo "### link 2: does 106 section 17 record it as an unrepaired finding?"
sed -n '/^## 17\./,/^## 18\./p' "$PANEL/106_giesen_consolidation_the_strategy_axis.md" \
  | grep -E '^- \*\*' | sed 's/^/  /' | cut -c1-110
printf '  criterion mentioned in section 17: %s\n' \
  "$(sed -n '/^## 17\./,/^## 18\./p' "$PANEL/106_giesen_consolidation_the_strategy_axis.md" | grep -icE "$C|criterion" || true)"
echo "  107 recorded six defects. Section 17 accounts for three."

echo
echo "### link 3: does any row carry it?"
printf '  content hits across proposal.toml and law.toml : %s\n' \
  "$(grep -icE "$C" "$REG"/proposal.toml "$REG"/law.toml | awk -F: '{s+=$2} END{print s+0}')"

echo
echo "### CONTROL 1: a claim from the same file that DID reach a row"
printf '  \"no total join / conservatism order\" in the rows : %s\n' \
  "$(grep -ic 'conservatism order\|no_total_join' "$REG"/proposal.toml || true)"
echo "  non-zero, so the search finds 97's claims where they are present."

echo
echo "### is it writable today? every axis its region names, against dimension.toml"
printf '  %-34s %-22s %s\n' 'SOURCE SPELLING' 'AXIS' 'DECLARED'
m(){ printf '  %-34s %-22s %s\n' "$1" "$2" "$(grep -qE "^id = \"$2\"\$" "$D" && echo yes || echo NO)"; }
m 'W in {4,5,6}'                    total_width
m 'F in {0,1,2}'                    fraction_width
m 'signedness in {unsigned,signed}' signedness
m 'overflow in {wrap,saturate}'     overflow_policy
m 'rounding in {truncate,nearest}'  rounding
m 'operations {add,sub,mul}'        operation
m 'arity 2 and 3'                   arity
m 'threads = 1'                     threads
m 'target features any'             target_features
echo "  and the one it names that is not an axis:"
m 'operand window in {full, decl}'  declared_operand_window
echo
echo "  nine of ten declared. The tenth names both of its values, so leaving it"
echo "  out is the blind spot 182 section 5.1 already accepted for R12's container"
echo "  half rather than a negative claim. The row is writable."
