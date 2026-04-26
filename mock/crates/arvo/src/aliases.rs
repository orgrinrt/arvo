//! Numeric aliases at canonical widths.
//!
//! Domain-default aliases over `UFixed` / `IFixed`. Strategy default
//! is `Warm` where Warm is valid (`I + F <= 32`); above that, the
//! alias requires explicit strategy or fails compilation with a
//! `UContainerFor`-bound error pointing the consumer at strategy
//! choice (via the `#[diagnostic::on_unimplemented]` attribute).

use crate::ifixed::IFixed;
use crate::newtype::{fbits, ibits};
use crate::strategy::Warm;
use crate::ufixed::UFixed;

pub type Uint5<S = Warm> = UFixed<{ ibits(5) }, { fbits(0) }, S>;
pub type Uint6<S = Warm> = UFixed<{ ibits(6) }, { fbits(0) }, S>;
pub type Uint7<S = Warm> = UFixed<{ ibits(7) }, { fbits(0) }, S>;
pub type Uint8<S = Warm> = UFixed<{ ibits(8) }, { fbits(0) }, S>;
pub type Uint16<S = Warm> = UFixed<{ ibits(16) }, { fbits(0) }, S>;
pub type Uint32<S = Warm> = UFixed<{ ibits(32) }, { fbits(0) }, S>;
pub type Uint64<S> = UFixed<{ ibits(64) }, { fbits(0) }, S>;

pub type Int7<S = Warm> = IFixed<{ ibits(7) }, { fbits(0) }, S>;
pub type Int8<S = Warm> = IFixed<{ ibits(8) }, { fbits(0) }, S>;
pub type Int13<S = Warm> = IFixed<{ ibits(13) }, { fbits(0) }, S>;
pub type Int16<S = Warm> = IFixed<{ ibits(16) }, { fbits(0) }, S>;
pub type Int32<S = Warm> = IFixed<{ ibits(32) }, { fbits(0) }, S>;
pub type Int64<S> = IFixed<{ ibits(64) }, { fbits(0) }, S>;
