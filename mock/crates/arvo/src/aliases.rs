//! Numeric aliases at canonical widths plus generic ergonomic aliases.
//!
//! Two groups of aliases live here:
//!
//! - **Generic aliases** [`Fixed`] / [`Signed`] forward bare `u8`
//!   const-generics into the `IBits` / `FBits`-wrapped form of
//!   `UFixed` / `IFixed`. These give domain-author and consumer call
//!   sites clean syntax (`Fixed<13, 3, Warm>`) without the
//!   `{ ibits(...) }` / `{ fbits(...) }` boilerplate.
//! - **Canonical-width aliases** (`Uint5` ... `Uint64`, `Int7` ...
//!   `Int64`) name the common widths once. End-user code uses these
//!   instead of building the type inline.
//!
//! Strategy default is `Warm` where Warm is valid (`I + F <= 32`);
//! above that, the alias requires explicit strategy or fails
//! compilation with a `UContainerFor`-bound error pointing the
//! consumer at strategy choice (via the
//! `#[diagnostic::on_unimplemented]` attribute on `UContainerFor`).

use crate::ifixed::IFixed;
use crate::newtype::{fbits, ibits};
use crate::strategy::Warm;
use crate::ufixed::UFixed;

/// Unsigned fixed-point with bare-`u8` const-generic ergonomics.
///
/// `Fixed<I, F, S>` is `UFixed<{ ibits(I) }, { fbits(F) }, S>`. Use
/// this at consumer call sites and in domain alias definitions to
/// avoid `{ ibits(...) }` / `{ fbits(...) }` boilerplate. The
/// strategy default is `Warm`; choose `Hot` / `Precise` explicitly
/// for `I + F > 32`.
///
/// ```ignore
/// type Angle = arvo::Fixed<9, 7, Warm>;        // 9.7 unsigned
/// type Counter = arvo::Fixed<32, 0, Hot>;       // 32-bit counter
/// ```
pub type Fixed<const I: u8, const F: u8, S = Warm> =
    UFixed<{ ibits(I) }, { fbits(F) }, S>;

/// Signed fixed-point with bare-`u8` const-generic ergonomics.
///
/// `Signed<I, F, S>` is `IFixed<{ ibits(I) }, { fbits(F) }, S>`.
/// Mirror of [`Fixed`] for signed values. The strategy default is
/// `Warm`; choose `Hot` / `Precise` explicitly for `1 + I + F > 32`.
///
/// ```ignore
/// type Coord = arvo::Signed<15, 16, Warm>;     // 15.16 signed
/// type Delta = arvo::Signed<7, 0, Hot>;         // signed 8-bit
/// ```
pub type Signed<const I: u8, const F: u8, S = Warm> =
    IFixed<{ ibits(I) }, { fbits(F) }, S>;

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
