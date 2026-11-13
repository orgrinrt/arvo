# Wrapping over a union of magnitude ranges, two readings

Three placeholders in `arvo-format`, for the sum, the product and the quotient over a magnitude-indexed quantum, waited on a canon row saying what wrapping means where a type's representable set is a union of magnitude ranges at more than one quantum. No ratified row settled it and no `open` row reserved it, so it was put to two experts, one after the other, the second without the first's file. Their notes are `READ-1.md` and `READ-2.md` beside this one, kept whole.

Both reached the same answer by different routes, and that convergence is the provisional stamp the row in `proposal` carries, with question ticket `29bd8e5f1b6c` asking for it to be made formal.

## Where the two readings agree

Wrapping is fixed, and fixed uniquely, only where the representable set is one arithmetic progression, and there it is the ordinary reduction modulo the member count times the step. On a set that is a union of magnitude ranges and not one progression, no map carries the name, so a signature naming wrapping there is refused, at compile time and from the declared signature. What is offered instead is saturation, which is defined on any bounded set, converting onto a format that is one progression and wrapping there, or an adaptation under a name of its own that declares how it extends past the range and what its modulus is.

Both found that no dimension declared in `mock/registry/dimension.toml` indexes the set's shape, so the row carries no predicate and an axis is owed (ticket `3c82c008442e`). Both found that the crate's present test, `is_constant_family` in `mock/crates/arvo-format/src/quantum.rs`, reads the quantum law's slope rather than the set, and so also refuses degenerate laws whose set is one progression (ticket `bb82327ee374`). Neither rests on an instrument, and an exhaustive enumeration with a must-fail control is owed before promotion (ticket `990f0e11c61c`).

## Where they differ

The first reads it as a refusal by theorem: given that wrapping carries the group law, as `ruling::a_min_plus_top_absorbs_addition_under_saturation_and_never_under_wrapping` uses it, and that every rounding mode sends a value between members to a neighbour, a congruent wrap forces the set to have no gap. Its instance is slots 0 to 3 at quanta 1, 2 and 4.

The second reads it as a definition that admits no map: off one progression the set fixes no grid past its range, so the candidates for the name disagree (`12 + 8` on slots -3 to 3, radix 2, slope 1, three magnitudes completes to -8, to -12 or -4, or to 2 or 3), and under `ruling::the_derivation_is_a_placement_and_the_operation_set_is_an_admission_rule` a name two realisations disagree on is a signature missing a coordinate.

The row is filed as `normative`, the first reading's kind. The second's `definition` would need the term to be one the row defines whole, and both readings agree it defines wrapping only on one progression.

## The derivation

The row's `because` carries the first reading's proof in full; the second reading's sections 4.1 to 4.3 carry the disagreement argument. Neither instance has been run by a committed instrument.

## The positions

The second reading adds that the three positions are exact steps and read no overflow policy, so this refusal is not what blocks them; once `Exact` and `adapt` carry a magnitude coordinate, which is design, they have no canon blocker, and the refusal sits where `adapt` meets wrapping on a set that is not one progression.
