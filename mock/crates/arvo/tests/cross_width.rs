//! Cross-strategy (same-width) resolution tests.
//!
//! Cross-width arithmetic is deferred (see ufixed.rs / ifixed.rs
//! TODOs); this file covers what did land: strategy conversions
//! (`From` / `TryFrom`) at the same `<I, F>` shape, and the
//! `Resolve` trait table that cross-strategy ops will use once the
//! const-expr machinery supports them.

#![no_std]

use arvo::ifixed::IFixed;
use arvo::{FBits, IBits};
use arvo::strategy::{Cold, Hot, Precise, Resolve, Strategy, Warm};
use arvo::ufixed::UFixed;

// Resolve matrix: more conservative wins.

#[test]
fn resolve_hot_hot_is_hot() {
    assert_eq!(<Hot as Resolve<Hot>>::Out::NAME, "Hot");
}

#[test]
fn resolve_hot_warm_is_warm() {
    assert_eq!(<Hot as Resolve<Warm>>::Out::NAME, "Warm");
}

#[test]
fn resolve_warm_precise_is_precise() {
    assert_eq!(<Warm as Resolve<Precise>>::Out::NAME, "Precise");
}

#[test]
fn resolve_cold_precise_is_precise() {
    assert_eq!(<Cold as Resolve<Precise>>::Out::NAME, "Precise");
}

#[test]
fn resolve_hot_cold_is_cold() {
    assert_eq!(<Hot as Resolve<Cold>>::Out::NAME, "Cold");
}

// Strategy conversions for UFixed.

#[test]
fn ufixed_from_hot_to_warm_widens_losslessly() {
    type UHot = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    type UWarm = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    let a = UHot::from_raw(200);
    let b: UWarm = a.into();
    assert_eq!(b.to_raw(), 200u16);
}

#[test]
fn ufixed_from_hot_to_precise_widens_losslessly() {
    type UHot = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    type UPrecise = UFixed<{ IBits(8) }, { FBits::ZERO }, Precise>;
    let a = UHot::from_raw(200);
    let b: UPrecise = a.into();
    assert_eq!(b.to_raw(), 200u16);
}

#[test]
fn ufixed_from_warm_to_precise_widens_losslessly() {
    type UWarm = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    type UPrecise = UFixed<{ IBits(8) }, { FBits::ZERO }, Precise>;
    let a = UWarm::from_raw(300);
    let b: UPrecise = a.into();
    assert_eq!(b.to_raw(), 300u16);
}

#[test]
fn ufixed_try_from_warm_to_hot_in_range_succeeds() {
    type UHot = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    type UWarm = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    let a = UWarm::from_raw(200);
    let narrowed: Result<UHot, ()> = a.try_into();
    assert_eq!(narrowed.unwrap().to_raw(), 200u8);
}

#[test]
fn ufixed_try_from_warm_to_hot_out_of_range_fails() {
    type UHot = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    type UWarm = UFixed<{ IBits(8) }, { FBits::ZERO }, Warm>;
    // 300 doesn't fit in 8 logical bits (max 255).
    let a = UWarm::from_raw(300);
    let narrowed: Result<UHot, ()> = a.try_into();
    assert!(narrowed.is_err());
}

#[test]
fn ufixed_try_from_precise_to_hot_out_of_range_fails() {
    type UHot = UFixed<{ IBits(8) }, { FBits::ZERO }, Hot>;
    type UPrecise = UFixed<{ IBits(8) }, { FBits::ZERO }, Precise>;
    let a = UPrecise::from_raw(256);
    let narrowed: Result<UHot, ()> = a.try_into();
    assert!(narrowed.is_err());
}

// Strategy conversions for IFixed.

#[test]
fn ifixed_from_hot_to_warm_widens_losslessly() {
    type IHot = IFixed<{ IBits(7) }, { FBits::ZERO }, Hot>;
    type IWarm = IFixed<{ IBits(7) }, { FBits::ZERO }, Warm>;
    let a = IHot::from_raw(-50);
    let b: IWarm = a.into();
    assert_eq!(b.to_raw(), -50i16);
}

#[test]
fn ifixed_try_from_warm_to_hot_out_of_range_fails() {
    type IHot = IFixed<{ IBits(7) }, { FBits::ZERO }, Hot>;
    type IWarm = IFixed<{ IBits(7) }, { FBits::ZERO }, Warm>;
    // 1000 doesn't fit in 8 logical bits signed (max 127).
    let a = IWarm::from_raw(1000);
    let narrowed: Result<IHot, ()> = a.try_into();
    assert!(narrowed.is_err());
}

#[test]
fn ifixed_try_from_warm_to_hot_in_range_succeeds() {
    type IHot = IFixed<{ IBits(7) }, { FBits::ZERO }, Hot>;
    type IWarm = IFixed<{ IBits(7) }, { FBits::ZERO }, Warm>;
    let a = IWarm::from_raw(-50);
    let narrowed: Result<IHot, ()> = a.try_into();
    assert_eq!(narrowed.unwrap().to_raw(), -50i8);
}

// Own-trait narrow methods return notko `Outcome<T, ()>`. The
// `.try_into()` tests above exercise the std `TryFrom` boundary;
// these four tests exercise the own-trait surface directly.

#[test]
fn u_try_narrow_outcome_ok() {
    use arvo::strategy::UNarrowFrom;
    use notko::Outcome;
    let raw: u16 = 200;
    let got = <Hot as UNarrowFrom<Warm, 8>>::u_try_narrow(raw);
    assert!(matches!(got, Outcome::Ok(200u8)));
}

#[test]
fn u_try_narrow_outcome_err() {
    use arvo::strategy::UNarrowFrom;
    use notko::Outcome;
    let raw: u16 = 300;
    let got = <Hot as UNarrowFrom<Warm, 8>>::u_try_narrow(raw);
    assert!(matches!(got, Outcome::Err(())));
}

#[test]
fn i_try_narrow_outcome_ok() {
    use arvo::strategy::INarrowFrom;
    use notko::Outcome;
    let raw: i16 = -50;
    let got = <Hot as INarrowFrom<Warm, 8>>::i_try_narrow(raw);
    assert!(matches!(got, Outcome::Ok(-50i8)));
}

#[test]
fn i_try_narrow_outcome_err() {
    use arvo::strategy::INarrowFrom;
    use notko::Outcome;
    let raw: i16 = 1000;
    let got = <Hot as INarrowFrom<Warm, 8>>::i_try_narrow(raw);
    assert!(matches!(got, Outcome::Err(())));
}
