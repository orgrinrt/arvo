//! The control for `the_outside_signed_sixty_four.rs`: the identical outside
//! signed 64-bit declaration, with the obligation not forced.
//!
//! If this compiles and its pair does not, the refusal is the `ADMITTED`
//! obligation and nothing else: not the orphan rules, not the arithmetic, not the
//! trait's shape. The declaration is byte for byte the same and the only
//! difference is whether a const forces `Slots::ADMITTED`.
//!
//! The arithmetic arm is carried here as well, so what the coordinate can name is
//! established in the arm that builds rather than in the arm that refuses.

use arvo_format::{Slot, Slots, Width};

/// The same 64-bit signed slot range, verbatim.
pub struct PlatformSigned64;

impl Slots for PlatformSigned64 {
    const MAX: Slot = Slot::at(i64::MAX);
    const MIN: Slot = Slot::at(i64::MIN);
    const WIDTH: Width = Width::bits(64);
}

/// The coordinates read back, without forcing the obligation.
pub const THE_DECLARATION_ITSELF_IS_WELL_FORMED: () = {
    assert!(<PlatformSigned64 as Slots>::WIDTH.count() == 64);
    assert!(<PlatformSigned64 as Slots>::MIN.index() == i64::MIN);
    assert!(<PlatformSigned64 as Slots>::MAX.index() == i64::MAX);
};

/// What the slot coordinate can name, which is a narrower claim than the crate's
/// own bound and does not contradict it.
///
/// `Slot` is a newtype over `i64` whose only constructor takes an `i64`, so the
/// slot indices any implementor anywhere can name are exactly the `i64`s. The
/// largest slot of an unsigned declaration of `W` bits is `2^W - 1`, which is
/// nameable through `W = 63` and not at 64. Computed in `u128`, a domain this
/// probe has and the coordinate does not.
///
/// **The crate stops at 62 on a different criterion and it is not leaving a width
/// on the table.** `slot_count` is part of the surface and returns `MAX - MIN + 1`,
/// which for an unsigned `W` is `2^W` rather than `2^W - 1`, so the bound is where
/// the *count* stops fitting rather than where the largest index does.
/// `arvo-format/src/tests/the_inventory.rs:303`,
/// `the_widest_admitted_width_is_where_the_count_stops_fitting`, derives 62 from
/// exactly that and would fail if the set moved. So the two numbers measure two
/// quantities and both are right; a first draft of this arm read the crate as one
/// width short, and its `E0080` is in
/// `the_outside_signed_sixty_four_first_draft.stderr`.
///
/// The loops run the whole range rather than three sampled widths, because the
/// claim is where a boundary falls and sampling is how a boundary gets missed.
pub const WHAT_THE_SLOT_COORDINATE_CAN_NAME: () = {
    let widest_nameable: u128 = i64::MAX as u128;

    // The largest unsigned index, nameable through 63 and not at 64.
    let mut w: u32 = 1;
    while w <= 63 {
        assert!(((1u128 << w) - 1) <= widest_nameable);
        w += 1;
    }
    assert!(((1u128 << 64) - 1) > widest_nameable);

    // The unsigned slot count, which is what the crate's bound is over: it fits
    // through 62 and not at 63, which is the 62 the shipped list stops at.
    let mut c: u32 = 1;
    while c <= 62 {
        assert!((1u128 << c) <= widest_nameable);
        c += 1;
    }
    assert!((1u128 << 63) > widest_nameable);

    // Signed indices: nameable through 64, because the range is the coordinate's
    // own. The count at 64 is `2^64`, so the count criterion stops this too, and
    // the two criteria disagreeing is the whole content of this arm.
    let mut s: u32 = 1;
    while s <= 64 {
        assert!(((1u128 << (s - 1)) - 1) <= widest_nameable);
        s += 1;
    }

    // The control on the comparison itself. If it were vacuous these would pass
    // too, and they must not: something genuinely past the coordinate is past it.
    assert!(((1u128 << 65) - 1) > widest_nameable);
    assert!((1u128 << 64) > widest_nameable);
};
