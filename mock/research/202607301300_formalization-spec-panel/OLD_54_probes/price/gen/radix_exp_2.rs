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

type K1 = H;
pub type G1 = Fx<Ten, PP8, ENeg<K1>, A1, BZero, Symmetric>;
const _: () = assert!(<G1 as Numeral>::EMIN == -1);
const _: () = assert!(<G1 as Numeral>::R == 10);
type K2 = O<H>;
pub type G2 = Fx<Ten, PP8, ENeg<K2>, A1, BZero, Symmetric>;
const _: () = assert!(<G2 as Numeral>::EMIN == -2);
const _: () = assert!(<G2 as Numeral>::R == 10);
pub fn forced() -> i64 {
    let mut acc = 0i64;
    acc += <G1 as Numeral>::EMIN;
    acc += <G2 as Numeral>::EMIN;
    acc
}
