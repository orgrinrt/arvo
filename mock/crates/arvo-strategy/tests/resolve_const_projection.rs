//! Const-context smoke test for `Resolve<Other>::Out` projection.
//!
//! Closes audit Finding 36 (round 202605041051, task #324).
//! `Resolve<Other>` is `pub const trait` with same-pair plus
//! cross-pair impls. This file exercises the type-level projection
//! `<<S1 as Resolve<S2>>::Out as Strategy>::RANK` in const context
//! across all 16 pairs of the four strategy markers, forcing
//! const-eval and asserting the rank value.

#![feature(const_trait_impl)]

use arvo_strategy::{Cold, Hot, Precise, Resolve, Strategy, Warm};

const HOT_HOT: u16 = <<Hot as Resolve<Hot>>::Out as Strategy>::RANK;
const HOT_WARM: u16 = <<Hot as Resolve<Warm>>::Out as Strategy>::RANK;
const HOT_COLD: u16 = <<Hot as Resolve<Cold>>::Out as Strategy>::RANK;
const HOT_PRECISE: u16 = <<Hot as Resolve<Precise>>::Out as Strategy>::RANK;

const WARM_HOT: u16 = <<Warm as Resolve<Hot>>::Out as Strategy>::RANK;
const WARM_WARM: u16 = <<Warm as Resolve<Warm>>::Out as Strategy>::RANK;
const WARM_COLD: u16 = <<Warm as Resolve<Cold>>::Out as Strategy>::RANK;
const WARM_PRECISE: u16 = <<Warm as Resolve<Precise>>::Out as Strategy>::RANK;

const COLD_HOT: u16 = <<Cold as Resolve<Hot>>::Out as Strategy>::RANK;
const COLD_WARM: u16 = <<Cold as Resolve<Warm>>::Out as Strategy>::RANK;
const COLD_COLD: u16 = <<Cold as Resolve<Cold>>::Out as Strategy>::RANK;
const COLD_PRECISE: u16 = <<Cold as Resolve<Precise>>::Out as Strategy>::RANK;

const PRECISE_HOT: u16 = <<Precise as Resolve<Hot>>::Out as Strategy>::RANK;
const PRECISE_WARM: u16 = <<Precise as Resolve<Warm>>::Out as Strategy>::RANK;
const PRECISE_COLD: u16 = <<Precise as Resolve<Cold>>::Out as Strategy>::RANK;
const PRECISE_PRECISE: u16 = <<Precise as Resolve<Precise>>::Out as Strategy>::RANK;

// Forces evaluation at compile time. Each `assert!` in a `const _` block
// is a const-time check; failure produces a build error.
const _SAME_PAIR_PROJECTIONS: () = {
    assert!(HOT_HOT == 0);
    assert!(WARM_WARM == 1);
    assert!(COLD_COLD == 2);
    assert!(PRECISE_PRECISE == 3);
};

const _CROSS_PAIR_PROJECTIONS_VS_HOT: () = {
    assert!(HOT_WARM == 1);
    assert!(HOT_COLD == 2);
    assert!(HOT_PRECISE == 3);
    assert!(WARM_HOT == 1);
    assert!(COLD_HOT == 2);
    assert!(PRECISE_HOT == 3);
};

const _CROSS_PAIR_PROJECTIONS_NON_HOT: () = {
    assert!(WARM_COLD == 2);
    assert!(WARM_PRECISE == 3);
    assert!(COLD_WARM == 2);
    assert!(COLD_PRECISE == 3);
    assert!(PRECISE_WARM == 3);
    assert!(PRECISE_COLD == 3);
};

#[test]
fn resolve_const_projection_runtime_parity() {
    assert_eq!(HOT_HOT, 0);
    assert_eq!(WARM_WARM, 1);
    assert_eq!(COLD_COLD, 2);
    assert_eq!(PRECISE_PRECISE, 3);
    assert_eq!(HOT_WARM, WARM_HOT);
    assert_eq!(HOT_COLD, COLD_HOT);
    assert_eq!(HOT_PRECISE, PRECISE_HOT);
    assert_eq!(WARM_COLD, COLD_WARM);
    assert_eq!(WARM_PRECISE, PRECISE_WARM);
    assert_eq!(COLD_PRECISE, PRECISE_COLD);
}
