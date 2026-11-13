# The calls `arvo-numeral` makes across the `arvo-format` border

Written on `work/arvo-values` for the per-width adaptation door and the slot widening that other
branches carry, so that either can see what a value reads before it moves anything. The same text
sits in the channel document `briefs/arvo-door-shape.md` as T3's side; this is the copy the corpus
can find.

A value computes in its carrier where the carrier arithmetic is the format's, and calls the format's
own function on slots where it is not. So the border is crossed in two ways: constants read at build
to decide which path a signature takes, and slot functions called for the rest.

This branch added four things to `arvo-format` that the value needed and the crate did not have:
`WithOverflow` and `WithRounding`, which name a sibling signature differing in one coordinate, the
point `Fixed`, and `multiplicative_identity` with its suite. Nothing that was already there changed.

## The calls the value makes

| Call | Signature | Read by |
|---|---|---|
| `adapt` | `const fn adapt<S: DeclaredSignature>(Exact, Dither) -> Slot` | `num.rs` (the `adapted` door), `ratio.rs` |
| `panic_on_overflow` | `const fn panic_on_overflow<S: DeclaredSignature>(Exact, Dither) -> Bool`, true when out of range | `num.rs`, `ratio.rs`: the debug check and `fits` |
| `mul`, `product_position` | `const fn mul<S: DeclaredSignature>(Slot, Slot, Dither) -> Slot`, `const fn product_position<F: Format>(Slot, Slot) -> Exact` | `num.rs`: the product where it is not a carrier product, and its check |
| `div`, `quotient_position` | `const fn div<S: DeclaredSignature>(Slot, Divisor<S::Format>, Dither) -> Slot`, `const fn quotient_position<F: Format>(Slot, Divisor<F>) -> Exact` | `ratio.rs` |
| `convert`, `converted_position` | `const fn convert<F: Format, S: DeclaredSignature>(Slot, Dither) -> Slot`, `const fn converted_position<F: Format, T: Format>(Slot) -> Exact` | `ratio.rs`, and `Accumulator::finish` through it |
| `slot_in_range` | `const fn slot_in_range<S: Slots>(Slot) -> Bool` | `num.rs`: the `constant` door |
| `Exact::on_grid` | `const fn on_grid(Slot) -> Exact` | `host.rs`, `ratio.rs`, `identity.rs` |
| `Slot::at`, `Slot::index`, `Slot::ZERO` | `const fn at(i128) -> Slot`, `const fn index(self) -> i128` | every module |
| `has_additive_identity`, `multiplicative_identity` | `const fn has_additive_identity<F: Format>() -> Bool`, `const fn multiplicative_identity<F: Format>() -> Maybe<Slot>` | `identity.rs` |
| `quantum::is_constant_family` | `const fn is_constant_family<Q: Quantum>() -> Bool` | `admit.rs` |
| `radix` | `radix::<F>().base()` | `accumulator.rs` |
| `Dither::UNUSED`, `Bool::of`, `Bool::get`, `Width::count` | constants and accessors | every module |

Read as declared, by their associated items: `DeclaredSignature` (`Format`, `Adaptation`), `Format`
(`Slots`, `Quantum`, `PHASE` with `is_zero`), `Slots` (`MIN`, `MAX`, `WIDTH`, `ADMITTED`), `Quantum`,
`Adaptation`, `Overflow::POLICY` with `Policy::{Wrap, Saturate, Clamp}`, `Rounding::MODE` with
`Mode::Stochastic`, `Adapt`, `Signature`, `WithOverflow`, `WithRounding`, the points `Integer`,
`Fixed` and `UFixed`, and `Divisor`, which the crate also exports again beside `Bool`.

The sum, the difference and the negation never call across. They are carrier arithmetic, and `add`,
`sub` and `neg` of `arvo-format` are read only by the tests, as the oracle the exhaustive bands
compare against.

## What the value needs from the door

1. `Sole::ADMITTED` reads `Overflow::POLICY` and `Rounding::MODE` as closed enums to decide whether
   the carrier arithmetic is the format's. A per-width adaptation door that keeps both constants on
   the traits changes nothing. One that moves the completion behind a call needs a constant that
   still says wrap or saturate at build, or the value loses its carrier path and falls to the slot
   functions for everything.
2. A remainder. The format ships none, so a value has none: ticket
   `arvo_format_ships_no_remainder_so_a_value_has_no_remainder_p`.
3. A product over the unsigned 64-bit range. `Uint<64>` has no `mul` until
   `arvo_s_product_over_the_unsigned_64_bit_range_needs_a_slot_c`. The fixed-point product goes
   through the 128-bit slot function meanwhile:
   `arvo_numeral_s_fixed_point_product_goes_through_the_128_bit`.

## What a slot widening may change

A widening that keeps `Slot::at(i128)` and `Slot::index() -> i128` changes nothing here: `whole_range`
in `admit.rs` and the accumulator's range sums compute in `i128` and refuse a width past 64. One that
changes them ports `admit.rs`, `accumulator.rs`, `num.rs`, `ratio.rs`, `host.rs` and `identity.rs` in
the same branch, and the check is `cargo test -p arvo-numeral` green with each `mutant-*` feature
still red against its committed run.
