#![no_std]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// A saturating-add reduction over four i32 lanes, the shape a Precise-strategy
// fold over a saturating-resolution numeral produces. Saturating addition is
// non-associative, so no regrouping recovers vector lanes; there is also no
// LLVM IR flag granting saturating-arithmetic reassociation the way
// `reassoc` grants it for float. Written two ways to control for the
// vectorisable-loop-idiom finding (test 4): plain sequential application
// (no bounds-check idiom at all, so idiom sensitivity cannot be the cause
// of any scalar result here).
#[no_mangle]
pub fn sat_sum4(xs: [i32; 4]) -> i32 {
    let mut acc = 0i32;
    acc = acc.saturating_add(xs[0]);
    acc = acc.saturating_add(xs[1]);
    acc = acc.saturating_add(xs[2]);
    acc = acc.saturating_add(xs[3]);
    acc
}

// the wrapping control: identical shape, non-saturating. LLVM has a real
// reassociation theorem for wrapping addition (file 16/50's own finding)
// and should vectorise or at least tree-reduce this one.
#[no_mangle]
pub fn wrap_sum4(xs: [i32; 4]) -> i32 {
    let mut acc = 0i32;
    acc = acc.wrapping_add(xs[0]);
    acc = acc.wrapping_add(xs[1]);
    acc = acc.wrapping_add(xs[2]);
    acc = acc.wrapping_add(xs[3]);
    acc
}
