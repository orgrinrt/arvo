//! Float `ConstSign` impls, which feed the predicate blankets.
//!
//! Split out of `traits.rs`, which carried all seven trait families in
//! one file well past the size limit.

use crate::float::{FastFloat, StrictFloat};
use arvo_transparent::Transparent;

// --- Float ConstSign impls — feed the predicate-wrapper blankets --------
//
// arvo-numeric-contracts ships `IsPositiveOf<T>` / `IsNonNegativeOf<T>`
// / `IsZeroOrPositiveOf<T>` blanket Predicate impls bound on `T:
// [const] ConstSign`. Fixed types pick up ConstSign automatically via
// the ConstOrd+Identity blanket. Floats opt out of ConstOrd (NaN
// breaks reflexivity), so they need direct ConstSign impls. Bodies
// use bare-primitive `>` / `>=` against 0.0 — const-callable on
// f32/f64. NaN compares as not-greater and not-greater-or-equal,
// consistently returning false for all sign predicates, which is the
// intended semantic.

use arvo_numeric_contracts::ConstSign;
use arvo_storage::Bool;

macro_rules! float_const_sign_impl {
    ($wrapper:ident, $inner:ty, $zero:expr) => {
        const impl ConstSign for $wrapper<$inner> {
            #[inline(always)]
            fn is_positive(self) -> Bool {
                Bool(<Self as Transparent>::raw(self) > $zero)
            }
            #[inline(always)]
            fn is_non_negative(self) -> Bool {
                Bool(<Self as Transparent>::raw(self) >= $zero)
            }
            #[inline(always)]
            fn is_zero_or_positive(self) -> Bool {
                Bool(<Self as Transparent>::raw(self) >= $zero)
            }
        }
    };
}

float_const_sign_impl!(FastFloat, f32, 0.0_f32);
float_const_sign_impl!(FastFloat, f64, 0.0_f64);
float_const_sign_impl!(StrictFloat, f32, 0.0_f32);
float_const_sign_impl!(StrictFloat, f64, 0.0_f64);
