// Probe C. Spelling two from 110:2759-2760: "the four preset names stay nullary
// and the semantic aliases map each to a per-kind marker".
// Question: what stops a consumer pairing the fixed-point marker with a float
// numeral? D52 (110:3366) makes compositions "public and bindable by anyone".
#![no_std]
#![feature(const_trait_impl)]
#![allow(dead_code)]
extern crate base;
use base::*;
use core::marker::PhantomData;

pub const trait Policy {
    type Quantisation: Quantisation;
}
pub const trait Lowering {
    type StoredWidth: StoredWidth;
    type Layout: StorageLayout;
    type Door: LoweringDoor;
    type Container;
}

// Four names become eight markers.
pub struct WarmFixed;
pub struct WarmFloat;

pub struct QFixedWarm;
const impl Quantisation for QFixedWarm {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = TowardNegative;
    type UnderRange = TowardPositive;
}
pub struct QFloatWarm;
const impl Quantisation for QFloatWarm {
    type UnderMidpoint = ToEven;
    type OnMidpoint = ToEven;
    type OverMidpoint = ToEven;
    type OverRange = FarPoint;
    type UnderRange = FarPoint;
}

const impl Policy for WarmFixed {
    type Quantisation = QFixedWarm;
}
const impl Lowering for WarmFixed {
    type StoredWidth = DoubleLogical; // the ratified fixed-point row, 110:2674
    type Layout = Dense;
    type Door = Inert;
    type Container = u32;
}
const impl Policy for WarmFloat {
    type Quantisation = QFloatWarm;
}
const impl Lowering for WarmFloat {
    type StoredWidth = Minimum; // the ratified float row, 110:2727
    type Layout = Dense;
    type Door = HostFloat<DefaultEnv>;
    type Container = u32;
}

pub struct Number<N: Numeral, S: Policy + Lowering> {
    datum: <S as Lowering>::Container,
    _numeral: PhantomData<N>,
}

// Binary32 is a Ranged numeral. WarmFixed is the fixed-point marker.
// Nothing in the type of Number relates the two, so:
pub type Nonsense = Number<Binary32, WarmFixed>;

// And a consumer can write it as a value-bearing signature, which is what
// D52 licenses (110:3366): compositions are public and bindable by anyone.
pub fn consumer_writes_nonsense(n: Nonsense) -> Nonsense {
    n
}
