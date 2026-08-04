#![no_std]
#![feature(float_algebraic)]
#![allow(improper_ctypes_definitions)]
#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// `fold`'s own emitted form once interior safety is proven: file 50/51's
// measured shape (`50:439-453`, `51_probes/probe_4_licence_reassoc_vectorizes.rs`)
// reproduced against an 8-element array, `.algebraic_add()` throughout,
// exactly the form a build layer is licensed to emit per 51 section 2.4
// once the fold's own `FoldGrowth`-shaped projection shows no interior
// quantisation. `#[inline(never)]` matches probe 4's own methodology.
#[no_mangle]
#[inline(never)]
pub extern "C" fn fold_interior_safe(xs: &[f32; 8]) -> f32 {
    let mut acc = 0.0f32;
    let mut i = 0;
    while i < 8 {
        acc = acc.algebraic_add(xs[i]);
        i += 1;
    }
    acc
}

// `fold_compensated`'s own emitted form, forever: the Kahan step, plain
// `+`/`-`, NEVER `.algebraic_*`, because the licence this fixture's
// sibling function exercises must never reach this combinator
// (51_probes/probe_6_licence_destroys_compensation.rs; `49:184-186`,
// "the one genuinely shaped fold"). What this function pins is not "the
// algebraic form is wrong here" (probe 6 already showed that, once);
// it is "the plain form this combinator actually compiles from stays
// unreassociated on this pin", so a future LLVM change that starts
// reassociating float ops WITHOUT an explicit fast-math flag (which
// would be a much larger event than this one fixture, but is exactly
// the kind of silent drift a regression test exists to catch) shows up
// here rather than only in a lucky-or-unlucky benchmark run.
#[no_mangle]
#[inline(never)]
pub extern "C" fn fold_compensated_step(sum: f32, y: f32) -> f32 {
    let t = sum + y;
    (t - sum) - y
}
