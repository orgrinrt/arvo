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

type D32 = O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<I<O<O<O<O<O<O<I<I<I<I<I<O<I<I<I<O<O<I<I<O<I<O<I<I<O<I<O<O<O<O<I<I<I<O<I<I<O<I<O<I<O<O<O<O<O<I<O<I<O<I<I<O<I<I<O<I<O<I<I<O<I<O<O<O<I<I<I<O<I<I<I<O<O<H>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>;
pub type AD32 = Ratio<H, D32>;
pub type G32 = Fx<Two, PP8, EZero, AD32, BZero, Symmetric>;
const _: () = assert!(<G32 as Numeral>::R == 2);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G32 as Numeral>::EMIN;
    acc
}
