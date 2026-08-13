// PROBE p6 (file 89). The const gate for a SIGNED saturating law over a
// declared operand window, deciding 82's law at the shipped width by the box
// criterion instead of by a structural argument.
//
// 84 section 6 on 82's three constructions: "the band validates the closed
// form's implementation; the three-line integer argument carries the verdict;
// that argument is a trusted-base item until mechanised, and it is the only
// load-bearing unmechanised thing in the construction." This file removes it.
// The width-64 verdict is computed at width 64, in the compiler, by walking
// the degree box of the declared window. Nothing is declared and nothing is
// transferred.
//
// The law: over a declared operand window [LO, HI], every parenthesisation of
// a fold of signed saturating add agrees. Checked here as left fold == right
// fold and left fold == balanced tree, at n = 8.
//
// DESIGN-SHAPED per I14: no alloc, no Vec, no Box, no dyn, no TypeId, no std,
// sizes const, no feature gates.
//
// cfg variants:
//   default              window [0, MAX], admissible, law true. ACCEPT.
//   --cfg straddle       window [MIN, MAX]. The admissibility check must
//                        REFUSE: with both clamps reachable the clamped set is
//                        not an up-set and Theorem C has no hypothesis.
//   --cfg unchecked_straddle
//                        the same window with the admissibility check removed.
//                        The criterion then LICENSES the law, and the law is
//                        false at width 64 with the witness (MAX, MAX, MIN)
//                        asserted in the same file. Accepting is the finding.

#![no_std]
#![no_main]

const NV: usize = 8;
const MAXN: usize = 64;
const STACK: usize = 16;

const OP_V: u8 = 0;
const OP_ADD: u8 = 1;

#[derive(Clone, Copy)]
struct Node {
    op: u8,
    var: usize,
}

#[derive(Clone, Copy)]
struct Term {
    nodes: [Node; MAXN],
    len: usize,
}

const fn empty() -> Term {
    Term {
        nodes: [Node { op: OP_V, var: 0 }; MAXN],
        len: 0,
    }
}

/// left fold: v0 v1 + v2 + v3 + ...
const fn left_fold(n: usize) -> Term {
    let mut t = empty();
    t.nodes[0] = Node { op: OP_V, var: 0 };
    t.len = 1;
    let mut i = 1;
    while i < n {
        t.nodes[t.len] = Node { op: OP_V, var: i };
        t.nodes[t.len + 1] = Node { op: OP_ADD, var: 0 };
        t.len += 2;
        i += 1;
    }
    t
}

/// right fold: v0 (v1 (v2 ... )) in postfix is v_{n-1}, then prepending, which
/// in postfix reads v0 v1 v2 ... v_{n-1} then n-1 adds folded from the right.
/// Postfix for v0 + (v1 + (v2 + v3)) is: v0 v1 v2 v3 + + +
const fn right_fold(n: usize) -> Term {
    let mut t = empty();
    let mut i = 0;
    while i < n {
        t.nodes[i] = Node { op: OP_V, var: i };
        i += 1;
    }
    t.len = n;
    let mut j = 1;
    while j < n {
        t.nodes[t.len] = Node { op: OP_ADD, var: 0 };
        t.len += 1;
        j += 1;
    }
    t
}

/// genuine balanced tree over n = 8:
/// ((v0+v1)+(v2+v3)) + ((v4+v5)+(v6+v7))
/// postfix: v0 v1 + v2 v3 + ADD v4 v5 + v6 v7 + ADD ADD
const fn tree_fold8() -> Term {
    let mut t = empty();
    let mut w = 0;
    let mut half = 0;
    while half < 2 {
        let base = half * 4;
        let mut i = 0;
        while i < 4 {
            t.nodes[w] = Node {
                op: OP_V,
                var: base + i,
            };
            t.nodes[w + 1] = Node {
                op: OP_V,
                var: base + i + 1,
            };
            t.nodes[w + 2] = Node { op: OP_ADD, var: 0 };
            w += 3;
            i += 2;
        }
        t.nodes[w] = Node { op: OP_ADD, var: 0 };
        w += 1;
        half += 1;
    }
    t.nodes[w] = Node { op: OP_ADD, var: 0 };
    t.len = w + 1;
    t
}

const fn three(a: i128, b: i128, c: i128) -> [i128; NV] {
    let mut xs = [0i128; NV];
    xs[0] = a;
    xs[1] = b;
    xs[2] = c;
    xs
}

const fn smin(w: u32) -> i128 {
    -(1i128 << (w - 1))
}
const fn smax(w: u32) -> i128 {
    (1i128 << (w - 1)) - 1
}

const fn eval(t: &Term, xs: &[i128; NV], w: u32) -> i128 {
    let lo = smin(w);
    let hi = smax(w);
    let mut st = [0i128; STACK];
    let mut sp: usize = 0;
    let mut i: usize = 0;
    while i < t.len {
        let nd = t.nodes[i];
        if nd.op == OP_V {
            st[sp] = xs[nd.var];
            sp += 1;
        } else {
            let b = st[sp - 1];
            let a = st[sp - 2];
            sp -= 2;
            let s = a + b;
            st[sp] = if s < lo {
                lo
            } else if s > hi {
                hi
            } else {
                s
            };
            sp += 1;
        }
        i += 1;
    }
    st[0]
}

/// The hypothesis of Theorem C, checked rather than claimed. A declared
/// operand window admits the box criterion when only one clamp is reachable
/// from it, which for additive terms is exactly sign uniformity.
const fn window_admissible(lo: i128, hi: i128) -> bool {
    lo >= 0 || hi <= 0
}

/// THE VERDICT. sat_add has per-variable degree 1, so the box is
/// PROD_i {LO, min(LO+1, HI)}, walked as a bit counter.
const fn box_verdict(a: &Term, b: &Term, n: usize, w: u32, lo: i128, hi: i128) -> bool {
    let step: i128 = if lo + 1 <= hi { 1 } else { 0 };
    let total: u64 = 1u64 << n;
    let mut mask: u64 = 0;
    while mask < total {
        let mut xs = [lo; NV];
        let mut i = 0;
        while i < n {
            if (mask >> i) & 1 == 1 {
                xs[i] = lo + step;
            }
            i += 1;
        }
        if eval(a, &xs, w) != eval(b, &xs, w) {
            return false;
        }
        mask += 1;
    }
    true
}

const W: u32 = 64;
const N: usize = 8;

#[cfg(all(not(straddle), not(unchecked_straddle)))]
const WIN_LO: i128 = 0;
#[cfg(all(not(straddle), not(unchecked_straddle)))]
const WIN_HI: i128 = smax(W);

#[cfg(any(straddle, unchecked_straddle))]
const WIN_LO: i128 = smin(W);
#[cfg(any(straddle, unchecked_straddle))]
const WIN_HI: i128 = smax(W);

const LEFT: Term = left_fold(N);
const RIGHT: Term = right_fold(N);
const TREE: Term = tree_fold8();

/// rung 0: the checker's implementation, validated against exhaustive brute
/// force at a width small enough to sweep. The model band demoted to the one
/// job 84 section 4 licenses it for.
const IMPL_CHECK: bool = {
    let mut ok = true;
    let mut w: u32 = 2;
    while w <= 4 {
        // brute force over [0, smax(w)]^3 against the box criterion, for the
        // arity-3 fold, on the admissible window
        let a = left_fold(3);
        let b = right_fold(3);
        let hi = smax(w);
        let mut brute = true;
        let mut x0: i128 = 0;
        while x0 <= hi {
            let mut x1: i128 = 0;
            while x1 <= hi {
                let mut x2: i128 = 0;
                while x2 <= hi {
                    let xs = three(x0, x1, x2);
                    if eval(&a, &xs, w) != eval(&b, &xs, w) {
                        brute = false;
                    }
                    x2 += 1;
                }
                x1 += 1;
            }
            x0 += 1;
        }
        if box_verdict(&a, &b, 3, w, 0, hi) != brute {
            ok = false;
        }
        // and on the straddling window, where the criterion is expected to
        // DISAGREE with brute force: that disagreement is what the
        // admissibility check exists to prevent, and its absence would mean
        // the control has stopped controlling.
        let lo2 = smin(w);
        let mut brute2 = true;
        let mut y0: i128 = lo2;
        while y0 <= hi {
            let mut y1: i128 = lo2;
            while y1 <= hi {
                let mut y2: i128 = lo2;
                while y2 <= hi {
                    let ys = three(y0, y1, y2);
                    if eval(&a, &ys, w) != eval(&b, &ys, w) {
                        brute2 = false;
                    }
                    y2 += 1;
                }
                y1 += 1;
            }
            y0 += 1;
        }
        if brute2 {
            ok = false;
        } // 82's measured falsity
        if !box_verdict(&a, &b, 3, w, lo2, hi) {
            ok = false;
        } // and the criterion's blindness to it
        w += 1;
    }
    ok
};
const _: () = assert!(IMPL_CHECK, "the box criterion disagreed with brute force on an admissible window, or the straddling control stopped controlling");

/// the law is false at width 64 on the straddling window, witnessed, computed
/// independently of the criterion
const STRADDLE_WITNESS_FALSIFIES: bool = {
    let xs = three(smax(W), smax(W), smin(W));
    eval(&left_fold(3), &xs, W) != eval(&right_fold(3), &xs, W)
};
const _: () = assert!(
    STRADDLE_WITNESS_FALSIFIES,
    "the width-64 witness stopped witnessing"
);

trait Numeral {
    const WIDTH: u32;
}
struct N64;
impl Numeral for N64 {
    const WIDTH: u32 = W;
}

trait FoldReassocLicence {
    const LICENSED: ();
}
impl<T: Numeral> FoldReassocLicence for T {
    const LICENSED: () = {
        #[cfg(not(unchecked_straddle))]
        assert!(
            window_admissible(WIN_LO, WIN_HI),
            "the declared operand window straddles zero, so both clamps are reachable and the box criterion has no hypothesis"
        );
        assert!(
            box_verdict(&LEFT, &RIGHT, N, T::WIDTH, WIN_LO, WIN_HI),
            "left fold and right fold disagree at the gated width, decided there by the box criterion"
        );
        assert!(
            box_verdict(&LEFT, &TREE, N, T::WIDTH, WIN_LO, WIN_HI),
            "left fold and the balanced tree disagree at the gated width, decided there by the box criterion"
        );
    };
}
const FORCED: () = <N64 as FoldReassocLicence>::LICENSED;

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn probe_entry() -> u32 {
    let _ = FORCED;
    if IMPL_CHECK && STRADDLE_WITNESS_FALSIFIES {
        1
    } else {
        0
    }
}
