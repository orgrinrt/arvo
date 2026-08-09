//! p3: the stage nobody named. A law is derived, validated and erased, and then what?
//!
//! For a container, "erase" is the end of the story, because the container IS the
//! output: derive `u32`, validate it holds the declared range, erase the derivation,
//! and what is left is u32 arithmetic, which was the point. For a law there is nothing
//! left after erasure unless the law SELECTED something. A law that is derived,
//! validated and erased without selecting a different lowering has cost compile time
//! and bought nothing.
//!
//! So this file measures the select stage. One computation, two lowerings, one of
//! which is only legal where a law holds:
//!
//!   general : round((x*a) >> F) + round((x*b) >> F)
//!   fused   : round((x*(a+b)) >> F)
//!
//! The two agree exactly at F = 0, where there is no shift and no rounding, and
//! disagree above it. That is the panel's own measured region for distributivity, and
//! it is a const predicate on the typestate, which is the shape op's I13 names.
//!
//! Three arms, all in one binary so the emitted code can be compared side by side:
//!   sel_static_f0   : generic over const F, instantiated at F = 0, selects fused
//!   sel_static_f8   : the same generic function at F = 8, selects general
//!   sel_dynamic     : the same choice made from a runtime value
//!
//! Build:
//!   rustc --edition 2021 -O --emit asm p3_select_and_erase.rs -o p3.s
//!   rustc --edition 2021 -O p3_select_and_erase.rs -o p3 && ./p3
//!
//! Toolchain: nightly-2026-05-28, aarch64-apple-darwin. No feature gates.

#[inline(always)]
const fn general(x: i64, a: i64, b: i64, f: u32) -> i64 {
    let half = if f == 0 { 0 } else { 1i64 << (f - 1) };
    ((x * a + half) >> f) + ((x * b + half) >> f)
}

#[inline(always)]
const fn fused(x: i64, a: i64, b: i64, f: u32) -> i64 {
    let half = if f == 0 { 0 } else { 1i64 << (f - 1) };
    (x * (a + b) + half) >> f
}

/// The law's region, as a const predicate on the typestate. This is the whole content
/// of "the law is derived": at this instantiation, does distributivity hold.
const fn distributes(f: u32) -> bool {
    f == 0
}

/// The select stage, at compile time. `F` is a const parameter, so the branch is
/// resolved during monomorphisation and the unselected arm is never instantiated.
#[inline(never)]
pub fn sel_static<const F: u32>(x: i64, a: i64, b: i64) -> i64 {
    if distributes(F) {
        fused(x, a, b, F)
    } else {
        general(x, a, b, F)
    }
}

#[inline(never)]
pub fn sel_static_f0(x: i64, a: i64, b: i64) -> i64 {
    sel_static::<0>(x, a, b)
}

#[inline(never)]
pub fn sel_static_f8(x: i64, a: i64, b: i64) -> i64 {
    sel_static::<8>(x, a, b)
}

/// The same selection made from a value. Identical source shape, one word changed:
/// `F` is a parameter rather than a const parameter.
#[inline(never)]
pub fn sel_dynamic(x: i64, a: i64, b: i64, f: u32) -> i64 {
    if distributes(f) {
        fused(x, a, b, f)
    } else {
        general(x, a, b, f)
    }
}

/// Controls, so a reader can see what each arm would have emitted alone.
#[inline(never)]
pub fn only_fused_f0(x: i64, a: i64, b: i64) -> i64 {
    fused(x, a, b, 0)
}
#[inline(never)]
pub fn only_general_f0(x: i64, a: i64, b: i64) -> i64 {
    general(x, a, b, 0)
}
#[inline(never)]
pub fn only_general_f8(x: i64, a: i64, b: i64) -> i64 {
    general(x, a, b, 8)
}

fn main() {
    let (x, a, b) = (
        std::hint::black_box(7i64),
        std::hint::black_box(5i64),
        std::hint::black_box(3i64),
    );

    println!("p3: the select stage");
    println!(
        "  F=0 : general={} fused={} agree={}",
        general(x, a, b, 0),
        fused(x, a, b, 0),
        general(x, a, b, 0) == fused(x, a, b, 0)
    );
    println!(
        "  F=8 : general={} fused={} agree={}",
        general(x, a, b, 8),
        fused(x, a, b, 8),
        general(x, a, b, 8) == fused(x, a, b, 8)
    );

    // The arm has to be shown to actually differ somewhere, or the select stage is
    // selecting between two spellings of one thing and buys nothing. Operands are
    // swept over raw fixed-point magnitudes comparable to the quantum, since a
    // rounding difference cannot show up on operands whose products are all far
    // below it.
    const STEP: i64 = 97;
    const RANGE: i64 = 4096;
    let mut total = 0u64;
    let mut d8 = 0u64;
    let mut d0 = 0u64;
    let mut xx = -RANGE;
    while xx < RANGE {
        let mut aa = -RANGE;
        while aa < RANGE {
            let mut bb = -RANGE;
            while bb < RANGE {
                total += 1;
                if general(xx, aa, bb, 8) != fused(xx, aa, bb, 8) {
                    d8 += 1;
                }
                if general(xx, aa, bb, 0) != fused(xx, aa, bb, 0) {
                    d0 += 1;
                }
                bb += STEP;
            }
            aa += STEP;
        }
        xx += STEP;
    }
    println!(
        "  swept raw operands in [-{}, {}) step {}: {} triples",
        RANGE, RANGE, STEP, total
    );
    println!("  at F=8 the two lowerings disagree on {} of {}", d8, total);
    println!("  at F=0 they disagree on {} of {}", d0, total);

    println!(
        "  sel_static_f0={} sel_static_f8={} sel_dynamic(f=0)={} sel_dynamic(f=8)={}",
        sel_static_f0(x, a, b),
        sel_static_f8(x, a, b),
        sel_dynamic(x, a, b, std::hint::black_box(0)),
        sel_dynamic(x, a, b, std::hint::black_box(8))
    );
}
