//! The per-strategy arithmetic impl macros.
//!
//! Split out of `arith.rs`, which carried the trait declarations, these
//! macros, their several hundred invocations, the scalar euclidean
//! contract and the identity surface behind one name. Every one of them
//! is `pub(crate) use`-exported so `arith_impls` can invoke them; a
//! `macro_rules!` is otherwise scoped to the module that defines it.

// already selected.
macro_rules! impl_u_arith_wrapping {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) >> FRAC }
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    // DoubleLogical container holds `a << FRAC` for F <= N; div-by-zero returns the numerator.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        // `wrapping_div` for uniformity with the signed macros, where `/` panics
                        // on MIN / -1. Unsigned division cannot overflow once the zero divisor is
                        // guarded above, so this is the same operation; one spelling across both
                        // stops a reader copying the unguarded form into a signed body.
                        (a << FRAC).wrapping_div(b)
                    }
                }
            }
        )+
    };
}

// Precise saturates at the LOGICAL bound, not the container bound (design topic
// `*_topic.precise-saturates-at-container-not-logical-bound.md`, round 202606231229). Precise uses a
// DoubleLogical (2x) container, so a result that exceeds the logical `0..=(1<<N)-1` range can still fit the
// container. The contract is that Precise clamps at the logical width N, so every op computes its result in
// the wide container then clamps to the logical bound. The bound is derived at const time from the
// const-generic width `$bits` over the dispatched container type, so no `$container` threading or per-width
// regrouping is needed.
macro_rules! impl_u_arith_saturating {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let v = a.saturating_add(b);
                    if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.saturating_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let v = a.saturating_mul(b);
                    if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero: clamp to the logical MAX.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        hi
                    } else {
                        let v = a / b;
                        if v > hi { hi } else { v }
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    // DoubleLogical container holds the 2N product, so wrapping is exact; clamp to logical.
                    let v = a.wrapping_mul(b) >> FRAC;
                    if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Unsigned>>::T = (1 << $bits) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero: clamp to the logical MAX. DoubleLogical holds
                    // `a << FRAC` for F <= N; clamp the quotient to the logical bound.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        hi
                    } else {
                        // `saturating_div` for uniformity with the signed macros. Unsigned
                        // division cannot overflow once the zero divisor is guarded above, so
                        // this is the same operation; one spelling across both stops a reader
                        // copying the unguarded form into a signed body.
                        let v = (a << FRAC).saturating_div(b);
                        if v > hi { hi } else { v }
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_wrapping {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) >> FRAC }
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    // DoubleLogical container holds `a << FRAC` for F <= N; div-by-zero returns the numerator.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        // `wrapping_div`, not `/`. Signed MIN / -1 has no representable answer and
                        // `/` panics on it, which a never-panic strategy must not do.
                        (a << FRAC).wrapping_div(b)
                    }
                }
            }
        )+
    };
}

// Signed counterpart of the Precise logical-bound clamp (see the unsigned macro). The logical bound is
// `-(1<<(N-1)) ..= (1<<(N-1))-1`, derived at const time from the const-generic width `$bits`.
macro_rules! impl_i_arith_saturating {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    let v = a.saturating_add(b);
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    let v = a.saturating_sub(b);
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    let v = a.saturating_mul(b);
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero, and clamps to the bound on the
                    // side the quotient heads toward. Reaching for `hi` unconditionally makes
                    // `-5 / 0` positive, which is the wrong end of the range and not a clamp of
                    // anything; `lo` above was unreachable from this branch.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        if a < <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY { lo } else { hi }
                    } else {
                        // Guard signed overflow (MIN / -1) with `saturating_div`, then clamp to logical.
                        let v = a.saturating_div(b);
                        if v < lo { lo } else if v > hi { hi } else { v }
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    // DoubleLogical container holds the 2N product; arithmetic-shift floors, then clamp.
                    let v = a.wrapping_mul(b) >> FRAC;
                    if v < lo { lo } else if v > hi { hi } else { v }
                }
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let hi: <Self as BitsContainerFor<$bits, Signed>>::T = (1 << ($bits - 1)) - 1; // lint:allow(no-bare-numeric) reason: const logical-bound max over the dispatched container primitive; tracked: #256
                    let lo: <Self as BitsContainerFor<$bits, Signed>>::T = -(1 << ($bits - 1)); // lint:allow(no-bare-numeric) reason: const logical-bound min over the dispatched container primitive; tracked: #256
                    // Precise never panics on div-by-zero, and clamps to the bound on the
                    // side the quotient heads toward, exactly as the sibling `i_div` does.
                    // DoubleLogical holds `a << FRAC` for F <= N; the quotient is clamped to
                    // the logical bound below.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        if a < <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY { lo } else { hi }
                    } else {
                        // `saturating_div`, matching the sibling `i_div` above. `/` panics on
                        // signed MIN / -1, and `wrapping_div` answers it with the minimum, which
                        // the clamp below reads as already inside the bound and passes through.
                        // The mathematical answer is +2^(N-1), so a saturating strategy owes the
                        // maximum here; wrapping and saturating disagree in sign at this one input.
                        let v = (a << FRAC).saturating_div(b);
                        if v < lo { lo } else if v > hi { hi } else { v }
                    }
                }
            }
        )+
    };
}

// Widening fixed-point multiply for Min-container wrapping strategies (Hot / Cold) at logical widths
// 1..=64. Their container equals the logical width, so a raw product overflows before the `>> FRAC`
// rescale; the fixed-point multiply widens to the native 2x type (i128/u128, wide enough for any product
// of two <=64-bit-container values), multiplies, shifts, and narrows back to `$container`. add/sub/mul/div
// are the same wrapping ops as `impl_*_arith_wrapping`. The `$container` parameter names the concrete
// container so the widen / narrow casts resolve. Logical widths 65..=128 (i128/u128 container) stay on the
// non-widening macro: their 2x is a 256-bit WideBits, deferred until a >64-bit fixed-point multiply is
// first needed (the non-widening body is correct for FRAC == 0).
macro_rules! impl_u_arith_wrapping_widen {
    ($strategy:ty, $container:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    let ac: $container = a;
                    let bc: $container = b;
                    let prod: u128 = (ac as u128).wrapping_mul(bc as u128); // lint:allow(no-bare-numeric) reason: native 2x widen target for the fixed-point multiply; tracked: #256
                    let narrowed: $container = (prod >> FRAC) as $container;
                    narrowed
                }
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    // (a << FRAC) / b widened to the native 2x so `a << FRAC` does not overflow `$container`.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        let num: u128 = (a as u128) << FRAC; // lint:allow(no-bare-numeric) reason: native 2x widen numerator for the fixed-point divide; tracked: #256
                        (num / (b as u128)) as $container // lint:allow(no-bare-numeric) reason: native 2x widen divisor for the fixed-point divide; tracked: #256
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_wrapping_widen {
    ($strategy:ty, $container:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    let ac: $container = a;
                    let bc: $container = b;
                    let prod: i128 = (ac as i128).wrapping_mul(bc as i128); // lint:allow(no-bare-numeric) reason: native 2x widen target for the fixed-point multiply; tracked: #256
                    let narrowed: $container = (prod >> FRAC) as $container;
                    narrowed
                }
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    // (a << FRAC) / b widened to the native 2x; signed `/` truncates toward zero.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        let num: i128 = (a as i128) << FRAC; // lint:allow(no-bare-numeric) reason: native 2x widen numerator for the fixed-point divide; tracked: #256
                        (num / (b as i128)) as $container // lint:allow(no-bare-numeric) reason: native 2x widen divisor for the fixed-point divide; tracked: #256
                    }
                }
            }
        )+
    };
}

// 256-bit intermediate for the >64-bit-logical fixed-point widening multiply. Min-container Hot / Cold at
// logical 65..=128 use a u128 / i128 container; the product of two 128-bit values needs 256 bits before the
// `>> FRAC` rescale. These const helpers form the 256-bit product as two u128 limbs and shift it back to one
// container word. All const-evaluable on the pinned nightly.
//
// SAFETY (const_unsigned_bigint_helpers carve-out, vetted ALLOWED, tracking rust-lang/rust#152015):
// `u128::carrying_mul` is runtime-stable since 1.91; only its const use is gated. Pure arithmetic, no
// soundness hole, and core uses the bigint helpers internally. `carrying_mul(b, 0)` returns the
// `(low, high)` u128 limbs of the full 128x128 product.
//
// TODO(perf, task #4 / research 202606231500): `carrying_mul` is portable (LLVM lowers u128 mul on every
// target, native 128-bit where available, software multi-limb / `__multi3` libcall where not), so this is
// correct on all hardware with no fallback needed. It is NOT hardware-gated. The open item is a
// perf-optimal per-target path: on targets where the generic i128 lowering is suboptimal (32-bit, no
// 128-bit-mul ISA), a cfg-gated explicit limb / intrinsic route may beat it. Add cfg-gated arms per
// arvo's always-optimal-internals (Kind 1 structural lowering), bench-driven, when a target warrants it.
#[inline(always)]
pub(crate) const fn umul256(a: u128, b: u128) -> (u128, u128) {
    // lint:allow(no-bare-numeric) reason: 256-bit widen limbs for the fixed-point multiply; tracked: #256
    a.carrying_mul(b, 0) // lint:allow(no-bare-numeric) reason: carrying_mul carry-in seed; tracked: #256
}

// 256-bit logical shift-right by `frac`, returning the low u128 (the narrowed container word).
#[inline(always)]
pub(crate) const fn shr256_lo(lo: u128, hi: u128, frac: u32) -> u128 {
    // lint:allow(no-bare-numeric) reason: 256-bit widen limbs; tracked: #256
    if frac == 0 {
        // lint:allow(no-bare-numeric) reason: shift-amount compare; tracked: #256
        lo
    } else if frac < 128 {
        // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
        (lo >> frac) | (hi << (128 - frac)) // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
    } else if frac == 128 {
        // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
        hi
    } else {
        hi >> (frac - 128) // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
    }
}

// Unsigned 128-bit-container fixed-point multiply: full 256-bit product, shift, narrow.
#[inline(always)]
pub(crate) const fn u_mul_fixed_128(a: u128, b: u128, frac: u32) -> u128 {
    // lint:allow(no-bare-numeric) reason: 256-bit widen limbs; tracked: #256
    let (lo, hi) = umul256(a, b);
    shr256_lo(lo, hi, frac)
}

// Signed 128-bit-container fixed-point multiply: magnitude product, arithmetic-shift floor toward minus
// infinity (subtract 1 when the result is negative and any shifted-out low bit was set), reapply sign. This
// matches the `>> FRAC` floor the 1..=64 native-widen path and the catalogue assertions use.
#[inline(always)]
pub(crate) const fn i_mul_fixed_128(a: i128, b: i128, frac: u32) -> i128 {
    // lint:allow(no-bare-numeric) reason: 256-bit widen limbs; tracked: #256
    let neg = (a < 0) != (b < 0); // lint:allow(no-bare-numeric) reason: sign test; tracked: #256
    let (lo, hi) = umul256(a.unsigned_abs(), b.unsigned_abs());
    let mag = shr256_lo(lo, hi, frac);
    if !neg {
        mag as i128 // lint:allow(no-bare-numeric) reason: magnitude back to signed container; tracked: #256
    } else {
        let dropped = if frac == 0 {
            // lint:allow(no-bare-numeric) reason: shift-amount compare; tracked: #256
            false
        } else if frac < 128 {
            // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
            (lo & ((1u128 << frac) - 1)) != 0 // lint:allow(no-bare-numeric) reason: low-bit drop mask; tracked: #256
        } else if frac == 128 {
            // lint:allow(no-bare-numeric) reason: u128 limb width; tracked: #256
            lo != 0 // lint:allow(no-bare-numeric) reason: low-limb drop test; tracked: #256
        } else {
            lo != 0 || (hi & ((1u128 << (frac - 128)) - 1)) != 0 // lint:allow(no-bare-numeric) reason: drop mask above 128; tracked: #256
        };
        let m = mag as i128; // lint:allow(no-bare-numeric) reason: magnitude back to signed container; tracked: #256
                             // Wrapping negation, not `-m`. At the container minimum the magnitude
                             // is 2^127, which casts to `i128::MIN`, and negating that overflows: a
                             // panic from the two strategies that promise not to panic, and a hard
                             // error under const evaluation. `wrapping_neg` is identical for every
                             // other input and yields the minimum itself there, which is the correct
                             // two's-complement answer.
        if dropped {
            m.wrapping_neg().wrapping_sub(1) // lint:allow(no-bare-numeric) reason: floor correction; tracked: #256
        } else {
            m.wrapping_neg()
        }
    }
}

// Widening fixed-point multiply for Min-container Hot / Cold at logical 65..=128 (u128 / i128 container).
// add / sub / mul / div are the same wrapping ops as `impl_*_arith_wrapping`; only `*_mul_fixed` routes
// through the 256-bit helper, because at a 128-bit container the 2x intermediate is 256 bits. The 1..=64
// path stays on `impl_*_arith_wrapping_widen` (native i128 / u128 intermediate).
macro_rules! impl_u_arith_wrapping_widen256 {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const UArith<$bits> for $strategy {
                #[inline(always)]
                fn u_add(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn u_sub(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn u_mul(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn u_div(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T)
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn u_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T { u_mul_fixed_128(a, b, FRAC as u32) } // lint:allow(no-bare-numeric) reason: FRAC widened to the helper shift-amount type; tracked: #256
                #[inline(always)]
                fn u_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Unsigned>>::T, b: <Self as BitsContainerFor<$bits, Unsigned>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Unsigned>>::T {
                    // CATALOGUE (tracked, task #5): correct only when `a << FRAC` fits the u128 container; the
                    // 65..=128 case needs 256/128 long division (no `carrying_div` intrinsic). The ignored
                    // catalogue test pins the target. div-by-zero returns the numerator.
                    if b == <<Self as BitsContainerFor<$bits, Unsigned>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        // `wrapping_div` for uniformity with the signed macros, where `/` panics
                        // on MIN / -1. Unsigned division cannot overflow once the zero divisor is
                        // guarded above, so this is the same operation; one spelling across both
                        // stops a reader copying the unguarded form into a signed body.
                        (a << FRAC).wrapping_div(b)
                    }
                }
            }
        )+
    };
}

macro_rules! impl_i_arith_wrapping_widen256 {
    ($strategy:ty, $($bits:literal),+) => {
        $(
            impl const IArith<$bits> for $strategy {
                #[inline(always)]
                fn i_add(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_add(b) }
                #[inline(always)]
                fn i_sub(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_sub(b) }
                #[inline(always)]
                fn i_mul(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { a.wrapping_mul(b) }
                #[inline(always)]
                fn i_div(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T)
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        a.wrapping_div(b)
                    }
                }
                #[inline(always)]
                fn i_mul_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T { i_mul_fixed_128(a, b, FRAC as u32) } // lint:allow(no-bare-numeric) reason: FRAC widened to the helper shift-amount type; tracked: #256
                #[inline(always)]
                fn i_div_fixed<const FRAC: u16>(a: <Self as BitsContainerFor<$bits, Signed>>::T, b: <Self as BitsContainerFor<$bits, Signed>>::T) // lint:allow(no-bare-numeric) reason: const-generic shift carrier mirrors const N: u16; tracked: #256
                    -> <Self as BitsContainerFor<$bits, Signed>>::T {
                    // CATALOGUE (tracked, task #5): correct only when `a << FRAC` fits the i128 container; the
                    // 65..=128 case needs 256/128 long division. The ignored catalogue test pins the target.
                    // div-by-zero returns the numerator; signed `/` truncates toward zero.
                    if b == <<Self as BitsContainerFor<$bits, Signed>>::T as Identity<Additive>>::IDENTITY {
                        a
                    } else {
                        // `wrapping_div`, not `/`. Signed MIN / -1 has no representable answer and
                        // `/` panics on it, which a never-panic strategy must not do.
                        (a << FRAC).wrapping_div(b)
                    }
                }
            }
        )+
    };
}

// Wrapping strategies: Hot / Warm / Cold.

pub(crate) use impl_i_arith_saturating;
pub(crate) use impl_i_arith_wrapping;
pub(crate) use impl_i_arith_wrapping_widen;
pub(crate) use impl_i_arith_wrapping_widen256;
pub(crate) use impl_u_arith_saturating;
pub(crate) use impl_u_arith_wrapping;
pub(crate) use impl_u_arith_wrapping_widen;
pub(crate) use impl_u_arith_wrapping_widen256;
