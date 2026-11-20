//! The contracts a numeral satisfies, for code generic over one.
//!
//! Each contract is a trait a value type implements, and its law is stated over the instance's own
//! members: the member a method returns is the member the format's own function gives for the
//! operands. A consumer that bounds on these depends on this crate alone, no carrier, no placement
//! and no value, and names no feature of the toolchain. `arvo-numeral` implements every numeric
//! contract for its value, and a type of a consumer's own can be an instance on the same terms.
//!
//! The contracts with methods are const traits, so arvo's own generic `const fn`s can be written
//! over them. A consumer that takes no gate bounds on them plainly and calls through them at run
//! time; a value at build time comes from the value's own inherent `const fn`s.

#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]
#![feature(const_trait_impl)]

mod arithmetic;
mod constant;
mod identity;
mod monotone;
mod named;
mod order;
mod sign;

pub use arithmetic::{Difference, Product, Sum};
pub use constant::Constant;
pub use identity::{Additive, Identity, Multiplicative};
pub use monotone::{MonotoneCompletion, MonotoneSum};
pub use named::{Reciprocal, SquareRoot};
pub use order::Order;
pub use sign::{Magnitude, Sign};
