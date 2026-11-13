//! `add` at 13 bits in 16, signed and unsigned, under `Wrap` and `Saturate`, and
//! at full width 64 under `Wrap` beside native `wrapping_add`, for arms A and B.
//!
//! Each probe takes and returns the value type, so what is read is `add` and
//! nothing that builds its operands.
//!
//! Read with `cargo rustc -p asm-probe --release --lib -- --emit asm`. An ad-hoc
//! reading of one build, `release` profile, x86_64 with the default target
//! features: nothing here is a measurement.

#![no_std]

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::TowardZero;
use arvo_format::{Adapt, Signature};

type I13W = Signature<Integer<13>, Adapt<TowardZero, Wrap>>;
type I13S = Signature<Integer<13>, Adapt<TowardZero, Saturate>>;
type U13W = Signature<UFixed<13, 0>, Adapt<TowardZero, Wrap>>;
type U13S = Signature<UFixed<13, 0>, Adapt<TowardZero, Saturate>>;
type I64W = Signature<Integer<64>, Adapt<TowardZero, Wrap>>;

macro_rules! probes {
    ($arm:ident: $($name:ident, $s:ty;)+) => {$(
        #[unsafe(no_mangle)]
        pub fn $name(a: $arm::Num<$s>, b: $arm::Num<$s>) -> $arm::Num<$s> {
            a.add(b)
        }
    )+};
}

probes! { arm_a:
    a_i13_wrap, I13W;
    a_i13_sat, I13S;
    a_u13_wrap, U13W;
    a_u13_sat, U13S;
    a_i64_wrap, I64W;
}

probes! { arm_b:
    b_i13_wrap, I13W;
    b_i13_sat, I13S;
    b_u13_wrap, U13W;
    b_u13_sat, U13S;
    b_i64_wrap, I64W;
}

/// The native operation the full-width wrapping add is read beside.
#[unsafe(no_mangle)]
pub fn native_i64_wrapping_add(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}
