//! PROBE 7: can the cheap check be an ordinary test, with no tool, no IR, no
//! flags, no build layer.
//!
//! Probe 5 detects an inert axis by hashing generated bodies. That needs an IR
//! dump and a scanner. If the same fact is observable from inside a running
//! test, the tool should not be built, because the design has to stay
//! maintainable while it keeps moving, and a test in the suite survives things
//! an out-of-tree scanner does not.
//!
//! Two candidate observations, both measured here rather than argued:
//!
//!   A. function-pointer identity. If two monomorphisations compile to
//!      identical code, LLVM's mergefunc or the linker's identical-code folding
//!      may give them one address. Then `f::<Hot> as usize == f::<Cold> as
//!      usize` IS the inert-axis check, in one line, in the ordinary suite.
//!
//!   B. reading the first bytes of each function's machine code and comparing
//!      them. Crude, works without ICF, but needs a length and is only sound
//!      because a test binary may read its own text segment.
//!
//! Run with `--cfg inert` to make the axis do nothing and see whether either
//! observation notices.

#![allow(dead_code)]

pub trait Fidelity {
    const FUSE: bool;
}
pub struct Strict;
pub struct Relaxed;
impl Fidelity for Strict {
    const FUSE: bool = false;
}
impl Fidelity for Relaxed {
    const FUSE: bool = true;
}

#[inline(never)]
pub fn dot<P: Fidelity>(a: &[f64], b: &[f64]) -> f64 {
    let mut acc = 0.0f64;
    let mut i = 0;
    while i < a.len() && i < b.len() {
        #[cfg(not(inert))]
        {
            if P::FUSE {
                acc = a[i].mul_add(b[i], acc);
            } else {
                acc += a[i] * b[i];
            }
        }
        #[cfg(inert)]
        {
            acc += a[i] * b[i];
        }
        i += 1;
    }
    acc
}

fn text(f: usize, n: usize) -> Vec<u8> {
    // Reading the process's own text segment. Legitimate in a test binary and
    // nowhere else, which is itself part of the answer.
    unsafe { core::slice::from_raw_parts(f as *const u8, n) }.to_vec()
}

fn main() {
    let hot = dot::<Strict> as usize;
    let cold = dot::<Relaxed> as usize;

    println!("A. address identity");
    println!("   Strict  @ {hot:#x}");
    println!("   Relaxed @ {cold:#x}");
    println!(
        "   same address: {}   (an inert axis SHOULD fold to one, if ICF ran)",
        hot == cold
    );

    println!("B. first 128 bytes of each body");
    let (x, y) = (text(hot, 128), text(cold, 128));
    let differs = x.iter().zip(&y).position(|(p, q)| p != q);
    match differs {
        Some(i) => println!("   differ at byte {i}: the axis generated something"),
        None => println!("   identical for 128 bytes: the axis generated NOTHING"),
    }

    // Keep the calls, so nothing is dead.
    let v = [1.0f64, 2.0, 3.0, 4.0];
    println!(
        "   (values {} {})",
        dot::<Strict>(&v, &v),
        dot::<Relaxed>(&v, &v)
    );
}
