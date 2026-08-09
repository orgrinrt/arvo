//! p2b: p1b's mechanism, unchanged in shape, at a width arvo ships, and the
//! asymmetry that shows up when you try it.
//!
//! p1b turns a law from an author-written declaration into a compile-time computation
//! over the policy's own map. It does so over a 4-bit model window. This file is the
//! identical construction with the width changed to 8, run twice: once at a policy
//! where the law is FALSE, once at a policy where the law is TRUE.
//!
//!   rustc --edition 2021 -O --cfg sat  p2b_swept_verdict_at_shipped_width.rs -o p2b_sat
//!   rustc --edition 2021 -O --cfg wrap p2b_swept_verdict_at_shipped_width.rs -o p2b_wrap
//!
//! Expected, and this is the point of the file:
//!   sat  : refuses in well under a second. The check hits a counterexample early and
//!          returns, so the domain is never enumerated. A NEGATIVE verdict is cheap.
//!   wrap : the check must visit all 2^24 triples before it can say yes, and rustc
//!          refuses under `long_running_const_eval`. A POSITIVE verdict is not
//!          reachable at this width.
//!
//! The asymmetry runs the wrong way for the design. The verdict that licenses an arm
//! is the positive one, and it is the one the compiler cannot produce.
//!
//! Frontier context: p2_frontier.py measures the widest width at which an exhaustive
//! arity-3 law check is evaluated at all, and it is 5.
//!
//! Toolchain: nightly-2026-05-28. No feature gates.

const W: i32 = 8; // the only change from p1b's model window
const LO: i32 = -(1 << (W - 1));
const HI: i32 = (1 << (W - 1)) - 1;
const N: i32 = HI - LO + 1;

const fn sat_add(a: i32, b: i32) -> i32 {
    let s = a + b;
    if s > HI {
        HI
    } else if s < LO {
        LO
    } else {
        s
    }
}

const fn wrap_add(a: i32, b: i32) -> i32 {
    let mut r = (a + b - LO) % N;
    if r < 0 {
        r += N;
    }
    r + LO
}

const fn op(a: i32, b: i32) -> i32 {
    #[cfg(sat)]
    {
        sat_add(a, b)
    }
    #[cfg(wrap)]
    {
        wrap_add(a, b)
    }
}

/// Identical in shape to p1b's `assoc_holds`. Arity 3, exhaustive over the window.
const fn assoc_holds() -> bool {
    let mut a = LO;
    while a <= HI {
        let mut b = LO;
        while b <= HI {
            let mut c = LO;
            while c <= HI {
                if op(op(a, b), c) != op(a, op(b, c)) {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

trait AssocProven {
    const PROOF: () = assert!(assoc_holds(), "not associative at this width");
}
struct Policy;
impl AssocProven for Policy {}

fn licensed<P: AssocProven>() -> i32 {
    let () = <P as AssocProven>::PROOF;
    0
}

fn main() {
    println!("{}", licensed::<Policy>());
}
