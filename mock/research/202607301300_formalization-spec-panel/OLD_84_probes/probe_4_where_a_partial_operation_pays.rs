//! probe 4: the four homes for a failure that is genuinely not a range event, priced.
//!
//! `quantize`'s hard failure turned out to be a range event on the numeral it targets
//! (probes 1 and 2). The failure kind that is genuinely not a range event is the
//! partial-function kind: division by zero, `Recip` at zero, `Sqrt` of a negative.
//! The design already names it, as the grade's `invalid` / `divideByZero` generators,
//! "causes with no quantiser origin, raised by the operation, on operands, before any
//! rounding" (`50:305`, carried into `58` section 1.14 and unchanged at `78` section
//! 1.14).
//!
//! Naming the generator does not say what value is delivered, and that is the open
//! half. Four homes are available and this probe prices them on the same operation
//! (`recip`, at `i64` with a scaled fixed-point reciprocal) and the same shape (one
//! call, then a per-element loop over a column of 64):
//!
//!   (i)   in the value: an absorbing bottom in a spare pattern.
//!   (ii)  in the grade: a published type-level fact beside the value.
//!   (iii) in the result type: `Outcome<T, DivideByZero>`, checked at every step.
//!   (iv)  at the declaration: the operand's type carries the proof, so the
//!         operation is total and returns `T`.
//!
//! (ii) is not a candidate on its own here, because a grade is a claim *about* a
//! value and `1/0` has no value to attach one to; it is compiled anyway, alongside
//! (i), so that the pairing is visible rather than argued.
//!
//! Nothing here is a timing claim. The artifacts are the emitted code and the sizes.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

/// The scaled reciprocal: `SCALE / x`, a fixed-point reciprocal at one model.
const SCALE: i64 = 1_000_000;

// ---------------------------------------------------------------------------
// (i) in the value: an absorbing bottom carried in a spare pattern.
// ---------------------------------------------------------------------------

/// `i64::MIN` stands in for a numeral's spare pattern. The obligation this home
/// carries is that the bottom absorbs under *every* operation, including selection,
/// which is the `minNum`/`maxNum` defect named at `05:200-215`.
const BOTTOM: i64 = i64::MIN;

#[inline]
pub const fn recip_bottom(x: i64) -> i64 {
    if x == 0 || x == BOTTOM {
        BOTTOM
    } else {
        SCALE / x
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bottom_loop(src: &[i64; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = recip_bottom(src[i]);
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// (iii) in the result type: the refusing carrier.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
pub enum Refusing {
    Ok(i64),
    DivideByZero,
}

#[inline]
pub const fn recip_carrier(x: i64) -> Refusing {
    if x == 0 {
        Refusing::DivideByZero
    } else {
        Refusing::Ok(SCALE / x)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn carrier_loop(src: &[i64; 64], dst: &mut [Refusing; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = recip_carrier(src[i]);
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// (iv) at the declaration: the operand's type carries the proof.
// ---------------------------------------------------------------------------

/// A value carrying the proof that it is nonzero. The design already ships this
/// concept twice over: notko's `NonZeroable`, and `arvo-numeric-contracts`' own
/// `IsNonZero` predicate. `repr(transparent)` so the proof costs no layout, and the
/// field is private so the perimeter rule's own test is met: there is no route to a
/// value of this type that did not pass through the door
/// (`what-you-can-observe-is-what-you-guaranteed.md`).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Nz(i64);

impl Nz {
    /// The one door. This is where the fallibility lives, once, at the point the
    /// fact enters, rather than at every operation that consumes it.
    #[inline]
    pub const fn new(v: i64) -> Option<Nz> {
        if v == 0 {
            None
        } else {
            Some(Nz(v))
        }
    }
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// Total. No carrier, no branch, no grade, no bottom.
#[inline]
pub const fn recip_total(x: Nz) -> i64 {
    SCALE / x.get()
}

#[unsafe(no_mangle)]
pub extern "C" fn total_loop(src: &[Nz; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = recip_total(src[i]);
        i += 1;
    }
}

/// The check the fourth home relocates rather than removes: establishing the column's
/// invariant once, at the boundary, instead of per element per operation. One pass,
/// one branch per element, and every downstream operation on the column is free.
#[unsafe(no_mangle)]
pub extern "C" fn admit_column(src: &[i64; 64], dst: &mut [Nz; 64]) -> bool {
    let mut i = 0;
    while i < 64 {
        match Nz::new(src[i]) {
            Some(v) => dst[i] = v,
            None => return false,
        }
        i += 1;
    }
    true
}

// ---------------------------------------------------------------------------
// Sizes: what each home costs in layout, which is arvo's own identity axis.
// ---------------------------------------------------------------------------

pub const SIZE_PLAIN: usize = core::mem::size_of::<i64>();
pub const SIZE_BOTTOM: usize = core::mem::size_of::<i64>();
pub const SIZE_CARRIER: usize = core::mem::size_of::<Refusing>();
pub const SIZE_NZ: usize = core::mem::size_of::<Nz>();

const _: () = assert!(SIZE_NZ == SIZE_PLAIN, "the proof must cost no layout");
const _: () = assert!(
    SIZE_BOTTOM == SIZE_PLAIN,
    "a bottom in a spare pattern costs no layout"
);
const _: () = assert!(
    SIZE_CARRIER > SIZE_PLAIN,
    "the carrier is the home that costs layout"
);

// The three homes agree wherever the operand is admissible, checked in const position
// over a spread that includes both signs, one, and the extremes.
const PROBES: [i64; 9] = [1, -1, 2, -2, 1000, -1000, i64::MAX, i64::MIN + 1, 7];

const fn homes_agree() -> bool {
    let mut i = 0;
    while i < PROBES.len() {
        let x = PROBES[i];
        let want = SCALE / x;
        if recip_bottom(x) != want {
            return false;
        }
        match recip_carrier(x) {
            Refusing::Ok(v) if v == want => {}
            _ => return false,
        }
        match Nz::new(x) {
            Some(nz) if recip_total(nz) == want => {}
            _ => return false,
        }
        i += 1;
    }
    // and all three treat zero as the failure, each in its own way
    if recip_bottom(0) != BOTTOM {
        return false;
    }
    if !matches!(recip_carrier(0), Refusing::DivideByZero) {
        return false;
    }
    if Nz::new(0).is_some() {
        return false;
    }
    true
}

const _: () = assert!(
    homes_agree(),
    "the three homes must agree on admissible operands"
);

// The bottom home's real obligation, compiled rather than argued: an absorbing
// element must absorb under selection too, and a plain total order does not give
// that. `i64::MIN` sorts *below* every value, so a running maximum silently discards
// it; `05:200-215` names this as exactly IEEE 754-2008's `minNum`/`maxNum` defect,
// replaced in 754-2019 by propagating `minimum`/`maximum`.
const fn max_loses_the_bottom() -> bool {
    let vals = [3i64, BOTTOM, 5i64];
    let mut acc = vals[0];
    let mut i = 1;
    while i < 3 {
        if vals[i] > acc {
            acc = vals[i];
        }
        i += 1;
    }
    acc != BOTTOM // the bottom was discarded by the selection
}

const _: () = assert!(
    max_loses_the_bottom(),
    "a bottom that sorts low is discarded by a total-order maximum: this is the \
     obligation the value home carries, not a free consequence"
);
