# Doc changelist: the ring operations through the value

**Date:** 2026-11-13
**Round:** 202611131949-qrwo, branch `work/arvo-values`, PLAN step 3.
**Topics consumed:** `202611131949-qrwo_topic.the-ring-operations-through-the-value.md`, the only flat
topic on this branch or on `origin/dev`.
**Sketches:** none. The round decides nothing a sketch would settle: each operation is checked against
the format's own, exhaustively, in the source changelist's window.
**Scope:** `mock/crates/arvo-numeral/DESIGN.md.tmpl`, `mock/crates/arvo-numeral/README.md.tmpl`,
`mock/crates/arvo-format/DESIGN.md.tmpl`. No registry file.

## Canon

`ruling::the_format_spine_is_canon`: an operation is an exact step and one named adaptation, and the
value's difference, negation and product are the format's own, by equality on every pair.
`ruling::never_a_runtime_check_and_one_lowered_path` and
`ruling::the_overflow_panic_is_permitted_and_bounded`: the three verdicts and the abort under `Abort`,
as for the sum.
`proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`, at one expert and on a
provisional stamp: built on as provisional, as the crate's design already says, and not promoted.
`proposal::a_missing_identity_is_refused_as_a_failed_constant`, at one expert and on a provisional
stamp: the shape the multiplicative identity takes, on the same terms.

## CHANGE: `arvo-numeral`, the difference, the negation and the product

File: `mock/crates/arvo-numeral/DESIGN.md.tmpl`.

`## What this crate is` states the contract for all four operations and not the sum alone, and says the
crate adds nothing to `arvo-placement` and reads two named things of `arvo-format` that this round adds
there.

`## How the value holds its carrier` gains the rung arithmetic of the three: the signed difference and
negation as the machine's wrapping operation followed by the adaptation below the carrier's width and
the machine's own wrapping or saturating operation at it; the unsigned saturating difference and
negation as the machine's `saturating_sub` and zero, which do not pass through the adaptation; the
product as the machine's wrapping product then the adaptation under wrapping, and the machine's checked
product under saturation, pinned to the end on the product's sign where it leaves the machine integer.

`## The operations` gains `sub`, `neg` and `mul` with `fits_sub`, `fits_neg` and `fits_mul`. It states
the product's two arms: at a quantum exponent of zero the product is carrier arithmetic, and at any
other exponent it is `arvo_format::mul` on the two slots, because the position there is rounded and the
rounding has one home. It states that the arm is a constant of the instantiation, that the fractional
arm runs the format's wide step until a per-width adaptation exists in `arvo-format`, and that `mul`
forces the format's multiplication obligation in both arms, so the unsigned 64-bit integer is refused
with the format's message until the format admits it. It states that the negation of an unsigned member
is admitted because the format admits it.

`## The debug check` names the three new verdicts beside `fits_add`, and says `fits_mul` asks
`panic_on_overflow` of `product_position`.

`## Identities` gains `Multiplicative` and `Identity<Multiplicative>` for every `Num<S, C>`, read from
`arvo_format::multiplicative_identity`, refused as a failed constant where the format has no member
that is one. Its last sentence, that the multiplicative identity waits, goes.

`## What is proved, and how` gains the three operations at every place the sum is proved, the
fixed-point product exhaustive at six bits over `Fixed<6, -3>` and `UFixed<6, -3>` with a tie at a
negative product in each mode, one mutant per operation behind `mutant-sub`, `mutant-neg` and
`mutant-mul`, and the new compile-fail pairs. The paragraph on `UFixed<6, 3>` keeps its claim for the
sum and says the product is where the exponent does move the slot.

`## What is not here` loses subtraction, negation and the product, and keeps the accumulator and the
lanes. It gains the per-width adaptation as the format crate's to write.

File: `mock/crates/arvo-numeral/README.md.tmpl`. The second paragraph names the difference, the
negation and the product beside the sum, and both identities.

## CHANGE: `arvo-format`, two additions

File: `mock/crates/arvo-format/DESIGN.md.tmpl`. The crate's overhaul is another lane's; these two are
additions beside what stands and change nothing that stands.

`### The operation and the surface` under multiplication gains `multiplicative_identity`: the slot of
the member that is one, or nothing. With the phase `n / d` in lowest terms, the radix `r` and the
exponent `e`, the candidate is slot `r^-e - n` where `d` is one and `e` is at or below zero; it is
answered only where it is in the slot range and the product's own step, at the candidate against each
end of the range, returns that end on the grid. It forces no obligation, so it answers for a format the
product does not admit. Over a magnitude-indexed quantum it answers nothing, under the same ticket as
the product there. `### The laws this section owes tests for` gains its law: where it answers, the
product position of the answer and every member is that member, exhaustive at widths 1 to 6 over each
point family, and where it answers nothing no member has that property at those widths.

`## The surface this design names` gains `Fixed` in the `Format` row: the signed fixed-point point, an
alias of `Biased` at a zero phase count, for the reason `USize` is an alias.

## What the source changelist will carry

The rung functions, the three operations and verdicts, `Multiplicative` and its impl, the two additions
to `arvo-format` with their tests, the tests named above, the three mutants with their red runs
committed under `mock/research/sketches/202611131949-qrwo_value-mutants/`, and the consumer crate
reaching each new public item.
