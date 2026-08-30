//! PROBE 4 (source half): the residue file 17 closed with a per-arch intrinsic,
//! restated as a portable per-function rewrite the build layer can perform.
//!
//! `residue_relaxed_reduce` is the ONE operation whose licence is not
//! expressible in portable no_std source (`20_probes/01`): the backend's
//! freedom to reassociate a float reduction into vector lanes. arvo emits it
//! un-inlined, so it survives as a symbol whose v0 name carries the composition
//! (`20_probes/02`). The build layer's job is then a rewrite on that one
//! function, not a flag on the compilation unit.
//!
//! Build: see 04_run.sh

#![crate_type = "lib"]
#![no_std]

#[no_mangle]
#[inline(never)]
pub fn residue_relaxed_reduce(xs: &[f64]) -> f64 {
    let mut a = 0.0;
    let mut i = 0;
    while i < xs.len() {
        a += xs[i];
        i += 1;
    }
    a
}

#[no_mangle]
pub fn caller(xs: &[f64]) -> f64 {
    residue_relaxed_reduce(xs) * 2.0
}
