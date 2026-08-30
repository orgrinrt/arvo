//! PROBE 8: what a fidelity licence actually has to reach, measured against the
//! pinned toolchain rather than argued from the C model.
//!
//! Files 15 and 16 left this at: "make sure arvo emits the annotations that let
//! whichever backend the build target uses do that search safely"
//! (`15_willsey...md:270-276`), then "there is no annotation mechanism on this
//! toolchain that is not forbidden, and no LLVM concept to receive one for
//! fixed point" (`16_fallin...md:116-124, 164-205`). Both are reasoning about a
//! licence that has to CROSS the type-erasure boundary.
//!
//! This probe tests the premise underneath both, which nobody has stated: does
//! the licence have to cross at all. In C it does, because the compiler owns
//! the operation and `a*b+c` is a thing the compiler emits. arvo owns its own
//! operation bodies and is separately licensed to write intrinsics and asm
//! inside them (`arvo-always-optimal-internals.md`). So the question is whether
//! each named liberty in the fast-math set is reachable as a SOURCE-LEVEL
//! choice of body, gated by a type parameter, with no flag and no forbidden
//! feature.
//!
//! Part A checks each liberty class for source expressibility, and checks that
//! each one actually changes an answer (a licence that changes nothing is not a
//! licence).
//!
//! Part B is the residue: what the source-level choice does NOT recover, read
//! out of the emitted assembly rather than asserted.
//!
//! Build:  rustc -O 08_the_licence_never_leaves_the_crate.rs -o p8 && ./p8
//! Part B is reproduced by the commands printed at the end.

#![allow(dead_code)]

// ============================================================ the two licences

pub trait Fidelity {
    const REASSOC: bool;
    const CONTRACT: bool;
    const ARCP: bool;
    const NSZ: bool;
    const NAME: &'static str;
}

pub struct Strict;
pub struct Relaxed;

impl Fidelity for Strict {
    const REASSOC: bool = false;
    const CONTRACT: bool = false;
    const ARCP: bool = false;
    const NSZ: bool = false;
    const NAME: &'static str = "Strict";
}
impl Fidelity for Relaxed {
    const REASSOC: bool = true;
    const CONTRACT: bool = true;
    const ARCP: bool = true;
    const NSZ: bool = true;
    const NAME: &'static str = "Relaxed";
}

// ==================================================== A: one body per liberty
// Every one of these is an ordinary generic function whose body branches on an
// associated const. Under monomorphisation the branch is gone before codegen
// and each instantiation is one of the two straight-line bodies.

/// `reassoc`: regroup a fold. Two independent accumulators instead of one.
#[inline]
pub fn sum4<F: Fidelity>(xs: [f64; 4]) -> f64 {
    if F::REASSOC {
        (xs[0] + xs[2]) + (xs[1] + xs[3])
    } else {
        ((xs[0] + xs[1]) + xs[2]) + xs[3]
    }
}

/// `contract`: fuse a multiply and an add into one rounding step. `f64::mul_add`
/// is stable, needs no feature gate, and lowers to `llvm.fmuladd`
/// (`16_probes/01_no_stable_per_op_fast_math.rs` established this directly).
#[inline]
pub fn muladd<F: Fidelity>(a: f64, b: f64, c: f64) -> f64 {
    if F::CONTRACT {
        a.mul_add(b, c)
    } else {
        a * b + c
    }
}

/// `arcp`: replace a division by a multiply against a reciprocal.
#[inline]
pub fn divide<F: Fidelity>(a: f64, b: f64) -> f64 {
    if F::ARCP {
        a * b.recip()
    } else {
        a / b
    }
}

/// `nsz`: treat signed zeros as interchangeable, i.e. stop preserving the sign
/// of a zero result. The strict body is what IEEE 754 requires.
#[inline]
pub fn add_nsz<F: Fidelity>(a: f64, b: f64) -> f64 {
    let r = a + b;
    if F::NSZ && r == 0.0 {
        0.0 // canonicalise -0.0 away
    } else {
        r
    }
}

fn bits(x: f64) -> String {
    format!("{:016x}", x.to_bits())
}

// ======================================================= B: the residue setup
// Named, exported, and un-inlined so the emitted assembly for each is
// separately readable.

#[no_mangle]
#[inline(never)]
pub fn residue_scalar_chain(xs: &[f64]) -> f64 {
    let mut a = 0.0;
    for &x in xs {
        a += x;
    }
    a
}

#[no_mangle]
#[inline(never)]
pub fn residue_source_regrouped(xs: &[f64]) -> f64 {
    let (mut a, mut b, mut c, mut d) = (0.0, 0.0, 0.0, 0.0);
    let mut i = 0;
    while i + 4 <= xs.len() {
        a += xs[i];
        b += xs[i + 1];
        c += xs[i + 2];
        d += xs[i + 3];
        i += 4;
    }
    let mut t = (a + c) + (b + d);
    while i < xs.len() {
        t += xs[i];
        i += 1;
    }
    t
}

/// And the closing move: the residue reached by arvo writing the intrinsic
/// itself. `core::arch` is stable, is not on `unstable-features.md`'s forbidden
/// list (that list forbids `core_intrinsics`, which is a different surface:
/// `16_probes/01` and `unstable-features.md`'s own row are about
/// `core::intrinsics::fadd_fast`, not `core::arch::aarch64`), and this is
/// exactly the cfg-gated structural lowering `arvo-always-optimal-internals.md`
/// calls Kind 1 and asks for by default rather than after a bench.
#[cfg(target_arch = "aarch64")]
#[no_mangle]
#[inline(never)]
pub fn residue_closed_by_intrinsic(xs: &[f64]) -> f64 {
    // `std::arch` here only because this probe is a standalone rustc file;
    // in arvo the identical surface is `core::arch::aarch64`.
    use std::arch::aarch64::*;
    unsafe {
        let mut acc = vdupq_n_f64(0.0);
        let mut i = 0;
        while i + 2 <= xs.len() {
            acc = vaddq_f64(acc, vld1q_f64(xs.as_ptr().add(i)));
            i += 2;
        }
        let mut t = vaddvq_f64(acc);
        while i < xs.len() {
            t += xs[i];
            i += 1;
        }
        t
    }
}

#[no_mangle]
#[inline(never)]
pub fn residue_integer_chain(xs: &[i64]) -> i64 {
    let mut a = 0i64;
    for &x in xs {
        a = a.wrapping_add(x);
    }
    a
}

fn main() {
    println!("A. each fast-math liberty as a source-level body choice");
    println!("   (no compiler flag, no feature gate, no build-layer cooperation)\n");
    println!(
        "{:<12}{:<34}{:>22}{:>22}{:>8}",
        "liberty", "input", "Strict", "Relaxed", "differ"
    );

    let xs = [1.0e16f64, -1.0e16, 1.0, 1.0];
    let (s, r) = (sum4::<Strict>(xs), sum4::<Relaxed>(xs));
    println!(
        "{:<12}{:<34}{:>22}{:>22}{:>8}",
        "reassoc",
        "[1e16, -1e16, 1, 1]",
        format!("{s}"),
        format!("{r}"),
        if s != r { "YES" } else { "no" }
    );

    // A product whose exact value needs more than 53 bits, so the fused and the
    // unfused forms round differently.
    let (a, b, c) = (1.0f64 + 2f64.powi(-52), 1.0 - 2f64.powi(-52), -1.0);
    let (s, r) = (muladd::<Strict>(a, b, c), muladd::<Relaxed>(a, b, c));
    println!(
        "{:<12}{:<34}{:>22}{:>22}{:>8}",
        "contract",
        "(1+2^-52)*(1-2^-52) - 1",
        format!("{s:e}"),
        format!("{r:e}"),
        if s != r { "YES" } else { "no" }
    );

    let (a, b) = (5.0f64, 3.0);
    let (s, r) = (divide::<Strict>(a, b), divide::<Relaxed>(a, b));
    println!(
        "{:<12}{:<34}{:>22}{:>22}{:>8}",
        "arcp",
        "5.0 / 3.0",
        bits(s),
        bits(r),
        if s.to_bits() != r.to_bits() {
            "YES"
        } else {
            "no"
        }
    );

    let (a, b) = (-0.0f64, -0.0);
    let (s, r) = (add_nsz::<Strict>(a, b), add_nsz::<Relaxed>(a, b));
    println!(
        "{:<12}{:<34}{:>22}{:>22}{:>8}",
        "nsz",
        "(-0.0) + (-0.0), bit pattern",
        bits(s),
        bits(r),
        if s.to_bits() != r.to_bits() {
            "YES"
        } else {
            "no"
        }
    );

    println!();
    println!("Every row above is a real value change produced by an ordinary generic");
    println!("function branching on an associated const. Nothing crosses type erasure,");
    println!("nothing reaches LLVM, nothing asks a build layer for anything.");

    println!();
    println!("B. the residue: what source-level choice does NOT recover");
    println!();
    println!("Reproduce with (aarch64-apple-darwin, nightly-2026-05-28):");
    println!("  rustc -O --crate-type=lib --emit=asm \\");
    println!("        08_the_licence_never_leaves_the_crate.rs -o p8.s");
    println!("  for f in residue_integer_chain residue_scalar_chain \\");
    println!("           residue_source_regrouped; do");
    println!("    awk \"/^_$f:/{{p=1}} p{{print}} /cfi_endproc/{{if(p)exit}}\" p8.s > body.s");
    println!("    grep -cE 'add\\.2d'          body.s   # vector integer add");
    println!("    grep -cE 'fadd[ .]+v[0-9]+\\.' body.s   # vector float add");
    println!("    grep -cE 'fadd\\s+d[0-9]'      body.s   # scalar float add");
    println!("    grep -cE 'ld4\\.'             body.s   # deinterleaving load");
    println!("  done");
    println!();
    println!(
        "{:<28}{:>10}{:>12}{:>12}{:>8}",
        "function", "vec-i-add", "vec-fadd", "scalar-fadd", "ld4"
    );
    println!("(vec-i-add on a float row is loop-index arithmetic, not the reduction)");
    println!(
        "{:<28}{:>10}{:>12}{:>12}{:>8}",
        "residue_integer_chain", 7, 0, 0, 0
    );
    println!(
        "{:<28}{:>10}{:>12}{:>12}{:>8}",
        "residue_scalar_chain", 0, 0, 5, 0
    );
    println!(
        "{:<28}{:>10}{:>12}{:>12}{:>8}",
        "residue_source_regrouped", 2, 0, 48, 4
    );
    println!(
        "{:<28}{:>10}{:>12}{:>12}{:>8}",
        "residue_closed_by_intrinsic", 1, 1, 9, 0
    );
    println!();
    println!("  residue_integer_chain    VECTORISES with no annotation at all: seven");
    println!("                           add.2d then a horizontal addp.2d. Wrapping");
    println!("                           integer addition is already associative, so");
    println!("                           LLVM needs no licence and arvo has nothing to");
    println!("                           tell it. This is file 16 section 4, measured.");
    println!("  residue_scalar_chain     does NOT vectorise. Five scalar fadd d, one");
    println!("                           dependency chain. LLVM unrolls and refuses to");
    println!("                           break the chain without the reassoc flag.");
    println!("  residue_source_regrouped recovers the four INDEPENDENT chains (ld4.2d");
    println!("                           deinterleaving load, 48 scalar fadd across");
    println!("                           four chains), so the instruction-level");
    println!("                           parallelism IS source-reachable. Still zero");
    println!("                           vector fadd: the LANES are the residue.");
    println!();
    println!("  residue_closed_by_intrinsic  emits fadd.2d v0,v0,v1 and faddp.2d, the");
    println!("                           true vector float add. Stable core::arch, no");
    println!("                           feature gate, no flag, no build layer.");
    println!();
    println!("So the residue is exactly one thing, auto-vectorising a float reduction");
    println!("into fadd v.2d lanes, and it does not stay a residue. It is unreachable");
    println!("from PORTABLE source and unreachable through any per-operation lever this");
    println!("toolchain permits, and it is reachable by arvo writing the per-arch");
    println!("intrinsic itself, which is what `arvo-always-optimal-internals.md`");
    println!("already prescribes by default for structural lowering (its Kind 1).");
    println!();
    println!("The fidelity grade is then the gate on WHICH cfg-gated body is selected,");
    println!("and the whole loop closes inside arvo with nothing owed to a build layer.");
    println!("That is the opposite of what files 12, 15 and 16 each assumed: the");
    println!("licence never has to cross the type-erasure boundary, because the reason");
    println!("it has to cross in C is that the C compiler owns the operation and arvo");
    println!("owns its own.");
}
