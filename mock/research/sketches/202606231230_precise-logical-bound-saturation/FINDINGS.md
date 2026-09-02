# Findings: Precise saturation at the logical bound

**Hypothesis:** Precise can saturate at the LOGICAL bound (`i_MIN..i_MAX` for logical
width N) instead of the container bound by computing the logical MIN/MAX in the
container type from the const `N` and clamping the result, with no widening. The
invariant that makes this safe: Precise is always DoubleLogical, so container =
2x logical, hence `N - 1 < container_bits` always and `1 << (N - 1)` never overflows
the container.

**Outcome: WORKS.** `sketch_b.rs` and `body_shape.rs` compile and run on
`nightly-2026-05-28`. They reproduce the target
(`i_mul_fixed::<4>(100, 100) == 127` for Precise N=8) and the negative clamp
(`-128`), and show the same clamp tail generalizes to add/sub (and by the same
shape, div). No unstable feature beyond what arvo already enables is needed.

Run: `rustc +nightly-2026-05-28 --edition 2024 sketch_b.rs -o /tmp/sb && /tmp/sb`
prints `ALL B PASS`; `body_shape.rs` prints `body shape OK`.

## The working shape (const-generic N, threaded container)

The arithmetic is trivial; the real question is the macro shape. The clamp keyed on
the const-generic `N` and the container type:

```rust
const fn i_logical_clamp<const N: u16>(v: $container) -> $container {
    let hi: $container = (1 << (N - 1)) - 1;   // logical MAX, signed
    let lo: $container = -(1 << (N - 1));      // logical MIN, signed
    if v < lo { lo } else if v > hi { hi } else { v }
}
```

Fixed-point multiply, Precise, threaded with `$container` and the literal `$bits = N`:

```rust
const fn i_mul_fixed_precise<const N: u16, const FRAC: u16>(a: $C, b: $C) -> $C {
    let prod = a.wrapping_mul(b);            // 2x container holds the in-range product
    i_logical_clamp::<N>(prod >> FRAC)
}
```

Same clamp tail generalizes (this is what the task asks be confirmed):

```rust
fn i_add_precise<const N: u16>(a: $C, b: $C) -> $C { i_logical_clamp::<N>(a.saturating_add(b)) }
fn i_sub_precise<const N: u16>(a: $C, b: $C) -> $C { i_logical_clamp::<N>(a.saturating_sub(b)) }
// i_div: clamp the existing saturating_div result the same way.
```

Unsigned counterpart: `MAX = (1 << N) - 1`, `MIN = 0`, clamp `if v > hi { hi } else { v }`
(no lower clamp needed; unsigned floor is 0 which the container already enforces).

## The macro change the implementer must make (the load-bearing structural point)

`impl_i_arith_saturating!` / `impl_u_arith_saturating!` are keyed on the `$bits`
literal but, unlike `impl_*_arith_wrapping_widen!`, they DO NOT take a `$container`
parameter. To write the typed logical-bound literals (`(1 << (N-1)) - 1` typed as the
container) the implementer must thread `$container` into the saturating macros exactly
as the widen macros already do, then call the Precise saturating macro with the
container per width band (i8/i16/.../i128, u8/.../u128), mirroring the widen call sites
at arith.rs lines 357-518.

`$bits` inside the body is the literal N, so `N - 1`, `1 << (N - 1)` etc. are
const-evaluable directly; no extra const-generic plumbing is required. (`body_shape.rs`
proves the const-generic-N form type-checks; the macro form substitutes the literal.)

## Gotchas an implementer must know

1. **DoubleLogical invariant is what removes the widen.** Precise container = 2x
   logical, so for logical N the container has >= 2N bits. Then `N - 1 < container_bits`
   and `1 << (N - 1)` fits with room to spare; the product of two N-bit-logical operands
   fits the 2x container before the clamp. This is why B needs only a clamp, never a
   256-bit widen. (If a future Precise band ever became Min-container, B would also need
   A's widen; today Precise is DoubleLogical across its whole 1..64 range.)

2. **Clamp AFTER rescale.** Clamp the `(prod >> FRAC)` result, not the raw product:
   the logical bound is in rescaled (raw-at-FRAC) units. Clamping the pre-shift product
   would clamp at the wrong scale.

3. **Replace `saturating_mul` with `wrapping_mul` + logical clamp for the fixed-point
   arm.** The current Precise `i_mul_fixed` does `a.saturating_mul(b) >> FRAC`, which
   saturates at the CONTAINER bound before the shift (the bug). The fix multiplies into
   the 2x container (no container overflow for in-range DoubleLogical operands) and
   clamps at the logical bound after the shift. For add/sub/div the existing
   `saturating_*` container ops are fine as the inner op (they cannot exceed the
   container), and the logical clamp on top brings them to the logical bound.

4. **Arvo-wide scope.** The KNOWN BUG comment in arith.rs (above
   `impl_u_arith_saturating!`) states this affects EVERY Precise op. The clamp tail in
   this sketch is the single mechanism that fixes all of add/sub/mul/div/mul_fixed;
   apply it uniformly. The catalogue test only pins `i_mul_fixed`; add per-op
   regression tests for the others when landing.

5. **No new unstable feature.** B uses only plain const arithmetic and shifts already
   available; nothing beyond arvo-strategy's existing feature set. (Contrast with A,
   which needs `const_unsigned_bigint_helpers`.)
