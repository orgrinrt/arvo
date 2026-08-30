#!/usr/bin/env bash
# `a_composed_expressions_region_is_never_inherited_from_its_parts` is marked
# `normative`, which is the one sentence_kind the region check exempts. 182
# section 6.1 flags the row itself and gives this defence:
#
#   "The honest alternative is `theorem` with no region, which the checker
#    refuses, so the row would not exist."
#
# This tests that defence. If the region is in the source, the defence is false
# and the row is an established claim wearing the exempt mark.
#
# CASE THAT MUST FAIL: control 1 runs the same axis-mapping over a row that is
# a genuine stipulation, where no source predicate should be found. Control 2
# checks each mapped axis against `dimension.toml`, so a claim that an axis is
# writable is checked rather than asserted.
set -uo pipefail
cd "$(dirname "$0")/.."
D=../../registry/dimension.toml

echo "=== the row, as committed ==="
awk '$0=="id = \"a_composed_expressions_region_is_never_inherited_from_its_parts\""{f=1}
     f{print} f&&/^keywords/{exit}' ../../registry/proposal.toml | grep -E '^(id|sentence_kind|standing|predicate|because)' | cut -c1-200
echo "  predicate field present: $(awk '$0=="id = \"a_composed_expressions_region_is_never_inherited_from_its_parts\""{f=1} f&&/^predicate/{print "YES";exit} f&&/^keywords/{print "NO";exit}' ../../registry/proposal.toml)"

echo
echo "=== the source, two lines below the sentence the row's because quotes ==="
grep -n 'Predicate as `79` stated it' -A1 90_giesen_consolidation_derived_algebraic_laws.md

echo
echo "=== every axis that predicate names, mapped, and checked against dimension.toml ==="
printf '%-26s %-34s %s\n' 'SOURCE SPELLING' 'REGISTRY AXIS' 'DECLARED'
map() { d=$(grep -cE "^id = \"$2\"\$" "$D"); printf '%-26s %-34s %s\n' "$1" "$2" "$( [ "$d" -gt 0 ] && echo yes || echo NO )"; }
map 'N = 8'                  total_width
map 'sign = unsigned'        signedness
map 'policy = saturate'      overflow_policy
map 'op pair = {+, -}'       operation
map 'F = 0'                  fraction_width
map 'threads any'            threads
map 'features any'           target_features

echo
echo "=== CONTROL 1: an axis the corpus names that is NOT declared, which must read NO ==="
map 'operand window'         declared_operand_window

echo
echo "=== CONTROL 2: the same search for a source predicate on a genuine stipulation ==="
echo "row: one_container_hosts_many_systems_so_the_canon_types_the_system"
echo "hits for a predicate line near its source in 74:"
grep -cE '^`holds for:|^Predicate as' 74_giesen_consolidation_the_number_system_concept.md || true
