# The value stance, a second read from the rows

A second read of what the ratified rows say about the value a format holds, made from the rows before any other reading of them was opened. The first part answers four questions with the rows quoted. The second part sets the read against the stance file, the third classifies every question with no ruling edge, and the fourth names the proposal rows filed beside this note and the fate of each crate of the old tree.

Rows read, at `dev` `d841c5e1`: `ruling::the_format_spine_is_canon` (`ruling.toml:1436`), `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` (`ruling.toml:1459`), `ruling::a_min_plus_top_absorbs_addition_under_saturation_and_never_under_wrapping` (`ruling.toml:1786`), `ruling::a_format_concept_closed_over_v_cross_v_to_v_cannot_state_a_composite_claim` (`ruling.toml:1824`, the brief said `:1823`, which is its `[[ruling]]` header), `question::what_a_datum_stands_for` (`question.toml:93`), `obligation::a_platform_sized_unsigned_integer_at_an_api_position` (`obligation.toml:82`), `obligation::a_primitive_for_every_position_a_bare_number_would_take` (`obligation.toml:147`). Where a ruling ratifies a proposal, the proposal's own `says` was read as the ratified text, since the ruling's `ratifies` edge carries it.

## Part one: four questions

### (a) Is a datum of a format one point of the ambient domain

Yes for a format's datum, and the rows decide it to that extent. The question row itself is not answered by them, since none carries an `answers` edge to it.

The spine:

> A format is identified by its ambient domain and its representable set, and that set is a constant of the type. Membership in it is one affine predicate over one parameterisation, of which integers, fixed point, scaled integers and floats are points.

A datum of a format is a member of its representable set, and the set is a subset of the ambient domain, so a datum is one point of it. The ratified proposal under the spine (`proposal.toml:72`) adds: "a value set that depends on other data is not a format but storage."

The fourth option of `what_a_datum_stands_for`, a set admitted generally, is excluded for formats by `ruling.toml:1824`:

> Compositions over formats, stored-pair rationals, intervals, error-carrying values, are not format instances; they consume the format concept and owe their own laws.

So a set-valued datum is a composition's datum and never a format's.

The second option, an absorbing top denoting everything above it, is not what the min-plus ruling says. That ruling states absorption as a law over representable operands: "addition's saturating top absorbs: `top + x = top` for every representable `x`", and calls it "the absorption half only of what a min-plus computation needs from the numeral". The spine puts saturation where the second option would put a denotation, on the adaptation: "Arithmetic on a format is an exact operation in the ambient domain composed with a named total adaptation onto that set, and the adaptation is a first-class object with its own laws." Many exact results mapping to one top is a property of the adaptation's preimages, and the top itself stays one point. That is the first option's wording, "saturation's absorbing behaviour is a documented restriction on where a fold is sound rather than a denotation."

The third option, a constructor-level clause, is not excluded by any of these rows. The rows do not decide it, and the question stays open for the panel, which is its declared `decider`.

### (b) Do these rows license a type that holds one value at sole occupancy

Yes. `ruling.toml:1459`:

> The footprint is nonetheless observable, through exactly one observation the design does not own, the host language's layout observation on a sized type, and that observation exists only where the value is the sole logical occupant of its allocation. At a shared placement no per-element footprint exists to observe.

The ruling names sole occupancy as one of two placements and states what holds there, which presupposes a sized type holding one value. Its ratified `at_shared_occupancy_no_per_element_footprint_observation_exists` (`proposal-the-later-topics.toml:765`) makes placement the distinction and not the primitive: "two selections differing only in the carrier over one packed column are one primitive with two placements rather than two primitives."

The two obligations require it. `obligation.toml:82` asks for "an unsigned integer of the platform's size, usable in a public error type", a field of a fixed-shape enum, which is a sole occupant. `obligation.toml:147` asks for "a primitive of arvo's own for every public API position across the stack where a bare integer, float, bool or platform-sized index would otherwise sit"; every such position (an argument, a return, a field) holds one value at sole occupancy.

The licence carries one condition from the same ruling: "every operation the design declares is a function of the declared width and never of the machine carrier". The type may own a carrier, and none of its operations may read it.

### (c) Is ingest of a host value the declared adaptation

Yes, by derivation, and the rows do not name ingest. The spine factors arithmetic, not conversion, but a host value of an integer or a float is a point of the ambient domain (the spine: "integers, fixed point, scaled integers and floats are points"), and the adaptation is total onto the representable set. Ingest is then the identity as the exact operation composed with the format's adaptation, and the factoring covers it with no new statement. The ratified proposal at `proposal.toml:56` closes the alternative: "an operation that fuses one invisibly is an operation and an adaptation point wearing one name, and the model's statements apply to it only when the two are separated." An ingest that clamped or wrapped by a rule of its own would be that fused case.

The rows do not decide what ingest does with a host value that is not a point of the ambient domain, a float's NaN or infinity among them.

### (d) Does the row at `:1824` license a named exact intermediate beside the value type

It licenses a name for the exact intermediate, and it does not decide that the name is an accumulator type, nor that type's width. `ruling.toml:1824`:

> A concept whose operations have signature `V x V -> V` has no name for the intermediate a composite claim quantifies over, so a claim about a composite (that `rho(f(g(x, y), z))` equals something) has no expressible form against a concept closed that way. This is the statability clause only. The companion list, that the format concept supplies the layers above with the width algebra of exact results, the named adaptation, and the exactness predicate, is not promoted by this ruling and stays at one expert.

The clause is negative: closed over `V x V -> V`, a composite claim cannot be stated. With the spine, which makes the exact operation and the adaptation two first-class stages, the intermediate is the exact operation's result before `rho`, and stating any composite claim needs it named. That much is licensed.

What the name is owed to carry is the companion list, the width algebra of exact results above all, and the same row says that list "is not promoted by this ruling and stays at one expert". So the rows do not decide that the intermediate is a type of its own with a declared width beside the value type, and they do not decide its width. A proposal may state it; a ruling may not rest on `:1824` alone for it.

Read beside the spine at `ruling.toml:1436`, as the coordinator's correction asked, the licence is stronger and its limit is the same. The spine says "the adaptation is a first-class object with its own laws". A first-class adaptation is a map with a domain, and its domain is the exact results in the ambient domain, so the spine alone already needs the adaptation's input expressible; head's `Exact` (`arvo-format/src/apply/position.rs:158`) is that input, unbounded and carried as a slot and a remainder. What neither row says is that the input has a finite register per declared signature, which is what an accumulator type beside the value is. The ambient domain the spine names has no width, so a width for the intermediate is a realisation, and the rows do not decide it.

The questions of part one reached this read already worded toward their answers: (b) as "a type that holds one value at sole occupancy", (d) as "a named exact intermediate (an accumulator type)", and part four's list as "a datum is a point; a value is the sole occupancy placement of a declared signature; ingest is adaptation". Each answer above is argued from the rows quoted, and where the rows stop short of the wording, (d) above all, the answer says so rather than taking the wording.

### Status, set against the stance file after the above was written

On (b), CONVERGED. Section 1.5 of the stance file reaches the value type from the same four rulings and the same two obligations, and its item 4 is this read's first quotation. It goes further than the rows in one sentence, "A value type whose representation is the sole occupancy output of the placement derivation is the first placement, used", which this read reaches through the ratified `at_shared_occupancy_no_per_element_footprint_observation_exists` ("one primitive with two placements") rather than by assertion.

On (d), CONVERGED with the stance file and with the roadmap's first read of call 10, and against the reading recorded there as K6. Section 1.5 is silent on (d). Elsewhere the stance file uses head's `Exact` (`arvo-format/src/apply/position.rs:158`) as a named exact position at its ingest door and keeps the double-width register of a lane "an internal arm and not a declared coordinate" (its section 3.5), and neither claims the rows owe a declared accumulator. The roadmap's member reads `:1824` as "ratifies only that a `V x V -> V` concept cannot name the intermediate". K6 reads the same row as saying "the format concept supplies the width algebra, so the value crate owes the accumulator", and the row says the opposite of that in its own words: the list naming the width algebra "is not promoted by this ruling and stays at one expert". So an accumulator type is licensed, as a name the rows make necessary for a composite claim and nothing forbids, and it is not owed by them; its width algebra is a proposal's to state.

### A second read of call 4: does a debug build abort when a declared-saturating operation saturates

Added on the coordinator's correction. The stance file's call 4 puts three options: (a) yes, always; (b) yes by default, and a consumer may declare in the type that completion is the meaning, which turns the assertion off for that type; (c) no, an explicitly declared policy is never an error and only presets imitating native integers assert.

`ruling.toml:949`, `never_a_runtime_check_and_one_lowered_path`, ratified:

> Runtime code can exist. Its branching should be done as much as possible with const-time ifs erased by monomorphisation, const solving and ultimately the backend. Never any runtime checks, ever: invalids are caught at compile time and unused paths are cleared when lowered. Unless there is a justifiable reason that is blessed by a lead designer.

`ruling.toml:1052`, `the_overflow_panic_is_permitted_and_bounded`, ratified:

> Panic is the go-to for what cannot be caught at compile time through the typestate, on debug builds, in every strategy rather than only where a native primitive is being imitated. On a release build pretty much nothing panics, because the typestate and the debug panics are what force soundness in, so the panic is not needed there. What stands in its place on release is the guarantee plus an explicit declaration of the mode, saturate or wrap or whatever applies to the overflow or underflow at hand, which then lowers and behaves accordingly. None of this is black and white and the intent is deliberately loose.

Its ratification quotes op: "Idea is to make things forced sound, which means usually panic over saturation or overflow", "even on hot", and "it should be ratified as ambiguously as I've stated it here, perhaps this verbatim to start with, and sharpen as we find the boundaries and real constraints to this and can name them precisely".

`ruling.toml:1713`, `the_panel_finishes_the_canon_without_him`, ratified by op:

> Every remaining canon question is the panel's. [...] His absence is not a licence to invent: silence in his corpus is still not permission, and where his words genuinely do not reach a question the answer is derived from the intent inside its spirit and put through two independent agreements. Nothing is parked awaiting him.

What they demand. The debug panic is the one runtime check the first row's exception allows, blessed by the lead designer in the second, and bounded to debug builds. It covers every strategy, so (c) is refused by the row's own words: the explicit declaration of the mode is what "stands in its place on release", a release substitute and not a debug exemption. The scope of the panic is "what cannot be caught at compile time through the typestate", and its purpose is soundness forced in.

What it means for a consumer that saturates by design. Kaski's proxy says saturation is its intended semantics and not its error path. Under (a), its debug runs abort the first time a quantity reaches the end of its range, which is a state its design reaches on purpose, so a debug run cannot exercise the design at all; the release bits do not change. Under (b), the consumer writes in the type that reaching the end is the meaning. That declaration is typestate, so the saturation is no longer something "that cannot be caught at compile time through the typestate": the type has already said what reaching the end means and nothing is left unsound for the panic to catch. So (b) is derivable from the first clause of `:1052` rather than an exception to it, and its declaration has to live in the type, never in a build flag or a runtime switch, which `:949` refuses.

The rows do not decide between (a) and (b) in words. "Usually panic over saturation" and "even on hot" lean to (a); "what cannot be caught at compile time through the typestate", "deliberately loose" and "sharpen as we find the boundaries and real constraints" leave (b) inside the row, and a consumer whose design saturates on purpose is such a constraint found. This read takes (b), with the default asserting in every strategy as the row demands. Under `:1713` it is one agreement of the two the panel owes before it stands.

## Part two: the stance file's ten stances against the rows

Each stance of the stance file's section 2 is set against the registry. Agree means a row says what the stance says or what it rests on; disagree names the row that says otherwise; silent means no row decides it and the stance is a design call.

- A, a value is the slot index of a member at sole occupancy in the derived carrier. Agree. `ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves` states sole occupancy as the placement where the footprint is observable, and part one (b) reads it as licensing the type. Which ladder rung a width takes is silent in the rows; the stance takes it from head's code, which is the right tier for it.
- B, the law of every operation is equality with the oracle. Agree. It is `ruling::the_format_spine_is_canon`'s factoring used as a test: "an exact operation in the ambient domain composed with a named total adaptation onto that set". The 64-bit bound it accepts is `question::arbitrary_width_demands_in_the_canon`, open, and the stance says so.
- C, rounding and overflow are written by whoever stores the bytes. Agree. `ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names` fixes the six names, `ruling::half_up_denotes_ties_toward_positive_infinity` the tie, and the spine's adaptation is "named" and per signature. Leaving `Clamp` off values agrees with `ruling::never_a_runtime_check_and_one_lowered_path` as well as with head's own comment, since a clamp with no bound to carry is saturation under another name.
- D, presets ship as bindings with their cells unsettled. Agree, and it leaves `ruling::wrap_or_clamp_stays_open_and_both_get_priced` standing as the brief requires. Its finding that `Warm` at `HalfEven` would give `7 / 2 = 4` meets `ruling::warm_behaves_as_a_native_rust_primitive_would`, which `awaiting-a-ruling` reports retired by `warms_objective_is_the_intuitive_best_choice`; the stance cites the retired row and the successor carries the same intent, so the finding stands on the successor.
- E, one signature per operation, crossing formats by explicit conversion. Silent. `question::mixed_numeral_addition` (Q3) is open with no ruling edge, and the stance takes its third option as a default that a later answer extends without breaking anything written against it.
- F, a debug build asserts on overflow, and a consumer may declare completion as the meaning. Agree, read through the row's scope. `ruling::the_overflow_panic_is_permitted_and_bounded` says "on debug builds, in every strategy" and its ratification "usually panic over saturation", which a type-level opt-out departs from in the letter; the same row scopes the panic to "what cannot be caught at compile time through the typestate", and a type declaring completion as its meaning is caught there. The second read of call 4 above argues it. The stance marks the call op's; under `ruling::the_panel_finishes_the_canon_without_him` it is the panel's, at two agreements.
- G, fixed-point aliases take a fraction length. Agree. `question::which_width_coordinates_a_consumer_writes` (Q2) carries an `answered` field putting the written pair in the design round, so no row is owed.
- H, const generic parameters are bare integers. Agree. `obligation::a_primitive_for_every_position_a_bare_number_would_take` quotes the exception in op's words, and `obligation::the_unstable_machinery_does_not_reach_a_consumer` is the reason `adt_const_params` leaves the consumer.
- I, division takes a divisor that is non-zero by type. Agree with `ruling::never_a_runtime_check_and_one_lowered_path`. The pole policy it sets aside is the reading `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`'s admission clause ("where two realisations of one name disagree, the signature is missing a coordinate") would suggest, and the stance names it as additive later.
- J, capacity its own crate, masks to graph in slice two, no facade. Agree on order with `ruling::arvo_is_a_library_and_the_value_composes_on_top` ("The numeral base has to work so the bigger things can be built on it") and `ruling::the_next_unit_is_chosen_bottom_up`. Which crate holds capacity, and whether a facade exists, are silent.

## Part three: the census of questions with no ruling edge

`question.toml` holds 110 rows. Eighteen are named by a ruling's `answers`, so 92 have no ruling edge. Six of the 92 carry an `answered` field written in op's stead, which is where the earlier count of 86 came from: 92 less those six. `cargo mock unasked-questions` reports none of the 92 waiting on op ("Every question that names him has an answering ruling"). Twenty-nine of the 92 are named by a proposal's `answers`.

Classes: slice the first slice depends on it; no it does not; second read a proposal answers it and a second read is owed. The first slice is the stance file's section 3: the value type at sole occupancy, its doors, `add` `sub` `neg` `mul` `div` `convert`, compare, identities, the integer, fixed, index, count and boolean aliases, capacity, presets as bindings, and the exact accumulator of the roadmap's AR7a. A question a proposal answers is classed second read even where the slice also leans on it, and that is said on its line.

- `which_width_coordinates_a_consumer_writes` (Q2): second read; the slice leans on it for its alias spelling, and the `answered` field already sends the spelling to the design round.
- `mixed_numeral_addition` (Q3): no; the slice spells no cross-numeral addition and any answer is additive (stance E).
- `what_a_datum_stands_for` (Q4): slice; answered for formats in part one and proposed in part four.
- `arithmetic_column_one_axis_or_two` (Q5): no; the slice keys values on format and `Adapt<R, O>` and leaves preset cells unsettled.
- `does_warm_wrap_or_clamp` (Q6): no; deferred by `ruling::wrap_or_clamp_stays_open_and_both_get_priced`, presets ship unsettled.
- `which_carrier_the_packing_claim_is_about` (Q7): no; deferred by `ruling::the_carrier_question_waits_on_the_contention_measurement`, the slice ships sole occupancy only.
- `one_numeral_family_or_several` (Q8): slice; one `Num<S>` over every slot family presumes one family, which the spine's closed concept implies and no row states.
- `the_width_surface_crossing` (Q9): slice; a written width crosses into the type as a bare const generic under stance H.
- `inclusion_order_singleton_amendment` (Q10): no.
- `what_a_numeral_guarantees_to_a_fold` (Q11): second read; the exact accumulator leans on it.
- `reduction_order_or_associativity` (Q12): second read.
- `which_axes_a_build_arm_may_move` (Q13): second read.
- `are_the_axes_independently_resolvable` (Q15): no.
- `which_sense_composition_carries` (Q16): no.
- `does_the_canon_carry_a_numeric_threshold`: no.
- `is_the_derived_numeral_required_to_be_tightest`: no; the slice derives no numeral.
- `does_precision_count_the_sign_digit`: no; the slice writes total width and fraction length, never precision.
- `does_narrowing_compose`: second read; the slice states one conversion step and nothing about two.
- `is_the_cross_kind_join_closed_or_priced`: no.
- `should_phase_collide_across_two_vocabularies`: no.
- `where_wrapping_lives`: second read.
- `adaptation_in_identity_or_realisation` (Q18): second read; the slice puts the adaptation in the signature, which the `answered` field already reads.
- `are_the_level_hierarchies_the_same_cut` (Q19): second read.
- `is_the_number_system_inventory_open` (Q20): second read.
- `is_number_system_broad_enough_for_non_magnitude` (Q21): second read.
- `are_set_valued_carriers_admitted` (Q22): no; part one places set-valued data as compositions above the format, and the slice ships none.
- `is_the_role_set_closed` (Q23): no.
- `does_the_canon_speak_about_cost` (Q24): second read.
- `how_the_law_inventory_is_named` (Q25): second read.
- `is_interoperation_conversion_or_resolution` (Q27): second read; the slice's `convert` is conversion.
- `arbitrary_width_demands_in_the_canon` (Q28): no; the slice is bounded at 64 declared bits and says so.
- `what_the_admission_contract_asks_a_candidate_to_expose` (Q29): second read.
- `is_admission_a_predicate_or_a_location` (Q30): second read.
- `one_word_or_two_for_is_a_number_system` (Q31): second read.
- `is_the_ambient_operation_family_fixed` (Q33): second read; the slice's operations are addition and multiplication and their inverses.
- `two_shapes_of_aggregate_composition` (Q34): no.
- `the_ownership_key_as_a_structural_axis` (Q35): second read.
- `whose_reduction_governs_a_lossy_crossing` (Q36): slice; `convert::<T>` adapts onto `T` with `T`'s declared adaptation, which takes one answer.
- `does_the_canon_name_crossing_classes` (Q37): second read.
- `when_is_an_order_owed_at_a_crossing` (Q37): second read.
- `where_a_law_verdict_is_established` (Q38): second read; the slice's L8 exposes associativity as a const verdict.
- `may_an_arms_predicate_read_data` (Q39): second read.
- `which_route_a_law_verdict_takes_to_closed_form` (Q40): second read.
- `are_strategies_partially_ordered_by_chain_laws` (Q41): second read.
- `which_reassociated_arm_a_law_licenses` (Q42): no.
- `what_goes_into_the_compiler_winner_or_cost_table` (Q43): second read.
- `must_a_weighting_be_strictly_positive` (Q44): no.
- `arms_no_weighting_can_select` (Q45): no.
- `what_the_cost_coordinates_are` (Q48): second read.
- `is_i9_about_a_strategy_or_one_component` (Q50): second read.
- `what_transfers_from_a_model_width` (Q53): second read.
- `are_consumer_terms_trees_or_dags` (Q54): no.
- `what_the_double_rounding_mechanism_is` (Q57): no; one conversion step only.
- `is_the_firewall_carried_unpredicated` (Q62): no.
- `why_the_default_rounding_position_is_chosen`: no; the slice has no default rounding, every signature writes one.
- `which_units_a_weighting_is_expressed_in`: no.
- `chains_and_ops_as_two_things_or_one_phrase`: no.
- `which_chain_carrier_ships`: no; the accumulator is a per-signature exact intermediate, not a chain carrier.
- `chain_or_region_between_observations`: no.
- `does_the_position_keyed_dither_arm_ship`: no; `Stochastic` is not shipped on values.
- `does_a_consumer_supplied_seed_surface_exist`: no; same.
- `is_the_non_terminating_contention_crate_a_defect`: no.
- `are_open_domain_dimensions_stated_as_open`: no.
- `does_a_named_policy_selection_survive_above_the_numeral`: no.
- `which_closed_panel_statements_the_absolute_framing_carve_out_reaches`: no.
- `is_root_check_cost_stated_within_a_behaviour_or_across_a_design`: no.
- `is_w1bs_theorem_stated_on_its_argument_or_its_sweep`: no.
- `does_the_order_based_realisation_family_have_more_than_one_member`: no.
- `are_a_consumers_derivations_trees_or_dags`: no.
- `does_the_rounding_variance_form_hold_at_a_second_fraction_width`: no.
- `do_arvos_consumers_want_value_keying_or_position_keying`: no; neither keyed member ships in the slice.
- `does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members`: no.
- `which_selector_does_the_design_ship_linear_or_augmented_chebyshev`: no.
- `is_the_accumulator_cell_stated_conservatively_or_with_a_schedule_dimension`: no; it is the strategy object's cell, not the value-level accumulator.
- `is_the_dominance_gate_symmetric_or_conservative`: no.
- `is_a_profitability_predicate_resolved_pessimistically_when_not_const_available`: no.
- `is_the_strategy_weighting_objective_linear_or_log_scale`: no.
- `which_strategys_laws_govern_a_cross_strategy_operation` (Q66): second read.
- `which_repair_the_refuted_kleene_identification_takes`: no.
- `which_successor_carries_the_strategy_shape_ratification`: no; presets ship unsettled.
- `whether_a_law_layers_consumer_is_the_lowering_or_one_of_several`: no.
- `what_the_ingest_row_claims_and_what_turns_on_it`: slice; the `adapted` door is the ingest, and part one (c) and the ingest proposal of part four bear on what `ruling::ingest_is_the_consumers_and_the_c_abi_is_where_it_ends_up` claims.
- `what_stops_two_files_by_one_persona_reading_as_two_experts`: no.
- `can_a_claim_about_the_canons_own_structure_carry_a_region`: no.
- `is_the_rounding_vocabulary_complete_at_six`: no; a seventh name is additive to the slice's rounding parameter.
- `which_of_the_six_a_retired_word_names_on_each_row`: no.
- `which_tie_direction_an_unqualified_nearest_names`: no; the slice spells `half_up` and `half_even`, both settled.
- `what_region_does_a_predicate_naming_no_mode_state`: no.
- `how_the_stamped_spine_rows_phase_clause_gets_corrected` (Q67): slice; the identities exist "only where the format has one", and the phase clause is what says when it does.
- `where_the_standing_vocabulary_is_declared`: no.
- `does_a_cold_brief_withhold_rows_the_proposal_author_wrote`: no.
- `did_the_admitted_obligations_answer_the_exposure_question_in_the_design`: slice; the slice's `Sole` contract carries an `ADMITTED` line, which is the exposure the question is about.

Totals: slice 7, no 56, second read 29.

## Part four: the proposals filed beside this note

The rows are in `mock/registry/proposal-the-value-stance.toml`, each at `one_expert` and none a ruling. Promotion is the coordinator's gate under `ruling::the_panel_finishes_the_canon_without_him`. One obligation row joins `obligation.toml` for kaski, since the demand side named hilavitkutin and never kaski.

### The value stance

- `a_datum_of_a_format_is_one_point_of_its_ambient_domain` answers `question::what_a_datum_stands_for` with its first option, for formats, from part one (a). Set-valued data is a composition's, and saturation's many-to-one is the adaptation's, not the datum's.
- `a_value_is_the_sole_occupancy_placement_of_a_declared_signature` is part one (b) as a definition: a value of a declared signature is that signature's sole occupancy placement, one member's slot in the carrier the placement derives, and every operation on it reads the declared width only.
- `ingest_of_a_host_value_is_the_declared_adaptation` is part one (c): a host value enters through the format's own declared adaptation, with the identity as the exact step, and no ingest carries a rounding or overflow rule of its own.
- `the_exact_intermediate_is_named_beside_the_value` is part one (d): (d) holds as far as the rows take it, so the accumulator is proposed as a name for the exact intermediate, the exact step's result before the adaptation, with its own operations and the adaptation as its one way back to a value. Its width algebra is stated as the proposal's own and not as the row's.
- `the_first_slice_answers_to_hilavitkutin_and_kaski` names the two consumers whose stated needs the slice is measured against, with the obligation edges.

### Kaski's lane

`obligation::a_deterministic_fixed_point_lane_with_declared_rounding` states kaski's need: fixed-point arithmetic whose rounding the consumer declares, half to even on the product for kaski today, computing the same bits on every target the simulation runs on, native and wasm32 among them, because replay and the digest are compared across them. Its quotes are kaski's own canon rows.

### The old tree, crate by crate

Read at `1b2796e92ad2b8176c2177d72a9cffa161b7b92e` with `git show 1b2796e9:mock/crates/<crate>/src/lib.rs`; the fetch worked. Import lines are `use <crate>` statements under `hilavitkutin/mock/crates` at hilavitkutin `dev` `af823b7`, counted per statement, so one `use arvo::{..}` over several names is one line. The facade crate `arvo` re-exported `arvo-storage`, `arvo-strategy`, `arvo-numeric-contracts` and `arvo-transparent`, so those four have no direct import and are reached through its 243.

#### arvo

Exported the facade: `Uint`, `Int`, `Fixed`, `Signed`, `UFixed`, `IFixed`, `Float`, `FastFloat`, `StrictFloat`, the shape markers and the re-exports below. 243 import lines; by name, `arvo::USize` 185, `arvo::strategy::*` 55, `arvo::Bool` 20 and its constants 9, `Uint` 3, `UFixed` 4. Returns in slice one changed: the numerals as the value crate's aliases, and no facade (stance J). Floats are not in slice one and return changed in slice two.

#### arvo-bitmask

Exported `Mask`, `BitMatrix`, `NodeId`, `SetBitsIter`, `propagate_dirty`, `cap_size`. 22 import lines (`BitAccess` 7, `NodeId` 7, `Mask` 2). Returns changed in slice two, first of it, as masks over `arvo-bits` and the capacity crate.

#### arvo-bits-contracts

Exported `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`, `IBitPrim`, `UBitContainer`, `IBitContainer`, `Widen`, `Narrow`, `NarrowFromU64`. 0 import lines. Gone: the bit contracts fold into `arvo-bits` at head, and widening and narrowing of a numeral is conversion, which is adaptation.

#### arvo-bits

Exported `Bits` with a strategy parameter and the aliases `Bit`, `Nibble`, `Byte`, `Word`, `DWord`, `QWord`. 10 import lines. Returns in slice one, as head's `Bits<N>` with no strategy parameter and `.cast::<M>()` for `narrow_to` and `Widen`.

#### arvo-comb

Exported `bin_pack`, `matrix_chain_dp`, `greedy_group`, `Range`. 0 import lines; named in hilavitkutin's manifest and design only. Gone until a consumer states a need.

#### arvo-graph

Exported `components`, `longest_path`, `downward_rank`, `upward_rank`, `spanning_tree`, `topo_sort`, `renumber`, `waist_detect`, `waist_detect_const`. 2 import lines. Returns changed in slice two, over the value crate's index type.

#### arvo-hash

Exported `ConstHash`, `Hasher`, `fnv1a_64`, `Fnv1a`, `XxHash3`, `xxhash3_64`, `ContentHash`. 4 import lines. Returns changed in slice two, its width named by the consumer per `obligation::a_content_hash`.

#### arvo-mask-contracts

Exported `MaskOps`. 0 import lines. Gone as a crate of its own; the contract goes with masks in slice two.

#### arvo-numeric-contracts

Exported `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, and the predicates `IsZero`, `IsPositive`, `IsNonZero`, `IsNonNegative`, `IsZeroOrPositive`. 0 direct import lines, `FromConstant` reached twice through the facade. Gone: compare and the `constant` door are the value crate's own operations, the non-zero predicate becomes the refined divisor of stance I, and `Sqrt`, `Abs` and `Recip` wait for the elementary functions, which stay in kaski in slice one.

#### arvo-refit

Exported `Widen`, `Widened`, `Narrow`, `Narrowed`, re-exported from the bit contracts. 2 import lines. Gone: a refit between numerals is `convert`, one adaptation onto the target.

#### arvo-sparse

Exported `Csr`, `CsrBidirectional`, `block_diagonal`, `rcm_reorder`, the adjacency and Dulmage-Mendelsohn functions. 4 import lines. Returns changed in slice two.

#### arvo-spectral

Exported `fiedler_vector`, `laplacian`, `Matrix`, `LinearOperator`, `SparseLaplacian`, `spectral_bisection`, `k_way_partition`, `power_iteration`. 2 import lines. Returns changed in slice two, last of it.

#### arvo-storage

Exported `Bits`, `USize`, `NUSize`, `Bool`, `AsBool`, `BoolResidual`, `Cap`, `FBits`, `IBits`, `MetaCarrier`, `Width`, `fbits`, `ibits`, `width`, and the bridges. 0 direct import lines; `USize`, `Bool` and `Cap` are the most imported names in hilavitkutin through the facade. Gone as a crate: `Bits` is `arvo-bits`, `USize` and `Bool` are value aliases, `Cap` is the capacity crate, and the width wrappers go with stance H.

#### arvo-strategy

Exported the strategy markers (`Hot`, `Warm`, `Cold` and the rest), the identity and arithmetic markers (`Identity`, `Additive`, `IArith`, `UArith` and the saturating twins), alignment, `WideBits` and the width helpers. 0 direct import lines, 55 through the facade. Returns in slice one changed: head's `arvo-strategy` keeps the presets as bindings over a signature, and the identities move to the value crate.

#### arvo-tensor

Exported `Capacity`, `ConstCapacity`, `Dim`, `cap`, `cap_size`, `Array`, `Matrix`, `Enumerator`. 60 import lines (`Capacity` 49, `cap_size` 21, `Dim` 17, `ConstCapacity` 9, `cap` 5 by name). Returns changed in slice one as the capacity crate, a count as typestate with its array projection; `Array`, `Matrix` and `Enumerator` return changed in slice two.

#### arvo-transparent

Exported `NumericPrimitive`, `Transparent`, `ConstDeref`, `ConstAsRef`, `raw`. 0 import lines. Gone: the value type's unwrap doors are declared on it, so a general transparency contract has nothing left to carry.
