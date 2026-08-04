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

type D38 = O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<I<O<O<I<O<O<O<I<O<O<O<I<O<I<O<O<O<I<I<O<O<I<O<O<O<O<O<I<O<I<I<I<I<O<O<O<I<O<O<O<I<I<O<I<I<O<O<O<O<I<O<I<O<I<I<O<I<O<O<O<O<I<O<I<O<I<O<O<I<I<O<O<I<O<I<I<O<I<I<I<O<O<I<I<O<I<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;
pub type AD38 = Ratio<H, D38>;
pub type G38 = Fx<Two, PP8, EZero, AD38, BZero, Symmetric>;
const _: () = assert!(<G38 as Numeral>::R == 2);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G38 as Numeral>::EMIN;
    acc
}
