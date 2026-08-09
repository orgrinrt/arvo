//! Probe 1: MATLAB slope-and-bias scaling against the post-collapse identity contract.
//!
//! Hypothesis, three parts, all const-evaluated (compiling this file is the evidence):
//!
//! (a) MATLAB's general scaling is value = F * 2^E * k + B with F in [1, 2) and B
//!     "any value" (MathWorks: "the slope and bias can take on any value"; both are
//!     stored as doubles, hence dyadic rationals). The spec's own D68 sentence
//!     (202607301200_topic.the-formalization-spec.md:111-114) claims this "in full".
//!
//! (b) With Bias restricted to a type-level integer (36:222 "So `Bias` is a signed
//!     integer", carried into 38:337-338 "Bias is an Int"), the value SET of a legal
//!     MATLAB numerictype is unrepresentable. Witness: slope 1, bias 1/2. The set
//!     {k + 1/2} contains no integer and {k + B} for integer B contains only
//!     integers; the sets are disjoint, so no integer bias reproduces it. More
//!     generally {A*k + B} = {A*k + B'} iff B - B' is an integer multiple of A, so
//!     integer bias covers exactly the biases in Z + A*Z = (1/q)*Z for A = p/q in
//!     lowest terms, and MATLAB permits biases outside that subgroup for every slope.
//!
//! (c) The ratified biased-multiplication closure formula (31:397-400: adjustment =
//!     gcd(A1*A2, A1*B2, A2*B1), bias = B1*B2) already operates in rational-bias
//!     algebra: instantiated with non-integer rational biases it closes exactly.
//!     So the formula the contract ships contradicts the integer reading of Bias,
//!     and the carrier Bias needs is the same Ratio shape Adjustment already has
//!     (file 36's gcd machinery covers it with nothing new).
//!
//! Everything below is `const`; a wrong claim is an E0080 at compile time.
//! Negative control: flipping the first witness assertion fails E0080 (verified
//! during authoring; the diagnostic is quoted in OUTCOMES.md).

// A rational as (num, den), den > 0, normalised by gcd. Const, no_std-shaped.
#[derive(Clone, Copy)]
struct Q {
    n: i128,
    d: i128,
}

const fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (if a < 0 { -a } else { a }, if b < 0 { -b } else { b });
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

const fn q(n: i128, d: i128) -> Q {
    // d != 0; normalise sign into n, reduce by gcd.
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    let g = gcd(n, d);
    if g == 0 {
        Q { n: 0, d: 1 }
    } else {
        Q { n: n / g, d: d / g }
    }
}

const fn qmul(a: Q, b: Q) -> Q {
    q(a.n * b.n, a.d * b.d)
}

const fn qadd(a: Q, b: Q) -> Q {
    q(a.n * b.d + b.n * a.d, a.d * b.d)
}

const fn qeq(a: Q, b: Q) -> bool {
    // both normalised
    a.n == b.n && a.d == b.d
}

const fn qsub(a: Q, b: Q) -> Q {
    qadd(a, q(-b.n, b.d))
}

// Is a/b an integer multiple of c/d, i.e. (a/b)/(c/d) in Z?
const fn is_int_multiple(x: Q, of: Q) -> bool {
    if of.n == 0 {
        return x.n == 0;
    }
    let num = x.n * of.d;
    let den = x.d * of.n;
    num % den == 0
}

// gcd of rationals: gcd(a/b, c/d) = gcd(a*d, c*b) / (b*d). Standard, and the
// unique nonnegative generator of the subgroup a/b Z + c/d Z of Q.
const fn qgcd(a: Q, b: Q) -> Q {
    q(gcd(a.n * b.d, b.n * a.d), a.d * b.d)
}

// value of stored integer k under (adjustment*2^E collapsed into slope A, bias B)
const fn val(a: Q, b: Q, k: i128) -> Q {
    qadd(qmul(a, q(k, 1)), b)
}

// ---------------------------------------------------------------------------
// (b) The witness: slope 1, bias 1/2. Legal MATLAB numerictype
//     ("the slope and bias can take on any value"). No integer bias reproduces
//     its value set: {A*k + B} = {A*k + B'} iff B - B' in A*Z, and
//     1/2 - B' is a half-integer for every integer B', never a multiple of 1.
//     Checked exhaustively for B' in -1000..=1000, and the parity argument
//     covers the rest (a half-integer is never an integer).
// ---------------------------------------------------------------------------
const _: () = {
    let a = q(1, 1);
    let bias = q(1, 2);
    let mut bp: i128 = -1000;
    while bp <= 1000 {
        // sets equal iff (bias - bp) is an integer multiple of a
        assert!(!is_int_multiple(qsub(bias, q(bp, 1)), a));
        bp += 1;
    }
};

// The general subgroup statement: integer bias reaches exactly (1/q)Z for slope
// p/q in lowest terms. Checked for slope 3/8: bias 1/8 IS reachable from an
// integer bias (1/8 - 1 = -7/8 = -7 * 3/8? no; but some integer works: the
// subgroup Z + (3/8)Z = (1/8)Z), while bias 1/16 is NOT (outside (1/8)Z).
const _: () = {
    let a = q(3, 8);
    // generator of Z + aZ is gcd(1, a) = 1/8
    let g = qgcd(q(1, 1), a);
    assert!(qeq(g, q(1, 8)));
    // 1/16 not in (1/8)Z
    assert!(!is_int_multiple(q(1, 16), g));
    // and exhaustively: no integer bias B' in -1000..=1000 with 1/16 - B' in aZ
    let mut bp: i128 = -1000;
    while bp <= 1000 {
        assert!(!is_int_multiple(qsub(q(1, 16), q(bp, 1)), a));
        bp += 1;
    }
    // while 1/8 is reachable: 1/8 - (-1) = 9/8 = 3 * (3/8). B' = -1 works.
    assert!(is_int_multiple(qsub(q(1, 8), q(-1, 1)), a));
};

// ---------------------------------------------------------------------------
// (c) The closure formula with rational biases, exhaustive over a k-range.
//     v1 = A1*k1 + B1, v2 = A2*k2 + B2 with non-integer B1, B2.
//     adjustment = gcd(A1*A2, A1*B2, A2*B1), bias = B1*B2 (31:397-400).
//     For every (k1, k2) the product must be adjustment * k + bias with k in Z.
// ---------------------------------------------------------------------------
const _: () = {
    // MATLAB-shaped: slope1 = 3/8 (F=3/2, E=-2), bias1 = 1/2
    //                slope2 = 1/4 (F=1,  E=-2), bias2 = 5/2
    let (a1, b1) = (q(3, 8), q(1, 2));
    let (a2, b2) = (q(1, 4), q(5, 2));
    let adj = qgcd(qgcd(qmul(a1, a2), qmul(a1, b2)), qmul(a2, b1));
    let bias = qmul(b1, b2);
    let mut k1: i128 = -8;
    while k1 <= 8 {
        let mut k2: i128 = -8;
        while k2 <= 8 {
            let p = qmul(val(a1, b1, k1), val(a2, b2, k2));
            // p - bias must be an integer multiple of adj
            assert!(is_int_multiple(qsub(p, bias), adj));
            k2 += 1;
        }
        k1 += 1;
    }
};

// ---------------------------------------------------------------------------
// (a) The MATLAB decomposition itself: slope 3/8 normalises to F = 3/2 in
//     [1, 2) times 2^-2, and F is a reduced Ratio (coprime), i.e. exactly the
//     carrier shape Adjustment already has (Ratio<N, D> with gcd = 1, file 36).
// ---------------------------------------------------------------------------
const _: () = {
    // 3/8 = (3/2) * 2^(-2)
    let f = q(3, 2);
    let two_pow_neg2 = q(1, 4);
    assert!(qeq(qmul(f, two_pow_neg2), q(3, 8)));
    // F in [1, 2): 1 <= 3/2 < 2
    assert!(f.n * 1 >= f.d * 1 && f.n * 1 < f.d * 2);
    // reduced: gcd(3, 2) = 1
    assert!(gcd(f.n, f.d) == 1);
};

fn main() {
    // All content is const; reaching main means every claim above held.
    println!("probe 1: all const assertions held");
}
