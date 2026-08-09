// Probe A. The current shape, as 110:3090-3102 declares it: nullary associated
// types on the marker. The two ratified tables (110:2674 fixed, 110:2727 float)
// disagree on Warm::StoredWidth. Try to state both.
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
use base::*;

pub const trait Policy {
    type Quantisation: Quantisation;
}
pub const trait Lowering {
    type StoredWidth: StoredWidth;
    type Layout: StorageLayout;
    type Door: LoweringDoor;
}

pub struct Warm;

pub struct WarmFixedQ;
const impl Quantisation for WarmFixedQ {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive; // clamp
}
const impl Policy for Warm {
    type Quantisation = WarmFixedQ;
}

// The fixed-point row: StoredWidth = doubled (110:2674).
const impl Lowering for Warm {
    type StoredWidth = DoubleLogical;
    type Layout = Dense;
    type Door = Inert;
}

// The float row: StoredWidth = minimum (110:2727). Same marker, same trait.
const impl Lowering for Warm {
    type StoredWidth = Minimum;
    type Layout = Dense;
    type Door = HostFloat<DefaultEnv>;
}
