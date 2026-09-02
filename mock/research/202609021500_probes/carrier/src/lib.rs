//! The carrier mutation. See `Cargo.toml` for what the two modules differ in.
#![no_std]
#![allow(dead_code, clippy::all)]

pub mod width;

#[path = "slots_i64.rs"]
pub mod shipped;

#[path = "slots_i128.rs"]
pub mod mutated;

/// The same declaration put to both copies.
///
/// A 63-bit two's complement grid: contiguous, ordered, the width addressing it
/// exactly. Nothing about it is not a grid, and the two copies disagree about
/// whether it is admitted. The only thing that differs between them is the
/// machine type a slot index is carried in.
pub mod verdict {
    pub struct ShippedGrid63;
    impl crate::shipped::Slots for ShippedGrid63 {
        const MIN: crate::shipped::Slot = crate::shipped::Slot::at(-4611686018427387904);
        const MAX: crate::shipped::Slot = crate::shipped::Slot::at(4611686018427387903);
        const WIDTH: crate::width::Width = crate::width::Width::bits(63);
    }

    pub struct MutatedGrid63;
    impl crate::mutated::Slots for MutatedGrid63 {
        const MIN: crate::mutated::Slot = crate::mutated::Slot::at(-4611686018427387904);
        const MAX: crate::mutated::Slot = crate::mutated::Slot::at(4611686018427387903);
        const WIDTH: crate::width::Width = crate::width::Width::bits(63);
    }

    /// The shipped carrier refuses it.
    pub const SHIPPED_ADMITS_63: bool =
        crate::shipped::is_admissible::<ShippedGrid63>().get();

    /// The wider carrier admits it.
    pub const MUTATED_ADMITS_63: bool =
        crate::mutated::is_admissible::<MutatedGrid63>().get();

    // The finding, as two const assertions that cannot both hold if the admitted
    // set is a fact about grids rather than about a carrier.
    const _: () = assert!(!SHIPPED_ADMITS_63);
    const _: () = assert!(MUTATED_ADMITS_63);

    /// The control: a grid that is malformed as a grid is refused by both, so the
    /// difference above is the carrier and not the copying.
    pub struct ShippedInverted;
    impl crate::shipped::Slots for ShippedInverted {
        const MIN: crate::shipped::Slot = crate::shipped::Slot::at(8);
        const MAX: crate::shipped::Slot = crate::shipped::Slot::at(-8);
        const WIDTH: crate::width::Width = crate::width::Width::bits(8);
    }
    pub struct MutatedInverted;
    impl crate::mutated::Slots for MutatedInverted {
        const MIN: crate::mutated::Slot = crate::mutated::Slot::at(8);
        const MAX: crate::mutated::Slot = crate::mutated::Slot::at(-8);
        const WIDTH: crate::width::Width = crate::width::Width::bits(8);
    }
    const _: () = assert!(!crate::shipped::is_admissible::<ShippedInverted>().get());
    const _: () = assert!(!crate::mutated::is_admissible::<MutatedInverted>().get());

    /// The second control: a grid both admit, so neither copy is stuck refusing.
    pub struct ShippedGrid8;
    impl crate::shipped::Slots for ShippedGrid8 {
        const MIN: crate::shipped::Slot = crate::shipped::Slot::at(-128);
        const MAX: crate::shipped::Slot = crate::shipped::Slot::at(127);
        const WIDTH: crate::width::Width = crate::width::Width::bits(8);
    }
    pub struct MutatedGrid8;
    impl crate::mutated::Slots for MutatedGrid8 {
        const MIN: crate::mutated::Slot = crate::mutated::Slot::at(-128);
        const MAX: crate::mutated::Slot = crate::mutated::Slot::at(127);
        const WIDTH: crate::width::Width = crate::width::Width::bits(8);
    }
    const _: () = assert!(crate::shipped::is_admissible::<ShippedGrid8>().get());
    const _: () = assert!(crate::mutated::is_admissible::<MutatedGrid8>().get());
}

/// Classifying the five slot assertions by measurement rather than by reading
/// their message text.
///
/// The test: hold the declaration fixed and change only the machine type a slot
/// index is carried in. An assertion whose verdict moves is about the machine. An
/// assertion whose verdict does not is about the grid. Nothing here reads a
/// string.
pub mod classify {

    /// The five conditions, restated over plain numbers so each can be asked on
    /// its own. `carrier_bits` is the width of the slot index type and
    /// `width_bound` the largest declared width the copy admits, which is the
    /// only thing the mutation changed.
    pub const fn verdicts(
        min: i128,
        max: i128,
        width: u32,
        carrier_max: i128,
        width_bound: u32,
    ) -> [bool; 5] {
        [
            min <= max,
            width >= 1,
            width <= width_bound,
            max - min < carrier_max,
            max - min < (1i128 << width),
        ]
    }

    /// The shipped copy: slot index `i64`, declared width bounded at 62.
    pub const fn shipped(min: i128, max: i128, width: u32) -> [bool; 5] {
        verdicts(min, max, width, i64::MAX as i128, 62)
    }

    /// The widened copy: slot index `i128`, declared width bounded at 126.
    pub const fn widened(min: i128, max: i128, width: u32) -> [bool; 5] {
        verdicts(min, max, width, i128::MAX, 126)
    }

    /// A witness per assertion: a declaration that trips exactly that one under
    /// the shipped carrier, with every earlier assertion passing.
    pub const W_INVERTED: (i128, i128, u32) = (8, -8, 8);
    pub const W_ZERO_WIDTH: (i128, i128, u32) = (0, 0, 0);
    pub const W_PAST_LADDER: (i128, i128, u32) = (-4611686018427387904, 4611686018427387903, 63);
    pub const W_WIDTH_DOES_NOT_COVER: (i128, i128, u32) = (-128, 200, 8);

    const fn trips(v: [bool; 5], i: usize) -> bool {
        !v[i]
    }

    // Assertion 1, the inverted range. Trips under both carriers: invariant, so
    // it is about the grid.
    const _: () = assert!(trips(shipped(W_INVERTED.0, W_INVERTED.1, W_INVERTED.2), 0));
    const _: () = assert!(trips(widened(W_INVERTED.0, W_INVERTED.1, W_INVERTED.2), 0));

    // Assertion 2, the zero width. Invariant.
    const _: () = assert!(trips(shipped(W_ZERO_WIDTH.0, W_ZERO_WIDTH.1, W_ZERO_WIDTH.2), 1));
    const _: () = assert!(trips(widened(W_ZERO_WIDTH.0, W_ZERO_WIDTH.1, W_ZERO_WIDTH.2), 1));

    // Assertion 3, the ladder bound. Trips under the shipped carrier and not
    // under the widened one: it moves, so it is about the machine.
    const _: () = assert!(trips(shipped(W_PAST_LADDER.0, W_PAST_LADDER.1, W_PAST_LADDER.2), 2));
    const _: () = assert!(!trips(widened(W_PAST_LADDER.0, W_PAST_LADDER.1, W_PAST_LADDER.2), 2));

    // Assertion 5, the width not covering the range. Invariant.
    const _: () = assert!(trips(
        shipped(W_WIDTH_DOES_NOT_COVER.0, W_WIDTH_DOES_NOT_COVER.1, W_WIDTH_DOES_NOT_COVER.2),
        4
    ));
    const _: () = assert!(trips(
        widened(W_WIDTH_DOES_NOT_COVER.0, W_WIDTH_DOES_NOT_COVER.1, W_WIDTH_DOES_NOT_COVER.2),
        4
    ));

    /// Assertion 4 has no witness, and this is why.
    ///
    /// Exhaustive over the only free dimension. Assertion 3 bounds the declared
    /// width at 62 and assertion 5 bounds the span by `2^width`, so the largest
    /// span any declaration passing both can carry is `2^62 - 1`. Assertion 4
    /// asks whether the span is below `i64::MAX`, which is `2^63 - 1`. Every
    /// admitted width is checked, not a sample of them, so this is a proof rather
    /// than a search that found nothing.
    pub const ASSERTION_FOUR_IS_IMPLIED: bool = {
        let mut w = 1u32;
        let mut ok = true;
        while w <= 62 {
            // The largest span assertion 5 admits at this width.
            let largest = (1i128 << w) - 1;
            if largest >= i64::MAX as i128 {
                ok = false;
            }
            w += 1;
        }
        ok
    };
    const _: () = assert!(ASSERTION_FOUR_IS_IMPLIED);

    /// The control on that proof: at width 63, which assertion 3 refuses, the
    /// implication fails. So the sweep above is not vacuously true and the bound
    /// at 62 is what carries it.
    pub const AT_SIXTY_THREE_IT_FAILS: bool = ((1i128 << 63) - 1) >= i64::MAX as i128;
    const _: () = assert!(AT_SIXTY_THREE_IT_FAILS);
}

/// The shipped test case for assertion four, put to the classifier.
///
/// `SpanTooWide` at `arvo-format/src/tests/the_inventory.rs:415` is the range the
/// crate keeps permanently to pin assertion four, and the test's own message says
/// "a span of 2^63 was admitted, so counting it overflows". The verdict form it
/// asserts against returns one `Bool` for all five conditions, so the test cannot
/// see which one refused. This asks each condition separately.
pub mod span_too_wide {
    use crate::classify::shipped;

    pub const MIN: i128 = -4611686018427387904;
    pub const MAX: i128 = 4611686018427387903;
    pub const WIDTH: u32 = 62;

    pub const V: [bool; 5] = shipped(MIN, MAX, WIDTH);

    // Assertion four refuses it, which is what the test believes it is pinning.
    const _: () = assert!(!V[3]);
    // And so does assertion five, on its own, which the test cannot see.
    const _: () = assert!(!V[4]);
    // The first three pass, so the case really does reach the later two.
    const _: () = assert!(V[0] && V[1] && V[2]);

    /// So deleting assertion four leaves this case refused, and the test green.
    pub const STILL_REFUSED_WITHOUT_ASSERTION_FOUR: bool = !(V[0] && V[1] && V[2] && V[4]);
    const _: () = assert!(STILL_REFUSED_WITHOUT_ASSERTION_FOUR);

    /// The control: deleting assertion five instead does change the verdict for
    /// some declaration, so "deleting an assertion changes nothing" is not
    /// something this instrument says of every assertion.
    pub const DELETING_FIVE_CHANGES_A_VERDICT: bool = {
        // A range assertion five refuses and the other four admit.
        let v = shipped(-128, 200, 8);
        v[0] && v[1] && v[2] && v[3] && !v[4]
    };
    const _: () = assert!(DELETING_FIVE_CHANGES_A_VERDICT);
}
