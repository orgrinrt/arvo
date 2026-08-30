// PROBE p2 (file 89). The const-gate form of the saturating verdict, which
// 86 section 10 item 6 names as not built: "the piecewise form does not [have
// its const demonstration] and its const-evaluability is an expectation from
// F3's budget arithmetic, not a demonstration".
//
// It is built here, on p1's degree criterion rather than on 86's piecewise
// procedure, because p1's Theorem B removes the binary search, the piece
// bookkeeping and the clamp-indicator induction. What remains is:
//
//   verdict(A, B, W) = agreement of the two saturating evaluations at
//                      x = 0, 1, ..., min(D, MAX_W), D the syntactic degree.
//
// DESIGN-SHAPED, per I14: no alloc, no Vec, no Box, no dyn, no TypeId, no
// std, sizes const, no feature gates. A term is a const array in postfix
// form; evaluation is a const fn stack machine over a const-sized array.
//
// What the file demonstrates, all at compile time on the pinned nightly:
//   rung 0  the criterion's IMPLEMENTATION is validated against a const
//           exhaustive sweep at widths 1..=8, in a crate-level const. This is
//           84 section 5's demotion of the model band to an implementation
//           check, and it is the only job 84 section 4 licenses a band for.
//   rung 0  fragment membership is CHECKED rather than claimed: a const fn
//           rejects any term carrying a non-monotone node.
//   rung 3  the per-law licence is an associated const in a generic impl,
//           evaluated per reached instantiation, which is where 84 section 7
//           argues an arm's claim belongs.
//
// cfg variants, each of which must refuse:
//   --cfg use_e63     gate the arm on E_63, false at width 64. Must refuse.
//   --cfg perturb     corrupt the criterion (sample one point short). The
//                     rung-0 implementation check must refuse.
//   --cfg nonfragment gate on a term carrying sat_sub. The fragment check
//                     must refuse.
//   --cfg unchecked_nonfragment  the same term with the fragment check
//                     removed, showing the criterion returns a WRONG verdict
//                     when its trusted input is violated (negative control:
//                     the licence is granted and the law is false at 64).

#![no_std]
#![no_main]

// ---------------------------------------------------------------- term encoding

// postfix opcodes
const OP_X: u8 = 0;
const OP_C: u8 = 1;
const OP_ADD: u8 = 2;
const OP_MUL: u8 = 3;
const OP_SUB: u8 = 4; // saturating subtract: OUTSIDE the monotone fragment

#[derive(Clone, Copy)]
struct Node {
    op: u8,
    imm: u64,
}

const fn n(op: u8) -> Node {
    Node { op, imm: 0 }
}
const fn k(v: u64) -> Node {
    Node { op: OP_C, imm: v }
}

const MAXN: usize = 160;
const STACK: usize = 16;

#[derive(Clone, Copy)]
struct Term {
    nodes: [Node; MAXN],
    len: usize,
}

const fn empty_term() -> Term {
    Term {
        nodes: [Node { op: OP_X, imm: 0 }; MAXN],
        len: 0,
    }
}

/// x^d as a left-fold saturating multiply chain, in postfix.
const fn pow_term(d: u32) -> Term {
    let mut t = empty_term();
    t.nodes[0] = n(OP_X);
    t.len = 1;
    let mut i = 1;
    while i < d {
        t.nodes[t.len] = n(OP_X);
        t.nodes[t.len + 1] = n(OP_MUL);
        t.len += 2;
        i += 1;
    }
    t
}

/// x * c
const fn scale_term(c: u64) -> Term {
    let mut t = empty_term();
    t.nodes[0] = n(OP_X);
    t.nodes[1] = k(c);
    t.nodes[2] = n(OP_MUL);
    t.len = 3;
    t
}

/// sat_add(sat_sub(c, x), x), deliberately outside the fragment: the
/// subtraction makes the subterm DECREASING in x, so p1's min-form lemma
/// fails and the degree criterion loses its warrant. This term equals c for
/// x <= c and equals min(x, MAX) above, so it agrees with the constant c at
/// x = 0..=D (D = 1) and diverges at x = c+1: exactly the shape the criterion
/// cannot see.
const fn nonmono_term(c: u64) -> Term {
    let mut t = empty_term();
    t.nodes[0] = k(c);
    t.nodes[1] = n(OP_X);
    t.nodes[2] = n(OP_SUB);
    t.nodes[3] = n(OP_X);
    t.nodes[4] = n(OP_ADD);
    t.len = 5;
    t
}

/// the constant c as a term
const fn const_term(c: u64) -> Term {
    let mut t = empty_term();
    t.nodes[0] = k(c);
    t.len = 1;
    t
}

// ---------------------------------------------------------------- const machinery

const fn umax(w: u32) -> u128 {
    if w >= 64 {
        u64::MAX as u128
    } else {
        (1u128 << w) - 1
    }
}

/// saturating evaluation of a postfix term at x, width w
const fn eval(t: &Term, x: u128, w: u32) -> u128 {
    let m = umax(w);
    let mut st = [0u128; STACK];
    let mut sp: usize = 0;
    let mut i: usize = 0;
    while i < t.len {
        let nd = t.nodes[i];
        if nd.op == OP_X {
            st[sp] = x;
            sp += 1;
        } else if nd.op == OP_C {
            let c = nd.imm as u128;
            st[sp] = if c > m { m } else { c };
            sp += 1;
        } else {
            let b = st[sp - 1];
            let a = st[sp - 2];
            sp -= 2;
            let v = if nd.op == OP_ADD {
                let s = a + b;
                if s > m {
                    m
                } else {
                    s
                }
            } else if nd.op == OP_MUL {
                let s = a * b;
                if s > m {
                    m
                } else {
                    s
                }
            } else {
                // OP_SUB, saturating
                if a < b {
                    0
                } else {
                    a - b
                }
            };
            st[sp] = v;
            sp += 1;
        }
        i += 1;
    }
    st[0]
}

/// syntactic degree; exact for the monotone fragment (no cancellation)
const fn degree(t: &Term) -> u32 {
    let mut st = [0u32; STACK];
    let mut sp: usize = 0;
    let mut i: usize = 0;
    while i < t.len {
        let nd = t.nodes[i];
        if nd.op == OP_X {
            st[sp] = 1;
            sp += 1;
        } else if nd.op == OP_C {
            st[sp] = 0;
            sp += 1;
        } else {
            let b = st[sp - 1];
            let a = st[sp - 2];
            sp -= 2;
            st[sp] = if nd.op == OP_MUL {
                a + b
            } else if a > b {
                a
            } else {
                b
            };
            sp += 1;
        }
        i += 1;
    }
    st[0]
}

/// FRAGMENT MEMBERSHIP, checked rather than claimed. Every node must be one
/// of x, a nonnegative constant, saturating add, saturating multiply. This is
/// the whole of the monotonicity induction 86's least-certain item 2 left
/// unmechanised: each admitted operation is nondecreasing in both arguments
/// on the nonnegative domain, so every subterm is nondecreasing, so the exact
/// polynomial has nonnegative coefficients.
const fn in_monotone_fragment(t: &Term) -> bool {
    let mut i: usize = 0;
    while i < t.len {
        let op = t.nodes[i].op;
        if op != OP_X && op != OP_C && op != OP_ADD && op != OP_MUL {
            return false;
        }
        i += 1;
    }
    true
}

/// THE VERDICT. p1 Theorem B: agreement on x = 0..=min(D, MAX) decides the
/// law at width w, for terms in the monotone fragment.
const fn verdict(a: &Term, b: &Term, w: u32) -> bool {
    let m = umax(w);
    let da = degree(a);
    let db = degree(b);
    let d = if da > db { da } else { db } as u128;
    #[cfg(perturb)]
    let hi = if d == 0 {
        0
    } else {
        (if d < m { d } else { m }) - 1
    };
    #[cfg(not(perturb))]
    let hi = if d < m { d } else { m };
    let mut x: u128 = 0;
    while x <= hi {
        if eval(a, x, w) != eval(b, x, w) {
            return false;
        }
        x += 1;
    }
    true
}

/// exhaustive sweep, for the rung-0 implementation check only
const fn sweep(a: &Term, b: &Term, w: u32) -> bool {
    let m = umax(w);
    let mut x: u128 = 0;
    while x <= m {
        if eval(a, x, w) != eval(b, x, w) {
            return false;
        }
        x += 1;
    }
    true
}

// ------------------------------------------------- rung 0: implementation check

/// The criterion agrees with the exhaustive sweep at every width the sweep can
/// reach, over the E_d family and two constant-carrying pairs. Attached to no
/// function: unskippable, per 82's F8 rung 0.
const IMPL_CHECK: bool = {
    let mut ok = true;
    let mut d: u32 = 2;
    while d <= 9 {
        let a = pow_term(d);
        let b = pow_term(d + 1);
        let mut w: u32 = 1;
        while w <= 8 {
            if verdict(&a, &b, w) != sweep(&a, &b, w) {
                ok = false;
            }
            w += 1;
        }
        d += 1;
    }
    let mut c: u64 = 1;
    while c <= 64 {
        let a = scale_term(c);
        let b = scale_term(c + 1);
        let mut w: u32 = 1;
        while w <= 8 {
            if verdict(&a, &b, w) != sweep(&a, &b, w) {
                ok = false;
            }
            w += 1;
        }
        c *= 2;
    }
    ok
};
const _: () = assert!(
    IMPL_CHECK,
    "the verdict criterion disagrees with the sweep at a swept width"
);

/// The E_d family's truth set really is 1..=d at every swept width: the band's
/// second licensed job, checking a claimed shape rather than transferring it.
const TRUTH_SET_SHAPE: bool = {
    let mut ok = true;
    let mut d: u32 = 2;
    while d <= 9 {
        let a = pow_term(d);
        let b = pow_term(d + 1);
        let mut w: u32 = 1;
        while w <= 12 {
            let want = w <= d;
            if verdict(&a, &b, w) != want {
                ok = false;
            }
            w += 1;
        }
        d += 1;
    }
    ok
};
const _: () = assert!(TRUTH_SET_SHAPE, "E_d truth set is not 1..=d");

// ------------------------------------------------- the gated arm, rung 3

const W_SHIPPED: u32 = 64;

#[cfg(all(not(use_e63), not(nonfragment), not(unchecked_nonfragment)))]
const LAW_A: Term = pow_term(64);
#[cfg(all(not(use_e63), not(nonfragment), not(unchecked_nonfragment)))]
const LAW_B: Term = pow_term(65);

#[cfg(use_e63)]
const LAW_A: Term = pow_term(63);
#[cfg(use_e63)]
const LAW_B: Term = pow_term(64);

#[cfg(any(nonfragment, unchecked_nonfragment))]
const LAW_A: Term = nonmono_term(100);
#[cfg(any(nonfragment, unchecked_nonfragment))]
const LAW_B: Term = const_term(100);

trait Numeral {
    const WIDTH: u32;
}
struct N64;
impl Numeral for N64 {
    const WIDTH: u32 = W_SHIPPED;
}

trait RewriteLicence {
    const LICENSED: ();
}

impl<T: Numeral> RewriteLicence for T {
    const LICENSED: () = {
        #[cfg(not(unchecked_nonfragment))]
        assert!(
            in_monotone_fragment(&LAW_A) && in_monotone_fragment(&LAW_B),
            "the law is outside the monotone saturating fragment; the criterion does not apply to it"
        );
        assert!(
            verdict(&LAW_A, &LAW_B, T::WIDTH),
            "the law is FALSE at the gated width, decided there by the degree criterion"
        );
    };
}

/// force the instantiation, so the transcript cannot be read as covering an
/// arm that was never evaluated (84 section 7)
const FORCED: () = <N64 as RewriteLicence>::LICENSED;

/// the negative control's evidence: the non-fragment law is FALSE at width
/// 64, witnessed at x = 101, established independently of the criterion.
const NONFRAGMENT_WITNESS: u128 = 101;
const NONFRAGMENT_TRUTH_AT_64: bool = {
    let a = nonmono_term(100);
    let b = const_term(100);
    eval(&a, NONFRAGMENT_WITNESS, 64) == eval(&b, NONFRAGMENT_WITNESS, 64)
};
const _: () = assert!(!NONFRAGMENT_TRUTH_AT_64, "control assumption broken");

/// THE NEGATIVE CONTROL. With the fragment check removed, the criterion
/// LICENSES a law that is false at the gated width. This is what the trusted
/// input is worth: violating fragment membership does not degrade the
/// verdict, it inverts it.
#[cfg(unchecked_nonfragment)]
const _: () = assert!(
    verdict(&LAW_A, &LAW_B, 64) && !NONFRAGMENT_TRUTH_AT_64,
    "the negative control did not reproduce: the criterion did not lie about the non-fragment law"
);

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn probe_entry() -> u32 {
    let _ = FORCED;
    if IMPL_CHECK && TRUTH_SET_SHAPE {
        1
    } else {
        0
    }
}
