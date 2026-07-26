//! The arithmetic impl invocations, one per `(strategy, container, width)`.
//!
//! Separated from the macros that emit them so a reader looking for which
//! widths a strategy covers is not reading macro bodies, and a reader
//! changing an operation's semantics is not scrolling past the table.

use crate::arith_macros::*;
use crate::arith_macros::{i_mul_fixed_128, u_mul_fixed_128};
// The macro bodies name these; a `macro_rules!` body resolves where it
// expands rather than where it is written, so the import list belongs
// with the invocations.
use crate::identity::{Additive, Identity};
use crate::{BitsContainerFor, Cold, Hot, IArith, Precise, Signed, UArith, Unsigned, Warm};

impl_u_arith_wrapping_widen!(Hot, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping_widen!(Hot, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping_widen!(
    Hot, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_wrapping_widen!(
    Hot, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Hot 65..=128: u128 container; add/sub/mul/div wrap, the fixed-point multiply takes the 256-bit widen
// path (round 202606231229). The 1..=64 Hot path above uses the native i128/u128-intermediate widen.
#[rustfmt::skip]
impl_u_arith_wrapping_widen256!(
    Hot,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_u_arith_wrapping_widen!(Cold, u8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping_widen!(Cold, u16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping_widen!(
    Cold, u32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_u_arith_wrapping_widen!(
    Cold, u64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Cold 65..=128: u128 container; fixed-point multiply takes the 256-bit widen path (round 202606231229).
#[rustfmt::skip]
impl_u_arith_wrapping_widen256!(
    Cold,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_u_arith_wrapping!(Warm, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_wrapping!(Warm, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_wrapping!(Warm, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
// Round 202604280500: Warm 33..=64 wrapping (u128 carrier).
#[rustfmt::skip]
impl_u_arith_wrapping!(
    Warm,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_u_arith_saturating!(Precise, 1, 2, 3, 4, 5, 6, 7, 8);
impl_u_arith_saturating!(Precise, 9, 10, 11, 12, 13, 14, 15, 16);
impl_u_arith_saturating!(Precise, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
#[rustfmt::skip]
impl_u_arith_saturating!(
    Precise,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

// Signed.
impl_i_arith_wrapping_widen!(Hot, i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping_widen!(Hot, i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping_widen!(
    Hot, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping_widen!(
    Hot, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Hot 65..=128: i128 container; fixed-point multiply takes the signed 256-bit widen path (round 202606231229).
#[rustfmt::skip]
impl_i_arith_wrapping_widen256!(
    Hot,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_i_arith_wrapping_widen!(Cold, i8, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping_widen!(Cold, i16, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping_widen!(
    Cold, i32, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
);
#[rustfmt::skip]
impl_i_arith_wrapping_widen!(
    Cold, i64,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
// Cold 65..=128: i128 container; fixed-point multiply takes the signed 256-bit widen path (round 202606231229).
#[rustfmt::skip]
impl_i_arith_wrapping_widen256!(
    Cold,
    65, 66, 67, 68, 69, 70, 71, 72,
    73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88,
    89, 90, 91, 92, 93, 94, 95, 96,
    97, 98, 99, 100, 101, 102, 103, 104,
    105, 106, 107, 108, 109, 110, 111, 112,
    113, 114, 115, 116, 117, 118, 119, 120,
    121, 122, 123, 124, 125, 126, 127, 128
);

impl_i_arith_wrapping!(Warm, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_wrapping!(Warm, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_wrapping!(Warm, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
#[rustfmt::skip]
impl_i_arith_wrapping!(
    Warm,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);

impl_i_arith_saturating!(Precise, 1, 2, 3, 4, 5, 6, 7, 8);
impl_i_arith_saturating!(Precise, 9, 10, 11, 12, 13, 14, 15, 16);
impl_i_arith_saturating!(Precise, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
#[rustfmt::skip]
impl_i_arith_saturating!(
    Precise,
    33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64
);
