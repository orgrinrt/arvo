# What Wrapping Means on a Union of Magnitude Ranges

This is a read of one question against arvo's registry, made in the worktree `/home/user/arvo` on branch `work/arvo-registry-clean`, read-only. The question comes from three placeholders in `mock/crates/arvo-format`: the sum, negation and difference positions, the product position, and the quotient and conversion positions each refuse a magnitude-indexed quantum, and their tickets name two blockers, one in the design (the exact position and `adapt` carry no magnitude coordinate) and one in the canon (no row says what wrapping means on a union of magnitude ranges). This read is about the second blocker only.

The answer in one paragraph. The canon fixes what wrapping is on a representable set that is one arithmetic progression, and there it fixes it uniquely: reduction modulo the range, the one completion that is a translation congruence on the format's own grid. A magnitude-indexed format whose union of shells is not one progression has no such grid past its range, and at least three determinate completions that could carry the name disagree on a single exact sum. Under the ratified admission rule that disagreement means the name is missing a coordinate, so a signature naming wrap over such a set does not denote one map and is not admitted. What the signature can name instead is saturation or clamp, a conversion onto a constant-quantum format followed by wrap there, or a new member of the open overflow inventory that declares its extension grid and its modulus and carries its own name and its own laws. The region this holds over cannot be written as a registry predicate today, because no declared dimension indexes the quantum law or the shape of the representable set, so the proposed row is a definition with a refusal, and it carries no region.

A side finding changes where the placeholders should sit. The three positions are exact steps and read no overflow policy, so wrapping is not their question at all; it is the adaptation's. Once the design half lands, the positions have no canon blocker, and the wrap refusal attaches to a declared signature that names wrap over a non-progression set.

## 1. The shape of the three positions, as code

This section is about code, not design, and claims nothing about correctness.

The code does this. `mock/crates/arvo-format/src/addition.rs` declares `Addable::<F>::ADMITTED`, which asserts three things in order: the phase denotes, `is_constant_family::<F::Quantum>()` holds, and the sum is carried by the slot coordinate. `sum_position`, `negation_position` (in `addition/subtraction.rs`) and `difference_position` force that constant and return an `Exact`, a slot plus a remainder. `product_position` in `multiplication.rs` and `quotient_position` in `division.rs`, and the conversion in `conversion.rs`, carry the same quantum assertion under their own FIXMEs. None of these positions reads `F`'s overflow policy; the policy enters at `adapt::<S>`, where `S` is a `DeclaredSignature`.

`is_constant_family` in `quantum.rs` is `Q::SLOPE.power() == Exponent::ZERO.power()`. It reads the slope coordinate and not the set the coordinates denote, and it does not read the magnitude count. That matters in section 4.

## 2. The rows read, and the sentence each one is leaned on for

Ratified, and governing:

- `ruling::the_format_spine_is_canon` (ratified, both). "A format is identified by its ambient domain and its representable set, and that set is a constant of the type. Membership in it is one affine predicate over one parameterisation, of which integers, fixed point, scaled integers and floats are points. Arithmetic on a format is an exact operation in the ambient domain composed with a named total adaptation onto that set, and the adaptation is a first-class object with its own laws. The concept is closed and the inventory of admitted instances is open." It ratifies `proposal::a_format_is_identified_by_its_ambient_domain_and_its_representable_set`, whose own words are that "adaptation choice and encoding are realisation ... and not part of identity", and `proposal::membership_of_the_representable_set_is_one_affine_predicate`, "an affine slot function, a quantum per magnitude and a phase".
- `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` (ratified, experts). "An operation is admitted exactly when it is a function of the declared signature, and where two realisations of one name disagree, the signature is missing a coordinate."
- `ruling::a_format_datum_is_one_point_and_a_saturating_end_is_one_point` (ratified, experts). "A saturating end of the set is one point: the exact results a saturating adaptation sends to it are that adaptation's fibre, and the adaptation is not part of the format's identity."
- `ruling::arms_over_regions_are_the_fundamental_heart` (ratified, both). "Small arms and spans that hold where they are optimal and nowhere else, composed ... Each arm carries a const predicate over the region it holds in."
- `ruling::a_predicate_lists_only_what_holds` (ratified, op). "What is not known is assumed not true until proven true."
- `ruling::never_a_runtime_check_and_one_lowered_path` (ratified, op). "Never any runtime checks, ever: invalids are caught at compile time."
- `ruling::the_standards_bound_starts_at_two_and_reserves_the_rest` (ratified, op). "The parity suite is built against MATLAB `fi`/`fimath` and IEEE 754."
- `ruling::warms_objective_is_the_intuitive_best_choice` (ratified, op, I4). Intuition is the intent and Rust convention its baseline.
- `ruling::every_family_of_the_old_arvo_returns_and_the_shape_is_open` (ratified, op). "Every family the old arvo carried returns in function and in feature."
- `ruling::the_panel_finishes_the_canon_without_him` (ratified, op). "Every remaining canon question is the panel's ... Nothing is parked awaiting him."
- `ruling::a_min_plus_top_absorbs_addition_under_saturation_and_never_under_wrapping` (ratified, experts), for the one place a ratified `says` states what wrapping is: "addition modulo `2^n` is a group".
- `ruling::a_composed_expressions_region_is_never_inherited_from_its_parts` (ratified, experts). A composite's region "needs its own derivation", which is why no wrap law transports to a new set without one.

Stated, binding as direction:

- `ruling::a_shipped_overflow_default_is_a_generic_overridable_per_case`. "The overflow behaviour is not called one way. A default ships and is easy to override per case ... that default may follow one of the parity standards."
- `ruling::the_standard_is_parity_in_output_not_in_the_internals`. "The standards bound is met by agreeing with what a standard documents as its result, for the cases it covers."
- `ruling::there_is_no_universal_answer_take_the_win_and_gate_it`. "Take the win where it applies, gate it out from where it does not."
- `ruling::the_option_set_is_not_a_boundary`. "A shape nobody has written down is admissible."
- `ruling::the_fused_result_is_composable_except_at_signed_saturating`. Its note: "wrap composes because it is a ring homomorphism while saturation is not."

Open, and checked for whether it reserves this question:

- `ruling::wrap_or_clamp_stays_open_and_both_get_priced` (rung open). "Whether the intuitive-compromise strategy wraps or clamps is not settled." It reserves which member one strategy selects. It does not reserve what a member denotes or on which sets it exists. Its priced readings, the bench families, are integer and fixed-point families.
- `question::where_wrapping_lives`, decider panel. It asks whether wrapping is one reduction slot classified by law role or an axis value with an exception list. It is a filing question, and both of its options presuppose that wrapping is defined wherever it is filed.
- `question::does_warm_wrap_or_clamp`, decider panel, narrowed by the stated overflow-default row to "which standard the default follows and how the override is spelled".

Claims, not canon, read for what they say about wrap:

- `proposal::the_adaptation_slot_is_derived_and_a_strategy_selects_a_member_per_operation` (one expert). "The space of total reductions onto the representable set is derived, not chosen."
- `proposal::the_laws_of_a_format_are_derived_from_two_hypotheses_rather_than_enumerated_per_policy` (one expert). "Wrapping's kernel is always a ring congruence."
- `proposal::the_realisation_map_is_one_map_with_two_regions` (three or more, normative, unratified). "Rounding between grid points, and completion outside the representable range."
- `probe::the_two_law_roles_of_a_reduction_are_independent`. Wrapping "fails the first and holds the second", the adaptation laws and coherence; wrapping is the member characterised by coherence.
- `proposal::a_value_admits_a_constant_quantum_a_zero_phase_and_a_deterministic_mode` (one expert). The value tier already admits wrap only at a constant quantum.
- `dimension::overflow_policy`: "What the operation does when a result leaves the representable range." Every predicate on a wrap law in `law.toml` names `fraction_width`, a coordinate that presupposes one quantum.

Evidence, not canon: `mock/research/202608072330_the-numeral-canon-panel/240_the_format_layer_derived_from_its_denotation.md`, section 2.1 (the union of shells `V = { phi + s * r^(sigma*m) : s in [a, b], m in [0, M) }` denotes the IEEE binary set at a symmetric slot range), 2.3 (`adapt` is a total map onto `V` exactly when `M = 1`), 3.2 (identity is the set, and the coordinates are not quotiented by it) and 3.4 (the quantum law's coordinates have no declared dimension). I read the sections; I did not re-run `q3` or `q5`, so the evidence gate is met for the claims I take from 240 only as far as its prose reports its own outputs, and nothing in my derivation rests on its numbers.

`mock/registry/dimension.toml` declares 25 axes. None indexes the quantum law, the magnitude count, or whether the representable set is one progression. `phase`, `radix`, `fraction_width`, `total_width` and `signedness` are the nearest.

## 3. The two gates

Canon gate. No ratified row settles what wrapping means on a union of magnitude ranges: the one ratified `says` that names wrapping's structure, the min-plus row, states it as addition modulo `2^n`, which is a one-grid statement, and no ratified row defines the term. No `open` row reserves the question: `wrap_or_clamp_stays_open_and_both_get_priced` reserves a selection, not a denotation. `where_wrapping_lives` is a panel question about filing. So the gate passes to a derivation, and the derivation is the panel's under `the_panel_finishes_the_canon_without_him`.

Evidence gate. The disagreement example in section 4 is computed by hand from the presentation in 240 section 2.1 and is shown in full so it can be checked by reading. It is not a committed probe, and it owes one before anything rests on it as measured.

## 4. The derivation

### 4.1 On one progression, wrapping is determined and unique

Let the representable set be one arithmetic progression, `V = { phi + s*g : s in [a, b] }`, with `n = b - a + 1 >= 2`. Then `V` determines its grid: the step `g` is the common difference and the offset is `phi` modulo `g`, so the unbounded grid the round-first step rounds onto is forced as `Gamma = phi + g*Z`, the one progression whose intersection with `[min V, max V]` is `V`.

A completion is a map `c: Gamma -> V` that is the identity on `V`. Ask that it be a translation congruence on the grid, so `c(x) = c(y)` exactly when `x - y` lies in a subgroup `K` of `g*Z`. Each class then meets `V` exactly once: at most once because `c` is the identity on `V`, at least once because `c` is total and `x` and `c(x)` share a class. So `K` has index `n` in `g*Z`, and the only such subgroup of a cyclic group is `n*g*Z`. Therefore `c(x)` is the one member congruent to `x` modulo `n*g`. That map exists, because `n` consecutive steps are a complete residue system modulo `n*g`, and it is what `overflow.rs` documents as "Reduce modulo the range, so the value re-enters at the other end".

This is the whole of what wrap is in every claim the corpus makes about it: a reduction whose kernel is a congruence, which is why the probe finds wrapping to be the coherent member and why the ratified min-plus row can say addition modulo `2^n` is a group. It holds at any phase, radix and width, because nothing in the argument used them. The phase decides whether exact sums land on `V`'s coset, which is the rounding region's business, and not whether the completion exists.

By the ratified identity clause, the region is the set and not the presentation. A quantum law with a nonzero slope whose union of shells is one progression is the same format as the constant law that presents that progression. Two such sets, from the presentation in 240 section 2.1 at radix 2, slope 1, two magnitudes: slot range `[0, 1]` gives `{0, 1} union {0, 2} = {0, 1, 2}`, and slot range `[-1, 1]` gives `{-1, 0, 1} union {-2, 0, 2} = {-2, -1, 0, 1, 2}`. Both are one progression, so wrap is determined on both. `is_constant_family` refuses both, because it reads the slope. That is a fact about the code (section 1) and it is a place where the code is narrower than the derivation.

### 4.2 Off one progression, the representable set does not fix a grid past its range

If `V` is not one progression, no progression `Gamma` has `Gamma` intersected with `[min V, max V]` equal to `V`, since that would make `V` one. So "the format's own grid" past the range is not something the set determines. Any extension is a presentation choice, and by the identity clause a presentation choice is not part of what the format is.

A lemma says why the constant-family argument cannot be repeated on a finer grid. Let `h` be the greatest common divisor of the differences of `V`, `N = |V| >= 3`, and let a map `rho` from the points `phi + h*Z` onto `V` be a translation congruence whose classes each meet `V` once, so its kernel is `N*h*Z`. Ask also that it round inside the range: a point strictly between two consecutive members `u < w` goes to `u` or to `w`. If `V` is not one progression of step `h`, some consecutive pair has `w - u >= 2*h`, and `x = u + h` is a point between them. `rho(x) = u` puts `h` in `N*h*Z`, so `N = 1`, which is excluded. `rho(x) = w` puts `w - u - h` in `N*h*Z`, and it is positive, so the gap is at least `(N + 1)*h` and `u + 2*h` is also between them; it cannot go to `u` (that puts `2*h` in the kernel, so `N <= 2`) and cannot go to `w` (with the previous step that puts `h` in the kernel). So on a set with an in-range gap, no adaptation that rounds inside the range and completes outside it is one translation congruence on the grid its own differences generate. The coincidence in section 4.3's fifth reading, a set that is a complete residue system, does not escape this: its congruence sends an in-range gap point to a member that is not a neighbour. On one progression the whole adaptation is one congruence, and that is what makes wrap there the coherent member rather than one completion among several.

### 4.3 The candidates that could carry the name disagree

Take the presentation of 240 section 2.1 at radix 2, slope 1, three magnitudes, phase 0, slot range `[-3, 3]`. The shells are `{-3..3}`, `{-6, -4, -2, 0, 2, 4, 6}` and `{-12, -8, -4, 0, 4, 8, 12}`, so

`V = {-12, -8, -6, -4, -3, -2, -1, 0, 1, 2, 3, 4, 6, 8, 12}`, fifteen members, not one progression (5 is an in-range gap). Its top shell is the step-4 lattice over the whole range.

The exact sum of the members 12 and 8 is 20, past the range. Three determinate completions, each a function of the signature once its own extra choice is fixed:

1. Extend past the range on the coarsest shell's lattice and reduce modulo its period `L = 24 + 4 = 28`. 20 is on the lattice, and `20 - 28 = -8`, a member.
2. Extend by continuing the quantum law, so the next shell has step 8, and reduce modulo the same `L`. 20 is a tie between 16 and 24 on that shell; `half_even` takes 16, giving `16 - 28 = -12`, and `half_up` takes 24, giving `-4`.
3. Wrap the exponent rather than the position: scale by `2^(sigma*M) = 8` until the value is in range, then round. `20 / 8 = 2.5`, a tie between 2 and 3.

One sum, three maps, answers drawn from `{-8}`, `{-12, -4}` and `{2, 3}`. A fourth reading, wrapping the slot index and ignoring the magnitude, is what an `adapt` returning a bare slot would do, and 240 section 2.3 shows a bare slot names up to three members of such a set, so it is not a map onto `V` at all. A fifth, a complete residue system modulo `|V|` over the finest step, exists only by coincidence: here the residues modulo 15 collide (`-12` and `3`), so it does not exist for this set.

Under the ratified admission rule, "where two realisations of one name disagree, the signature is missing a coordinate." The missing coordinate is visible: how the grid extends past the range, and what the modulus is. Neither is a property of `V`.

### 4.4 What the standards and the intents add

`the_standard_is_parity_in_output_not_in_the_internals` asks for agreement with what a standard documents as its result. MATLAB `fi` documents wrap on fixed-point types, which are one progression. IEEE 754's default overflow result is an infinity or the largest finite value depending on the rounding direction, which is the saturating shape and not wrap; this is from my own knowledge and was not checked against the standard text in this read. Neither standard documents a wrapped result for a magnitude-indexed format under default handling, so neither obliges one. I recall that IEEE 754-1985 delivered a scaled result, with the exponent wrapped by a fixed bias, to an enabled overflow trap handler; that is unverified here, and if a current revision documents such a result as an output, the parity bound would owe it as a member, which is candidate 3 under its own name and not the bare name wrap (section 8).

`warms_objective_is_the_intuitive_best_choice` takes Rust as the baseline for intuition. Rust's floating types have no wrapping arithmetic and overflow to infinity. A float that re-enters at the other end is the opposite of the baseline. That bears on a default and not on admission, and it points the same way.

`every_family_of_the_old_arvo_returns_and_the_shape_is_open` would cut against a refusal if the old arvo carried a wrapping float. I found no sign of one, but the history files I grepped (`arvo-history.md`, `arvo-numeric-contracts-history.md`, `arvo-strategy-history.md`) mention neither wrapping nor floats at all, so that zero says nothing and the question is unchecked.

## 5. The outcomes, each argued

The canon already determines it. For: the spine names the adaptation as a total first-class map, identity is the set, the admission rule turns disagreement into a missing coordinate, and every claim the corpus makes about wrap is a one-grid claim; read together they determine wrap on one progression and determine that the bare name denotes nothing elsewhere. Against: no ratified row defines the term, the coherence characterisation sits in one-expert proposals and a probe, and `where_wrapping_lives` is still open, so saying the canon already states this overstates what it states. The determination is derived, not written.

The canon determines it only in a region. For: there is a sharp region where the term has exactly one referent (section 4.1), and outside it the candidates disagree, which is what `there_is_no_universal_answer_take_the_win_and_gate_it` and the arms ruling ask to be stated as an arm and a gate. Against: the deciding property, whether the representable set is one progression, has no declared dimension, so the region cannot be written as a registry predicate, and `a_predicate_lists_only_what_holds` forbids writing a near axis as a stand-in. Nor is it a region where a property holds in the measured sense; it is the domain on which a term has a referent, which is a definition.

Wrapping is not defined there and the type does not admit the arm. For: the admission rule refuses a name that is not one function of the signature, `never_a_runtime_check_and_one_lowered_path` puts that refusal at the bound, which is where the code already has it, and the instead is real: saturation and clamp are total there, conversion onto a constant-quantum grid and wrap there is a composition the consumer writes, and any of the three candidates can join the open inventory once it declares its extension and modulus. Against: the admission rule's own remedy is to add the coordinate rather than refuse, and `arvo-toolbox-not-policer` warns against deciding for a consumer. The answer to both is that the refusal is of the unparameterised name only; nothing stops a consumer naming a parameterised member, so nothing is policed.

The canon does not reach it and the call is his. For: no ratified row defines wrapping on any set, and which candidate is wanted, if any, is a design preference. Against: `the_panel_finishes_the_canon_without_him` makes every remaining canon question the panel's, no `open` row reserves this one, and the answer does not need a preference among the candidates, because it says the bare name selects none of them. A preference among them is an admission into the open inventory, which is design work under the closed concept, not a canon call.

## 6. The answer

A definition with a refusal, derived from the ratified spine, the identity clause and the admission rule. On a representable set that is one arithmetic progression, wrap is determined and unique: reduction modulo the range on the format's own grid. On a set that is not one progression, including every magnitude-indexed format whose union of shells is not one progression, the name wrap denotes no single map, and a signature naming it there is not admitted. The region is real and cannot yet be written as a predicate, because the axis that would carry it is not declared.

What follows for the placeholders, as a recommendation to whoever holds the round: the exact positions are functions of the format alone and owe nothing to this question, so the quantum assertion in `Addable::ADMITTED` and its siblings belongs to the design half of the FIXME only, and the wrap refusal belongs on the declared signature, forced where `adapt` meets wrap over a set that is not one progression, with this row's instead in its message.

Confidence: moderate on the derivation, which is short and checkable by reading; low on its standing, which is one read. Two independent agreements are owed before it is canon.

### The proposed row

```toml
[[proposal]]
id = "wrapping_names_one_map_only_on_a_representable_set_that_is_one_progression"
kind = "refusal"
sentence_kind = "definition"
standing = "one_expert"
topic = "overflow_policy"
defines = "wrap"
says = "Wrap is the completion that sends a result past the range to the one member congruent to it modulo the range, on the format's own grid. It is defined on a representable set that is one arithmetic progression, whatever coordinates present that set, and there it is unique. On a representable set that is not one progression, which includes every magnitude-indexed format whose union of magnitude ranges is not one progression, wrap names no single map, and a declared signature naming it there is not admitted."
because = "By `ruling::the_format_spine_is_canon` a format is its ambient domain and its representable set, and the adaptation is a named total map onto that set. A set that is one progression fixes its unbounded grid, and the only completion on that grid that is a translation congruence with one member per class is reduction modulo the count times the step, which is what the corpus calls wrap and why wrap is the coherent member. A set that is not one progression fixes no grid past its range, no adaptation that rounds inside the range is a congruence on the grid its own differences generate when the set has an in-range gap, and the completions that could carry the name disagree: at slot range `[-3, 3]`, radix 2, slope 1 and three magnitudes, the exact sum 20 of the members 12 and 8 completes to -8 on the coarsest shell's lattice, to -12 or -4 on the continued quantum law, and to 2 or 3 under an exponent wrap. Under `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule`, two realisations of one name disagreeing means the signature is missing a coordinate, here how the grid extends past the range and what the modulus is, and neither is a property of the representable set."
instead = "Saturation or clamp, both total on such a set; a conversion onto a constant-quantum format followed by wrap there, written by the consumer as a composition; or a new member of the open overflow inventory that declares its extension grid and its modulus as coordinates, carries a name of its own, and derives its own laws, since no law stated for wrap transports to it."
note = "A definition, so no region, and the reason is a gap in the dimension set rather than a claim of universality: the domain this states is decided by whether the representable set is one progression, and no declared dimension indexes that, the quantum law or the magnitude count. By the identity clause a nonzero-slope presentation whose union is one progression, such as slot range `[-1, 1]` at radix 2, slope 1 and two magnitudes, which denotes `{-2, -1, 0, 1, 2}`, is the same format as its constant presentation, and wrap is defined on it. It constrains what wrap denotes and not which member a strategy selects, so `ruling::wrap_or_clamp_stays_open_and_both_get_priced` is untouched; where a strategy's alias would sit on a magnitude-indexed format, its wrap reading has no referent there, which is a consequence for the coordinator to read against that row. The disagreement example is computed by hand and owes a committed probe."
gap = "A dimension for the shape of the representable set, or for the quantum law's coordinates, is owed before this can become a gated arm; per the dimension set's own header it moves on two independent readings and is not declared here. Whether any extension and modulus is wanted as a named member is a design question. Whether IEEE 754 documents a wrapped-exponent result an implementation delivers is unchecked."
provenance = ["mock::research/<stamp>_wrapping-over-a-union-of-magnitude-ranges/NOTES::#4-the-derivation", "panel::202608072330_the-numeral-canon-panel::240_the_format_layer_derived_from_its_denotation::#21-the-union-of-shells-denotes-a-float-subnormals-included", "panel::202608072330_the-numeral-canon-panel::240_the_format_layer_derived_from_its_denotation::#32-the-coordinates-are-not-quotiented-by-the-set-they-denote"]
keywords = ["wrap", "wrapping", "modulo the range", "completion", "magnitude-indexed", "union of magnitude ranges", "quantum per magnitude", "float", "one progression", "translation congruence", "admission rule", "missing coordinate", "extension grid", "exponent wrap"]
```

The first provenance entry names where this note would land, with the stamp left for whoever lands it; the two `panel::` anchors are my slugging of 240's headings and want the citation lint run over them.

## 7. Rows and laws this would touch if stamped

None is invalidated: the row adds a definition and a refusal and changes no existing claim. Every wrap law in `law.toml` and `law-the-later-topics.toml` already sits inside the domain it states, since each names `fraction_width`. `dimension::overflow_policy` is unchanged. `question::where_wrapping_lives` is not answered, and under either of its options the domain this states is what the filed member ranges over.

## 8. What would change the answer

- A ratified row that defines wrap by a law role alone, rather than by reduction modulo the range, would reopen 4.1 and could admit a coincidental complete residue system as wrap.
- A declared dimension for the representable set's shape or the quantum law would turn this into a theorem with a predicate, an arm gated on that axis, rather than a definition.
- A current IEEE 754 revision documenting a scaled, wrapped-exponent result as an output would make the parity bound owe candidate 3 as a member under its own name. It would not make the bare name wrap denote it. The 1985 trap-handler behaviour is my recollection and unverified; fetching the standard is the check.
- Evidence that the old arvo carried a wrapping magnitude-indexed numeral would put `every_family_of_the_old_arvo_returns_and_the_shape_is_open` against the refusal, and the instead would then owe that family's function under a declared coordinate.
- A committed probe finding the three candidates agreeing over some class of formats would narrow 4.3 to its complement. By hand they disagree at the first out-of-range sum tried.
- A consumer obligation naming wrapping on a floating format would make the instead's third route owed rather than offered.
