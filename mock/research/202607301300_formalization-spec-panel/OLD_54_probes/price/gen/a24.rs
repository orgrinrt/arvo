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

type D24 = O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<I<O<O<O<O<I<O<I<I<O<I<I<O<I<I<I<O<O<I<I<O<O<I<I<O<I<I<I<O<O<I<I<I<I<O<I<I<O<O<O<O<I<O<O<O<O<I<I<I<I<O<O<I<O<I<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;
pub type AD24 = Ratio<H, D24>;
pub type G24 = Fx<Two, PP8, EZero, AD24, BZero, Symmetric>;
const _: () = assert!(<G24 as Numeral>::R == 2);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G24 as Numeral>::EMIN;
    acc
}
