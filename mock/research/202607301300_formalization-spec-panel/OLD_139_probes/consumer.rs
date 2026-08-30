// The bridge without a table: the consumer writes the literal, the crossing is an
// expansion, and the derivation is 137's ladder unchanged.
#![no_std]
#![allow(dead_code)]
extern crate ladder;
extern crate natmac;
use ladder::*;
use natmac::nat;

// --- the surface. one macro, no impls, no cap, no width listed anywhere ------
macro_rules! UFixed {
    ($i:literal, $f:literal, $s:ty) => { Fixed<nat!($i), nat!($f), $s> };
}
pub struct Warm;
pub struct Hot;
pub struct Fixed<I, F, S>(PhantomData<(I, F, S)>);
use core::marker::PhantomData;

// --- does the expansion agree with 137's hand-written aliases? ---------------
// Type equality is checked by the compiler: these transmute-free coercions only
// build if the expansion produced the identical type.
const _: () = {
    fn same<T>(_: PhantomData<T>, _: PhantomData<T>) {}
    fn check() {
        same::<nat!(0)>(PhantomData, PhantomData::<T0>);
        same::<nat!(3)>(PhantomData, PhantomData::<T3>);
        same::<nat!(8)>(PhantomData, PhantomData::<T8>);
        same::<nat!(13)>(PhantomData, PhantomData::<T13>);
        same::<nat!(16)>(PhantomData, PhantomData::<T16>);
        same::<nat!(24)>(PhantomData, PhantomData::<T24>);
        same::<nat!(30)>(PhantomData, PhantomData::<T30>);
        same::<nat!(40)>(PhantomData, PhantomData::<T40>);
        same::<nat!(41)>(PhantomData, PhantomData::<T41>);
        same::<nat!(64)>(PhantomData, PhantomData::<T64>);
        same::<nat!(100)>(PhantomData, PhantomData::<T100>);
        same::<nat!(200)>(PhantomData, PhantomData::<T200>);
        same::<nat!(777)>(PhantomData, PhantomData::<T777>);
    }
};

// --- widths arvo never listed, at three orders of magnitude ------------------
pub type Odd1 = nat!(4099);
pub type Odd2 = nat!(65537);
pub type Odd3 = nat!(1000003);

// --- the surface, with the literal spelling ----------------------------------
pub type A = UFixed!(13, 3, Warm);
pub type B = UFixed!(777, 41, Hot);
pub type C = UFixed!(4099, 0, Hot);
