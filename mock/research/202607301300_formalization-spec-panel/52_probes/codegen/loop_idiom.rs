#![no_std]
#![allow(improper_ctypes_definitions)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// the assert-equal-lengths idiom, exactly as file 32/34 wrote it: one
// boolean conjunction, checked once, ahead of the loop. extern "C" and
// #[inline(never)] match the original probe's methodology
// (32_probes/identity_model/src/lib.rs) so the two functions cannot be
// folded into each other or reordered relative to the assert.
#[no_mangle]
#[inline(never)]
pub extern "C" fn add_assert_idiom(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert!(a.len() == b.len() && b.len() == out.len());
    for i in 0..a.len() {
        out[i] = a[i].wrapping_add(b[i]);
    }
}

// the no-assert idiom: identical indexing loop, no prior length check at
// all, each index carries its own independent bounds check instead.
#[no_mangle]
#[inline(never)]
pub extern "C" fn add_no_assert_idiom(a: &[i64], b: &[i64], out: &mut [i64]) {
    for i in 0..a.len() {
        out[i] = a[i].wrapping_add(b[i]);
    }
}
