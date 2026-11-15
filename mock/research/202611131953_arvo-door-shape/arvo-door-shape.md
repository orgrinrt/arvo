Source: kaski channel (coord/kaski of clause-dev), briefs/arvo-door-shape.md, first posted 2026-11-13 19:53 UTC, read 2026-11-15.

# arvo door shape, T1's side

T1 (`work/arvo-standards`, tip `f0eeb2e3`) reads `arvo-format` across the border only through the calls below. This is what T2's per-width adaptation door and job 19's slot widening must keep, or port in the same branch, so the parity suites stay the regression gate the triage made them. T1 writes nothing in `arvo-format/src` outside `standards/`.

## The calls the parity suites make

| Call | Signature on dev and on `f0eeb2e3` | Read by |
|---|---|---|
| `apply::adapt` | `const fn adapt<S: DeclaredSignature>(Exact, Dither) -> Slot` | every suite |
| `apply::panic_on_overflow` | `const fn panic_on_overflow<S: DeclaredSignature>(Exact, Dither) -> Bool`, true when out of range | `tests/ieee754/mod.rs` (magnitude search) |
| `apply::Exact::at` | `const fn at(Slot, Remainder) -> Exact` | `tests/ieee754/mod.rs` |
| `apply::Remainder::of` | `const fn of(u128, u128) -> Remainder` | `tests/ieee754/mod.rs` |
| `apply::Exact::between`, `Fraction::of` | `const fn between(Slot, Fraction)`, `const fn of(i64, i64)` | MATLAB suites |
| `slots::Slot::at`, `Slot::index` | `const fn at(i128) -> Slot`, `const fn index(self) -> i128` | every suite |
| `format::step_exponent` | `const fn step_exponent<F: Format>(Magnitude) -> Exponent` | IEEE suites |
| `format::radix` | `radix::<F>().base()` | IEEE suites |
| `quantum::Magnitude::at` | `const fn at(u32) -> Magnitude` | IEEE suites |
| `Dither::UNUSED`, `Bool::get` | constants and accessor | every suite |

The types named in `use` lines beside these (`Format`, `Quantum`, `Slots`, `Signature`, `Adapt`, the rounding modes, the overflow policies, `Integer`, `Floating`) are read as declared, by their associated constants.

## What T1 needs from the door

1. A magnitude coordinate on adaptation. `adapt` lands on one slot index; a floating format is a union of magnitudes, so the IEEE suites find the landing magnitude themselves by calling `panic_on_overflow` per magnitude from the finest up (`tests/ieee754/mod.rs`, `land`). The door that lands a value on a magnitude-indexed format replaces that search; ticket `arvo_landing_a_value_on_a_magnitude_indexed_format_is_a_call` (`85bfc9dad5a2`) asks for it. When it lands, `land` becomes one call to it and the suites keep their oracles unchanged.
2. A public constructor for an exact position with a wide remainder, beside `Exact::between`. The IEEE suites build positions with `Exact::at(Slot, Remainder::of(u128, u128))` today, which serves; the open ticket `arvo_has_no_public_constructor_for_an_exact_position_whose_r` is the same door from the consumer side, and T1 reads whatever shape it takes.

## What job 19 may change

`Slot` already holds `i128`. A widening that keeps `Slot::at` and `Slot::index` changes nothing for T1. One that changes them ports the three IEEE suites and two MATLAB suites in the same branch, and the check is unchanged: `cargo test -p arvo-format --test ieee754_binary_parity --test ieee754_decimal_parity --test matlab_fi_parity --test matlab_fimath_parity` green, each planted arm still missing, `compile_fail` refusing binary128 and decimal128.

# arvo door shape, T3's side

T3 (`work/arvo-values`, tip `9a3d5149`) is `arvo-numeral`, and its source outside the tests reads `arvo-format` through the calls below. T3 added four things to `arvo-format` and changed nothing that was there: `WithOverflow`, `WithRounding`, the point `Fixed`, and `multiplicative_identity` with its suite. The same text is in the repo at `mock/research/202611132145_review-arvo-values/THE_CALLS_ACROSS_THE_FORMAT_BORDER.md`. A value computes in its carrier where the carrier arithmetic is the format's, and calls the format's own function on slots where it is not, so the border is crossed in two ways: constants read at build to decide which, and slot functions called for the rest.

## The calls the value makes

| Call | Signature on `9a3d5149` | Read by |
|---|---|---|
| `adapt` | `const fn adapt<S: DeclaredSignature>(Exact, Dither) -> Slot` | `num.rs` (the `adapted` door), `ratio.rs` |
| `panic_on_overflow` | `const fn panic_on_overflow<S: DeclaredSignature>(Exact, Dither) -> Bool`, true when out of range | `num.rs`, `ratio.rs`: the debug check and `fits` |
| `mul`, `product_position` | `const fn mul<S: DeclaredSignature>(Slot, Slot, Dither) -> Slot`, `const fn product_position<F: Format>(Slot, Slot) -> Exact` | `num.rs`: the product where it is not a carrier product, and its check |
| `div`, `quotient_position` | `const fn div<S: DeclaredSignature>(Slot, Divisor<S::Format>, Dither) -> Slot`, `const fn quotient_position<F: Format>(Slot, Divisor<F>) -> Exact` | `ratio.rs` |
| `convert`, `converted_position` | `const fn convert<F: Format, S: DeclaredSignature>(Slot, Dither) -> Slot`, `const fn converted_position<F: Format, T: Format>(Slot) -> Exact` | `ratio.rs` |
| `slot_in_range` | `const fn slot_in_range<S: Slots>(Slot) -> Bool` | `num.rs`: the `constant` door |
| `Exact::on_grid` | `const fn on_grid(Slot) -> Exact` | `host.rs`, `ratio.rs`, `identity.rs` |
| `Slot::at`, `Slot::index`, `Slot::ZERO` | `const fn at(i128) -> Slot`, `const fn index(self) -> i128` | every module |
| `has_additive_identity`, `multiplicative_identity` | `const fn has_additive_identity<F: Format>() -> Bool`, `const fn multiplicative_identity<F: Format>() -> Maybe<Slot>` | `identity.rs` |
| `quantum::is_constant_family` | `const fn is_constant_family<Q: Quantum>() -> Bool` | `admit.rs` |
| `Dither::UNUSED`, `Bool::of`, `Bool::get`, `Width::count` | constants and accessors | every module |

Read as declared, by their associated items: `DeclaredSignature` (`Format`, `Adaptation`), `Format` (`Slots`, `Quantum`, `PHASE` with `is_zero`), `Slots` (`MIN`, `MAX`, `WIDTH`, `ADMITTED`), `Quantum`, `Adaptation`, `Overflow::POLICY` with `Policy::{Wrap, Saturate, Clamp}`, `Rounding::MODE` with `Mode::Stochastic`, `Adapt`, `Signature`, `WithOverflow`, `WithRounding`, the points `Integer`, `Fixed` and `UFixed`, and `Divisor`, which the crate also exports again beside `Bool`.

The sum, the difference and the negation never call across: they are carrier arithmetic, and `add`, `sub` and `neg` of `arvo-format` are read only by the tests, as the oracle the exhaustive bands compare against.

## What T3 needs from the door

1. `Sole::ADMITTED` reads `Overflow::POLICY` and `Rounding::MODE` as closed enums to decide whether the carrier arithmetic is the format's. A per-width adaptation door that keeps both constants on the traits changes nothing. One that moves the completion behind a call needs a constant that still says wrap or saturate at build, or the value loses its carrier path and falls to the slot functions for everything.
2. A remainder. The format ships none, so a value has none: `arvo_format_ships_no_remainder_so_a_value_has_no_remainder_p`.
3. A product over the unsigned 64-bit range. `Uint<64>` has no `mul` until `arvo_s_product_over_the_unsigned_64_bit_range_needs_a_slot_c`. The fixed-point product goes through the 128-bit slot function meanwhile: `arvo_numeral_s_fixed_point_product_goes_through_the_128_bit`.

## What job 19 may change

A widening that keeps `Slot::at(i128)` and `Slot::index() -> i128` changes nothing for T3: `whole_range` in `admit.rs` computes in `i128` and refuses a width past 64. One that changes them ports `admit.rs`, `num.rs`, `ratio.rs`, `host.rs` and `identity.rs` in the same branch, and the check is `cargo test -p arvo-numeral` green with each `mutant-*` feature still red against its committed run.
