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

type K100 = O<O<I<O<O<I<H>>>>>>;
pub type G100 = Fx<Ten, PP8, ENeg<K100>, A1, BZero, Symmetric>;
const _: () = assert!(<G100 as Numeral>::EMIN == -100);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G100 as Numeral>::EMIN;
    acc
}
