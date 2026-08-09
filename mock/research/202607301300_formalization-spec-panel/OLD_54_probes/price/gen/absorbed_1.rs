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
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G1 as Numeral>::EMIN;
    acc
}
