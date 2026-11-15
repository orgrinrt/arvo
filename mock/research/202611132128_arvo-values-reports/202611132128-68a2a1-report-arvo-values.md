Source: kaski channel (coord/kaski of clause-dev), att/20261113T212823432297305-arvo-values-68a2a1/report-arvo-values.md, first posted 2026-11-13 21:28 UTC, read 2026-11-15.

# arvo-values, track T3: report

Branch `work/arvo-values` of `orgrinrt/arvo`, tip `ae4efecb`, which is the last round's close merged
with `origin/dev` as it stood on 13 November 2026. No round is open. No pull request was opened.

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
| 6 | `a_test_that_bakes_a_path_with_env_measures_the_main_clone_wh`, arvo-numeral's half | `202611132123-jozr` | this tip |

Job 5: the sketch `mock/research/sketches/202611130342_value-add-disassembly/` builds against the
current value. The six first probes emit the same instructions as before, and two probes repeated
under `Abort` are emitted as aliases of their `Quiet` twins. The paragraph of the numeral design that
cites it holds as written.

Job 6: `tests/a_consumer_crate.rs` reads its crate directory from the run's environment and derives
its scratch directory from where the test binary sits. Two other sites in arvo still read the path
with `env!`, in `mock/tools/a-panel-catalogue-is-readable/src/tests.rs` and
`mock/lints/the_write_hook_excepts_what_the_canon_excepts.rs`. They are outside this track and are
named on the ticket.

## Tests, run on the tip

`cargo test -p arvo-numeral` from `mock/`, dev profile, all green: lib 12, `band_1_to_8` 11, the
three other bands 2 each, `a_consumer_crate` 2, `compile_fail` 32 pairs, `fixed_point_product` 6,
`the_accumulator` 10, `the_conversion` 6, `the_crossing` 6, `the_debug_check` 27 and its release twin
6, `the_host_door` 5, `the_lane` 4, `the_quotient` 5.

Under their features: `the_count` 4 with `hlist-const`, `the_count_without_const` 3 with `hlist`.
Neither runs in the default run, which is ticket
`arvo_numeral_the_hlist_count_targets_run_only_when_their_fea`.

`cargo mock` reports no lint error. Mutant evidence for each operation is committed under
`mock/research/sketches/*_value-mutants/`.

## Reds checked on dev

None is red on the tip. `a_consumer_crate` was expected red in a cloud container under
`the_container_git_config_rewrites_ssh_git_sources_to_https_a`. It ran green here, in this container,
on this tip. That is one run in one container and says nothing about the ticket.

The warning that `Rounded::at` is never used in `arvo-format/src/apply.rs` is on `dev` too and is
another lane's.

## Tickets filed or bumped

Filed: `arvo_format_ships_no_remainder_so_a_value_has_no_remainder_p`,
`arvo_numeral_s_fixed_point_product_goes_through_the_128_bit`,
`arvo_numeral_the_hlist_count_targets_run_only_when_their_fea`,
`cargo_mock_test_prints_every_passing_test_so_a_red_run_is_cu`.

Question for op: `arvo_numeral_gains_an_optional_const_trait_impl_gate_for_the`. The const path of the
count enables `const_trait_impl` behind the feature `hlist-const`, off by default.

Bumped: `arvo_a_runtime_divisor_over_a_whole_phase_range_without_the`,
`a_test_that_bakes_a_path_with_env_measures_the_main_clone_wh`.

## What is left

`Uint<64>` has no product until `arvo_s_product_over_the_unsigned_64_bit_range_needs_a_slot_c`.
A value has no remainder until the format ships one. The two `env!` sites named above.
