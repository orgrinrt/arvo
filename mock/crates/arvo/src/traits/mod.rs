//! Numeric traits.
//!
//! Five composable traits give generic algorithms a minimum vocabulary
//! for arithmetic over arvo's numeric family without pulling in
//! per-strategy arithmetic impls.
//!
//! | Trait          | Surface                                     | Expected on         |
//! |----------------|---------------------------------------------|---------------------|
//! | `TotalOrd`     | `total_cmp(&self, &other) -> Ordering`      | all numerics        |
//! | `Sqrt`         | `sqrt(self) -> Self`                        | floats, integer UFixed |
//! | `Recip`        | `recip(self) -> Self`                       | floats              |
//! | `Abs`          | `abs(self) -> Self`                         | signed + UFixed (id) |
//! | `FromConstant` | `from_constant(USize) -> Self`              | every concrete type |
//!
//! Fractional UFixed / IFixed do NOT get `Sqrt` / `Recip` in this
//! round — those require fixed-point arithmetic tables that land in
//! a later round. The trait surface exists; the per-type impls stop
//! at the unambiguous cases (integer UFixed sqrt via `u*::isqrt`, and
//! every float wrapper).

mod abs;
mod const_sign;
mod euclid;
mod from_constant;
mod recip;
mod sqrt;
mod total_ord;

pub use arvo_numeric_contracts::{Abs, FromConstant, Recip, Sqrt, TotalOrd};
pub use euclid::{EuclidDiv, EvenShares, EvenSplittable, ScalarEuclid};
