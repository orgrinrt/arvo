//! The five consumer cases, written once and expanded in each arm's consumer
//! crate, which has no `#![feature]` line. What expands there is what a
//! consumer writes; the one place the arms differ is the bound case (3) needs,
//! which each consumer passes in and which the findings report.

/// Expands the five cases against the value type `$num` (a type taking one
/// signature parameter), with `$($bound)*` the bounds case (3) writes on `S`.
#[macro_export]
macro_rules! consumer_cases {
    ($num:ident, [$($bound:tt)*]) => {
        use $crate::__private::arvo_format::overflow::Wrap;
        use $crate::__private::arvo_format::points::{Integer, UFixed};
        use $crate::__private::arvo_format::rounding::TowardZero;
        use $crate::__private::arvo_format::{Adapt, Signature};
        use $crate::Lane;

        /// A 13-bit signed wrapping signature, which the map puts in 16 bits.
        pub type S13 = Signature<Integer<13>, Adapt<TowardZero, Wrap>>;

        /// A 40-bit signed wrapping signature, which the map puts in 64 bits.
        pub type S40 = Signature<Integer<40>, Adapt<TowardZero, Wrap>>;

        /// (1) The value held in a struct field.
        pub struct Holder {
            pub value: $num<S13>,
        }

        /// (2) `add` called in a `const` item.
        pub const SUM: $num<S13> = $num::<S13>::from_raw(4000).add($num::<S13>::from_raw(300));

        /// (3) `add` called from a function generic over the signature.
        pub fn generic_over_signature<S: $($bound)*>(a: $num<S>, b: $num<S>) -> $num<S> {
            a.add(b)
        }

        /// (4) `add` called from a function generic over a const fraction
        /// length at width 64.
        pub fn generic_over_fraction<const FRAC: i32>(
            a: $num<Signature<UFixed<64, FRAC>, Adapt<TowardZero, Wrap>>>,
            b: $num<Signature<UFixed<64, FRAC>, Adapt<TowardZero, Wrap>>>,
        ) -> $num<Signature<UFixed<64, FRAC>, Adapt<TowardZero, Wrap>>> {
            a.add(b)
        }

        /// (5) A kernel written against `Lane` only: the exact sum of a run,
        /// less its first member, adapted once.
        pub fn lane_kernel<L: Lane>(run: &[L]) -> L {
            let mut acc = L::WIDE_ZERO;
            for x in run {
                acc = L::wide_add(acc, x.widen());
            }
            if let Some(first) = run.first() {
                acc = L::wide_sub(acc, first.widen());
            }
            L::narrow(acc)
        }

        /// Case (5) instantiated with the arm's value type.
        pub fn lane_kernel_at_13(run: &[$num<S13>]) -> $num<S13> {
            lane_kernel(run)
        }

        /// Cases (3) and (4) instantiated, so they are monomorphised and not
        /// only checked.
        pub fn instantiated() -> (i64, u64) {
            // Turbofished: under arm D `S` cannot be inferred from `Num<S>`, a
            // projection, and the call is refused without it (consumer-d's
            // `generic_call` pair).
            let a = generic_over_signature::<S40>($num::<S40>::from_raw(1), $num::<S40>::from_raw(2));
            type F = Signature<UFixed<64, -16>, Adapt<TowardZero, Wrap>>;
            let b = generic_over_fraction::<-16>($num::<F>::from_raw(u64::MAX), $num::<F>::from_raw(2));
            (a.raw(), b.raw())
        }

        #[cfg(test)]
        mod consumer_tests {
            use super::*;

            #[test]
            fn the_cases_compute() {
                // 4300 wraps once in 13 bits.
                assert_eq!(SUM.raw(), 4300 - 8192);
                assert_eq!(instantiated(), (3, 1));
                let run = [
                    $num::<S13>::from_raw(4000),
                    $num::<S13>::from_raw(4000),
                    $num::<S13>::from_raw(1),
                ];
                // 4000 + 1 exactly after the first member's removal, adapted once.
                assert_eq!(lane_kernel_at_13(&run).raw(), 4001);
                let held = Holder { value: SUM };
                assert_eq!(held.value, SUM);
            }
        }
    };
}
