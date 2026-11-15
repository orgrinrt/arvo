Source: kaski channel (coord/kaski of clause-dev), att/20261113T223847808627261-arvo-values-03f6f3/report-arvo-values.md, first posted 2026-11-13 22:38 UTC, read 2026-11-15.

# arvo-values, track T3: report

Branch `work/arvo-values` of `orgrinrt/arvo`, tip `2b413b50`, on `origin/dev` at `4d6aa22c`. No
round is open. No pull request was opened.

Reviewed three times, and the last verdict is DO NOT MERGE. The branch is handed back with that
verdict and is not offered as ready to land. What the third reading holds against it is under "The
third verdict" below.

The value definition the crate is built on,
`proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`, stands on a provisional
stamp. Every round says so and none cites it as ratified.

## The six jobs

| Job | Ticket | Round | Posted |
| --- | --- | --- | --- |
| 1 | `arvo_ring_operations_sub_neg_mul_on_num_through_the_value` | closed | `ready e9f307ee` |
| 2 | `arvo_shipped_overflow_default_and_the_per_case_override` | closed | `ready bdc6999a` |
| 3 | `arvo_division_conversion_lanes_and_the_accumulator_through_t` | `202611132039-hccc`, `202611132057-rtzg` | `ready 4beb5c52`, `ready dd47015e` |
| 4 | `arvo_implements_notko_hlist_s_cardinal_for_usize` | `202611132111-ew52` | `ready 14eef25b` |
| 5 | `the_value_add_disassembly_sketch_no_longer_builds_against_ar` | none, research only | `ready d94375ad` |
| 6 | `a_test_that_bakes_a_path_with_env_measures_the_main_clone_wh`, arvo-numeral's half | `202611132123-jozr` | `ready ae4efecb` |

## What the readings changed

Three fresh `pr-reviewer-senior` seats on opus read the branch, each in its own worktree, and their
reports are committed verbatim under `mock/research/202611132145_review-arvo-values/` as `PASS_1.md`,
`PASS_2.md` and `PASS_3.md`, with `RESPONSE.md` saying what was done about each point.

Job 3 no longer ships all it was briefed to. The accumulator is off the branch.
`question::which_chain_carrier_ships` is open with the panel as decider and its first option is the
named accumulator; the first reading called the type ambiguous against it and the second misaligned.
The source as it stood is at commit `a303d07b`, and its rounds and mutant run stay in the tree.
Ticket `does_the_name_accumulator_in_arvo_numeral_pre_empt_which_cha` holds the question for op.

`Num::holds_index` is private. It was a public verdict on a host integer, against
`ruling::the_ingest_door_waits_on_a_known_use`. Ticket
`is_a_verdict_on_whether_a_host_integer_names_a_member_wanted` holds that question for op.

The branch adds four things to `arvo-format`, a crate other lanes hold: `WithOverflow`,
`WithRounding`, the point `Fixed`, and `multiplicative_identity` with its suite. Nothing that was
there changed.

`div`, `convert` and the product at a non-zero exponent are proved against arithmetic in the test
crate, `tests/common/oracle.rs`, at widths up to 64 bits, and no longer against the function of
`arvo-format` they call.

## The third verdict

`PASS_3.md`, at `e4e5b456`, DO NOT MERGE. Nothing it found was fixed, since a branch gets three
readings. The tip `2b413b50` differs from what it read only by its own report and the response.

It blocks on one canon point and on the suite. `Num::convert` and `fits_convert` build the second
option of `question::whose_reduction_governs_a_lossy_crossing`, open with the panel as decider. The
registry row reads as it says. It is one seat's reading and calls itself ambiguous, because
`arvo_format::convert` takes a target signature on `dev` already. The call is the coordinator's or
op's: ticket `does_num_convert_pre_empt_whose_reduction_governs_a_lossy_cr`. Nothing else on the
branch depends on `convert`.

On the suite: the quotient, the conversion and the fractional product are swept at sampled widths
where the bands take every width, no numeral test names a decimal format, and the private verdict
in `from_index` is observed at four bits only. Ticket
`arvo_numeral_sweeps_div_convert_and_the_fractional_mul_at_sa`.

## Tests, run on `e4e5b456`

`cargo test -p arvo-numeral` from `mock/`, dev profile, all green: lib 12, `band_1_to_8` 11, the
three other bands 2 each, `a_consumer_crate` 2, `compile_fail` 27 pairs, `fixed_point_product` 7,
`the_conversion` 7, `the_crossing` 6, `the_debug_check` 25, `the_host_door` 5, `the_lane` 4,
`the_quotient` 6.

`the_debug_check_in_release` runs no test in the dev profile, where it is compiled out, and 5 under
`cargo test --release`, green.

Under their features: `the_count` 4 with `hlist-const`, `the_count_without_const` 3 with `hlist`.
Neither runs in the default run, which is ticket
`arvo_numeral_the_hlist_count_targets_run_only_when_their_fea`.

`cargo mock` reports all lints passed. Mutant evidence is committed under
`mock/research/sketches/*_value-mutants/`.

## Reds checked on dev

`cargo test -p arvo-format` has one red,
`the_inverse_of_three_round_trips_at_every_unsigned_width_from_one_to_63`. `red-check` finds it red
on `dev` at `4d6aa22c` as well, in a file the branch does not touch. Ticket
`arvo_format_inverse_round_trip_underflows_at_width_1`, bumped.

The warning that `Rounded::at` is never used in `arvo-format/src/apply.rs` is on `dev` too and is
another lane's.

## Tickets filed or bumped

Filed earlier: `arvo_format_ships_no_remainder_so_a_value_has_no_remainder_p`,
`arvo_numeral_s_fixed_point_product_goes_through_the_128_bit`,
`arvo_numeral_the_hlist_count_targets_run_only_when_their_fea`,
`cargo_mock_test_prints_every_passing_test_so_a_red_run_is_cu`.

Filed from the readings: `arvo_numeral_carries_its_test_mutants_as_cargo_features_in_s`,
`usize_and_isize_carry_from_host_and_host_beside_from_index_a`,
`red_check_tells_an_arvo_helper_to_run_cargo_mock_known_red_s`.

Questions for op: `arvo_numeral_gains_an_optional_const_trait_impl_gate_for_the`,
`does_the_name_accumulator_in_arvo_numeral_pre_empt_which_cha`,
`is_a_verdict_on_whether_a_host_integer_names_a_member_wanted`,
`does_num_convert_pre_empt_whose_reduction_governs_a_lossy_cr`.

Task from the third reading: `arvo_numeral_sweeps_div_convert_and_the_fractional_mul_at_sa`.

Bumped: `arvo_a_runtime_divisor_over_a_whole_phase_range_without_the`,
`a_test_that_bakes_a_path_with_env_measures_the_main_clone_wh`,
`arvo_format_inverse_round_trip_underflows_at_width_1`.

## What is left

`Uint<64>` has no product until `arvo_s_product_over_the_unsigned_64_bit_range_needs_a_slot_c`.
A value has no remainder until the format ships one. Two sites outside this track still read a path
with `env!`, in `mock/tools/a-panel-catalogue-is-readable/src/tests.rs` and
`mock/lints/the_write_hook_excepts_what_the_canon_excepts.rs`, named on job 6's ticket.

Three commits on the branch cannot be corrected without rewriting pushed history: `9a3d5149` and
`ae4efecb` carry git's default merge subject, and `252d1ce4` is typed `test:` while changing a
source file that a later commit deletes.

The second reading asked whether the branch `work/numeral-integers` is dead. It is not this track's.
