# Plan for the slot coordinate tickets in `arvo-format`

**Date:** 2026-11-13
**Author:** a fable planner dispatched by job `arvo-format`, reading `5ac496a2` in its own worktree.
**Verbatim:** the planner's report as returned, kept whole below. It is one seat's reading; its canon
verdicts on tickets 1 and 3 owe a second independent read before a round built on them locks.

## Report

Read at `5ac496a2` in `/home/user/.worktrees/plan-slot-coordinate`. Nothing was written there. Scratch notes are in `/tmp/claude-0/-home-user/7c35e2ad-fb65-5898-963d-96f2bad980fd/scratchpad/plan-slot/`.

The four tickets name one cause, a slot coordinate that is too narrow or too loosely typed. Read against the canon and the design they are three different things. Two tickets and one cell of a third are about the exact position, which the design already says is not a slot. One ticket is about a type for membership. The remaining two cells of the law ticket are about how wide a declared range may be, which the canon holds open.

So the recommendation is two rounds and one hand-back. `Slot::at` and `Slot::index` stay as they are in both rounds, and no IEEE or MATLAB suite is ported.

### The canon gate

These verdicts are one seat's. The workspace rule asks for two independent agreements on what the canon permits, so a second read is owed before the first round locks, on the first and third rows below in particular.

| Ticket | Rows that bear | Verdict |
|---|---|---|
| 1, product over `UFixed<64, k>` at `k >= 0` | `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` (ratified), `ruling::the_format_spine_is_canon` with `proposal::arithmetic_on_a_format_factors_as_an_adaptation_of_an_exact_operation`, `ruling::never_a_runtime_check_and_one_lowered_path` (ratified) | Aligned. The product is a function of the signature, so holding it out is a carrier limit and not a refusal the canon makes. No canon row bounds a slot index or a position; the answer recorded on `question::did_the_admitted_obligations_answer_the_exposure_question_in_the_design` says so in those words. |
| 1, the fused multiply-add clause | `ruling::the_fused_result_is_composable_except_at_signed_saturating` (stated, one expert), `law::the_fused_and_the_stepwise_multiply_add_denote_one_function` | Aligned as a capacity, and nothing is built. The plan makes a sum of two 64-bit products holdable. It adds no fused operation, because the only row asking for one is at `stated`. |
| 2, quotient and conversion positions past the index | Same rows as ticket 1, plus `proposal::a_conversion_is_one_placement_onto_the_target_grid` (one expert, not canon) | Aligned, same reasoning. |
| 3, the cell at width 64 | `law::the_multiplicative_inverse_round_trip` | Aligned. It is reached the moment ticket 1 closes. |
| 3, the cells at 65 and 128 | `question::one_word_or_two_for_is_a_number_system` (open, decider panel), `ruling::a_platform_width_type_is_a_target_bound_member_of_the_format_family` (ratified; its promotion calls the 64-bit ceiling an artifact, on one instrument), `proposal::the_platform_width_notes_region_predates_the_sixty_four_wide_ladder` | Ambiguous, handed back. Reaching 65 needs a declared range past 64 bits, and 128 needs a slot wider than `i128` as well. The 64-bit bound sits in `Slots::ADMITTED` as a hosting condition, which is the first option of the open question taken at the design tier. Lifting it inside a round would answer a reserved question in a design. The sibling ticket `arvo_binary128_and_decimal128_need_a_slot_ladder_past_64_bit` waits on the same call. |
| 4, a runtime divisor over a whole-phase range without the zero | `ruling::never_a_runtime_check_and_one_lowered_path`, `ruling::the_ingest_door_waits_on_a_known_use` (stated: the typestate is the default answer), `proposal::a_quotient_s_divisor_is_non_zero_by_type` (one expert, not canon), `proposal::ingest_of_a_host_value_is_the_declared_adaptation` (one expert) | Aligned for a membership type whose doors are a build-checked constant and the declared adaptation. |
| 4, the values lane's addition (a value divided by a value where the zero is in range) | `ruling::the_ingest_door_waits_on_a_known_use` | Ambiguous at one point. A door that classifies a runtime slot as non-zero is the runtime membership door op did not bless. The plan reaches the use through the typestate instead: the divisor is a member of a zero-free format on the same grid. If kaski or hilavitkutin need a classifying door, that is a question ticket with op as holder, naming the known use. It is not built. |

Where the design disagrees with itself and with the tickets: the open item at the end of `DESIGN.md.tmpl` says admitting the product is "a widening of the slot coordinate past `i128`, which every other computation over a slot would then owe too". The section "Where the exact step happens, which is not in the set it lands in" says the opposite: "The step's intermediate is not a slot index and is not required to be one; only the answer is", and that carrying the intermediate "in a form that holds every position the step can reach removes the case rather than catching it". The second is the one derived from the spine. The open item, and the ticket wording copied from it, are wrong about the locus.

The 64-bit condition in `Slots::ADMITTED` remains a design-tier answer to an open canon question. This plan does not touch it and says so.

### The suite audit

`cargo test -p arvo-format --lib` at the tip: 319 passed, none failed, none ignored. The integration targets (`tests/compile_fail.rs`, the three IEEE and two MATLAB parity suites, the ties-away alias) were not run.

Bodies read: `multiplication/tests/the_edges.rs`, `the_step.rs` from the saturation test down, `the_inverse_round_trip.rs`, `division/tests/the_divisor.rs`, `division/tests/the_edges.rs`, the head of `apply/tests/totality.rs`, and the uncarried-product pair under `tests/ui/`. For `apply/tests/the_carry_past_the_index.rs`, `the_top_of_the_index.rs`, `the_far_end_of_the_index.rs`, `the_cross_end_saturation.rs`, `conversion/tests/the_edges.rs` and `wide/tests.rs` only the test names and module docs were read, so the audit is incomplete there.

Findings:

1. The edge sweeps have an independent oracle (`Big`, bit-serial division) and a control that must disagree (`floor_wrong > 0`, `floor_on_one == 0`). Sound.
2. `corners_are_held` and `oracle_admits` restate the source's own held criterion, `x.slot().index() < i128::MAX`, word for word from `placing::is_held`. The floor is independent and the criterion is not. Round one deletes the criterion, so the copy goes with it.
3. `a_position_past_the_index_saturates_on_its_sign`, the oracles' `reported`, and the `held_out == 3` and `held_out == 2` counts pin the placeholder. They protect the old shape and are rewritten or deleted in round one, not adapted.
4. The inverse round trip samples members above width 20. The law reduces to `3 * INV3 == 1` modulo `2^W`, and `x = 1` is in every sample, so the sample holds the deciding member. Its two failing controls are present.
5. No test asserts that a `Divisor` built by any door is non-zero as a property of the type. The doors are tested one at a time. Round two adds the perimeter test.

Nothing found refuses the planning work.

### Candidate shapes

Two questions are separate: where the width goes, and how membership is typed.

### Where the width goes

| | A: `Slot` itself is four limbs | B: `Slot` stays `i128`, the position's floor is four limbs | C: the carrier is chosen per format at the type level |
|---|---|---|---|
| Public surface | `Slot::index` can no longer return `i128` totally; it becomes a maybe or a wide type, which needs a public four-limb integer. `Slots::MIN` and `MAX`, `Reach`, `SlotCount` follow. | None in signatures. `Exact` keeps `on_grid`, `at`, `between`, `slot`, `is_on_grid`, `is_tie`. Its private fields change. | `Slot<F>` or `Exact<F>`; every signature that takes a slot or a position gains a parameter. |
| `Slot::at`, `Slot::index` | `at` can stay; `index` changes. | Both unchanged. | Both change. |
| `adapt` path | Every read of a slot is four-limb arithmetic, addition and the reach included. The headroom arguments in the index section are rewritten. | `round_slot` and `complete_slot` read a four-limb floor in place of an `i128` and an `i64` distance. The remainder they already read is two four-limb values. | The narrow path for formats that never leave `i128`, the wide one for the rest, settled at monomorphisation. |
| Consumers | `arvo-numeral` ports every `.index()` (11 source sites and its tests). The five standards suites port. | None. One `arvo-numeral` compile-fail case flips, see below. | Everything ports. |
| What it closes | Tickets 1, 2, 3 at 64, and makes 128 holdable later. | Tickets 1, 2, 3 at 64. Leaves 128 to the width decision. | Tickets 1, 2, 3 at 64. |
| Canon exposure | Widens a coordinate for widths the canon has not admitted. | None. | None, but a large surface change no row asks for. |

Under B the fourth condition of each obligation does not move, it disappears. A numerator formed in the four limbs over a denominator of at least one has a floor in the four limbs, and a position with a remainder has a denominator of at least two, so its floor is at most half the limbs' reach and the step up fits. "Carried" then follows from "formed". `Exact`'s `past` field goes too, since a carry of at most `2^63` from any `i128` slot is an ordinary four-limb value.

No timing claim is made for any shape. The cost column is a count of what is read, not a measurement.

### How membership is typed

| | M1: every operation takes and returns members | M2: a member type beside `Slot`, used where a guarantee is needed |
|---|---|---|
| Surface | `adapt`, `add`, `mul`, `div`, `convert` return `Member<F>`; operands are `Member<F>`. | `Member<F>` is new. Existing signatures stay. `Divisor` gains one door. |
| Doors into `Member<F>` | A build-checked constant, and the declared adaptation. | The same two. |
| `Slot::at`, `Slot::index` | Stay, but stop being how an operand is written. | Unchanged in use. |
| Consumers | `arvo-numeral` mints a member per operand; the five suites and every in-crate sweep port. | None forced. The values lane adopts `Member` when it wants a runtime divisor. |
| What it buys beyond ticket 4 | Non-member operands become unwritable, which dissolves the saturation arms and the open item on `sum_position`. | Nothing beyond ticket 4. |
| Canon exposure | Decides the design's open item on refusing non-member slots, and leans on `question::is_admission_a_predicate_or_a_location` (open). | None. |

### Recommendation

B and M2.

B is what the design's own section on the exact step asks for, it closes both placeholder tickets completely, and it meets the brief's constraint without a port. A widens a coordinate for a width question the canon reserves. C pays a full surface change for a cost nobody has measured.

M2 closes ticket 4 inside the typestate. M1 is the stronger end state and is worth an idea ticket, but it decides two things the design holds open and forces the ports the brief asks to avoid.

### Rounds and order

Two rounds, not one. Each is a whole feature, they share no source line, and their gates differ: the first needs only a notice to the values lane, the second needs the values lane to agree a shape it will build on. Taking them as one would hold the first behind that agreement.

1. Round one, "the exact position takes the four limbs". Tickets 1 and 2, and the width 64 cell of ticket 3. Crates: `arvo-format` only.
2. Round two, "a member is a type, and the divisor takes one". Ticket 4. Crates: `arvo-format`, then `arvo-numeral` in the values lane's own round.
3. No round for the 65 and 128 cells. Ticket 3 stays open, narrowed to those cells, pointing at `question::one_word_or_two_for_is_a_number_system`.

If `coord` wants one landing, the two doc changelists touch disjoint sections and can share a round, at the cost named above.

### Round one

### Design text, section by section

- "The exact position, which is what the map takes". Replace the two paragraphs on the carry (pinning at the integer's end, the distance kept beside it) with: the position's floor is held in the four limbs the exact steps form it in, so it names every position a formed step or a carry can reach. `between` is total with no pinning. `slot()` answers the floor where it is a slot index and the index's end on the position's side where it is not.
- "One map with two regions" and "The index's host integer, and what every computation over a slot owes it". Rewrite the last two paragraphs of the second: rounding answers with the floor and a step, with no distance; completion compares the floor against the range's ends, wraps by reducing the floor less the lowest slot modulo the span, and saturates by the floor's side. State that a slot is still an `i128` and that this section's headroom arguments about sums and the reach are unchanged.
- "Where the exact step happens". Amend the last sentence of the fifth paragraph: the rounding intermediate is the four-limb floor and one bit.
- "The exact step's ending, shared". Remove "past an end of the slot index" and the "held" sentence. A step is formed or not; a formed step has a position.
- Multiplication, "What it refuses, and what is not yet admitted". Three conditions, not four. Delete the paragraph on the product the index cannot carry and the paragraph on where the 64-bit ranges part. Say `UFixed<64, k>` is admitted wherever the step is formed.
- Division and Conversion, the same subsection in each. Three conditions. Delete the carried paragraphs.
- The three "laws this section owes tests for" subsections. Replace the uncarried compile-fail pairs and the held-out counts with the tests below.
- "Landing a point", "The position at a magnitude". A floor past every range is a position the range question rejects; nothing is said to "leave the slot index".
- "Open items". Delete the first item. Delete the fourth (the `past` distance), since the field is gone. Rewrite the last so it no longer speaks of a position `between` cannot name.
- The canon declaration at the top needs no new row.

The design changes, so the code under those sections is rewritten from the new text, not adapted: `Exact`, `Rounded`, `round_slot`, `complete_slot`, `Placed`, and the carried conditions.

### Source, file by file

- `src/apply/position.rs`. `Exact` becomes a four-limb floor and a `Remainder`. `past` is removed. `between` adds the carry in the limbs. `is_negative` and `is_even` read the floor. A crate-private constructor takes a four-limb floor.
- `src/apply.rs`. `Rounded` is the floor and the step. `lands_within`, the wrap and the saturation are restated over the floor. `panic_on_overflow` follows.
- `src/placing.rs`. `Placed::Past` and `is_held` are deleted. `over` always places. `saturated` answers the limbs' end on the sign, and is reached only by a step not formed.
- `src/wide.rs`. One addition if needed: the parity of a value. Everything else exists (`ordering`, `plus`, `floor_divided`, `index`, `is_negative`).
- `src/multiplication.rs`, `src/division.rs`, `src/conversion.rs`. Remove the fourth assert, its `FIXME`, `product_is_carried`, `quotient_is_carried` with `extreme_divisors`, and `conversion_is_carried`. `conditions` returns three verdicts. Module docs follow.
- `src/apply/landing.rs`. `position_at` drops the `Past` arm.
- `src/slots.rs`, `src/addition.rs`, `src/symmetry/`, `src/standards/`. Untouched. The doc comment on `Slot` that points at "the checked and the saturating computations elsewhere" stays true for addition.
- `tests/ui/multiplication_over_an_uncarried_product_is_refused.rs` and its `.stderr`. Deleted.

### Tests

Each failing case lands before its fix.

1. `is_multipliable::<UFixed<64, 0>>()` asserted true, and the inverse round trip at width 64 through the test-only unforced step and `adapt`. Red today at the verdict, and at the value because `(2^64 - 1) * INV3` passes `2^127` and saturates. After the fix the test moves onto `mul` and joins the width list.
2. `mul` under `Wrap` at `UFixed<64, 0>` of the top slot by itself is slot 1. Red today.
3. `is_divisible` at `Grid<BinaryRationals, Constant<-64>, Signed<64>, 0, 1>` is true, and `MIN / -1` under `Wrap` is slot 0. Today the saturated position wraps to `-1`.
4. A conversion of `Integer<64>`'s top member onto an 8-bit target at exponent `-70` under `Wrap` is slot 0. Today the saturated position wraps to 255.
5. A quotient past `2^127` that is off the grid, under `HalfEven` at a tie and under `TowardZero` on each sign, against the `Big` oracle. This covers the parity and the sign of a four-limb floor.
6. The three edge sweeps compare the step against the oracle's exact position everywhere, with `Answer::Past` and `reported` removed, and assert the verdict equals "every corner is formed".
7. `a_position_past_the_index_saturates_on_its_sign` becomes an exactness test for non-member slots whose step is formed, and keeps its one case past the four limbs.
8. The existing carry suites (`the_carry_past_the_index`, `the_top_of_the_index`, `the_far_end_of_the_index`, `the_cross_end_saturation`, `totality`) are the regression net and must stay green through `between`. Their helpers that read `past` or build a pinned position are ported; the pinned constructor stays as the negative control.

Compile cases: `multiplication_over_the_unsigned_64_bit_range_is_admitted.rs`, a division case at `Constant<-64>` over `Signed<64>`, and a conversion case whose range end lands past `2^127`. Each binds the position function in a `const` and fails to build today. The remaining refusal pairs (a step the four limbs do not form, a magnitude-indexed quantum, a phase that does not denote) keep their twins. `multiplication_one_bit_narrower_is_admitted.rs` loses its twin and is folded into the new admitted case.

Mutants, each held as a construction beside `the_broken_maps` where it can be:

- For the applied map: a completion that narrows the floor to the `i128` end before wrapping. Tests 2, 3 and 4 and the carry law go red.
- For the product: a step that saturates when the floor leaves `i128`. Test 1 goes red at width 64.
- For rounding: a parity read from the low limb of the narrowed floor. Test 5 goes red.

### Round two

### Design text

- "The coordinates, and why they are types at all". Add `Member` to the table: a slot of a format's range, declared with `Format`. State its perimeter where it is stated: a private field, two doors, one accessor.
- A new subsection under the slot range section, "A member is a slot the type knows is in range". The two doors are `Member::<F>::constant::<B>()`, refused at build outside the range, and `Member::<F>::adapted::<S>(exact, dither)` for a signature over `F`, which is `adapt` and is in range by its totality. No door classifies a slot. Name what enforces the claim: the constant's assert and `apply/tests/totality.rs`.
- "The divisor is a type". Three doors. The new one, `Divisor::<F>::member::<G>(Member<G>)`, is admitted where `G` sits on `F`'s grid (same radix, same constant exponent, equal reduced phase) and no member of `G` is the zero. With `G = F` this is the ticket's own case. With `G` a zero-free part of the grid it is the values lane's. Remove the sentence calling the whole-phase restriction the compromise of the stance, and the ticket reference.
- Division's laws subsection. Add the pairs and the perimeter test below.
- The canon declaration. Add `ruling::the_ingest_door_waits_on_a_known_use` as the bound on `Member`'s doors.

### Source

- `src/format.rs` or a new `src/format/member.rs`: `Member<F>`, its two doors, `slot()`.
- `src/division.rs`: the third door, its obligation and a verdict function beside it. The `FIXME` at `NoSlotNamesTheZero` is removed; `Divisor::of` itself is unchanged.
- `src/lib.rs`: re-export `Member`.
- A sketch under `mock/research/sketches/` first, committed before the doc changelist locks, showing that the cross-format obligation and the const door compile on the pinned toolchain.

### Tests

Failing first:

1. `Divisor::member` over `Positive` (slots 1 to 255) from a member read at run time gives the oracle's quotient at every member pair.
2. Over `Unsigned<8>` with divisors from `Positive` on the same grid, the same.
3. The perimeter: every `Member` any door produces, across all modes and shipped policies and the totality suite's edge positions, is in range, and every `Divisor` built from one has a non-zero factor.
4. The verdict function on both sides of each condition: a range holding the zero, a different exponent, a different phase, a different radix.

Compile-fail pairs, each twin differing in one token:

- A member divisor over `Unsigned<8>` (zero in range) refused, over a range from 1 admitted.
- `Member::constant` one past the range's top refused, at the top admitted.
- A member of a grid at another exponent refused as a divisor, at the same exponent admitted.
- A `Slot` where a `Member` is asked for refused, the member admitted.

Mutants:

- The zero-free condition reads `F`'s range and not `G`'s. Test 2 goes red at the verdict.
- `Member::constant` without its range assert. The second refusal case compiles, so the pair goes red.
- The same-grid condition drops the exponent comparison. The third refusal case compiles.

### What to agree before each round locks

For round one, a notice. Text for `coord`:

> arvo-format, slot widening round: no signature changes. `Slot::at`, `Slot::index`, `Slots`, `adapt`, `add`, `mul`, `div`, `convert`, `Divisor`, `panic_on_overflow` and the three position functions keep their shapes. `Exact` keeps `on_grid`, `at`, `between`, `slot`, `is_on_grid`, `is_tie`; its size and private fields change. The fourth condition of the product, quotient and conversion obligations is removed, so `UFixed<64, k>` multiplies at `k >= 0` and narrow ranges divide at fine quanta. For arvo-values: `tests/ui/a_product_over_the_unsigned_64_bit_range_is_refused` will start compiling, and the DESIGN paragraph saying the unsigned 64-bit product is not admitted goes stale. `Exact::slot()` on a position past `i128` still answers the end on its side.

For round two, a shape to agree. Text for `coord` and the values lane:

> arvo-format proposes `Member<F>`: a slot in `F`'s range, private field, doors `Member::<F>::constant::<const B: i128>()` and `Member::<F>::adapted::<S>(Exact, Dither)` with `S: DeclaredSignature<Format = F>`, accessor `slot() -> Slot`. And `Divisor::<F>::member::<G>(Member<G>) -> Divisor<F>`, admitted at build where `G` has `F`'s radix, constant exponent and reduced phase and no member of `G` is the zero. A value over `F` is then divided by a value over a zero-free `G`, for example slots 1 to 255 beside `Unsigned<8>`. Two things to settle: whether the value tier can admit such a `G` (its `an_outside_slot_family_over_part_of_a_range_is_refused` case suggests not today), and whether getting from a `Num` over `G` to `Member<G>` through `adapted` on a grid position is acceptable, since arvo-format will not ship an unchecked door. A door that tests a runtime slot for zero is not offered; it needs op's word under `ruling::the_ingest_door_waits_on_a_known_use`.

### Tickets to file

None were filed; this seat wrote nothing.

- Narrow `arvo_law_the_multiplicative_inverse_round_trip_is_unreachabl` to the 65 and 128 cells and relate it to `question::one_word_or_two_for_is_a_number_system` and to `arvo_binary128_and_decimal128_need_a_slot_ladder_past_64_bit`.
- A task: addition's "sum the slot coordinate cannot carry" refusal and `sum_position`'s saturation for non-member slots are the same class as tickets 1 and 2 and could ride the four-limb floor. Left out because `Reach` publishes positions as `Slot` and the associativity verdict leans on the `2^126` bound.
- A task: measure `adapt` before and after round one in `mock/benches`, on `add` and on a fixed-point `mul`, before anyone states what the change cost.
- An idea: operations take and return members (shape M1).
- A question with op as holder, only if a consumer shows the use: a classifying runtime door for a non-zero divisor.

### Assumptions not checked

- That "formed" implies "carried" under a four-limb floor is argued above from `Wide::floor_divided`'s own bound, not compiled. Round one's first commit is where it is tested.
- That `complete_slot` over a four-limb floor reproduces today's answers at every position the carry suites feed. The suites decide it; six of those files were read by name only.
- The integration suites were not run at the tip, so "no standards suite needs a port" rests on reading their imports of `adapt`, `land`, `Slot` and `Exact`, none of which change shape.
- That no consumer outside this repository reads `Exact::slot()` expecting a pinned end with a separate distance. Only `tests/the_ties_away_alias.rs` and `tests/ties_away/mod.rs` were found, and they read the sign.
- What `arvo-numeral` admits as a slot family was read from file names and its design text on `origin/work/arvo-values` at `2b413b50`, not from `admit.rs` in full.
- The cost on the `adapt` path is unmeasured for every shape.
- The ticket texts are as quoted in the brief; the tracker was not queried.
- The canon was searched by scanning `mock/registry/*.toml` for terms after `cargo mock query '<ns>.where(keywords~term)'` returned no rows for any term, and each row cited was then read through `cargo mock query '<ns>::<slug>'`. A row using none of the scanned words would have been missed.
- `Member` as a name covers the slot coordinate only. For a magnitude-indexed format a member also has a magnitude, and whether the name should say so is left to the round's topic.
