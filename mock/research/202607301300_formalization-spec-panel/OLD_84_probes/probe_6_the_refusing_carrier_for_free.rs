//! probe 6: the refusing carrier costs nothing in layout on any numeral that excludes
//! one pattern, stably, with zero feature gates.
//!
//! Probe 5 found that the refusing carrier's real cost is layout, not instructions: a
//! `repr(transparent)` newtype over a full-range primitive has no niche, so
//! `Option<T>` doubles the column, and that doubling is what makes refusal expensive
//! on a substrate whose identity is footprint. It also found that `core::num::NonZero`
//! gives both the niche and the codegen back, and that the mechanism `core` uses to
//! declare a validity range on this toolchain is the `pattern_types` language feature,
//! which rustc's own `internal_features` lint flags as internal to the compiler and
//! the standard library.
//!
//! This probe builds the construction that needs neither. An `Encoding` is already
//! allowed to change which datum carries a value (`78:193-195`), so a lowering may
//! store the datum biased by one. The excluded pattern becomes zero, `core`'s stable
//! `NonZero` niche applies, and the whole fallibility ladder gets its layout back:
//!
//!   `Option<Biased>` and `Result<Biased, E>` are the width of the datum, not twice it.
//!
//! Three questions, all compiled and run:
//!
//!   1. Does the niche appear? Sizes, against the unbiased control.
//!   2. Is the round trip exact over the whole domain, not a sample?
//!   3. What does the debias cost in a per-element loop? Emitted code, at `-O`.
//!
//! The construction is a `Lowering`/`Encoding` choice and touches no `Numeral` member,
//! so nothing about identity moves. It applies to any numeral with at least one
//! pattern outside its value set, which is every bounded fixed-point numeral and every
//! float numeral whose `Specials` is not the full IEEE product.

use core::num::NonZeroU16;

/// The biased carrier: `m + 1` stored in a `NonZeroU16`. `repr(transparent)` over a
/// type that already carries its validity range, so the niche is inherited rather
/// than declared, and no internal attribute or unstable language feature is named.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Biased(NonZeroU16);

impl Biased {
    /// The one door. Refuses the single pattern the bias cannot represent, which is
    /// the numeral's own top pattern, and which a bounded numeral does not use.
    #[inline]
    pub const fn new(m: u16) -> Option<Biased> {
        match NonZeroU16::new(m.wrapping_add(1)) {
            Some(n) => Some(Biased(n)),
            None => None,
        }
    }
    #[inline]
    pub const fn get(self) -> u16 {
        self.0.get() - 1
    }
}

/// The control: the same datum with no bias and therefore no niche.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Plain(u16);

// ---------------------------------------------------------------------------
// 1. the niche, asserted in const position so the compile is the check.
// ---------------------------------------------------------------------------

pub const SIZE_BIASED: usize = core::mem::size_of::<Biased>();
pub const SIZE_OPT_BIASED: usize = core::mem::size_of::<Option<Biased>>();
pub const SIZE_RES_BIASED: usize = core::mem::size_of::<Result<Biased, ()>>();
pub const SIZE_PLAIN: usize = core::mem::size_of::<Plain>();
pub const SIZE_OPT_PLAIN: usize = core::mem::size_of::<Option<Plain>>();

const _: () = assert!(SIZE_BIASED == SIZE_PLAIN, "the bias costs no width");
const _: () = assert!(
    SIZE_OPT_BIASED == SIZE_BIASED,
    "the infallible and refusing tiers must be the same width"
);
const _: () = assert!(
    SIZE_RES_BIASED == SIZE_BIASED,
    "a refusal carrying a cause must also fit in the niche"
);
const _: () = assert!(
    SIZE_OPT_PLAIN == 2 * SIZE_PLAIN,
    "the control must show the doubling the construction removes"
);

// ---------------------------------------------------------------------------
// 2. exactness over the whole domain, in const position, not a sample.
// ---------------------------------------------------------------------------

const fn round_trips_everywhere() -> bool {
    let mut m: u16 = 0;
    loop {
        match Biased::new(m) {
            Some(b) => {
                if b.get() != m {
                    return false;
                }
            }
            None => return false,
        }
        if m == u16::MAX - 1 {
            break;
        }
        m += 1;
    }
    // and the one pattern the construction spends is refused rather than aliased
    Biased::new(u16::MAX).is_none()
}

const _: () = assert!(
    round_trips_everywhere(),
    "the biased encoding must be exact over its whole domain and refuse exactly one pattern"
);

// ---------------------------------------------------------------------------
// 3. what the debias costs in a per-element loop.
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn biased_sum(src: &[Biased; 64]) -> u32 {
    let mut acc = 0u32;
    let mut i = 0;
    while i < 64 {
        acc += src[i].get() as u32;
        i += 1;
    }
    acc
}

#[unsafe(no_mangle)]
pub extern "C" fn plain_sum(src: &[Plain; 64]) -> u32 {
    let mut acc = 0u32;
    let mut i = 0;
    while i < 64 {
        acc += src[i].0 as u32;
        i += 1;
    }
    acc
}

/// A column of the refusing tier, at the same width as a column of the infallible
/// tier. This is the shape that is impossible without the niche: `[Option<Plain>; 64]`
/// is 256 bytes, `[Option<Biased>; 64]` is 128.
pub const COLUMN_BIASED: usize = core::mem::size_of::<[Option<Biased>; 64]>();
pub const COLUMN_PLAIN: usize = core::mem::size_of::<[Option<Plain>; 64]>();
const _: () = assert!(COLUMN_BIASED == 128 && COLUMN_PLAIN == 256);
