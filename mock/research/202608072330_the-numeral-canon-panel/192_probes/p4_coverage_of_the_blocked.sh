#!/usr/bin/env nutshell
# Which of the twenty-one and the eight now have an instrument row, and which do not.
#
# The mapping from a blocked claim to the probe row that would serve it is a
# READING and is written by hand below. What is mechanical is the other half:
# every row id the mapping names must exist in `probe.toml`, and every claim in
# the mapping must be one of the twenty-one. Neither half is trustworthy alone.
# A hand mapping that cites a row nobody wrote is the failure this checks for,
# and this dispatch has already produced two citations to files that were not
# there.
#
# `NONE` is a real entry and is the honest output of the dispatch: it says the
# claim's instrument was not found or no row was written for it, and why is in
# the findings file rather than here.
#
# Required outcomes, written before the run:
#
#   C1  every non-NONE row id must resolve to a row in probe.toml. A miss means
#       the mapping is fiction.
#   C2  the claim column must be exactly the twenty-one, no more and no fewer,
#       diffed against the extractor's own list rather than retyped.
#   C3  at least one NONE must be present, or the mapping is claiming complete
#       coverage and complete coverage was not achieved.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"
out="$here/p4_coverage_of_the_blocked.out"
map="$here/p4_coverage_map.tsv"

# claim <TAB> probe row that would serve it, or NONE
cat > "$map" <<'MAP'
absorption_decides_associativity_of_a_clamped_reduction	absorption_and_associativity_agree_for_clamped_addition
a_law_stated_as_an_author_written_marker_is_checked_by_nothing	a_declared_law_marker_compiles_clean_when_false
the_const_eval_frontier_collapses_along_arity_and_buys_three_bits_from_the_guard	the_const_eval_frontier_by_arity
the_licensed_category_is_const_available_and_four_constructions_bind_at_four_times	the_four_const_available_constructions_bind_at_four_times
a_compile_time_strategy_selection_leaves_no_residue_in_the_emitted_body	a_compile_time_strategy_selection_leaves_no_residue
chain_accuracy_cannot_be_served_by_an_operator_closed_over_its_operand_type	chain_accuracy_needs_an_intermediate_wider_than_the_operand_type
no_total_join_exists_over_the_observable_axes_so_the_operation_reports	no_conservatism_order_exists_on_the_overflow_axis
headroom_and_intermediate_precision_are_unobservable_inside_a_pure_ring_region	three_of_the_four_proposed_axes_change_no_answer
the_rationalisability_counts_on_the_committed_carrier_table	the_rationalisable_sections_on_the_committed_carrier_table
generation_relocates_the_check_rather_than_removing_it	generating_a_winner_table_from_a_stated_weighting
the_corpus_cannot_exhibit_the_accuracy_intents_because_a_coordinate_is_absent	the_argmin_mechanism_has_never_run_on_arms_that_disagree
a_coordinate_set_is_a_countable_ceiling_on_how_many_strategies_can_exist	the_cost_coordinate_census_and_its_calibration_control
the_four_consolidations_contradict_each_other_nowhere	NONE
most_committed_bench_regions_predate_the_harness_cross_variant_validation	most_committed_bench_regions_predate_the_validation_gate
a_coherent_reduction_needs_no_accumulator	the_accumulator_width_is_the_exact_sum_width_less_one_bit
an_incoherent_clamped_addition_needs_the_exact_sum_width_less_one_bit	the_accumulator_width_is_the_exact_sum_width_less_one_bit
the_multiplicative_guard_grows_linearly_and_the_saving_is_adaptation_fusion	the_multiplicative_rescale_saving_is_adaptation_fusion
a_nonzero_phase_leaves_the_representable_set_without_an_additive_identity	a_half_step_biased_grid_is_not_closed_under_addition
a_trajectory_condition_lifts_into_a_declaration_exactly_when_it_survives_closure	a_trajectory_condition_lifts_exactly_when_it_survives_closure
an_exposure_test_over_reduction_verdicts_alone_is_satisfied_by_a_system_that_computes_nothing	the_collapsed_declaration_cannot_be_made_to_fail
where_fusion_changes_the_answer_it_is_not_a_lowering	fusion_is_an_axis_position_rather_than_a_new_axis
MAP

{
  printf '=== p4 coverage of the blocked, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## C1: every named row must exist in probe.toml\n'
  miss=0
  while IFS=$(printf '\t') read -r claim row; do
    [ "$row" = NONE ] && continue
    if grep -q "^id = \"$row\"$" "$repo/mock/registry/probe.toml"; then :
    else printf '  MISSING ROW: %s (named for %s)\n' "$row" "$claim"; miss=$((miss+1)); fi
  done < "$map"
  if [ "$miss" -eq 0 ]; then printf 'C1 PASS: every named row resolves\n'
  else printf 'C1 FAIL: %s named rows do not exist\n' "$miss"; fi
  printf '\n'

  printf '## C2: the claim column must be exactly the twenty-one\n'
  cut -f1 "$map" | sort > "$here/.p4_mine"
  sed -n '/^## LIST A/,/^count:/p' "$here/p1_what_is_blocked.out" \
    | grep -oE '^ +[a-z_]+' | tr -d ' ' | sort > "$here/.p4_theirs"
  if diff -q "$here/.p4_mine" "$here/.p4_theirs" >/dev/null 2>&1; then
    printf 'C2 PASS: the two lists are identical\n'
  else
    printf 'C2 FAIL: the mapping and the extractor disagree\n'
    diff "$here/.p4_mine" "$here/.p4_theirs" | sed 's/^/  /'
  fi
  rm -f "$here/.p4_mine" "$here/.p4_theirs"
  printf '\n'

  none=$(awk -F'\t' '$2=="NONE"' "$map" | wc -l | tr -d ' ')
  served=$(awk -F'\t' '$2!="NONE"' "$map" | wc -l | tr -d ' ')
  printf '## C3: at least one NONE\n'
  if [ "$none" -gt 0 ]; then printf 'C3 PASS: %s claims still have no row\n' "$none"
  else printf 'C3 FAIL: the mapping claims complete coverage\n'; fi
  printf '\n'
  printf '## the count\n'
  printf 'blocked claims:                    %s\n' "$((none + served))"
  printf 'now served by an instrument row:   %s\n' "$served"
  printf 'still with none:                   %s\n' "$none"
  printf '\n'
  printf 'still with none:\n'
  awk -F'\t' '$2=="NONE"{printf "  %s\n", $1}' "$map"
  printf '\n'
  printf 'NOT DONE HERE. No `evidence` edge is written. Whether a given row is the\n'
  printf 'right instrument for a given claim is the call of whoever holds the\n'
  printf 'proposal rows, and this mapping is an input to that rather than a\n'
  printf 'substitute for it.\n'
} > "$out" 2>&1
cat "$out"
