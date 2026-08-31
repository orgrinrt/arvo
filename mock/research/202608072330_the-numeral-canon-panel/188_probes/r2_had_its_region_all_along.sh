#!/usr/bin/env bash
# WITHDRAWN. Read the verdict at the bottom before the body.
#
# The hypothesis was that `a_composed_expressions_region_is_never_inherited_from_
# its_parts` escapes the region check as `normative` while its region sat two
# lines below the sentence its `because` quotes, at `90:136-137`, on seven axes
# every one of which is declared. 182 section 6.1's stated defence for the label
# is "the honest alternative is `theorem` with no region, which the checker
# refuses, so the row would not exist", and that defence would then be false.
#
# Every fact above is true and the conclusion drawn from them is wrong. The
# region was carried, complete and verbatim, into the law row the proposal row
# points at. Part three is the check that found this, run because a severe
# finding is verified before it is reported.
set -uo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
PANEL="$ROOT/mock/research/202608072330_the-numeral-canon-panel"
D="$ROOT/mock/registry/dimension.toml"
P="$ROOT/mock/registry/proposal.toml"
L="$ROOT/mock/registry/law.toml"

echo "### PART ONE: the row carries no predicate and is exempt"
awk '$0=="id = \"a_composed_expressions_region_is_never_inherited_from_its_parts\""{f=1}
     f{print} f&&/^keywords/{exit}' "$P" | grep -E '^(id|sentence_kind|law)' | sed 's/^/  /'
echo "  predicate field present: $(awk '$0=="id = \"a_composed_expressions_region_is_never_inherited_from_its_parts\""{f=1} f&&/^predicate/{print "YES";exit} f&&/^keywords/{print "NO";exit}' "$P")"

echo
echo "### PART TWO: the source states a region, on seven declared axes"
grep -n 'Predicate as `79` stated it' -A1 "$PANEL/90_giesen_consolidation_derived_algebraic_laws.md" | sed 's/^/  /'
echo
printf '  %-26s %-24s %s\n' 'SOURCE SPELLING' 'REGISTRY AXIS' 'DECLARED'
m() { printf '  %-26s %-24s %s\n' "$1" "$2" "$(grep -qE "^id = \"$2\"\$" "$D" && echo yes || echo NO)"; }
m 'N = 8'             total_width
m 'sign = unsigned'   signedness
m 'policy = saturate' overflow_policy
m 'op pair = {+, -}'  operation
m 'F = 0'             fraction_width
m 'threads any'       threads
m 'features any'      target_features
echo
echo "  CONTROL: an axis the corpus names that is not declared, which must read NO"
m 'operand window'    declared_operand_window

echo
echo "### PART THREE: the check that withdrew the finding"
echo "  the row's own note, which gives a different defence than 182 section 6.1:"
awk '$0=="id = \"a_composed_expressions_region_is_never_inherited_from_its_parts\""{f=1}
     f&&/^note = /{print;exit}' "$P" | fold -s -w 96 | sed 's/^/    /'
echo
echo "  the law row it points at:"
awk '$0=="id = \"associativity_of_a_composed_saturating_add_and_subtract\""{f=1}
     f{print} f&&/^keywords/{exit}' "$L" | grep -E '^(id|fails|  "|provenance|  "panel)' | sed 's/^/    /'

echo
echo "### VERDICT: withdrawn"
cat <<'T'
  The seven axes are all present, in the same order and the same values, on the
  law row's `fails` list. Its provenance cites `90:137`, which is the predicate
  line itself. The proposal row wires to it with `law = [...]` and its note says
  in terms that the measured claim is the law row and this row is the rule drawn
  off it.

  So the region was not lost, the split was executed, and the commit message that
  landed these rows says exactly what it did: "splitting each measurement from the
  rule drawn off it".

  What survives is much smaller and is about 182 rather than about the rows:
  section 6.1's stated defence for the label is weaker than the one the row
  itself carries, and a reader working from 182 alone would conclude the region
  was unavailable when it is one field away.
T
