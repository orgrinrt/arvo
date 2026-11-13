# Edges the corrected notes left

This note records the work on ticket `draw_the_edges_and_write_the_rows_the_corrected_arvo_notes_l`. Nine registry rows had notes, corrected on `work/registry-repairs`, saying that an axis or a probe row did not exist where one does, and each was owed an edge, a region or a row of its own. All nine are written here on branch `work/arvo-registry-edges` as `proposal` or `law` rows at `standing = "one_expert"`. No `ruling` row was touched and no ratified content changed.

Before writing an edge, its probe's committed output was opened and checked, and the lines checked are given under each item. Each predicate was written to the region the instrument actually ran, which is sometimes narrower than the source clause it came from. Where the two differ, the item says so.

Two things turned up that no item asked for. The first is a mislabelled instrument output, under item 6. The second is a holding region on an existing law row that its cited instrument does not support, under item 5. Both are findings for a second reader, and neither was fixed by widening or narrowing a row that was not owed an edit.

## Item 1. `law::fusing_a_multiply_add_preserves_the_answer_under_unsigned`

Done. The row gets `evidence = [fusion_under_unsigned_over_six_rounding_modes, equivariance_is_domain_restricted, the_false_unsigned_fusion_clause_and_its_repair]`.

Checked: `147_probes/r1_out.txt` lines 10 to 15 and `151_probes/v1_output.txt` lines 19 to 26. Both print a per-`F` breakdown for unsigned saturating fusion: half-even at 0.93, 1.61, 2.02, 2.18 and 2.08 percent for `F` from 1 to 5, and zero for the other five modes. `149_probes/y2_out.txt` sweeps wrapping only.

The old note said no per-`F` breakdown existed for saturating, which is false: two instruments in two languages print the same rates. So the fails region widens from wrapping to `overflow_policy: in {wrap, saturating}`, and the note now gives the saturating rates. The note's sentence saying the row carries no evidence edge is gone. A second reader should check the widening.

## Item 2. `proposal::a_law_is_inherited_where_the_realisation_map_is_a_congruence_for_every_nesting_it_contains`

Done. The row gets `evidence = [one_congruence_property_predicts_every_law_verdict]`, and the measured figure moves into a new row, `proposal::the_congruence_criterion_decides_every_swept_law_verdict_in_both_directions`, which is `measured` and carries the same edge. The first row's note and gap name the new row. The comment above this group of rows was rewritten to say what each row now carries, without counting rows.

Checked: `97_probes/p2_congruence_predicts_the_laws.out` at lines 306 to 308, 619 to 621 and 932 to 934. For each of `W` in {4, 5, 6} it prints 184 cells, 0 soundness mismatches and 0 conservative mismatches. The instrument's `truncate` rounds toward zero, and its `nearest` breaks ties away from zero, so the rounding entry names those two modes rather than its labels.

Predicate: `W in {4, 5, 6}`; `F in {0, 1, 2}`; signedness in {unsigned, signed}; overflow policy in {wrap, saturate}; rounding in {toward_zero, nearest with ties away from zero}; operand window = full range; operation in {add, sub, mul}; arity in {2, 3}; threads = 1; target features any, by construction.

## Item 3. `law::quantise_then_reduce_commutes`

Done, as two rows. The source predicate at `136` section 7.3 is a union of two rectangles, and a row carries one region per field. So the wrapping arm went on this row and the saturating arm on a new twin, `law::quantise_then_reduce_commutes_under_saturation`. Both get `evidence = [rounding_commutes_with_the_overflow_policies]`.

Checked: `125_probes/p3_output.txt`. Under wrapping, the quotient-level section (lines 46 to 74) shows 0 quotient mismatches for floor, ceil, half_up and half_even at `W` in {3, 4, 5} over both signednesses, with toward_zero failing as the control. At the representative level (lines 1 to 30), floor alone is zero, against 60, 32 and 32 for ceil, half_up and half_even. The note states that the equality is read in the quotient. Under saturation at `W = 4` (lines 31 to 42), all five modes are zero over both signednesses, and the boundary-broken control shows 15.

Holds, wrapping: W any; F any; signedness any; ambient domain closed under negation; wrapping; rounding in {floor, ceil, half_up, half_even}; threads any.

Holds, saturation: the same with saturating and rounding in {floor, ceil, toward_zero, half_up, half_even}.

The width, fraction-width and signedness regions are the source's, argued by equivariance. The wrapping mode set is not: the source's wrapping arm is every translation-equivariant member, and half-even is not one, so its place in the set is this row's own derivation, that wrapping translates by an even multiple of the quantum, which the sweep agrees with at `W` in {3, 4, 5}. The instrument agrees with them where it ran and does not cover all of them. The new gap on the first row says its fails region names no ambient domain, which leaves the region holding nowhere a domain is defined. Writing one in would be a widening and is left undone.

## Item 4. `law::the_fused_and_the_stepwise_multiply_add_denote_one_function`

Done. The clause the source states as one question splits into two halves, and the note names a row for each:

- The reduction half is a new row, `proposal::a_reduction_relocation_is_free_exactly_where_the_reduction_is_a_congruence`, with `law` edges to the three fusion laws and `evidence = [the_fusion_cell_and_the_accumulator_cell_are_one_congruence_question]`.
- The rounding half is the existing `proposal::fusing_a_multiply_add_is_free_exactly_at_translation_equivariance`.

The gap is reduced to the missing witness. The probe row behind the reduction half is new; see the probe rows below.

Checked: `149_probes/y4_out.txt` lines 5 to 27. The verdicts are "W2: congruence predicts the fusion cell at 4 of 4" and "W3: congruence predicts the accumulator cell at 4 of 4", and the BADFOLD control must show congruence false with a nonzero fusion difference. The instrument rounds by arithmetic shift, which is floor.

Predicate: `W = 4`; `F in {0, 1}`; signedness in {unsigned, signed}; overflow in {wrap, saturating}; rounding = floor; accumulator width in {4, 6}; operation in {multiply-add, sum fold}; arity 3; chain length in {1, 2, 3}; container declared width; threads 1; target features host.

## Item 5. `law::coherence_of_a_reduction_onto_its_induced_operation`

Done, on a different axis from the one the ticket names. There is a new row, `law::coherence_of_unsigned_saturating_addition_holds_where_the_ambient_domain_is_non_negative`, with `evidence = [the_two_law_roles_of_a_reduction_are_independent]`, and the parent row's gap points at it.

Checked: `56_probes/q1_output.txt` line 3, "clamp unsigned ... coherent(+) true", and line 6, "clamp unsigned over a signed window: coherent(+) false". The windows in `q1_two_law_families.rs` are `profile(clamp_unsigned, 0, 15, 0, 64)` at line 128 and `profile(clamp_unsigned, 0, 15, -64, 64)` at line 174. The witness `a = -1, b = 1` is computed from the committed `clamp_unsigned`, because the run prints only the verdict.

Holds: W = 4; F = 0; unsigned; saturate; ambient domain = the integers in 0..=64; add; arity 2. Fails: the same with the integers in -64..=64.

There are two readings of the axis, and this is the call for a second reader:

- The ticket says `dimension::operand_window`. That axis is defined as a sub-range of the format's representable set. The window here, 0 to 64, runs past the representable 0 to 15, so it is not a sub-range of that set, and the signed window lies outside an unsigned format's set altogether.
- `dimension::ambient_domain` names the domain the exact values range over, which is what the instrument varies.

The row is written on `ambient_domain`. If the reading is that `operand_window` should be widened to cover exact operands beyond the representable set, then this row moves to it.

There is a second finding on the parent row, now in its gap. Its holding entries for unsigned wrapping and for unsigned saturating multiplication are not what `q1` measured. `q1` ran wrapping at signed only, and the unsigned clamp for addition only, so no row says which instrument those two entries rest on.

## Item 6. `proposal::the_realisation_map_has_an_algebraic_character`

Done as scoped by the probes. The licence claims were not all written from their source predicates, because doing so fails the suite gate. One claim was written at the region its instrument ran, and the recount is in the gap.

There is a new row, `proposal::which_operations_a_saturating_map_is_a_homomorphism_for_moves_with_the_ambient_domain`, with `evidence = [saturation_has_a_deferral_arm_on_a_one_signed_domain]`.

Checked: `122_probes/u3_output.txt` lines 5 to 16, the P1 table:

| Map, `uW4` | Domain | add | sub | mul |
| --- | --- | --- | --- | --- |
| Saturating | One-signed | 0 of 1089 | 391 of 1089 | 0 of 1089 |
| Saturating | Straddling | 720 of 2401 | 751 of 2401 | 256 of 2401 |
| Wrapping | Both | 0 | 0 | 0 |

The wrapping rows are the control.

The mislabelled output: u3 labels its domains `0..45` and `-48..48`. The code at `u3_saturation_has_a_deferral_arm_and_i_missed_it.py` lines 204 to 207 computes `0..=hi+span+1` and `lo-span..=hi+span+1`. With `Prim(4, False, ...)` at `114_probes/p1_...py` lines 94 to 96, that is `hi = 15`, `lo = 0` and `span = 16`, so the domains are `0..=32` and `-16..=32`. The pair counts agree with the code and not with the labels, since 1089 is 33 squared and 2401 is 49 squared. The row and the probe row use the computed domains. F122-4 at `122` line 609, which the row cites, repeats the labels, and was left as it is.

Predicate: W = 4; F = 0; unsigned; overflow policy in {saturate, wrap}; rounding = exact; radix = 2; operation in {add, sub, mul}; arity 2; ambient domain in {the integers in 0..=32, the integers in -16..=32}; threads = 1; target features any, by construction.

Why clause 4.4 is not written at its own predicate: `122` section 4.4 states saturation over `F in {0, 1, 2}`, both signednesses, and `rounding = truncation`, which is a word outside the ratified six. Its only committed instrument on a one-signed domain is u3, at `F = 0` and unsigned. `118` q3 ran at `F > 0`, but on a domain that straddles zero. Writing the clause's region would claim cells nothing ran.

The recount uses the axes declared now, `ambient_domain`, `radix` and `toolchain` among them. The clauses after 4.1 are nine, 4.2 to 4.10, and they carry eleven predicates because 4.5 and 4.7 carry two each. Ten of the eleven are still blocked on an undeclared coordinate:

| Clause | Undeclared coordinate |
| --- | --- |
| 4.2 | value-set size; an operation set written as containing addition |
| 4.3 | a construction being an interval |
| 4.5, both predicates | restrictions in force; a declared grid step |
| 4.6 and 4.7, both predicates | term shapes; declarations; the discharge check |
| 4.8 | term shapes; declarations |
| 4.9 | dimension and structure constants; declarations |
| 4.10 | crate type; feature gates; the term; the recursion limit |

4.4 alone names declared axes only, and it is the one partly written. The row's old figure, ten, matches neither the clause count of nine nor the predicate count of eleven. Ten is the number of predicates still blocked.

## Item 7. `proposal::the_realisation_map_factors_into_quantisation_and_range_policy`

Done. There is a new theorem row, `proposal::no_deterministic_quantisation_is_additive_off_the_grid_on_either_domain_shape`. It is a theorem, so it carries no evidence edge. The parent's note names it, and the parent's gap now holds the half that is still unwritten: the order-preserving half, that every deterministic member is monotone, has no row.

Checked: the proofs at `125` lines 69 to 95 (T1 by divisibility on the rationals, T1b by hand on the non-negative rationals) and the predicate at `132` lines 303 to 313. The sweep `125_probes/p1_output.txt` lines 8 to 12 shows every one of the five modes failing additivity on the window and on its one-signed half, with 0 failures on the grid. The row cites the sweep in `provenance` and its note, and holds no `probe` row for it.

Predicate: I any; F any; signedness any; radix any; rounding in {floor, ceil, toward_zero, half_up, half_even}; operation = add; ambient domain in {the rationals, the non-negative rationals}; threads any; target features any.

The source writes neither `radix any` nor the widths. They follow from the proof, which uses only that the grid is the multiples of one step, and the note says so. `stochastic` is outside the region, and the gap says why.

## Item 8. `proposal::staged_narrowing_disagrees_with_direct_narrowing_under_round_to_nearest_even`

Done. There is a new row, `proposal::narrowing_composes_exactly_where_the_modes_direction_switches_only_at_coarser_grid_points`, with `evidence = [narrowing_composes_where_the_modes_direction_switches_at_coarser_grid_points]`, and it answers `does_narrowing_compose` as the staged row does. The staged row's note names it.

Checked: `07_probes/p4.out`:

- Lines 22 to 27: on nested triples, the four modes that compose show 0 failures and the two nearest modes show 8, 16 or 32. Non-nested triples fail for every mode.
- Lines 37 and 38: the witness `x = 9/16`, half-even.
- Lines 53 to 57: Q2b, over a signed range.
- Lines 59 to 70: Q4, where pivots at 0, 1 and 2 give 0 failures and pivots at 1/2, 1/4, 3/4 and 3/2 give 7.
- `07_orchard_the_adjunction_frame` line 362: the adjunction is sufficient and not necessary.

Predicate: F = 5; signed; rounding in {floor, ceil, toward_zero, away from zero, half_even, nearest with ties away from zero, away from a pivot at 0, 1, 2, 1/2, 1/4, 3/4 or 3/2}; radix 2; ambient domain = the rationals on the grid of step 1/32, over the windows -2..=2 and -2..=3; threads 1; target features any, by construction.

`F = 5` over a window straddling zero is the one cell where both the named modes and the pivot family ran. The modes and the windows do not form a product, and the note says so. No width is named, because the instrument clamps at window ends rather than at a declared format's range, and the gap says so.

## Item 9. `proposal::an_output_of_a_derivation_is_a_fact_a_downstream_site_cannot_recover`

Done. There is a new row, `proposal::a_single_richer_output_suffices_only_where_it_is_a_type`, with `evidence = [a_single_richer_output_suffices_only_as_a_type]`. The parent's note now names it in place of saying it is not a row of its own.

Checked:

- `47_probes/p1_single_type_output.out` lines 11 to 20: widths 9 to 16 share the `u16` carrier and get eight distinct `Packed<Wn>` outputs.
- `p2_scalar_single_output_refused.err`: six "generic parameters may not be used in const operations" errors, at lines 95, 98 and 103 of the `.rs`, each with the help line naming `generic_const_exprs`, followed by four `E0091` errors, ten in all.
- `RUN.md`: zero `#![feature]` gates, and the pin `rustc 1.98.0-nightly (57d06900f 2026-05-27)`.
- `verify.sh`: compiling files at `-O`, refusals as a library at the default level, both at edition 2021.
- `p1_single_type_output.rs` lines 33 to 36: strategies `Hot`, `Warm`, `Cold` and `Precise`.

Predicate: W in {5, 9, 10, 11, 12, 13, 14, 15, 16, 31, 47}; F = 0; unsigned; S in {Hot, Warm, Cold, Precise}; occupancy = shared; container in {u8, u16, u32, u64}; toolchain rustc = 1.98.0-nightly (57d06900f 2026-05-27), edition = 2021; debug-assertions in {on, off}; threads any; target features any.

Occupancy is shared because the footprint column is a million values in one allocation, and sole occupancy is not claimed. The note records the split of profile between the two arms.

## The probe rows added

Three probe rows were added to `probe.toml`, each at `standing = "sound"`:

- `the_fusion_cell_and_the_accumulator_cell_are_one_congruence_question`, for y4.
- `the_two_law_roles_of_a_reduction_are_independent`, for q1.
- `saturation_has_a_deferral_arm_on_a_one_signed_domain`, for u3.

The corrected notes named these instruments, and no row held them, so items 4, 5 and 6 could not draw an `evidence` edge without them. The brief's rule for new rows covers `proposal` and `law` and says nothing of `probe`. The standing of each probe row was judged from its committed control, and a second reader should check it.

## Gate

`cargo mock --lint-only --commit` passes with rc 0, at 797 rows. It refused twice on the way, and both were fixed:

- A line citation past the end of `p4.out` was repointed to line 59.
- `toolchain` was named twice in one predicate, and the two entries were merged into one.

No em-dash and no bold appears in any added registry line.

## The pass after the second read

A second reader checked commit `bacbbd77` and agreed with three things: the widening of the fails region under item 1, `sound` as the standing of the three new probe rows, and the u3 domain correction. It also agreed that `ambient_domain` is the axis for the coherence row. Its other findings were fixed in this pass, each by the ratified rows as written. Where two of those rows pull against each other, this section says so rather than choosing.

### Warrant spelling on target features

`ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side` spells a warrant as `<axis>: <span>: <token>, <clause>`. It also says a `construction` token obliges the row's `evidence` to name an instrument that varied that axis and found no movement.

Four rows wrote `target features any, by construction, <clause>`:

- `which_operations_a_saturating_map_is_a_homomorphism_for_moves_with_the_ambient_domain`
- `the_congruence_criterion_decides_every_swept_law_verdict_in_both_directions`
- `narrowing_composes_exactly_where_the_modes_direction_switches_only_at_coarser_grid_points`
- the older `proposal.toml` row on the additive identity and the phase

None of their instruments varied target features, so no token is supportable. Each entry now reads `target_features: target features any` with no token, which claims the region and no warrant. Each row's note keeps the construction argument and says why the token is withheld.

The alternative was to drop the entry. The rows would then claim nothing wherever target features exist, which is the strongest negative the notation has, and `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked` was ratified to stop exactly that reading. If the axis should be dropped instead, it is one line per row.

The older row's rounding, overflow, threads and occupancy entries still use the old prose spelling `by construction`, and they are not respelled here.

### Proof markers

Under the warrant ruling's obligation, the marker `ruling::a_proof_and_a_bounded_range_get_markers_the_notation_lacked` asks of a proof can sit only on an axis some cited instrument varied.

On `law::quantise_then_reduce_commutes`, the width and signedness entries carry `construction`. `125_probes/p3` varied `W` over {3, 4, 5} and both signednesses, and the quotient verdict did not move (`p3_output.txt` lines 45 to 74). The rounding entry carries `swept`.

On `law::quantise_then_reduce_commutes_under_saturation`, only signedness carries `construction`, because saturation ran at `W = 4` alone. The rounding entry carries `swept`.

On the theorem `no_deterministic_quantisation_is_additive_off_the_grid_on_either_domain_shape`, no instrument varies any of its six open axes. The new probe row `every_deterministic_mode_is_monotone_and_none_is_additive_off_the_grid`, filed for `125_probes/p1`, gives it an `evidence` edge and a `swept` on its rounding entry. p1 runs at one step and one window, so it cannot discharge the marker.

So on F and threads in both laws, on W in the saturation law, and on the theorem's six open axes, the marker is owed and not written. The warrant ruling's own note names this price. What discharges it is a run that varies those axes, such as p1 at several steps for the fraction width. This is a call for the panel, because the two ratified rows cannot both be met on those axes with the instruments committed.

### The output count

`a_single_richer_output_suffices_only_where_it_is_a_type` now says it is about packaging. It cites `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` for the three facts owed at shared occupancy, the carrier, the access width and the stride, and it gives those three as projections of the one type.

The claim of eight distinct outputs is narrowed to the Cold strategy (`p1_single_type_output.rs` lines 186 to 193). The gap says that Hot, Warm and Precise appear at width 13 alone.

### The coherence row

The domains are named as the grammar names a domain, `the non-negative integers` against `the integers`, and the swept windows `0..=64` and `-64..=64` are the warrant clauses. Both regions add `threads = 1` and `target features = host default`, which the instrument fixes: `56_probes/RUN.md` records a single-threaded `rustc -O` build at the pin.

The parent law `coherence_of_a_reduction_onto_its_induced_operation` gets `evidence = [the_two_law_roles_of_a_reduction_are_independent]`. Its witness figures, 476 and 897, are `q1_output.txt` lines 1 and 4.

`dimension::operand_window` had counted the coherence row as a third row naming the signed declared-non-negative cell, at `W in {4, 5, 6}`. The coherence row named a different cell, and q1 never ran the signed one. The note and `moves` now say two rows, and the note gives the coherence row's own cell and where it is held.

### The relocation row

`a_reduction_relocation_is_free_exactly_where_the_reduction_is_a_congruence` now has chain length `{2, 3}`, because nothing in y4 runs a chain of one. The fraction width, accumulator width and chain length entries each carry a `swept` clause naming which arm ran at which value, and the note names the two combinations that did not run.

The other entries are now in the declared grammar:

- `operation in {multiply-add, sum fold}`
- `container = an i128 model reduced to the declared width at every step`
- `target features = host default`

### The u3 row and the probe rows

The u3 proposal's `says` reads "on the non-negative integers" in place of "of one sign". Its domain entry is spelled as on the coherence row, and its gap names the non-positive domain and the domain closed under negation as unrun.

On the probe rows:

- u3: the label mislabel moved from `note` to `defect`, and the control says it is printed and not asserted.
- q1: the control names the three checkers that fire, monotone, nearest and coherence. The retraction checker is true on every row.
- y4: the control says it is printed and not asserted. The unrecorded toolchain is its `defect`, since the probe schema has no toolchain field.

### Gate

`cargo mock --lint-only --commit` passes with rc 0, at 798 rows. On the way it refused once on invalid TOML in `law.toml`: a substitution of mine had written the witness text into the parent note. The note was restored from `bacbbd77` with the one sentence appended.
