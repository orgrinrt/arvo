//! probe 5b: reference measurement only. NOT a proposal, and NOT adoptable.
//!
//! Probe 5 established that the declaration home is free at the API and in layout and
//! is **not** free in the emitted code, because a `repr(transparent)` newtype carries
//! no validity range. This file measures what a declared range would buy, so the gap
//! is a number rather than an intuition, and so that a later reader does not have to
//! rediscover which mechanism `core` actually uses on this toolchain.
//!
//! **Vetting outcome, per `unstable-features.md`'s own procedure.** `pattern_types`
//! and `pattern_type_macro` compile on the pinned nightly outside `core`, and rustc's
//! `internal_features` lint fires on both: "the feature `pattern_types` is internal to
//! the compiler or standard library ... using it is strongly discouraged". `core`'s
//! own `niche_types` module states the placeholder status in its module attribute:
//! `#![unstable(feature = "temporary_niche_types", issue = "none", reason = "for core,
//! alloc, and std internals until pattern types are further along")]`. The rule's
//! std-internal carve-out therefore applies, and the carve-out's own first step
//! settles it: "First check whether a stable or public wrapper suffices. If it does,
//! use the wrapper instead." `core::num::NonZero<T>` is that wrapper wherever the
//! excluded pattern is zero, and probe 6 shows a stable bias makes the excluded
//! pattern zero for any bounded numeral. **So arvo does not adopt this feature**, and
//! this file exists to record the measurement and the reasoning, not to open a route.
//!
//! The earlier attribute route is closed outright and that is worth recording too: on
//! this toolchain `#[rustc_layout_scalar_valid_range_start]` is rejected even under
//! `#![feature(rustc_attrs)]` ("attributes starting with `rustc` are reserved for use
//! by the `rustc` compiler" / "cannot find attribute ... in this scope"). Any memory
//! of that attribute as the mechanism is stale.

#![feature(pattern_types, pattern_type_macro)]
#![allow(internal_features)]

use core::pattern_type;

const SCALE: i64 = 1_000_000;

/// The proof in the layout: strictly positive, not merely nonzero.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Pos64(pattern_type!(i64 is 1..));

impl Pos64 {
    #[inline]
    pub const fn get(self) -> i64 {
        // SAFETY: a pattern type is always a legal value of its base type.
        unsafe { core::mem::transmute(self) }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pattern_loop(src: &[Pos64; 64], dst: &mut [i64; 64]) {
    let mut i = 0;
    while i < 64 {
        dst[i] = SCALE / src[i].get();
        i += 1;
    }
}

pub const SIZE: usize = core::mem::size_of::<Pos64>();
pub const SIZE_OPT: usize = core::mem::size_of::<Option<Pos64>>();
pub const SIZE_RES: usize = core::mem::size_of::<Result<Pos64, ()>>();

const _: () = assert!(SIZE == 8 && SIZE_OPT == 8 && SIZE_RES == 8);
