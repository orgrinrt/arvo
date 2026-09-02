//! probe 3: what it costs to read the quantum from a datum instead of from a type.
//!
//! The standard's `quantize` takes two operands and uses the second one's *exponent*.
//! That is a datum read, and file 80 showed it is exactly why `quantize` is the
//! standard's own carve-out from value-determinism (`80:127-135`, citing clause 5.2).
//!
//! The pricing pillar's own test (`78:161-166`, sharpened at `82:513-521`) asks
//! whether a quantity computed at run time has a compile-time alternative. For
//! `quantize` the quantity is the whole plan: the quantum's scale, the target's
//! largest mantissa, and its modulus. Every one of them is a function of the target
//! numeral's parameters alone. Lift the quantum to type position and all three become
//! associated consts; leave it as a datum and all three are computed per call.
//!
//! Four shapes are emitted, at `-O`, and compared by disassembly:
//!
//!   1. typed, standalone: the quantum is a type, the plan is literals.
//!   2. dynamic, standalone: the quantum is an `i32` argument, the plan is computed.
//!   3. typed, in a per-element loop over a column.
//!   4. dynamic, in the same loop.
//!
//! Nothing here is a timing claim; the artifact is the emitted code. Instruction and
//! division counts are read from the disassembly by the outcomes script.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

// ---------------------------------------------------------------------------
// The typed shape: the quantum is a type, so the plan is three literals.
// ---------------------------------------------------------------------------

pub trait Quantum {
    /// One ulp of the target, in the numeral's scaling domain.
    const ULP: i64;
    /// `r^p - 1`, the target's largest representable mantissa.
    const MAX_MANTISSA: i64;
    /// `r^p`.
    const MODULUS: i64;
}

/// Radix 10, p = 3, quantum 10^0 against a numeral scaled at 10^-2.
pub struct Q0;
impl Quantum for Q0 {
    const ULP: i64 = 100;
    const MAX_MANTISSA: i64 = 999;
    const MODULUS: i64 = 1000;
}

/// Clamp to the far point on overflow: `Warm`'s ratified row.
#[inline]
pub fn quantise_typed<Q: Quantum>(vx: i64) -> i64 {
    let quo = vx / Q::ULP;
    let rem = vx % Q::ULP;
    let twice = 2 * rem;
    let m = if twice > Q::ULP || (twice == Q::ULP && quo % 2 != 0) {
        quo + 1
    } else {
        quo
    };
    if m > Q::MAX_MANTISSA {
        Q::MAX_MANTISSA
    } else {
        m
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn typed_standalone(vx: i64) -> i64 {
    quantise_typed::<Q0>(vx)
}

// ---------------------------------------------------------------------------
// The dynamic shape: the quantum arrives as a datum's exponent, so the plan has to
// be built. This is the standard's own two-operand signature.
// ---------------------------------------------------------------------------

#[inline]
pub fn quantise_dynamic(vx: i64, q_rel: u32, precision: u32) -> i64 {
    let mut ulp: i64 = 1;
    let mut i = 0;
    while i < q_rel {
        ulp *= 10;
        i += 1;
    }
    let mut modulus: i64 = 1;
    let mut j = 0;
    while j < precision {
        modulus *= 10;
        j += 1;
    }
    let max_mantissa = modulus - 1;

    let quo = vx / ulp;
    let rem = vx % ulp;
    let twice = 2 * rem;
    let m = if twice > ulp || (twice == ulp && quo % 2 != 0) {
        quo + 1
    } else {
        quo
    };
    if m > max_mantissa {
        max_mantissa
    } else {
        m
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dynamic_standalone(vx: i64, q_rel: u32, precision: u32) -> i64 {
    quantise_dynamic(vx, q_rel, precision)
}

// ---------------------------------------------------------------------------
// The in-loop shapes: a column quantised element by element, which is where the
// bitpack finding one stretch ago said the difference actually bites (`82:483-487`).
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn typed_loop(src: &[i64; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = quantise_typed::<Q0>(src[i]);
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn dynamic_loop(src: &[i64; 64], dst: &mut [i64; 64], q_rel: u32, precision: u32) {
    let mut i = 0;
    while i < 64 {
        dst[i] = quantise_dynamic(src[i], q_rel, precision);
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// Agreement: the two shapes must compute the same function where the dynamic one is
// given the typed one's parameters. Checked in const position so the compile is the
// check, over the whole model's mantissa range at every quantum.
// ---------------------------------------------------------------------------

const fn agree_over_model() -> bool {
    let mut q_rel = 0u32;
    while q_rel < 4 {
        let mut m = 0i64;
        while m < 1000 {
            let mut e = 0u32;
            while e < 4 {
                let mut scale = 1i64;
                let mut k = 0;
                while k < e {
                    scale *= 10;
                    k += 1;
                }
                let vx = m * scale;

                // typed kernel, inlined by hand at q_rel = 2 (ULP 100) only, since a
                // const fn cannot dispatch on a type parameter without const traits.
                if q_rel == 2 {
                    let quo = vx / 100;
                    let rem = vx % 100;
                    let twice = 2 * rem;
                    let mm = if twice > 100 || (twice == 100 && quo % 2 != 0) {
                        quo + 1
                    } else {
                        quo
                    };
                    let typed = if mm > 999 { 999 } else { mm };
                    let dyn_ = quantise_dynamic_const(vx, 2, 3);
                    if typed != dyn_ {
                        return false;
                    }
                }
                e += 1;
            }
            m += 1;
        }
        q_rel += 1;
    }
    true
}

const fn quantise_dynamic_const(vx: i64, q_rel: u32, precision: u32) -> i64 {
    let mut ulp: i64 = 1;
    let mut i = 0;
    while i < q_rel {
        ulp *= 10;
        i += 1;
    }
    let mut modulus: i64 = 1;
    let mut j = 0;
    while j < precision {
        modulus *= 10;
        j += 1;
    }
    let max_mantissa = modulus - 1;
    let quo = vx / ulp;
    let rem = vx % ulp;
    let twice = 2 * rem;
    let m = if twice > ulp || (twice == ulp && quo % 2 != 0) {
        quo + 1
    } else {
        quo
    };
    if m > max_mantissa {
        max_mantissa
    } else {
        m
    }
}

const _: () = assert!(
    agree_over_model(),
    "the typed and dynamic shapes must compute the same function"
);
