// PROBE p4. The decision procedure that replaces the transfer, for the
// wrapping fragment: finite differences.
//
// THE CLAIM (Kempner 1921 / Singmaster 1974, restated for m = 2^W). Write an
// integer polynomial p in the falling-factorial basis, p = sum a_j (x)_j.
// Then p vanishes identically on Z/2^W exactly when 2^W divides a_j * j! for
// every j. And a_j * j! is the j-th forward difference of p at 0, so:
//
//   "forall x mod 2^W: p(x) == 0"
//     <=>  every forward difference of p at 0, orders 0..=deg, is == 0 mod 2^W.
//
// Consequences this probe verifies mechanically:
//
//   1. The exact truth threshold of an equation law is computable from deg+1
//      integer evaluations: W* = min over nonzero differences of v2(diff).
//   2. The verdict AT ANY WIDTH, including 64, is computable through the
//      width's own wrapping map with deg+1 evaluations and a difference
//      table: constant in W, polynomial in degree. No sweep, no band, no
//      transfer. The multivariate version uses mixed differences over a box
//      of size prod(deg_v + 1).
//   3. The procedure's two trusted inputs are fragment membership (ring ops
//      only) and the degree bound. Two negative controls show a wrong verdict
//      when either is violated.
//
// Sufficiency of the criterion: (x)_j takes the value j!*C(x,j) at every
// nonnegative x, so each term a_j (x)_j is divisible by a_j j!. Necessity:
// evaluate at x = 0, 1, 2, ... in order; unitriangularity gives 2^W | a_j j!
// by induction. Wrapped evaluation points are harmless because integer
// polynomials respect argument congruence mod 2^W. The same argument runs per
// variable for the multivariate case.
//
// Toolchain: pinned nightly-2026-05-28. Runtime probe; no feature gates.

fn mask_of(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

fn v2_i128(x: i128) -> u32 {
    if x == 0 {
        u32::MAX
    } else {
        (x.unsigned_abs()).trailing_zeros()
    }
}

/// Exact integer evaluation of a monomial-basis polynomial.
fn eval_exact(coefs: &[i128], x: i128) -> i128 {
    let mut acc = 0i128;
    for &c in coefs.iter().rev() {
        acc = acc * x + c;
    }
    acc
}

/// Modular evaluation through the width-w wrapping map.
fn eval_mod(coefs: &[i128], x: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let md = 1i128 << w;
    let mut acc = 0u64;
    for &c in coefs.iter().rev() {
        let cr = (((c % md) + md) % md) as u64;
        acc = acc.wrapping_mul(x).wrapping_add(cr) & m;
    }
    acc
}

/// Exact certificate: W* = min v2 over nonzero forward differences at 0.
/// u32::MAX means the zero polynomial function over Z (true at every width).
fn wstar_exact(coefs: &[i128]) -> u32 {
    let d = coefs.len() - 1;
    let mut vals: Vec<i128> = (0..=d as i128).map(|i| eval_exact(coefs, i)).collect();
    // in-place forward-difference triangle: vals[j] becomes Delta^j p(0)
    for j in 1..=d {
        for i in (j..=d).rev() {
            vals[i] -= vals[i - 1];
        }
    }
    vals.iter().map(|&v| v2_i128(v)).min().unwrap_or(u32::MAX)
}

/// The width-w verdict computed through the map itself: deg+1 evaluations,
/// difference table mod 2^w, all entries zero.
fn diff_verdict_mod(coefs: &[i128], w: u32) -> bool {
    let d = coefs.len() - 1;
    let m = mask_of(w);
    let mut vals: Vec<u64> = (0..=d as u64).map(|i| eval_mod(coefs, i, w)).collect();
    for j in 1..=d {
        for i in (j..=d).rev() {
            vals[i] = vals[i].wrapping_sub(vals[i - 1]) & m;
        }
    }
    vals.iter().all(|&v| v == 0)
}

/// Exhaustive sweep verdict at width w.
fn sweep_verdict(coefs: &[i128], w: u32) -> bool {
    let n = 1u64 << w;
    (0..n).all(|x| eval_mod(coefs, x, w) == 0)
}

/// Monomial coefficients of the falling factorial (x)_k.
fn falling_coefs(k: usize) -> Vec<i128> {
    // product of (x - i) for i in 0..k
    let mut c = vec![1i128];
    for i in 0..k as i128 {
        let mut next = vec![0i128; c.len() + 1];
        for (j, &cj) in c.iter().enumerate() {
            next[j + 1] += cj; // * x
            next[j] += cj * (-i); // * (-i)
        }
        c = next;
    }
    c
}

struct Xorshift(u64);
impl Xorshift {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

// ---- multivariate: mixed differences over a box ----

/// verdict at width w for f over vars with degree bounds `degs`:
/// all mixed differences at the origin vanish mod 2^w.
fn mixed_diff_verdict(f: &dyn Fn(&[u64], u32) -> u64, degs: &[usize], w: u32) -> bool {
    let m = mask_of(w);
    let dims: Vec<usize> = degs.iter().map(|&d| d + 1).collect();
    let total: usize = dims.iter().product();
    // value tensor, row-major
    let mut t = vec![0u64; total];
    let mut idx = vec![0usize; dims.len()];
    for slot in 0..total {
        let point: Vec<u64> = idx.iter().map(|&i| i as u64).collect();
        t[slot] = f(&point, w);
        // increment odometer (last axis fastest)
        for ax in (0..dims.len()).rev() {
            idx[ax] += 1;
            if idx[ax] < dims[ax] {
                break;
            }
            idx[ax] = 0;
        }
    }
    // difference transform along each axis
    let mut stride_after: Vec<usize> = vec![1; dims.len()];
    for ax in (0..dims.len().saturating_sub(1)).rev() {
        stride_after[ax] = stride_after[ax + 1] * dims[ax + 1];
    }
    for ax in 0..dims.len() {
        let d = dims[ax];
        let stride = stride_after[ax];
        // iterate over all lines along this axis
        let line_count = total / d;
        for line in 0..line_count {
            // compute base offset of this line
            let block = stride * d;
            let base = (line / stride) * block + (line % stride);
            for j in 1..d {
                for i in (j..d).rev() {
                    let hi = base + i * stride;
                    let lo = base + (i - 1) * stride;
                    t[hi] = t[hi].wrapping_sub(t[lo]) & m;
                }
            }
        }
    }
    t.iter().all(|&v| v == 0)
}

fn mixed_sweep_verdict(f: &dyn Fn(&[u64], u32) -> u64, arity: usize, w: u32) -> bool {
    let n = 1u64 << w;
    let total = n.pow(arity as u32);
    let mut point = vec![0u64; arity];
    for lin in 0..total {
        let mut rem = lin;
        for slot in point.iter_mut() {
            *slot = rem % n;
            rem /= n;
        }
        if f(&point, w) != 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("p4: the finite-difference decision procedure, validated then used\n");

    // ---------------- univariate battery ----------------
    let mut rng = Xorshift(0x84_84_84_84_84);
    let mut battery: Vec<Vec<i128>> = Vec::new();
    // structured members: falling factorials 2..=8, scaled monomials, the p3 pair
    for k in 2..=8usize {
        battery.push(falling_coefs(k));
    }
    battery.push(vec![0, 24, 0, 8]); // 8x^3 + 24x
    battery.push(vec![0, 4, 4]); // 4x^2 + 4x
    battery.push(vec![0, 32]); // 32x
    battery.push(vec![0, 0, 0, 0, 16]); // 16x^4
                                        // random members. Coefficients carry a random extra power of two so the
                                        // battery's thresholds spread across the swept range instead of clustering
                                        // at zero (an odd coefficient anywhere pins the threshold at zero).
    for _ in 0..300 {
        let d = (rng.next() % 7) as usize; // degree 0..=6
        let coefs: Vec<i128> = (0..=d)
            .map(|_| {
                let base = (rng.next() % 129) as i128 - 64;
                let shift = rng.next() % 10;
                base << shift
            })
            .collect();
        battery.push(coefs);
    }

    let wmax = 12u32;
    let mut cert_mismatch = 0u64;
    let mut mod_mismatch = 0u64;
    let mut checked = 0u64;
    let mut nontrivial = 0u64; // laws with finite W* inside the swept range
    for coefs in &battery {
        let wstar = wstar_exact(coefs);
        if wstar >= 1 && wstar < wmax {
            nontrivial += 1;
        }
        for w in 1..=wmax {
            let swept = sweep_verdict(coefs, w);
            let cert = w <= wstar;
            let modv = diff_verdict_mod(coefs, w);
            checked += 1;
            if swept != cert {
                cert_mismatch += 1;
            }
            if swept != modv {
                mod_mismatch += 1;
            }
        }
    }
    println!(
        "univariate battery: {} laws x widths 1..={}",
        battery.len(),
        wmax
    );
    println!(
        "  (law, width) pairs checked:                     {}",
        checked
    );
    println!(
        "  laws whose threshold sits inside the range:     {}",
        nontrivial
    );
    println!(
        "  sweep vs exact certificate mismatches:          {}",
        cert_mismatch
    );
    println!(
        "  sweep vs modular-difference verdict mismatches: {}",
        mod_mismatch
    );
    assert!(cert_mismatch == 0 && mod_mismatch == 0);
    assert!(
        nontrivial > 100,
        "battery too easy: thresholds not exercised"
    );

    // the verdict at width 64 for the p2/p2b laws, by differences: constant work
    let l16 = falling_coefs(16);
    let l64 = falling_coefs(64);
    // exact certificate only for l16 (l64's integer values overflow i128 at
    // x = 64; the MODULAR verdict is the point anyway and never overflows)
    println!("\nverdicts at width 64 by modular differences (no sweep, no band):");
    println!(
        "  L_16: {} (17 evaluations; threshold 15, so false)",
        diff_verdict_mod(&l16, 64)
    );
    println!(
        "  L_64: {} (65 evaluations; threshold 63, so false)",
        diff_verdict_mod(&l64, 64)
    );
    println!("  L_16 exact certificate W* = {}", wstar_exact(&l16));
    assert!(!diff_verdict_mod(&l16, 64));
    assert!(!diff_verdict_mod(&l64, 64));
    assert!(wstar_exact(&l16) == 15);
    // L_128 at width 64: threshold 127, so TRUE, decided in 129 evaluations.
    let l128 = falling_coefs(128);
    println!(
        "  L_128: {} (129 evaluations; threshold 127, so true)",
        diff_verdict_mod(&l128, 64)
    );
    assert!(diff_verdict_mod(&l128, 64));

    // ---------------- multivariate ----------------
    let assoc_mul = |p: &[u64], w: u32| -> u64 {
        let m = mask_of(w);
        let l = (p[0].wrapping_mul(p[1]) & m).wrapping_mul(p[2]) & m;
        let r = p[0].wrapping_mul(p[1].wrapping_mul(p[2]) & m) & m;
        l.wrapping_sub(r) & m
    };
    let distrib = |p: &[u64], w: u32| -> u64 {
        let m = mask_of(w);
        let l = p[0].wrapping_mul(p[1].wrapping_add(p[2]) & m) & m;
        let r = (p[0].wrapping_mul(p[1]) & m).wrapping_add(p[0].wrapping_mul(p[2]) & m) & m;
        l.wrapping_sub(r) & m
    };
    let false3 = |p: &[u64], w: u32| -> u64 {
        // a*b - b*a + 8a: zero polynomial plus 8a, threshold 3
        let m = mask_of(w);
        let l = p[0].wrapping_mul(p[1]) & m;
        let r = p[1].wrapping_mul(p[0]) & m;
        l.wrapping_sub(r).wrapping_add(p[0].wrapping_mul(8)) & m
    };
    let chain4 = |p: &[u64], w: u32| -> u64 {
        let m = mask_of(w);
        let l = ((p[0].wrapping_mul(p[1]) & m).wrapping_mul(p[2]) & m).wrapping_mul(p[3]) & m;
        let r = p[0].wrapping_mul(p[1].wrapping_mul(p[2].wrapping_mul(p[3]) & m) & m) & m;
        l.wrapping_sub(r) & m
    };

    println!("\nmultivariate: mixed differences against exhaustive sweeps");
    let cases: Vec<(&str, &dyn Fn(&[u64], u32) -> u64, Vec<usize>, u32)> = vec![
        ("assoc(mul), arity 3", &assoc_mul, vec![2, 2, 2], 6),
        ("distrib, arity 3", &distrib, vec![2, 2, 2], 6),
        ("a*b - b*a + 8a", &false3, vec![2, 2], 8),
        ("chain assoc, arity 4", &chain4, vec![1, 1, 1, 1], 5),
    ];
    let mut multi_mismatch = 0u64;
    for (name, f, degs, wmax) in &cases {
        let arity = degs.len();
        let mut pattern = String::new();
        for w in 1..=*wmax {
            let dv = mixed_diff_verdict(*f, degs, w);
            let sv = mixed_sweep_verdict(*f, arity, w);
            if dv != sv {
                multi_mismatch += 1;
            }
            pattern.push(if dv { 'T' } else { 'f' });
        }
        let box_size: usize = degs.iter().map(|&d| d + 1).product();
        println!(
            "  {:<22} box {:>3} points, verdicts 1..={}: {}  verdict at width 64: {}",
            name,
            box_size,
            wmax,
            pattern,
            mixed_diff_verdict(*f, degs, 64)
        );
    }
    println!("  mixed-difference vs sweep mismatches: {}", multi_mismatch);
    assert!(multi_mismatch == 0);
    assert!(mixed_diff_verdict(&assoc_mul, &[2, 2, 2], 64));
    assert!(mixed_diff_verdict(&distrib, &[2, 2, 2], 64));
    assert!(!mixed_diff_verdict(&false3, &[2, 2], 64));
    assert!(mixed_diff_verdict(&chain4, &[1, 1, 1, 1], 64));

    // ---------------- the two trusted inputs, violated on purpose ----------------
    println!("\nnegative controls (each must produce a WRONG verdict):");

    // 1. fragment violation: a saturating law fed to the difference test.
    //    "forall x: sat_add(x, 1) == x + 1" is false at every width (x = MAX),
    //    but sat_add is not a ring term, and at the difference points 0..1 the
    //    clamp never fires, so the test reports true.
    let sat_law = |p: &[u64], w: u32| -> u64 {
        let m = mask_of(w);
        let s = if p[0] >= m { m } else { p[0] + 1 }; // unsigned sat_add(x, 1)
        s.wrapping_sub(p[0].wrapping_add(1) & m) & m
    };
    let dv = mixed_diff_verdict(&sat_law, &[1], 4);
    let sv = mixed_sweep_verdict(&sat_law, 1, 4);
    println!(
        "  saturating law, width 4: difference test says {}, sweep says {} => {}",
        dv,
        sv,
        if dv != sv {
            "wrong, as it must be (not a ring term)"
        } else {
            "UNEXPECTED AGREEMENT"
        }
    );
    assert!(dv && !sv);

    // 2. degree-bound violation: (x)_4 with claimed degree 2. Points 0..2 are
    //    all zeros of the product, so the test reports true; the law is false
    //    from width 4 up.
    let l4 = falling_coefs(4);
    let l4_fn = |p: &[u64], w: u32| -> u64 { eval_mod(&l4, p[0], w) };
    let dv2 = mixed_diff_verdict(&l4_fn, &[2], 6);
    let sv2 = mixed_sweep_verdict(&l4_fn, 1, 6);
    println!(
        "  (x)_4 with claimed degree 2, width 6: difference test says {}, sweep says {} => {}",
        dv2,
        sv2,
        if dv2 != sv2 {
            "wrong, as it must be (degree bound violated)"
        } else {
            "UNEXPECTED AGREEMENT"
        }
    );
    assert!(dv2 && !sv2);

    println!("\nall checks passed");
}
