//! Probe: is the acceptance criterion's pipeline expressible without forbidden features?
//!
//! Op's criterion: "have the typestate derive the matching container and numeral
//! representations, then validate, and erase". This probe checks, on the pinned
//! nightly (nightly-2026-05-28), with ZERO feature gates, that each verb of that
//! sentence has an expressible form:
//!
//!   derive   : a demand (value window) projects, through plain trait impls, to a
//!              storage representation and a distinct compute representation, and
//!              each names its container. Width is a type-level marker here, not a
//!              bare const, per the refused-bound decomposition discipline.
//!   validate : coverage, round-trip between the two representations through the
//!              abstract value, a redundant representation's normalisation law, and
//!              the law inventory of three overflow policies, all checked
//!              EXHAUSTIVELY in const context at a 4-bit model width.
//!   erase    : a repr(transparent) carrier over the derived container has exactly
//!              the container's size.
//!
//! The probe also computes two facts the deliverable relies on:
//!   1. the same numeral (4-bit two's complement) hosts operation families with
//!      DIFFERENT laws: wrapping addition is associative (exhaustive), signed
//!      saturating addition is not (counterexample found by exhaustive search),
//!      and unsigned saturating addition IS associative (exhaustive). The law
//!      outcome depends on (window, policy), not on the numeral.
//!   2. a redundant compute representation (a carry-save pair) preserves value
//!      under the 3:2 compressor step and names one value by several numerals.
//!
//! What this probe does NOT establish: the general width-to-container projection
//! for an arbitrary const N (that is a separate, known-hard question), any
//! performance claim, and anything about widths above the 4-bit model beyond the
//! uniformity of the constructions.
//!
//! Everything a probe decides incidentally (names, arities, which window) is
//! scaffolding, not design.

#![no_std]
#![deny(warnings)]

use core::mem::size_of;

// ---------------------------------------------------------------------------
// format tier: containers. A container is a bit box with no value semantics.
// ---------------------------------------------------------------------------

pub trait Container {
    type Raw: Copy;
    const BITS: u32;
}

pub struct C8;
impl Container for C8 {
    type Raw = u8;
    const BITS: u32 = 8;
}

pub struct C16;
impl Container for C16 {
    type Raw = u16;
    const BITS: u32 = 16;
}

// ---------------------------------------------------------------------------
// representation tier: a value map over a container. The abstract carrier is
// modelled as i64 (a stand-in; a shipping design carries laws, not a model type).
// ---------------------------------------------------------------------------

pub trait Representation {
    type C: Container;
    /// coverage window in the abstract carrier
    const LO: i64;
    const HI: i64;
    /// does one value have more than one numeral in this representation
    const REDUNDANT: bool;
}

/// storage representation: 4 significant bits, biased by +3. Window [-3, 12].
/// A deliberately non-power-of-two, non-symmetric window: the Cold shape.
pub struct Biased4InC8;
impl Representation for Biased4InC8 {
    type C = C8;
    const LO: i64 = -3;
    const HI: i64 = 12;
    const REDUNDANT: bool = false;
}
pub const fn biased4_decode(raw: u8) -> i64 {
    (raw & 0x0F) as i64 - 3
}
pub const fn biased4_encode(v: i64) -> u8 {
    ((v + 3) as u8) & 0x0F
}

/// compute representation of the same demand: two's complement in the full u8 box.
pub struct Twos8;
impl Representation for Twos8 {
    type C = C8;
    const LO: i64 = -128;
    const HI: i64 = 127;
    const REDUNDANT: bool = false;
}
pub const fn twos8_decode(raw: u8) -> i64 {
    raw as i8 as i64
}
pub const fn twos8_encode(v: i64) -> u8 {
    v as i8 as u8
}

// ---------------------------------------------------------------------------
// derivation tier: a demand projects to representations, one per role.
// "representations", plural, is the criterion's own word.
// ---------------------------------------------------------------------------

pub trait Demand {
    const LO: i64;
    const HI: i64;
}

/// the demanded value window: integers in [-3, 12]
pub struct WindowM3To12;
impl Demand for WindowM3To12 {
    const LO: i64 = -3;
    const HI: i64 = 12;
}

pub trait DeriveStorage: Demand {
    type Storage: Representation;
}
pub trait DeriveCompute: Demand {
    type Compute: Representation;
}
impl DeriveStorage for WindowM3To12 {
    type Storage = Biased4InC8;
}
impl DeriveCompute for WindowM3To12 {
    type Compute = Twos8;
}

// ---------------------------------------------------------------------------
// validate: coverage. Each derived representation covers the demanded window.
// ---------------------------------------------------------------------------

const fn covers(rep_lo: i64, rep_hi: i64, dem_lo: i64, dem_hi: i64) -> bool {
    rep_lo <= dem_lo && dem_hi <= rep_hi
}

const _: () = assert!(covers(
    <<WindowM3To12 as DeriveStorage>::Storage as Representation>::LO,
    <<WindowM3To12 as DeriveStorage>::Storage as Representation>::HI,
    <WindowM3To12 as Demand>::LO,
    <WindowM3To12 as Demand>::HI,
));
const _: () = assert!(covers(
    <<WindowM3To12 as DeriveCompute>::Compute as Representation>::LO,
    <<WindowM3To12 as DeriveCompute>::Compute as Representation>::HI,
    <WindowM3To12 as Demand>::LO,
    <WindowM3To12 as Demand>::HI,
));

// ---------------------------------------------------------------------------
// validate: round-trip. Exhaustive over the demanded window, in const context.
// storage -> value -> compute -> value -> storage is the identity on the window.
// ---------------------------------------------------------------------------

const fn roundtrip_storage_compute() -> bool {
    let mut v = <WindowM3To12 as Demand>::LO;
    while v <= <WindowM3To12 as Demand>::HI {
        if biased4_decode(biased4_encode(v)) != v {
            return false;
        }
        if twos8_decode(twos8_encode(v)) != v {
            return false;
        }
        if biased4_decode(biased4_encode(twos8_decode(twos8_encode(v)))) != v {
            return false;
        }
        v += 1;
    }
    true
}
const _: () = assert!(roundtrip_storage_compute());

// ---------------------------------------------------------------------------
// validate: a redundant compute representation and its normalisation law.
// A carry-save pair (s, c) names the value (s + c) mod 16. The 3:2 compressor
// absorbs an operand without propagating carries; the law is that value is
// preserved. Checked exhaustively over all 16^3 triples in const context.
// ---------------------------------------------------------------------------

const M4: u8 = 0x0F;

const fn cs_value(s: u8, c: u8) -> u8 {
    s.wrapping_add(c) & M4
}

const fn cs_absorb(s: u8, c: u8, x: u8) -> (u8, u8) {
    let sum = s ^ c ^ x;
    let carry = ((s & c) | (s & x) | (c & x)) << 1;
    (sum & M4, carry & M4)
}

const fn cs_law_exhaustive() -> bool {
    let mut a: u8 = 0;
    while a < 16 {
        let mut b: u8 = 0;
        while b < 16 {
            let mut x: u8 = 0;
            while x < 16 {
                let (s, c) = cs_absorb(a, b, x);
                let expect = a.wrapping_add(b).wrapping_add(x) & M4;
                if cs_value(s, c) != expect {
                    return false;
                }
                x += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}
const _: () = assert!(cs_law_exhaustive());

// redundancy witness: distinct numerals, one value.
const _: () = assert!(cs_value(5, 3) == cs_value(8, 0));

// ---------------------------------------------------------------------------
// computed fact: the law inventory depends on (window, policy), not the numeral.
// All three operation families below act on the SAME 4-bit numeral.
// ---------------------------------------------------------------------------

/// sign-extend a 4-bit two's-complement pattern held in a u8
const fn sext4(x: u8) -> i8 {
    (((x & M4) << 4) as i8) >> 4
}

const fn wrap_add4(a: u8, b: u8) -> u8 {
    a.wrapping_add(b) & M4
}

const fn sat_add_i4(a: i8, b: i8) -> i8 {
    let s = a + b; // operands are in [-8, 7]; the true sum fits i8
    if s > 7 {
        7
    } else if s < -8 {
        -8
    } else {
        s
    }
}

const fn sat_add_u4(a: u8, b: u8) -> u8 {
    let s = a + b; // operands are in [0, 15]; the true sum fits u8
    if s > 15 {
        15
    } else {
        s
    }
}

/// wrapping addition mod 16 is associative: the ring laws of Z/16 hold. Exhaustive.
const fn wrap_assoc_exhaustive() -> bool {
    let mut a: u8 = 0;
    while a < 16 {
        let mut b: u8 = 0;
        while b < 16 {
            let mut c: u8 = 0;
            while c < 16 {
                if wrap_add4(wrap_add4(a, b), c) != wrap_add4(a, wrap_add4(b, c)) {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}
const _: () = assert!(wrap_assoc_exhaustive());

/// signed saturating addition is NOT associative: exhaustive search finds a
/// counterexample. (7 sat+ 7) sat+ (-7) = 0 while 7 sat+ (7 sat+ -7) = 7.
const fn sat_i4_nonassoc_witnessed() -> bool {
    let mut a: i8 = -8;
    while a <= 7 {
        let mut b: i8 = -8;
        while b <= 7 {
            let mut c: i8 = -8;
            while c <= 7 {
                if sat_add_i4(sat_add_i4(a, b), c) != sat_add_i4(a, sat_add_i4(b, c)) {
                    return true; // counterexample exists
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    false
}
const _: () = assert!(sat_i4_nonassoc_witnessed());
const _: () = assert!(sat_add_i4(sat_add_i4(7, 7), -7) == 0);
const _: () = assert!(sat_add_i4(7, sat_add_i4(7, -7)) == 7);

/// unsigned saturating addition IS associative (truncated addition on a bounded
/// chain is a monoid). Exhaustive. The law outcome flips with the window's
/// signedness alone, under the same policy, on the same numeral.
const fn sat_u4_assoc_exhaustive() -> bool {
    let mut a: u8 = 0;
    while a < 16 {
        let mut b: u8 = 0;
        while b < 16 {
            let mut c: u8 = 0;
            while c < 16 {
                if sat_add_u4(sat_add_u4(a, b), c) != sat_add_u4(a, sat_add_u4(b, c)) {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}
const _: () = assert!(sat_u4_assoc_exhaustive());

// keep sext4 honest and used: decode of the wrap-sum agrees with mod-16 arithmetic
// on representatives, exhaustively.
const fn wrap_matches_mod16() -> bool {
    let mut a: u8 = 0;
    while a < 16 {
        let mut b: u8 = 0;
        while b < 16 {
            let lhs = sext4(wrap_add4(a, b)) as i64;
            let rhs = {
                // canonical representative of (sext4(a) + sext4(b)) mod 16 in [-8, 7]
                let s = sext4(a) as i64 + sext4(b) as i64;
                let m = s.rem_euclid(16);
                if m > 7 {
                    m - 16
                } else {
                    m
                }
            };
            if lhs != rhs {
                return false;
            }
            b += 1;
        }
        a += 1;
    }
    true
}
const _: () = assert!(wrap_matches_mod16());

// ---------------------------------------------------------------------------
// laws as compile-time contract: an algorithm may bound on a law and thereby
// refuse a (window, policy) pair that lost it. The refusal is the design.
// ---------------------------------------------------------------------------

pub struct WrapAdd4Op;
pub struct SatAddI4Op;
pub struct SatAddU4Op;

/// a law, as a marker contract. Implemented exactly where the exhaustive checks
/// above establish it. SatAddI4Op has NO impl; a call bounding on Associative
/// with it does not compile, which is the earliest possible failure.
pub trait Associative {}
impl Associative for WrapAdd4Op {}
impl Associative for SatAddU4Op {}

pub const fn requires_associative<O: Associative>() {}

const _: () = requires_associative::<WrapAdd4Op>();
const _: () = requires_associative::<SatAddU4Op>();
// const _: () = requires_associative::<SatAddI4Op>();  // does not compile, correctly

// ---------------------------------------------------------------------------
// erase: only the format survives to runtime.
// ---------------------------------------------------------------------------

#[repr(transparent)]
pub struct Num<R: Representation>(<<R as Representation>::C as Container>::Raw);

const _: () = assert!(size_of::<Num<Biased4InC8>>() == size_of::<u8>());
const _: () = assert!(size_of::<Num<Twos8>>() == size_of::<u8>());
const _: () = assert!(size_of::<Num<Twos8>>() == 1);
