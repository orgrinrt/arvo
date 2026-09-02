// Arm C, the position the canon actually ratifies. The coordinate is an
// ASSOCIATED CONST rather than a const generic parameter, and an associated
// const takes any type at all.
//
// `ruling::the_numeric_door_carries_the_coordinate_set_and_the_two_type_bound_is_not_canon`
// says the door carries the coordinate set "spelled in types the stack owns",
// and its `promotion` records seat 238's existence proof: an outside crate
// declaring a format naming no machine type on any line. This arm is a third,
// independent instance of that, built without reading that seat's probe.
//
// No feature attribute anywhere in this crate. That is the measurement.
#![no_std]

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Width(u32);

impl Width {
    pub const fn bits(n: u32) -> Self {
        Self(n)
    }
    pub const fn count(self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Radix(u32);

impl Radix {
    pub const fn of(base: u32) -> Self {
        Self(base)
    }
    pub const fn base(self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Exponent(i32);

impl Exponent {
    pub const fn of(power: i32) -> Self {
        Self(power)
    }
    pub const fn power(self) -> i32 {
        self.0
    }
}

/// The contract an outside crate implements to declare a format of its own.
/// Every coordinate is a stack-owned type at an associated const.
pub trait Declared {
    const WIDTH: Width;
    const RADIX: Radix;
    const EXPONENT: Exponent;
}

/// A computation over the coordinates, so the consumer's declaration is forced
/// rather than merely written. A `const` item is evaluated at check time.
pub const fn smallest_step<D: Declared>() -> i32 {
    D::EXPONENT.power()
}
