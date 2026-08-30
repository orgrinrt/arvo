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

type K50 = O<I<O<O<I<H>>>>>;
pub type G50 = Fx<Ten, PP8, ENeg<K50>, A1, BZero, Symmetric>;
const _: () = assert!(<G50 as Numeral>::EMIN == -50);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G50 as Numeral>::EMIN;
    acc
}
