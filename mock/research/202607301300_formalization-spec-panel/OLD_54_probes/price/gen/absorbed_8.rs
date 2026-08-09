#![allow(dead_code, unused_imports)]

#[path = "../../vu_bias_sealed_adj.rs"]
pub mod bias;
#[path = "../../numeral.rs"]
pub mod numeral;

use bias::nat::{Adjustment, Ratio, H, I, O};
use bias::BZero;
use numeral::*;

type PP8 = O<O<O<H>>>;
type A1 = Ratio<H, H>;

type D1 = O<I<O<H>>>;
pub type AD1 = Ratio<H, D1>;
pub type G1 = Fx<Two, PP8, EZero, AD1, BZero, Symmetric>;
const _: () = assert!(<AD1 as Adjustment>::DEN == 10u64);
const _: () = assert!(<G1 as Numeral>::R == 2);
type D2 = O<O<I<O<O<I<H>>>>>>;
pub type AD2 = Ratio<H, D2>;
pub type G2 = Fx<Two, PP8, EZero, AD2, BZero, Symmetric>;
const _: () = assert!(<AD2 as Adjustment>::DEN == 100u64);
const _: () = assert!(<G2 as Numeral>::R == 2);
type D3 = O<O<O<I<O<I<I<I<I<H>>>>>>>>>;
pub type AD3 = Ratio<H, D3>;
pub type G3 = Fx<Two, PP8, EZero, AD3, BZero, Symmetric>;
const _: () = assert!(<AD3 as Adjustment>::DEN == 1000u64);
const _: () = assert!(<G3 as Numeral>::R == 2);
type D4 = O<O<O<O<I<O<O<O<I<I<I<O<O<H>>>>>>>>>>>>>;
pub type AD4 = Ratio<H, D4>;
pub type G4 = Fx<Two, PP8, EZero, AD4, BZero, Symmetric>;
const _: () = assert!(<AD4 as Adjustment>::DEN == 10000u64);
const _: () = assert!(<G4 as Numeral>::R == 2);
type D5 = O<O<O<O<O<I<O<I<O<I<I<O<O<O<O<I<H>>>>>>>>>>>>>>>>;
pub type AD5 = Ratio<H, D5>;
pub type G5 = Fx<Two, PP8, EZero, AD5, BZero, Symmetric>;
const _: () = assert!(<AD5 as Adjustment>::DEN == 100000u64);
const _: () = assert!(<G5 as Numeral>::R == 2);
type D6 = O<O<O<O<O<O<I<O<O<I<O<O<O<O<I<O<I<I<I<H>>>>>>>>>>>>>>>>>>>;
pub type AD6 = Ratio<H, D6>;
pub type G6 = Fx<Two, PP8, EZero, AD6, BZero, Symmetric>;
const _: () = assert!(<AD6 as Adjustment>::DEN == 1000000u64);
const _: () = assert!(<G6 as Numeral>::R == 2);
type D7 = O<O<O<O<O<O<O<I<O<I<I<O<I<O<O<I<O<O<O<I<I<O<O<H>>>>>>>>>>>>>>>>>>>>>>>;
pub type AD7 = Ratio<H, D7>;
pub type G7 = Fx<Two, PP8, EZero, AD7, BZero, Symmetric>;
const _: () = assert!(<AD7 as Adjustment>::DEN == 10000000u64);
const _: () = assert!(<G7 as Numeral>::R == 2);
type D8 = O<O<O<O<O<O<O<O<I<O<O<O<O<I<I<I<I<O<I<O<I<I<I<I<I<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>;
pub type AD8 = Ratio<H, D8>;
pub type G8 = Fx<Two, PP8, EZero, AD8, BZero, Symmetric>;
const _: () = assert!(<AD8 as Adjustment>::DEN == 100000000u64);
const _: () = assert!(<G8 as Numeral>::R == 2);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G1 as Numeral>::EMIN;
    acc += <G2 as Numeral>::EMIN;
    acc += <G3 as Numeral>::EMIN;
    acc += <G4 as Numeral>::EMIN;
    acc += <G5 as Numeral>::EMIN;
    acc += <G6 as Numeral>::EMIN;
    acc += <G7 as Numeral>::EMIN;
    acc += <G8 as Numeral>::EMIN;
    acc
}
