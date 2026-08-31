//! Does the public surface of `arvo-strategy` have the same shape in both build
//! profiles?
//!
//! Run: `cargo build` and `cargo build --release` in this directory. Both must
//! succeed. Before the surfaces round they did not: `StatedIntent` was behind
//! `#[cfg(debug_assertions)]`, so a consumer that named it built in debug and
//! failed in release with `E0432: no StatedIntent in the root`, rustc naming the
//! cfg line as what configured it out.
//!
//! What must fail, stated before the run: if `arvo-strategy` ever again gates a
//! public item on the build profile, one of the two builds breaks and this probe
//! is what catches it. A probe that only ever built in debug would prove nothing,
//! which is why the check is two commands rather than one.

#![no_std]

use arvo_format::rounding::Mode;
use arvo_format::{overflow_of, rounding_of};
use arvo_placement::{derive_sole, Objective};
use arvo_strategy::presets::{Cold, Hot, Precise, Warm};
use arvo_strategy::{objective_of, Strategy};

/// Reaches every public item a consumer would, so a profile-gated one breaks the
/// build rather than being quietly absent.
pub fn every_public_item_a_consumer_reaches() -> (Objective, Mode) {
    let _ = objective_of::<Hot>();
    let _ = objective_of::<Cold>();
    let _ = objective_of::<Precise>();
    let _ = overflow_of::<<Warm as Strategy>::Adaptation>();

    use arvo_format::points::Integer;
    use arvo_format::rounding::Floor;
    use arvo_format::{Adapt, Signature};
    type Sig = Signature<Integer<13>, Adapt<Floor, arvo_format::overflow::Wrap>>;
    let _ = derive_sole::<Sig>(objective_of::<Cold>());

    (
        objective_of::<Warm>(),
        rounding_of::<<Warm as Strategy>::Adaptation>(),
    )
}
