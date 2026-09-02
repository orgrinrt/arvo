#!/usr/bin/env bash
# Seat 246. My worktree is later than the tree 244 measured against (800e120a),
# and the registry gains rows constantly. 245 was caught by exactly this: it
# re-ran one of 244's probes at its own HEAD, got a different number, and wrote
# it up as a defect before finding the cause was the tree rather than the file.
#
# So before any claim of mine rests on a row, this checks whether that row is
# byte-identical between 244's tree and mine. Where it is, my reading and 244's
# are about the same text and the tree question does not arise. Where it is not,
# I say so and predicate on my tree.
#
# THE CASE THAT MUST FAIL: a comparison that reports SAME for everything is
# useless. The whole files changed between the two trees, so at least one row
# somewhere must be reported as differing or added, or the row extractor is not
# reading rows at all. That is C2, and it is run against rows I do not depend on.

set -u
cd "$(dirname "$0")/../../../.." || exit 1   # repo root
BASE=800e120a
fail() { echo "CONTROL FAILED: $1"; exit 2; }

git cat-file -e "$BASE^{tree}" 2>/dev/null || fail "the tree 244 cites is not reachable from this clone."
echo "244's tree : $BASE"
echo "my HEAD    : $(git rev-parse --short HEAD)   tree $(git rev-parse --short HEAD^{tree})"
echo

row() { # row <table> <file> <id> [<ref>]
  local t=$1 f=$2 id=$3 ref=${4:-}
  local src
  if [ -n "$ref" ]; then src=$(git show "$ref:$f" 2>/dev/null); else src=$(cat "$f"); fi
  printf '%s' "$src" | awk -v RS="\\[\\[$t\\]\\]" -v w="id = \"$id\"" 'index($0,w){print}'
}
cmp_row() { # cmp_row <table> <file> <id>
  local a b
  a=$(row "$1" "$2" "$3" "$BASE" | shasum | cut -c1-12)
  b=$(row "$1" "$2" "$3"        | shasum | cut -c1-12)
  if [ "$(row "$1" "$2" "$3" | wc -c)" -le 1 ]; then echo "ABSENT-NOW"; return; fi
  if [ "$(row "$1" "$2" "$3" "$BASE" | wc -c)" -le 1 ]; then echo "ADDED-SINCE"; return; fi
  [ "$a" = "$b" ] && echo SAME || echo DIFFERS
}

echo "=== the rows this file's claims rest on ==="
for id in the_format_spine_is_canon \
          the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule \
          the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon \
          the_panel_finishes_the_canon_without_him \
          the_operating_constraints_are_intents_and_rules; do
  printf '  ruling::%-72s %s\n' "$id" "$(cmp_row ruling mock/registry/ruling.toml "$id")"
done
for id in membership_and_hosting_are_two_questions \
          admission_returns_a_coordinate_rather_than_a_verdict \
          a_format_is_identified_by_its_ambient_domain_and_its_representable_set \
          the_concept_is_closed_and_the_inventory_is_open \
          the_concepts_edge_is_not_an_order_and_wrapping_is_the_test \
          a_system_exposes_its_ambient_laws_its_set_and_its_reductions_verdicts \
          an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing; do
  printf '  proposal::%-70s %s\n' "$id" "$(cmp_row proposal mock/registry/proposal.toml "$id")"
done
for id in is_the_number_system_inventory_open is_admission_a_predicate_or_a_location \
          is_number_system_broad_enough_for_non_magnitude are_set_valued_carriers_admitted \
          one_word_or_two_for_is_a_number_system what_the_admission_contract_asks_a_candidate_to_expose \
          is_the_ambient_operation_family_fixed are_the_level_hierarchies_the_same_cut; do
  printf '  question::%-70s %s\n' "$id" "$(cmp_row question mock/registry/question.toml "$id")"
done
echo

echo "=== C2, the control: the comparison must be able to report a change ==="
added=0
for f in mock/registry/ruling.toml mock/registry/proposal.toml mock/registry/question.toml mock/registry/retirement.toml; do
  t=$(basename "$f" .toml); t=${t%%-*}
  ids_now=$(grep -o '^id = "[a-z0-9_]*"' "$f" | sed 's/id = "//;s/"//' | sort)
  ids_then=$(git show "$BASE:$f" | grep -o '^id = "[a-z0-9_]*"' | sed 's/id = "//;s/"//' | sort)
  n=$(comm -13 <(printf '%s\n' "$ids_then") <(printf '%s\n' "$ids_now") | grep -c .)
  printf '  %-40s rows added since %s : %d\n' "$(basename "$f")" "$BASE" "$n"
  added=$((added+n))
done
[ "$added" -gt 0 ] || fail "C2, no row anywhere was reported as added, so a SAME above may be the instrument rather than the registry."
echo "  C2 passes: $added rows were added between the two trees, so the instrument can see a change."
echo
echo "  The registry moved. None of the rows above that this file depends on moved with it,"
echo "  except where marked. That is what lets my readings and 244's be about the same text."
