// End to end. The consumer writes literals. Nothing is enumerated. The container
// is derived and the operation erases.
#![no_std]
#![allow(dead_code)]
extern crate ladder;
extern crate natmac;
use core::marker::PhantomData;
use ladder::*;
use natmac::nat;
#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub struct Warm;
pub struct Hot;
// total width of a written (I, F) pair, structurally. no const arithmetic anywhere.
pub type W<I, F> = <I as Add<F>>::O;
// the minimum aligned native container for that width, via 137's ladder.
pub type Cont<I, F> = <W<I, F> as Container>::C;

macro_rules! ufixed_container { ($i:literal, $f:literal) => { Cont<nat!($i), nat!($f)> } }

// --- the container the ladder derives, checked against 131:266-272's table ---
const _: () = {
    fn is<T>(_: PhantomData<T>, _: PhantomData<T>) {}
    fn check() {
        is::<ufixed_container!(3, 0)>(PhantomData, PhantomData::<u8>);
        is::<ufixed_container!(5, 3)>(PhantomData, PhantomData::<u8>); // 8 bits
        is::<ufixed_container!(13, 3)>(PhantomData, PhantomData::<u16>); // 16 bits
        is::<ufixed_container!(13, 4)>(PhantomData, PhantomData::<u32>); // 17 bits
        is::<ufixed_container!(29, 3)>(PhantomData, PhantomData::<u32>); // 32 bits
        is::<ufixed_container!(60, 4)>(PhantomData, PhantomData::<u64>); // 64 bits
        is::<ufixed_container!(60, 5)>(PhantomData, PhantomData::<u128>); // 65 bits
        is::<ufixed_container!(100, 28)>(PhantomData, PhantomData::<u128>); // 128
    }
};

// --- widths arvo never listed, well past any table ---------------------------
pub type Huge = ufixed_container!(4099, 0);
pub type Huger = ufixed_container!(65537, 1);

// --- the operation, over the derived container. does it erase? ---------------
#[unsafe(no_mangle)]
pub extern "C" fn derived16(a: u16, b: u16) -> u16 {
    let x: ufixed_container!(13, 3) = a;
    let y: ufixed_container!(13, 3) = b;
    x.wadd(y)
}
#[unsafe(no_mangle)]
pub extern "C" fn native16(a: u16, b: u16) -> u16 {
    a.wrapping_add(b)
}
#[unsafe(no_mangle)]
pub extern "C" fn derived64(a: u64, b: u64) -> u64 {
    let x: ufixed_container!(60, 4) = a;
    let y: ufixed_container!(60, 4) = b;
    x.wadd(y)
}
#[unsafe(no_mangle)]
pub extern "C" fn native64(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
