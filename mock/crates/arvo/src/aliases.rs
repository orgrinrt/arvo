//! Numeric aliases. Generic-only public surface.
//!
//! Four aliases live here, all generic over a const-generic bit count:
//!
//! - [`Fixed`] / [`Signed`] forward bare `u8` const-generics into the
//!   `IBits` / `FBits`-wrapped form of `UFixed` / `IFixed`. Use these
//!   when integer and fractional bits are both meaningful (`Fixed<13,
//!   3, Warm>` is 13 integer bits, 3 fractional bits).
//! - [`Uint`] / [`Int`] are integer-only convenience aliases for the
//!   common case `F = 0`. Total bit width is the sole const-generic
//!   parameter (`Uint<12>` is a 12-bit unsigned, `Int<8>` matches
//!   `i8`'s logical width with sign + 7 magnitude bits).
//!
//! No std-parallel per-N aliases. The substrate is built around
//! exact-bit-count thinking. Round 202604280034 deleted the eight
//! std-parallel canonical aliases (`Uint8`/`Uint16`/`Uint32`/`Uint64` +
//! signed counterparts) that round 202604271346 retained as a
//! transitional bridge. Consumers express exact widths directly
//! through `Uint<N, S>` / `Int<N, S>`. Std-parallel sugar reintroduces
//! bucket thinking and undermines the central premise.
//!
//! Strategy default is `Warm` where Warm is valid (`I + F <= 32`);
//! above that, the alias requires explicit strategy or fails
//! compilation with a `UContainerFor`-bound error pointing the
//! consumer at strategy choice (via the
//! `#[diagnostic::on_unimplemented]` attribute on `UContainerFor`).

use crate::ifixed::IFixed;
use arvo_storage::{fbits, ibits};
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
    UFixed<{ ibits(I as u16) }, { fbits(F as u16) }, S>;

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
    IFixed<{ ibits(I as u16) }, { fbits(F as u16) }, S>;

/// Unsigned fixed-point integer alias parameterised by bit count `N`.
///
/// `Uint<N, S>` is `UFixed<{ ibits(N) }, { fbits(0) }, S>`. Use at
/// consumer call sites for clean integer-typed bit counts. The
/// canonical-width survivors below are sugar for the four
/// std-parallel widths; every other width goes through this generic
/// form (`Uint<5>`, `Uint<12>`, `Uint<28>`, etc.).
///
/// ```ignore
/// type Counter = arvo::Uint<32, Hot>;     // 32-bit counter
/// type Width = arvo::Uint<12>;             // 12-bit width, Warm default
/// ```
pub type Uint<const N: u8, S = Warm> = UFixed<{ ibits(N as u16) }, { fbits(0u16) }, S>;

/// Signed fixed-point integer alias parameterised by total bit count `N`.
///
/// `Int<N, S>` is `IFixed<{ ibits(N - 1) }, { fbits(0) }, S>`. The
/// `N - 1` accounts for the IFixed sign bit so `Int<8, _>` matches
/// `i8`'s logical width. Use at consumer call sites for clean
/// integer-typed bit counts; the canonical-width survivors below are
/// sugar for the four std-parallel widths.
///
/// ```ignore
/// type Delta = arvo::Int<8, Hot>;          // signed 8-bit
/// type Offset = arvo::Int<16>;             // signed 16-bit, Warm default
/// ```
pub type Int<const N: u8, S = Warm> = IFixed<{ ibits((N - 1) as u16) }, { fbits(0u16) }, S>;

