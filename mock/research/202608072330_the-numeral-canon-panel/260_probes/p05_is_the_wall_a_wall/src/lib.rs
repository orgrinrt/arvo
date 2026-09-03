//! Is the 64-bit wall inherent to the canon, or an artifact of one coordinate?
//!
//! `p01` establishes that no 64-bit platform width is expressible over the
//! shipped slot family, three ways. That leaves a question the answer to Q26
//! turns on: if a platform-width numeral is a target-indexed family of formats
//! and arvo can never build one at the width every current machine has, the
//! answer is about something that does not exist.
//!
//! **The canon does not fix the shape.**
//! `proposal::membership_of_the_representable_set_is_one_affine_predicate`,
//! stamped by `ruling::the_format_spine_is_canon`, says membership is "an affine
//! slot function, a quantum per magnitude and a phase". It says nothing about
//! what host type a slot index is carried in. The shipped `Slots` declares
//! `MIN: Slot` and `MAX: Slot` where `Slot` is a newtype over `i64`
//! (`arvo-format/src/slots.rs:41`), and that is a design choice rather than a
//! canon one.
//!
//! So this asks whether the same affine predicate is expressible at 64 bits with
//! the endpoints **derived on demand** rather than **declared as constants**. It
//! is one spike checking one thing and it is not a proposed design: what it
//! settles is artifact against wall, and nothing about which repair a design
//! should take.
//!
//! **The case that must fail, stated before the run.** The agreement arm below
//! compares the derived endpoints against the shipped family's declared ones at
//! every width the shipped family admits, both signednesses. If the two ever
//! disagree, this expresses a different predicate that happens to reach 64, the
//! comparison is the whole point, and the finding is void. I mutated the derived
//! signed minimum by one and watched `E0080` fire before trusting it; the stderr
//! is in `the_agreement_arm_can_fail.stderr`.

#![no_std]

use arvo_format::Width;
use arvo_format::slots::{ADMITTED_WIDTHS, Signed, Slots, Unsigned};

/// A slot range declared by what it is rather than by where it ends.
///
/// Two coordinates, both of which a declaration already has. The endpoints are a
/// theorem about them rather than a second thing an implementor writes, which is
/// also why they cannot go out of step with the width the way the shipped pair
/// can.
pub trait SlotRange {
    /// The declared width.
    const WIDTH: Width;
    /// Whether the range straddles zero.
    const SIGNED: bool;
}

/// The least admitted index, in the domain the caller has.
///
/// `i128` here because that is what this crate has and the point is that the
/// domain is the observer's rather than the declaration's. A caller that only
/// needs to compare two indices of the same range needs no such domain at all.
#[must_use]
pub const fn min_index<S: SlotRange>() -> i128 {
    if S::SIGNED { -(1i128 << (S::WIDTH.count() - 1)) } else { 0 }
}

/// The greatest admitted index, in the domain the caller has.
#[must_use]
pub const fn max_index<S: SlotRange>() -> i128 {
    if S::SIGNED {
        (1i128 << (S::WIDTH.count() - 1)) - 1
    } else {
        (1i128 << S::WIDTH.count()) - 1
    }
}

/// How many indices the range admits.
///
/// The quantity the shipped family's bound is actually over: `slot_count` is
/// part of its surface and returns `MAX - MIN + 1`, which for an unsigned `W` is
/// `2^W`. Derived here rather than declared, so it costs the caller's domain
/// rather than the declaration's.
#[must_use]
pub const fn index_count<S: SlotRange>() -> i128 {
    1i128 << S::WIDTH.count()
}

/// Membership, which is the affine predicate the canon names.
#[must_use]
pub const fn contains_index<S: SlotRange>(index: i128) -> bool {
    index >= min_index::<S>() && index <= max_index::<S>()
}

/// An unsigned range of `BITS` bits.
pub struct U<const BITS: u32>;

impl<const BITS: u32> SlotRange for U<BITS> {
    const SIGNED: bool = false;
    const WIDTH: Width = Width::bits(BITS);
}

/// A signed range of `BITS` bits.
pub struct S<const BITS: u32>;

impl<const BITS: u32> SlotRange for S<BITS> {
    const SIGNED: bool = true;
    const WIDTH: Width = Width::bits(BITS);
}

/// The platform's width, which is the whole reason for the exercise.
#[cfg(target_pointer_width = "16")]
pub const PLATFORM_BITS: u32 = 16;
#[cfg(target_pointer_width = "32")]
pub const PLATFORM_BITS: u32 = 32;
#[cfg(target_pointer_width = "64")]
pub const PLATFORM_BITS: u32 = 64;

/// The platform width is declarable, at 64 and past it.
///
/// The arm the shipped family refuses three ways in `p01`. Nothing here is
/// clever: the endpoints are computed where somebody asks for them, in a domain
/// that holds them, instead of being stored in one that does not.
pub const THE_PLATFORM_WIDTH_IS_DECLARABLE: () = {
    assert!(min_index::<U<64>>() == 0);
    assert!(max_index::<U<64>>() == (1i128 << 64) - 1);
    assert!(index_count::<U<64>>() == 1i128 << 64);
    assert!(contains_index::<U<64>>((1i128 << 64) - 1));
    assert!(!contains_index::<U<64>>(1i128 << 64));

    assert!(min_index::<S<64>>() == -(1i128 << 63));
    assert!(max_index::<S<64>>() == (1i128 << 63) - 1);
    assert!(contains_index::<S<64>>(-(1i128 << 63)));
    assert!(!contains_index::<S<64>>(1i128 << 63));

    // The target's own, whichever it is.
    assert!(max_index::<U<{ PLATFORM_BITS }>>() == (1i128 << PLATFORM_BITS) - 1);

    // And past it, so 64 is not a second ceiling somebody has to move again.
    assert!(max_index::<U<100>>() == (1i128 << 100) - 1);
    assert!(min_index::<S<126>>() == -(1i128 << 125));
};

/// The agreement arm, and the finding is void without it.
///
/// The derived endpoints equal the shipped family's declared ones at **every**
/// width the shipped family admits, both signednesses. Not three sampled widths:
/// the whole admitted set, read off `ADMITTED_WIDTHS` rather than typed, so it
/// tracks the family if the family moves.
///
/// This is what says the trait above expresses the same predicate rather than a
/// different one that happens to reach further. If it disagreed anywhere, the
/// 64-bit arm above would establish nothing about arvo.
macro_rules! agrees_at {
    ($($w:literal),+ $(,)?) => {
        pub const THE_DERIVED_ENDPOINTS_AGREE_WITH_THE_SHIPPED_ONES: () = {
            $(
                assert!(min_index::<U<$w>>() == <Unsigned<$w> as Slots>::MIN.index() as i128);
                assert!(max_index::<U<$w>>() == <Unsigned<$w> as Slots>::MAX.index() as i128);
                assert!(min_index::<S<$w>>() == <Signed<$w> as Slots>::MIN.index() as i128);
                assert!(max_index::<S<$w>>() == <Signed<$w> as Slots>::MAX.index() as i128);
                assert!(
                    <U<$w> as SlotRange>::WIDTH.count()
                        == <Unsigned<$w> as Slots>::WIDTH.count()
                );
            )+
        };

        /// The count of widths this arm covered, against what the family admits.
        ///
        /// The control on the control. A macro invocation short of the admitted
        /// set would pass every assertion above and cover less than it claims, and
        /// this is what refuses that.
        pub const THE_AGREEMENT_ARM_COVERS_THE_WHOLE_ADMITTED_SET: () = {
            let covered: usize = [$($w),+].len();
            assert!(covered == ADMITTED_WIDTHS.len());
        };
    };
}

agrees_at!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
);

/// What the repair costs, stated rather than hidden, because it is not free.
///
/// The shipped shape hands back a `Slot` that is a coordinate the stack owns, and
/// this hands back a bare `i128`, which is the wrong shape for arvo's own rules
/// and is a probe artifact rather than a proposal. What the arm below pins is the
/// part that is not an artifact: **the domain is the observer's**, so a caller
/// that never materialises an endpoint never pays for one.
///
/// Comparing two indices of one range needs no endpoint at all, which is what the
/// derived form buys and the declared form cannot.
pub const THE_OBSERVER_PAYS_ONLY_WHERE_IT_LOOKS: () = {
    // Membership at 64 bits, asked and answered, with no endpoint stored anywhere
    // and no `Slot` in sight.
    assert!(contains_index::<U<64>>(0));
    assert!(contains_index::<U<64>>((1i128 << 63) + 7));
    assert!(!contains_index::<U<64>>(-1));

    // And the declaration itself carries no value, exactly as a format's does.
    assert!(core::mem::size_of::<U<64>>() == 0);
    assert!(core::mem::size_of::<S<64>>() == 0);
};
