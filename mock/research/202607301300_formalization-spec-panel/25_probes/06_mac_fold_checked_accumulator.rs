// Probe: the runtime-loop / fold case. A genuinely runtime-length loop
// cannot grow its accumulator's TYPE per iteration (the type would have to
// differ per trip count, which is not a runtime value). The MAC discipline
// (file 24 section 2) answer: the per-element product numeral is FIXED
// (same I1+I2, F1+F2 every iteration, since the two input numerals are
// fixed types across the loop), and only the accumulator needs headroom
// for the trip count N, which IS known at compile time (an array length /
// const generic). Sizing that headroom is a CHECKED BOUND, not a computed
// type: `ceil(log2 N)` never needs to appear in type position at all.
#![allow(dead_code)]

use core::marker::PhantomData;

pub trait Bit {
    const VAL: u16;
}
pub struct B0;
pub struct B1;
impl Bit for B0 {
    const VAL: u16 = 0;
}
impl Bit for B1 {
    const VAL: u16 = 1;
}

pub trait Width {
    const VALUE: u16;
}
pub struct UTerm;
pub struct UInt<Hi, Lo>(PhantomData<(Hi, Lo)>);
impl Width for UTerm {
    const VALUE: u16 = 0;
}
impl<Hi: Width, Lo: Bit> Width for UInt<Hi, Lo> {
    const VALUE: u16 = Hi::VALUE * 2 + Lo::VAL;
}

#[derive(Clone, Copy)]
pub struct Number<I: Width, F: Width>(pub i128, PhantomData<(I, F)>);
impl<I: Width, F: Width> Number<I, F> {
    pub const fn from_raw(raw: i128) -> Self {
        Number(raw, PhantomData)
    }
}

const fn ceil_log2(n: usize) -> u16 {
    if n <= 1 {
        return 0;
    }
    let mut bits = 0u16;
    let mut v = n - 1;
    while v > 0 {
        v >>= 1;
        bits += 1;
    }
    bits
}

// the checked bound: a compile-time assertion, not a type-level computation.
// fails to build (not silently narrows) when the accumulator is too small.
pub const fn assert_accumulator_sufficient<const N: usize>(product_width: u16, acc_width: u16) {
    assert!(
        acc_width >= product_width + ceil_log2(N),
        "accumulator numeral too narrow for this MAC's trip count"
    );
}

// one product numeral (fixed I1+I2/F1+F2), N of them summed, ONE checked
// accumulator numeral, ONE quantisation at the boundary via the caller.
pub fn mac<const N: usize, PI: Width, PF: Width, AccI: Width, AccF: Width>(
    products: [Number<PI, PF>; N],
) -> Number<AccI, AccF> {
    const { assert_accumulator_sufficient::<N>(PI::VALUE, AccI::VALUE) };
    // interior is exact integer addition: associative, reassociable freely,
    // because the checked bound above is exactly what makes that safe.
    let mut acc: i128 = 0;
    let mut i = 0;
    while i < N {
        acc += products[i].0;
        i += 1;
    }
    Number::from_raw(acc)
}

pub type U3 = UInt<UInt<UTerm, B1>, B1>; // width 3
pub type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>; // width 6 (product width for two 3-wide operands, Q1.2 x Q1.2 -> Q2.4 is a stand-in; widths here are illustrative, not literal I+F)
pub type U9 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>; // width 9, room for +3 headroom over width 6 (INSUFFICIENT for N=256, kept to demonstrate the refusal below)
pub type U14 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>; // width 14, +8 headroom over width 6 (SUFFICIENT for N=256)

fn main() {
    // 256 elements: ceil_log2(256) = 8, matching the 56000's own 8 guard
    // bits over its 48-bit product (file 24 section 2's own citation).
    let products: [Number<U6, U6>; 256] = [const { Number::from_raw(1) }; 256];
    // U9 is genuinely insufficient (headroom 3 < ceil_log2(256)=8); left
    // commented rather than deleted, since it is the adversarial case that
    // proves the check is load-bearing rather than decorative:
    //   let _acc: Number<U9, U9> = mac(products); // compile-time panic
    let acc: Number<U14, U14> = mac(products);
    println!(
        "acc.0 = {} (width 14, headroom 8, sufficient for N=256)",
        acc.0
    );
}

#[no_mangle]
pub extern "C" fn probe_mac_256(products: [Number<U6, U6>; 256]) -> Number<U14, U14> {
    mac(products)
}
