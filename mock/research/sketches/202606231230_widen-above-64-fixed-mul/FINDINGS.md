# Findings: >64-bit-logical widening fixed-point multiply

**Hypothesis:** the fixed-point multiply at 128-bit container (Hot/Cold N=65..128,
Warm/Precise N=33..64) can compute the overflow-safe `(a*b) >> FRAC` in const
context by forming a 256-bit intermediate product, arithmetic-shifting it right by
FRAC, and narrowing to the container. Candidate built it from the (now runtime-stable)
`u128::carrying_mul` full-multiply, which yields the two u128 limbs of the 256-bit
product directly.

**Outcome: WORKS.** `sketch_a.rs` compiles and runs on `nightly-2026-05-28`
(`rustc 1.98.0-nightly`). It reproduces the catalogue target
(`i_mul_fixed::<30>(1<<64, 1<<64) == 1<<98`, both signed and unsigned) plus the
FRAC corners (0, 128, >128) and, critically, the negative floor-rounding case the
catalogue test does NOT cover.

Run: `rustc +nightly-2026-05-28 --edition 2024 sketch_a.rs -o /tmp/sa && /tmp/sa`
prints `ALL A PASS`.

## The working shape

256-bit value = two u128 limbs `(lo, hi)`. `u128::carrying_mul(b, 0)` returns
`(low, high)` of the full 128x128 product, so the 256-bit intermediate is one call,
no manual limb splitting:

```rust
const fn umul256(a: u128, b: u128) -> (u128, u128) {
    a.carrying_mul(b, 0) // (low, high)
}

// 256-bit logical shift-right by FRAC, low u128 only (the narrowed container value).
const fn shr256_lo(lo: u128, hi: u128, frac: u32) -> u128 {
    if frac == 0 { lo }
    else if frac < 128 { (lo >> frac) | (hi << (128 - frac)) }
    else if frac == 128 { hi }
    else { hi >> (frac - 128) }
}

const fn u_mul_fixed_128(a: u128, b: u128, frac: u32) -> u128 {
    let (lo, hi) = umul256(a, b);
    shr256_lo(lo, hi, frac)
}
```

Signed: multiply magnitudes through the unsigned path, reapply sign, and CORRECT
FOR FLOOR (this is the part the magnitude/sign approach gets wrong by default):

```rust
const fn i_mul_fixed_128(a: i128, b: i128, frac: u32) -> i128 {
    let neg = (a < 0) != (b < 0);
    let (lo, hi) = umul256(a.unsigned_abs(), b.unsigned_abs());
    let mag = shr256_lo(lo, hi, frac);
    if !neg { mag as i128 }
    else {
        let dropped = if frac == 0 { false }
            else if frac < 128 { (lo & ((1u128 << frac) - 1)) != 0 }
            else if frac == 128 { lo != 0 }
            else { lo != 0 || (hi & ((1u128 << (frac - 128)) - 1)) != 0 };
        let m = mag as i128;
        if dropped { -m - 1 } else { -m }   // floor toward minus infinity
    }
}
```

## The feature gate the implementer must vet (load-bearing)

The body needs exactly one unstable gate: **`#![feature(const_unsigned_bigint_helpers)]`**
(tracking issue rust-lang/rust#152015). The METHOD `carrying_mul` is runtime-stable
(stabilized in Rust 1.91); only its CONST use is gated, and only that one gate is
required. No `bigint_helper_methods` (that name is now unknown on the pin), no
`signed_bigint_helpers`.

Per `unstable-features.md`, enabling a new `#![feature(...)]` in a shipping crate
root (here `arvo-strategy/src/lib.rs`) requires a vetting row before it lands. Vet
`const_unsigned_bigint_helpers` (read #152015): it is the const-stability tracker for
already-stable methods, no `I-unsound`, the const machinery is the only blocker, so it
fits the ALLOWED tier. "Mechanical to apply" is true ONLY after that row exists.

## Gotchas an implementer must know

1. **Floor rounding, not truncation.** The existing rescale is an arithmetic right
   shift (floor toward minus infinity), pinned by `i_mul_fixed_negative_floor_rounding`.
   A naive multiply-magnitudes-reapply-sign truncates toward zero and silently
   regresses negatives. The `dropped` correction above restores floor. The catalogue
   test uses positive operands, so it would go green with the bug present; the negative
   cases in the sketch (T3, T4) are what guard it. Add a negative >64-bit regression
   test when landing.

2. **`i128::MIN` is safe.** `i128::MIN.unsigned_abs()` returns `2^127` as u128 with no
   overflow, so the magnitude path has no MIN corner. (Plain `(-x)` would overflow; do
   not use it.)

3. **Shift corners.** Guard `frac == 0` (a `<< 128` in the
   `lo >> frac | hi << (128-frac)` arm is UB), `frac == 128`, and `frac >= 129`. The
   four-way branch covers all of `[0, 256)`. FRAC values in arvo are bounded by the
   type's `F`, so frac < 256 always, but the branches must still exist.

4. **Macro wiring.** `impl_i_arith_wrapping_widen!` / `impl_u_arith_wrapping_widen!`
   already take a `$container` param and already pass `$bits` (the literal N). The new
   256-bit body replaces only the `i_mul_fixed` / `u_mul_fixed` arms there, OR a new
   `impl_*_arith_wrapping_widen256!` macro is added for the 65..128 ranges that currently
   route through the non-widening `impl_*_arith_wrapping!`. The 33..64 Warm/Precise ranges
   that hit the 128-bit container similarly need the widen-256 body. Container is i128/u128
   in all these cases, so `umul256` (u128) is the single primitive; signed wraps it.

5. **Pure scalar, const-clean.** No alloc, no WideBits needed (the catalogue framing
   mentioned WideBits as a candidate; it is unnecessary at the 128-bit container, the
   two-u128-limb pair suffices). WideBits arithmetic is only relevant for the N>128
   container, which is out of scope for these two reds.

## Candidates evaluated

- `u128::widening_mul` / `i128::widening_mul`: **do not exist** on 128-bit types
  (E0599; they exist only on narrower types). Rejected.
- `i128::carrying_mul`: exists but needs `signed_bigint_helpers` (method, E0658,
  #151989) AND a const gate. Two unstable surfaces vs the unsigned path's one.
  Rejected in favour of unsigned-magnitudes + floor correction.
- Manual 4x u64 schoolbook limbs: works in principle but is more code and more
  corner-prone than the single `carrying_mul` call. Not needed since `carrying_mul`
  const-compiles on the pin.
- `WideBits` arithmetic: unnecessary at 128-bit container (see gotcha 5).
