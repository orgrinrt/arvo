#!/usr/bin/env bash
# Every one of the four consolidations was repaired after its entailment check.
# The port read the repaired files. So the question is not whether a refuted
# claim survived through an unrepaired source; it is whether the RESTORED
# material reached a row.
#
# Method: take the lines each repair commit ADDED, pull the distinctive terms
# out of them, and ask whether any registry row uses those terms.
#
# CASE THAT MUST FAIL: control 1 searches the rows for terms taken from the
# repair commits' DELETED lines instead. A term that was deleted from a source
# and still appears in a row is a row carrying the pre-repair framing, and if
# that count is the same as the added-term count the search is not
# discriminating. Control 2 searches for a term that is in no source at all.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"
P=mock/research/202608072330_the-numeral-canon-panel
REG=mock/registry
SIDE=${1:-added}
case "$SIDE" in added) MARK='^+';; deleted) MARK='^-';; esac

hits() { grep -oiE "$1" "$REG"/proposal.toml "$REG"/law.toml 2>/dev/null | wc -l | tr -d ' '; }

check() { # $1 label, $2 regex
  n=$(hits "$2")
  printf '  %-58s %s\n' "$1" "$( [ "$n" -gt 0 ] && printf 'PRESENT (%s hits)' "$n" || printf 'ABSENT' )"
}

echo "### side under test: $SIDE"
echo
echo "=== 63, repaired per 64: D-C the chain as a first-class typed object ==="
check "the label D-C"                 'D-C'
check "expression template"           'expression template'
check "the three-way direction question (D-A, D-B, D-C live)" 'D-A|D-B'
check "computation graph drift"       'computation graph'

echo
echo "=== 74, repaired per 75: three restorations ==="
check "2a: 65's change-test misfiles its own chain" 'change.test|misfile'
check "2b: 66 credited for posing conversion vs resolution" '66:3[0-9][0-9]|posed the split'
check "2c: the typestate cannot break the tie"      'typestate cannot break|break the tie'

echo
echo "=== 90, repaired per 91: R8 and the restored ordering candidate ==="
check "R8 repair (the shared-author caveat attaches elsewhere)" 'shared.author|share an author'
check "76's ordering candidate: Precise refines Hot"           'refinement of|partial order|refines'
check "honored set / honoured set"                             'honou?red set'

echo
echo "=== 106, repaired per 107: the law predicate and four dropped results ==="
check "the restored predicate: signedness on the distributivity claim" 'signedness = unsigned'
check "F-H: the declared non-negative operand window"                  'declared non-negative|operand window'
check "F-B: polynomial against exponential in the region count"        'polynomial|hyperplane'
check "the exchange-rate against priority reading"                     'exchange rate|lexicographic|priority'
check "98's five-tier ladder and the Pareto-admissible tier"           'ladder|Pareto|admissible'
check "97's criterion stated rather than scored"                       'ordered nesting|identity of exact'

echo
echo "=== CONTROL 2: a term in no source at all must be ABSENT ==="
check "zzq_not_a_real_term"           'zzq_not_a_real_term'
echo
echo "=== CONTROL 3: a term that is certainly in the rows must be PRESENT ==="
check "the word predicate"            'predicate'
