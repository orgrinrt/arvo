//! 68 probe p5: how far up does "validation is compile-time computation" reach
//! on the pinned toolchain, before rustc itself refuses the evaluation?
//!
//! 65's probe validates law inventories exhaustively at a 4-bit model width in
//! const context and transfers upward "by uniformity of construction". The
//! prior panel recorded (through the workspace's unstable-features rule) that
//! exhaustive const checking quadruples per bit and was refused at nine bits.
//! That record is inherited, not this panel's own. This probe re-establishes
//! the ceiling inside this panel's probe set: the identical exhaustive
//! signed-saturating associativity count, at a width selected by cfg.
//!
//! Build with exactly one of:
//!   rustc --edition 2024 --crate-type lib --cfg 'w6' p5_const_eval_ceiling.rs
//!   rustc --edition 2024 --crate-type lib --cfg 'w9' p5_const_eval_ceiling.rs
//!
//! Expected: w6 (64^3 = 262,144 triples) compiles quickly; w9
//! (512^3 = 134,217,728 triples) is where the recorded ceiling sits.
//! Outcome transcripts: p5_w6.txt, p5_w9.txt.

#![no_std]

#[cfg(w6)]
const W: i64 = 6;
#[cfg(w9)]
const W: i64 = 9;

const LO: i64 = -(1 << (W - 1));
const HI: i64 = (1 << (W - 1)) - 1;

const fn sat_add(a: i64, b: i64) -> i64 {
    let s = a + b;
    if s > HI {
        HI
    } else if s < LO {
        LO
    } else {
        s
    }
}

const fn assoc_failures() -> u64 {
    let mut n: u64 = 0;
    let mut a = LO;
    while a <= HI {
        let mut b = LO;
        while b <= HI {
            let mut c = LO;
            while c <= HI {
                if sat_add(sat_add(a, b), c) != sat_add(a, sat_add(b, c)) {
                    n += 1;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    n
}

/// signed saturating addition is not associative at any width; the count is
/// forced positive, so the assertion is real and the cost is the question.
pub const FAILURES: u64 = assoc_failures();
const _: () = assert!(FAILURES > 0);
