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

### Status, set against the stance file after the above was written

On (b), CONVERGED. Section 1.5 of the stance file reaches the value type from the same four rulings and the same two obligations, and its item 4 is this read's first quotation. It goes further than the rows in one sentence, "A value type whose representation **is** the sole occupancy output of the placement derivation is the first placement, used", which this read reaches through the ratified `at_shared_occupancy_no_per_element_footprint_observation_exists` ("one primitive with two placements") rather than by assertion.

On (d), CONVERGED with the stance file and with the roadmap's first read of call 10, and against the reading recorded there as K6. Section 1.5 is silent on (d). Elsewhere the stance file uses head's `Exact` (`arvo-format/src/apply/position.rs:158`) as a named exact position at its ingest door and keeps the double-width register of a lane "an internal arm and not a declared coordinate" (its section 3.5), and neither claims the rows owe a declared accumulator. The roadmap's member reads `:1824` as "ratifies only that a `V x V -> V` concept cannot name the intermediate". K6 reads the same row as saying "the format concept supplies the width algebra, so the value crate owes the accumulator", and the row says the opposite of that in its own words: the list naming the width algebra "is not promoted by this ruling and stays at one expert". So an accumulator type is licensed, as a name the rows make necessary for a composite claim and nothing forbids, and it is not owed by them; its width algebra is a proposal's to state.
