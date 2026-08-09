//! P8b. The negative half of P8, separated so the errors do not shadow each
//! other.
//!
//! An algorithm bounded on `T: Add<Output = T>` asks the bound for the one
//! thing a container derivation needs, the declared width. There is nothing
//! there to ask.
//!
//! Expected: refused, naming the absent associated item. This is the ground
//! for the claim that tier-one inference does not derive a representation, it
//! propagates one the consumer already fixed by choosing T.

#![no_std]

use core::ops::Add;

pub fn needs_the_width<T: Add<Output = T> + Copy>(_x: T) -> u32 {
    <T as Add>::WIDTH
}
