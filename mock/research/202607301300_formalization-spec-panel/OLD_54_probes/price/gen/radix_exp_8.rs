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
type K3 = I<H>;
pub type G3 = Fx<Ten, PP8, ENeg<K3>, A1, BZero, Symmetric>;
const _: () = assert!(<G3 as Numeral>::EMIN == -3);
const _: () = assert!(<G3 as Numeral>::R == 10);
type K4 = O<O<H>>;
pub type G4 = Fx<Ten, PP8, ENeg<K4>, A1, BZero, Symmetric>;
const _: () = assert!(<G4 as Numeral>::EMIN == -4);
const _: () = assert!(<G4 as Numeral>::R == 10);
type K5 = I<O<H>>;
pub type G5 = Fx<Ten, PP8, ENeg<K5>, A1, BZero, Symmetric>;
const _: () = assert!(<G5 as Numeral>::EMIN == -5);
const _: () = assert!(<G5 as Numeral>::R == 10);
type K6 = O<I<H>>;
pub type G6 = Fx<Ten, PP8, ENeg<K6>, A1, BZero, Symmetric>;
const _: () = assert!(<G6 as Numeral>::EMIN == -6);
const _: () = assert!(<G6 as Numeral>::R == 10);
type K7 = I<I<H>>;
pub type G7 = Fx<Ten, PP8, ENeg<K7>, A1, BZero, Symmetric>;
const _: () = assert!(<G7 as Numeral>::EMIN == -7);
const _: () = assert!(<G7 as Numeral>::R == 10);
type K8 = O<O<O<H>>>;
pub type G8 = Fx<Ten, PP8, ENeg<K8>, A1, BZero, Symmetric>;
const _: () = assert!(<G8 as Numeral>::EMIN == -8);
const _: () = assert!(<G8 as Numeral>::R == 10);
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
