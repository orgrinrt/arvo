//! Probe 3: the product-numeral map is associative and commutative, and its
//! n-ary closed form is the gcd of the mixed monomials.
//!
//! File 31 settles the binary case: for `v1 = A1*k1 + B1` and `v2 = A2*k2 + B2`,
//! the product numeral is `adjustment = gcd(A1*A2, A1*B2, A2*B1)`,
//! `bias = B1*B2` (31_arntzen_settling_the_identity_contract.md:399-400).
//!
//! That settles multiplication of two numerals. It does not, on its own, say
//! anything about three, and `mul_full` is not a binary operation on one set: it
//! is a family of maps `N1 x N2 -> mulnum(N1, N2)`. So "`mul_full` is
//! associative" is not an equation in one algebra at all until the numeral-level
//! map is itself known to be associative. Nobody has stated that, and without it
//! `(x*y)*z` and `x*(y*z)` are values in two different numerals and the equation
//! does not typecheck, let alone hold.
//!
//! CLAIM A. `mulnum` is commutative.
//!
//! CLAIM B. `mulnum` is associative, in both components. Checked exhaustively
//! over every ordered triple drawn from a 6x5 grid of (adjustment, bias) pairs,
//! 27000 triples.
//!
//! CLAIM C. Both bracketings equal the direct ternary closed form: the bias is
//! the all-B monomial `B1*B2*B3`, and the adjustment is the gcd of the seven
//! monomials that contain at least one A. This generalises to n factors as
//! stated, and it is why associativity holds: the seven-term set is symmetric
//! under permutation, so no bracketing can favour a factor.
//!
//! CLAIM D. Containment: every product of three window values lies in the
//! claimed lattice. This is the safe direction (the lattice contains the product
//! set, and is not claimed to be the finest such lattice, per 31:226-228).
//!
//! CLAIM E. Negative control: the naive ternary adjustment `A1*A2*A3`, dropping
//! the cross terms, fails containment on a witnessed triple. The cross terms are
//! load-bearing at arity three exactly as file 31 showed they are at arity two.
//!
//! Build: rustc --edition 2021 --crate-type lib probe_3_product_numeral_is_associative.rs
//! Outcome: WORKS. Clean exit against rustc 1.98.0-nightly (57d06900f 2026-05-27).

#![allow(dead_code)]

const fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (if a < 0 { -a } else { a }, if b < 0 { -b } else { b });
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

const fn gcd3(a: i64, b: i64, c: i64) -> i64 {
    gcd(gcd(a, b), c)
}

/// A numeral, reduced to the two members multiplication actually reads:
/// the adjustment (the quantum) and the bias. Radix, precision and the exponent
/// form do not enter, which is itself the point: this is a statement in
/// mathematical coordinates, and it is radix-free.
#[derive(Copy, Clone)]
struct Num {
    a: i64,
    b: i64,
}

/// The settled binary product numeral (31:399-400).
const fn mulnum(x: Num, y: Num) -> Num {
    Num {
        a: gcd3(x.a * y.a, x.a * y.b, y.a * x.b),
        b: x.b * y.b,
    }
}

/// The direct ternary closed form: bias is the all-B monomial, adjustment is
/// the gcd of every monomial carrying at least one A. Seven such monomials at
/// arity three: one AAA, three AAB, three ABB.
const fn mulnum3_direct(x: Num, y: Num, z: Num) -> Num {
    let aaa = x.a * y.a * z.a;
    let aab = x.a * y.a * z.b;
    let aba = x.a * y.b * z.a;
    let baa = x.b * y.a * z.a;
    let abb = x.a * y.b * z.b;
    let bab = x.b * y.a * z.b;
    let bba = x.b * y.b * z.a;
    let g = gcd(gcd(gcd(aaa, aab), gcd(aba, baa)), gcd(gcd(abb, bab), bba));
    Num {
        a: g,
        b: x.b * y.b * z.b,
    }
}

const fn same(x: Num, y: Num) -> bool {
    x.a == y.a && x.b == y.b
}

// The grid: adjustments 1 through 6, biases 0 through 4. Adjustment 0 is not a
// numeral (a quantum of zero represents one value and is not what multiplication
// is being asked about), so the sweep starts at 1.
const AMIN: i64 = 1;
const AMAX: i64 = 6;
const BMIN: i64 = 0;
const BMAX: i64 = 4;

const fn sweep_commutative() -> bool {
    let mut a1 = AMIN;
    while a1 <= AMAX {
        let mut b1 = BMIN;
        while b1 <= BMAX {
            let mut a2 = AMIN;
            while a2 <= AMAX {
                let mut b2 = BMIN;
                while b2 <= BMAX {
                    let x = Num { a: a1, b: b1 };
                    let y = Num { a: a2, b: b2 };
                    if !same(mulnum(x, y), mulnum(y, x)) {
                        return false;
                    }
                    b2 += 1;
                }
                a2 += 1;
            }
            b1 += 1;
        }
        a1 += 1;
    }
    true
}

/// Returns true when, for every triple in the grid, both bracketings agree with
/// each other AND with the direct ternary closed form.
const fn sweep_associative_and_closed_form() -> bool {
    let mut a1 = AMIN;
    while a1 <= AMAX {
        let mut b1 = BMIN;
        while b1 <= BMAX {
            let mut a2 = AMIN;
            while a2 <= AMAX {
                let mut b2 = BMIN;
                while b2 <= BMAX {
                    let mut a3 = AMIN;
                    while a3 <= AMAX {
                        let mut b3 = BMIN;
                        while b3 <= BMAX {
                            let x = Num { a: a1, b: b1 };
                            let y = Num { a: a2, b: b2 };
                            let z = Num { a: a3, b: b3 };
                            let left = mulnum(mulnum(x, y), z);
                            let right = mulnum(x, mulnum(y, z));
                            let direct = mulnum3_direct(x, y, z);
                            if !same(left, right) {
                                return false;
                            }
                            if !same(left, direct) {
                                return false;
                            }
                            b3 += 1;
                        }
                        a3 += 1;
                    }
                    b2 += 1;
                }
                a2 += 1;
            }
            b1 += 1;
        }
        a1 += 1;
    }
    true
}

// CLAIM A and CLAIM B and CLAIM C.
const _: () = assert!(sweep_commutative());
const _: () = assert!(sweep_associative_and_closed_form());

// ---------------------------------------------------------------------------
// CLAIM D: containment, over a window of the free integers.
// ---------------------------------------------------------------------------

const KLO: i64 = -3;
const KHI: i64 = 3;

const fn contained(x: Num, y: Num, z: Num, adj: i64, bias: i64) -> bool {
    let mut k1 = KLO;
    while k1 <= KHI {
        let mut k2 = KLO;
        while k2 <= KHI {
            let mut k3 = KLO;
            while k3 <= KHI {
                let v = (x.a * k1 + x.b) * (y.a * k2 + y.b) * (z.a * k3 + z.b);
                if (v - bias) % adj != 0 {
                    return false;
                }
                k3 += 1;
            }
            k2 += 1;
        }
        k1 += 1;
    }
    true
}

const T1: Num = Num { a: 4, b: 2 };
const T2: Num = Num { a: 6, b: 4 };
const T3: Num = Num { a: 3, b: 5 };

const P123: Num = mulnum3_direct(T1, T2, T3);

const _: () = assert!(contained(T1, T2, T3, P123.a, P123.b));

// A second triple with one zero bias, and a third with all biases zero, so the
// containment claim is not carried by one lucky choice of numbers.
const U1: Num = Num { a: 5, b: 0 };
const U2: Num = Num { a: 3, b: 7 };
const U3: Num = Num { a: 2, b: 1 };
const PU: Num = mulnum3_direct(U1, U2, U3);
const _: () = assert!(contained(U1, U2, U3, PU.a, PU.b));

const Z1: Num = Num { a: 4, b: 0 };
const Z2: Num = Num { a: 6, b: 0 };
const Z3: Num = Num { a: 3, b: 0 };
const PZ: Num = mulnum3_direct(Z1, Z2, Z3);
const _: () = assert!(contained(Z1, Z2, Z3, PZ.a, PZ.b));

// With every bias zero the formula collapses to the plain product of
// adjustments, which is the shipped exact-product width rule. The n-ary case
// generalises the binary collapse file 31 records (31:212-215) rather than
// being a second rule to keep in agreement with it.
const _: () = assert!(PZ.a == 4 * 6 * 3);
const _: () = assert!(PZ.b == 0);

// ---------------------------------------------------------------------------
// CLAIM E: the negative control. Dropping the cross terms breaks containment.
// ---------------------------------------------------------------------------

const NAIVE_ADJ: i64 = T1.a * T2.a * T3.a; // 4*6*3 = 72
const NAIVE_BIAS: i64 = T1.b * T2.b * T3.b; // 2*4*5 = 40

const _: () = assert!(NAIVE_ADJ == 72);
const _: () = assert!(NAIVE_BIAS == 40);
const _: () = assert!(!contained(T1, T2, T3, NAIVE_ADJ, NAIVE_BIAS));

// The true adjustment for that triple is strictly finer than the naive one, so
// the failure above is the cross terms doing work and not an arithmetic slip.
const _: () = assert!(P123.a < NAIVE_ADJ);
const _: () = assert!(NAIVE_ADJ % P123.a == 0);

/// Present so the same file can be compiled as a binary and print the numbers
/// the assertions only bound.
fn main() {
    println!("mulnum3(T1,T2,T3) = adjustment {} bias {}", P123.a, P123.b);
    println!(
        "naive               adjustment {} bias {}",
        NAIVE_ADJ, NAIVE_BIAS
    );
    println!("all-bias-zero       adjustment {} bias {}", PZ.a, PZ.b);
}
