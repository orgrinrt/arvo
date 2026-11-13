# IEEE 754 ties to even through `adapt`

**Outcome:** WORKS, for finite results.
**Run:** `cargo test --manifest-path mock/research/sketches/202611131910_ieee754-ties-to-even-through-adapt/Cargo.toml`,
two tests, both green at the commit that adds this file.

The question, from ticket `sketch_whether_an_ieee_754_ties_to_even_parity_arm_can_be_wr`, was whether an
IEEE 754 `roundTiesToEven` parity arm can be written through `adapt` in `arvo-format` before any float
value exists. It can, with no change to the crate's source.

Binary32 is the shipped `Floating<25, -149, 254>`. `adapt` takes a position in slots and carries no
magnitude, so the caller picks the magnitude: the finest one at which the rounded position lands in the
slot range, asked of `panic_on_overflow`. That is the standard's own rule, the nearest member at the
precision the binade allows, and a carry out of the top of a binade lands at the next magnitude rather
than being clamped. The expected side is the host's `f64 as f32`, which Rust defines as round to nearest
with ties to even, decoded from its bits, so no arvo code computes the expected answer.

The cases are ties on both sides of zero at both parities, the carry at `2^25 - 1`, ties and non-ties
below the smallest normal, and an ordinary value of each sign. The planted `half_up` fails at least one,
which is what says the arm separates the modes.

What it does not reach: a result past the largest finite member, where the standard answers an infinity
the format has no member for. The arvo-format parity suite written from this round,
`tests/ieee754_binary_parity.rs`, carries that refusal and the rest of the binary formats.
