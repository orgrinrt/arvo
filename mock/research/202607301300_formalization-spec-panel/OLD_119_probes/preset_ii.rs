#![allow(dead_code)]
use core::marker::PhantomData;

// ---- the two number kinds, as the design already has them --------------
pub trait ExponentForm {}
pub struct Implicit; // one fixed exponent: fixed-point
pub struct Ranged; // a window: float
impl ExponentForm for Implicit {}
impl ExponentForm for Ranged {}

pub trait Numeral {
    type Exponent: ExponentForm;
}
pub struct Fix13_3;
impl Numeral for Fix13_3 {
    type Exponent = Implicit;
}
pub struct Binary32;
impl Numeral for Binary32 {
    type Exponent = Ranged;
}

pub trait StoredWidth {}
pub struct Minimum;
impl StoredWidth for Minimum {}
pub struct DoubleLogical;
impl StoredWidth for DoubleLogical {}

pub trait Lowering {
    type StoredWidth: StoredWidth;
}

// ---- SPELLING (ii): the four names become eight markers ---------------
pub struct WarmFixed;
impl Lowering for WarmFixed {
    type StoredWidth = DoubleLogical;
}
pub struct WarmFloat;
impl Lowering for WarmFloat {
    type StoredWidth = Minimum;
}

pub struct Number<N: Numeral, S: Lowering>(PhantomData<(N, S)>);

// The consumer-facing alias hands the right marker to the right kind.
pub type UFixed<S> = Number<Fix13_3, S>;

// And nothing stops the wrong pairing being written directly, which D52
// says a consumer may do: "compositions are public and bindable by anyone".
pub type IllFormed = Number<Binary32, WarmFixed>;
pub fn takes_it(_x: IllFormed) {}
