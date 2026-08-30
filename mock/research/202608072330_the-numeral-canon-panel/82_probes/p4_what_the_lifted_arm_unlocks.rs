// PROBE p4. Does the lifted arm reach a lowering the backend cannot reach
// unaided, and does the declaration erase?
//
// 80 section 5.2 established that a law can be true and buy nothing: at F = 0
// the backend performed the distributive rewrite itself and the fused and
// general arms became the same symbol. So a law layer's real question is not
// whether the law holds but whether it unlocks something.
//
// 80 section 5.3 answered that for UNSIGNED saturating addition, where the law
// is universal and needs no declaration. This probe asks it for the SIGNED
// case, which is the one the register grades as a magma licensing no
// reassociation at all (`OPTIONS.md:1551-1555`), and which the sign-uniform
// declaration upgrades.
//
// Four arms plus two controls, each `#[no_mangle] extern "C"` and
// `#[inline(never)]` so the emitted assembly is per arm rather than one fused
// blob.
//
// The erasure check is 68's symbol-aliasing instrument: if the declaration is
// fully erased, two arms differing ONLY in which licensed window they name must
// assemble to the same symbol.

#![no_std]
#![allow(dead_code)]

pub trait Window {
    const LO: i32;
    const HI: i32;
}
pub struct NonNeg<const LO: u8, const HI: u8>;
pub struct NonPos<const MAG_LO: u8, const MAG_HI: u8>;
pub struct Win<const LO: i32, const HI: i32>;

impl<const LO: u8, const HI: u8> Window for NonNeg<LO, HI> {
    const LO: i32 = LO as i32;
    const HI: i32 = HI as i32;
}
impl<const MAG_LO: u8, const MAG_HI: u8> Window for NonPos<MAG_LO, MAG_HI> {
    const LO: i32 = -(MAG_HI as i32);
    const HI: i32 = -(MAG_LO as i32);
}
impl<const LO: i32, const HI: i32> Window for Win<LO, HI> {
    const LO: i32 = LO;
    const HI: i32 = HI;
}

pub trait ReassociableFold: Window {}
impl<const LO: u8, const HI: u8> ReassociableFold for NonNeg<LO, HI> {}
impl<const MAG_LO: u8, const MAG_HI: u8> ReassociableFold for NonPos<MAG_LO, MAG_HI> {}

#[inline(always)]
fn sat(x: i8, y: i8) -> i8 {
    x.saturating_add(y)
}

// ---------------------------------------------------------------------------
// ARM 0. The fold as written. Legal at any window. A backend cannot reassociate
// it, because signed saturating addition is not associative in general and the
// backend has no way to learn the operand window.
// ---------------------------------------------------------------------------
#[no_mangle]
#[inline(never)]
pub extern "C" fn sat_sum_seq(p: *const i8, n: usize) -> i8 {
    let xs = unsafe { core::slice::from_raw_parts(p, n) };
    let mut acc: i8 = 0;
    let mut i = 0;
    while i < xs.len() {
        acc = sat(acc, xs[i]);
        i += 1;
    }
    acc
}

// ---------------------------------------------------------------------------
// ARM 1. Licensed by the declaration: four independent accumulators, which is a
// reassociation. Indexes the slice, so the bounds are not provable. 80 section
// 5.3 found this shape LOST for the unsigned case; included so the same
// question is asked here rather than assumed away.
// ---------------------------------------------------------------------------
#[inline(never)]
fn lanes4_indexed<W: ReassociableFold>(xs: &[i8]) -> i8 {
    let mut a = [0i8; 4];
    let mut i = 0;
    while i + 4 <= xs.len() {
        a[0] = sat(a[0], xs[i]);
        a[1] = sat(a[1], xs[i + 1]);
        a[2] = sat(a[2], xs[i + 2]);
        a[3] = sat(a[3], xs[i + 3]);
        i += 4;
    }
    let mut t: i8 = 0;
    while i < xs.len() {
        t = sat(t, xs[i]);
        i += 1;
    }
    sat(sat(sat(a[0], a[1]), sat(a[2], a[3])), t)
}

#[no_mangle]
pub extern "C" fn sat_sum_lanes4_nonneg(p: *const i8, n: usize) -> i8 {
    lanes4_indexed::<NonNeg<0, 127>>(unsafe { core::slice::from_raw_parts(p, n) })
}

// ---------------------------------------------------------------------------
// ARM 2. Licensed by the same declaration, plus the bounds proof supplied the
// way 80 section 5.3's first attack supplied it: iterate `chunks_exact(16)` so
// no bound has to be proved.
// ---------------------------------------------------------------------------
#[inline(never)]
fn lanes16_chunked<W: ReassociableFold>(xs: &[i8]) -> i8 {
    let mut acc = [0i8; 16];
    let mut ch = xs.chunks_exact(16);
    for c in &mut ch {
        let mut k = 0;
        while k < 16 {
            acc[k] = sat(acc[k], c[k]);
            k += 1;
        }
    }
    let mut t: i8 = 0;
    for &x in ch.remainder() {
        t = sat(t, x);
    }
    let mut k = 0;
    while k < 16 {
        t = sat(t, acc[k]);
        k += 1;
    }
    t
}

#[no_mangle]
pub extern "C" fn sat_sum_lanes16_nonneg(p: *const i8, n: usize) -> i8 {
    lanes16_chunked::<NonNeg<0, 127>>(unsafe { core::slice::from_raw_parts(p, n) })
}

// ---------------------------------------------------------------------------
// ARM 3. The SAME arm at the other licensed window. Nothing about the emitted
// code should differ, because the declaration is erased. This is the erasure
// check: if the two symbols alias, the declaration cost nothing at runtime.
// ---------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn sat_sum_lanes16_nonpos(p: *const i8, n: usize) -> i8 {
    lanes16_chunked::<NonPos<0, 128>>(unsafe { core::slice::from_raw_parts(p, n) })
}

// A third licensed window, narrower, to make the aliasing check three-way
// rather than two-way.
#[no_mangle]
pub extern "C" fn sat_sum_lanes16_smallgain(p: *const i8, n: usize) -> i8 {
    lanes16_chunked::<NonNeg<3, 40>>(unsafe { core::slice::from_raw_parts(p, n) })
}

// ---------------------------------------------------------------------------
// CONTROL A. Wrapping addition, associative unconditionally, which the backend
// reassociates with no help from any typestate. This is the honest comparator:
// the density a backend reaches when it needs to be told nothing.
// ---------------------------------------------------------------------------
#[no_mangle]
#[inline(never)]
pub extern "C" fn wrap_sum_seq(p: *const i8, n: usize) -> i8 {
    let xs = unsafe { core::slice::from_raw_parts(p, n) };
    let mut acc: i8 = 0;
    let mut i = 0;
    while i < xs.len() {
        acc = acc.wrapping_add(xs[i]);
        i += 1;
    }
    acc
}

// ---------------------------------------------------------------------------
// CONTROL B. The chunked shape applied to the SEQUENTIAL semantics, i.e. the
// bounds proof supplied WITHOUT the law. If this reaches the same density as
// ARM 2, the law bought nothing and the whole win was the bounds proof. That is
// 80 section 5.2's failure mode and it has to be excluded explicitly rather
// than assumed away.
// ---------------------------------------------------------------------------
#[no_mangle]
#[inline(never)]
pub extern "C" fn sat_sum_seq_chunked_no_law(p: *const i8, n: usize) -> i8 {
    let xs = unsafe { core::slice::from_raw_parts(p, n) };
    let mut acc: i8 = 0;
    let mut ch = xs.chunks_exact(16);
    for c in &mut ch {
        let mut k = 0;
        while k < 16 {
            acc = sat(acc, c[k]);
            k += 1;
        }
    }
    for &x in ch.remainder() {
        acc = sat(acc, x);
    }
    acc
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
