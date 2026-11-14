//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What every band shares: the checks of one pair through the value type, the
//! edge lattice, and the seeded generator.

#![allow(dead_code)]

pub mod crossings;
pub mod exponents;
pub mod hosts;
pub mod oracle;
pub mod points;
pub mod shapes;
pub mod widths;

/// The wide integer of `arvo-format`'s own suites, read where it is kept so
/// that there is one of it. It is a test file of that crate and no part of its
/// surface, which is why it is reached by path.
#[path = "../../../arvo-format/src/tests/big.rs"]
pub mod big;

use arvo_format::{
    DeclaredSignature,
    Dither,
    Exact,
    Format,
    Slot,
    Slots,
    add,
    difference_position,
    neg,
    negation_position,
    panic_on_overflow,
    sub,
    sum_position,
};
use arvo_numeral::{Additive, Identity, Num, Quiet, Sole};

/// The seed the band of widths 9 to 64 draws its pairs from. Committed so a
/// failure reproduces.
pub const SEED: u64 = 0x0A7B_0D11_2026_1113;

/// How many seeded pairs each signature of the wide bands draws.
pub const SEEDED_PAIRS: usize = 4096;

type SlotsOf<S> = <<S as DeclaredSignature>::Format as Format>::Slots;

/// The slot range of the signature, as indices.
pub fn range<S: Sole>() -> (i128, i128) {
    (
        <SlotsOf<S> as Slots>::MIN.index(),
        <SlotsOf<S> as Slots>::MAX.index(),
    )
}

/// The value at a member's slot index, through the ingest door.
pub fn value<S: Sole>(index: i128) -> Num<S, Quiet> {
    Num::<S, Quiet>::adapted(Exact::on_grid(Slot::at(index)))
}

/// Disagreements found over a band. The four operations are counted, so that a
/// failure says how many disagreed; every other check asserts at once.
#[derive(Default)]
pub struct Tally {
    pub add_wrong: u64,
    pub sub_wrong: u64,
    pub neg_wrong: u64,
    pub mul_wrong: u64,
    pub pairs:     u64,
    pub members:   u64,
    pub products:  u64,
}

impl Tally {
    /// The band's sums, differences and negations all agree with the oracle.
    pub fn check(&self, band: &str) {
        assert!(self.pairs > 0, "{band}: no pair was checked");
        assert!(self.members > 0, "{band}: no member was negated");
        assert_eq!(
            self.add_wrong, 0,
            "{band}: `add` disagrees with the oracle on {} of {} pairs",
            self.add_wrong, self.pairs
        );
        assert_eq!(
            self.sub_wrong, 0,
            "{band}: `sub` disagrees with the oracle on {} of {} pairs",
            self.sub_wrong, self.pairs
        );
        assert_eq!(
            self.neg_wrong, 0,
            "{band}: `neg` disagrees with the oracle on {} of {} members",
            self.neg_wrong, self.members
        );
    }

    /// The band's products all agree with the oracle.
    pub fn check_products(&self, band: &str) {
        assert!(self.products > 0, "{band}: no product was checked");
        assert_eq!(
            self.mul_wrong, 0,
            "{band}: `mul` disagrees with the oracle on {} of {} pairs",
            self.mul_wrong, self.products
        );
    }
}

/// Everything one value is owed on its own: the doors, and the negation
/// against the oracle with its verdict against the format's.
pub fn check_value<S: Sole>(a: i128, tally: &mut Tally) {
    let v = value::<S>(a);
    tally.members += 1;
    if v.neg().slot() != neg::<S>(Slot::at(a), Dither::UNUSED) {
        tally.neg_wrong += 1;
    }
    let leaves =
        panic_on_overflow::<S>(negation_position::<S::Format>(Slot::at(a)), Dither::UNUSED).get();
    assert_eq!(
        v.fits_neg().get(),
        !leaves,
        "`fits_neg` disagrees with `panic_on_overflow` at -{a}"
    );
    assert_eq!(
        v.slot().index(),
        a,
        "the slot door does not return the member"
    );
    assert_eq!(
        Num::<S, Quiet>::adapted(Exact::on_grid(v.slot())),
        v,
        "the slot door and `adapted` do not round-trip the member"
    );
}

/// Everything one pair is owed: the sum against the oracle, the verdict against
/// the format's, and the comparisons against the slots' order.
pub fn check_pair<S: Sole>(a: i128, b: i128, tally: &mut Tally) {
    let (va, vb) = (value::<S>(a), value::<S>(b));
    let (sa, sb) = (Slot::at(a), Slot::at(b));

    tally.pairs += 1;
    if va.add(vb).slot() != add::<S>(sa, sb, Dither::UNUSED) {
        tally.add_wrong += 1;
    }

    let leaves = panic_on_overflow::<S>(sum_position::<S::Format>(sa, sb), Dither::UNUSED).get();
    assert_eq!(
        va.fits_add(vb).get(),
        !leaves,
        "`fits_add` disagrees with `panic_on_overflow` at {a} + {b}"
    );

    if va.sub(vb).slot() != sub::<S>(sa, sb, Dither::UNUSED) {
        tally.sub_wrong += 1;
    }
    let leaves =
        panic_on_overflow::<S>(difference_position::<S::Format>(sa, sb), Dither::UNUSED).get();
    assert_eq!(
        va.fits_sub(vb).get(),
        !leaves,
        "`fits_sub` disagrees with `panic_on_overflow` at {a} - {b}"
    );

    let order = a.cmp(&b);
    assert_eq!(va.cmp(vb), order, "`cmp` at {a}, {b}");
    assert_eq!(va.eq(vb).get(), order.is_eq(), "`eq` at {a}, {b}");
    assert_eq!(
        va == vb,
        order.is_eq(),
        "carrier equality is not slot equality at {a}, {b}"
    );
    assert_eq!(va.lt(vb).get(), order.is_lt(), "`lt` at {a}, {b}");
    assert_eq!(va.le(vb).get(), order.is_le(), "`le` at {a}, {b}");
    assert_eq!(va.gt(vb).get(), order.is_gt(), "`gt` at {a}, {b}");
    assert_eq!(va.ge(vb).get(), order.is_ge(), "`ge` at {a}, {b}");
    assert_eq!(va.min(vb).slot().index(), a.min(b), "`min` at {a}, {b}");
    assert_eq!(va.max(vb).slot().index(), a.max(b), "`max` at {a}, {b}");
}

/// The product of one pair and its verdict, against the tests' own
/// arithmetic: away from exponent zero the value's product is the format's
/// function, and `fits_mul` is at every exponent. Apart from `check_pair`
/// because a signature whose product the format does not admit has no `mul`
/// to call.
pub fn check_product<S: Sole>(a: i128, b: i128, tally: &mut Tally) {
    let (va, vb) = (value::<S>(a), value::<S>(b));
    let expected = oracle::product::<S>(a, b);
    tally.products += 1;
    if va.mul(vb).slot().index() != expected.slot {
        tally.mul_wrong += 1;
    }
    assert_eq!(
        va.fits_mul(vb).get(),
        expected.fits,
        "`fits_mul` at {a} * {b}"
    );
}

/// Every pair of members' product, exhaustively.
pub fn exhaustive_products<S: Sole>(tally: &mut Tally) {
    let (min, max) = range::<S>();
    for a in min ..= max {
        for b in min ..= max {
            check_product::<S>(a, b, tally);
        }
    }
}

/// The edge lattice's products, every pair, and the seeded pairs' products.
pub fn lattice_and_seeded_products<S: Sole>(tally: &mut Tally) {
    let l = lattice::<S>();
    for &a in l.as_slice() {
        for &b in l.as_slice() {
            check_product::<S>(a, b, tally);
        }
    }
    let (min, max) = range::<S>();
    let mut rng = SplitMix::seeded();
    for _ in 0 .. SEEDED_PAIRS {
        let (a, b) = (rng.member(min, max), rng.member(min, max));
        check_product::<S>(a, b, tally);
    }
}

/// The identity is slot zero, and adding it moves nothing.
pub fn check_identity<S: Sole>(a: i128) {
    let zero = <Num<S, Quiet> as Identity<Additive>>::IDENTITY;
    assert_eq!(
        zero.slot().index(),
        0,
        "the additive identity is not slot zero"
    );
    let v = value::<S>(a);
    assert_eq!(v.add(zero), v, "adding the identity moved {a}");
    assert_eq!(zero.add(v), v, "adding to the identity moved {a}");
}

/// Every pair of members, exhaustively.
pub fn exhaustive<S: Sole>(tally: &mut Tally) {
    let (min, max) = range::<S>();
    for a in min ..= max {
        check_value::<S>(a, tally);
        check_identity::<S>(a);
        for b in min ..= max {
            check_pair::<S>(a, b, tally);
        }
    }
}

/// A fixed-capacity set of slot indices.
pub struct Lattice {
    items: [i128; 160],
    len:   usize,
}

impl Lattice {
    fn new() -> Self {
        Self {
            items: [0; 160],
            len:   0,
        }
    }

    fn push(&mut self, x: i128, min: i128, max: i128) {
        if x < min || x > max || self.items[.. self.len].contains(&x) {
            return;
        }
        self.items[self.len] = x;
        self.len += 1;
    }

    pub fn as_slice(&self) -> &[i128] {
        &self.items[.. self.len]
    }
}

/// The edge lattice of a signature: each end of the range and its neighbours,
/// zero and its neighbours, and every value with one bit set in its carrier
/// pattern at the declared width.
pub fn lattice<S: Sole>() -> Lattice {
    let (min, max) = range::<S>();
    let width = <SlotsOf<S> as Slots>::WIDTH.count();
    let mut l = Lattice::new();
    for x in [min, min + 1, min + 2, max - 2, max - 1, max, -2, -1, 0, 1, 2] {
        l.push(x, min, max);
    }
    for bit in 0 .. width {
        let single = 1i128 << bit;
        // In a signed range the top bit alone is the minimum.
        l.push(
            if min < 0 && bit == width - 1 { min } else { single },
            min,
            max,
        );
    }
    l
}

/// The edge lattice, every pair.
pub fn on_the_lattice<S: Sole>(tally: &mut Tally) {
    let l = lattice::<S>();
    for &a in l.as_slice() {
        check_value::<S>(a, tally);
        check_identity::<S>(a);
        for &b in l.as_slice() {
            check_pair::<S>(a, b, tally);
        }
    }
}

/// SplitMix64, seeded from `SEED`. Small and enough to spread pairs over a
/// range; nothing here needs more from it.
pub struct SplitMix(u64);

impl SplitMix {
    pub fn seeded() -> Self {
        Self(SEED)
    }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A member of `min..=max`, by the low bits of the next draw.
    pub fn member(&mut self, min: i128, max: i128) -> i128 {
        let span = (max - min) as u128 + 1;
        min + (self.next() as u128 % span) as i128
    }
}

/// Seeded pairs over the whole range.
pub fn seeded<S: Sole>(tally: &mut Tally) {
    let (min, max) = range::<S>();
    let mut rng = SplitMix::seeded();
    for _ in 0 .. SEEDED_PAIRS {
        let (a, b) = (rng.member(min, max), rng.member(min, max));
        check_pair::<S>(a, b, tally);
    }
}

/// One signature over every admitted mode, for a body that takes the
/// signature as a type.
#[macro_export]
macro_rules! every_mode {
    ($point:ty, $policy:ty, $body:ident, $tally:expr) => {{
        use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
        use arvo_format::{Adapt, Signature};
        $body::<Signature<$point, Adapt<TowardZero, $policy>>>($tally);
        $body::<Signature<$point, Adapt<Floor, $policy>>>($tally);
        $body::<Signature<$point, Adapt<Ceil, $policy>>>($tally);
        $body::<Signature<$point, Adapt<HalfUp, $policy>>>($tally);
        $body::<Signature<$point, Adapt<HalfEven, $policy>>>($tally);
    }};
}

/// One width over both families, both overflow policies and every mode.
#[macro_export]
macro_rules! every_signature_at {
    ($w:literal, $body:ident, $tally:expr) => {{
        use arvo_format::points::{Integer, UFixed};
        use arvo_format::overflow::{Saturate, Wrap};
        $crate::every_mode!(Integer<$w>, Wrap, $body, $tally);
        $crate::every_mode!(Integer<$w>, Saturate, $body, $tally);
        $crate::every_mode!(UFixed<$w, 0>, Wrap, $body, $tally);
        $crate::every_mode!(UFixed<$w, 0>, Saturate, $body, $tally);
    }};
}
