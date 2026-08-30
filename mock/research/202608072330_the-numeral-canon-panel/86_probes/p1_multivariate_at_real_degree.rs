// PROBE p1. The multivariate difference criterion at genuine per-variable
// degree, which is 84's least-certain item 2 and the first target its closing
// section names.
//
// WHAT 84 ACTUALLY VERIFIED, checked at its source (84_probes/
// p4_difference_certificate.rs:331-336): four multivariate cases, degree
// BOUNDS (2,2,2), (2,2), (1,1,1,1), but every case's TRUE per-variable degree
// is at most 1 (assoc, distrib, the 8a law and the chain are all multilinear).
// So the multivariate criterion has never been exercised on a law with a
// genuinely quadratic-or-higher variable, and the necessity argument's
// interesting content (higher-order mixed differences) was never reached.
//
// THE CLAIM UNDER TEST (multivariate Kempner/Singmaster, full proof in file
// 86 section 2): for p in Z[x_1..x_k] with per-variable degrees d_v,
//
//   p == 0 identically on (Z/2^W)^k
//     <=>  every mixed forward difference of p at the origin, orders
//          J <= (d_1..d_k), is == 0 mod 2^W.
//
// AND THE SIMPLIFICATION THIS PROBE ALSO TESTS (86 section 3): the mixed
// difference tensor is the image of the grid-value tensor under a tensor
// product of unitriangular integer maps, whose inverse is again integer.
// Therefore
//
//   all differences == 0 mod 2^W   <=>   all GRID VALUES == 0 mod 2^W,
//
// i.e. the difference transform is unnecessary for the verdict: a false law
// has a witness inside the degree box itself, and the exact threshold is
//
//   W* = min over grid points of v2(p(point))    (computed over Z).
//
// This probe verifies, against exhaustive sweeps:
//   1. random multivariate batteries at arity 2 (per-variable degree to 6)
//      and arity 3 (per-variable degree to 4), coefficients carrying random
//      extra powers of two so thresholds spread across the swept range;
//   2. structured tensor falling factorials (x)_a (y)_b, genuinely degree
//      a and b per variable, against their Legendre-predicted thresholds
//      v2(a! b!) = (a - s2(a)) + (b - s2(b));
//   3. the grid-verdict == difference-verdict equivalence at every (law,
//      width) pair, and the threshold-from-grid == threshold-from-
//      differences equality on the exactly-representable battery;
//   4. a negative control: the degree bound understated in ONE variable of a
//      multivariate law produces a wrong verdict;
//   5. width-64 verdicts at genuine degree: (x)_33 (y)_33 (threshold 62,
//      false at 64, no band below 63 could catch it) and (x)_34 (y)_34
//      (threshold 64: TRUE at width 64, false at 65), each decided in a
//      degree box of evaluations through the width-64 map itself.
//
// Toolchain: pinned nightly-2026-05-28. Runtime probe; std/Vec are spike
// scaffolding per the panel's probe discipline, not design shape.

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
        x.unsigned_abs().trailing_zeros()
    }
}

// ---------- arity-2 coefficient-matrix polynomials (exact + modular) ----------

#[derive(Clone)]
struct Poly2 {
    // c[i][j] is the coefficient of x^i y^j
    c: Vec<Vec<i128>>,
}

impl Poly2 {
    fn dx(&self) -> usize {
        self.c.len() - 1
    }
    fn dy(&self) -> usize {
        self.c[0].len() - 1
    }

    fn eval_exact(&self, x: i128, y: i128) -> i128 {
        // Horner in x outside, y inside
        let mut acc = 0i128;
        for row in self.c.iter().rev() {
            let mut r = 0i128;
            for &cij in row.iter().rev() {
                r = r * y + cij;
            }
            acc = acc * x + r;
        }
        acc
    }

    fn eval_mod(&self, x: u64, y: u64, w: u32) -> u64 {
        let m = mask_of(w);
        let md = 1i128 << w;
        let mut acc = 0u64;
        for row in self.c.iter().rev() {
            let mut r = 0u64;
            for &cij in row.iter().rev() {
                let cr = (((cij % md) + md) % md) as u64;
                r = r.wrapping_mul(y).wrapping_add(cr) & m;
            }
            acc = acc.wrapping_mul(x).wrapping_add(r) & m;
        }
        acc
    }
}

/// Exhaustive sweep verdict at width w (arity 2).
fn sweep2(p: &Poly2, w: u32) -> bool {
    let n = 1u64 << w;
    for x in 0..n {
        for y in 0..n {
            if p.eval_mod(x, y, w) != 0 {
                return false;
            }
        }
    }
    true
}

/// Grid-value verdict: p vanishes mod 2^w at every point of the degree box.
fn grid_verdict2(p: &Poly2, w: u32) -> bool {
    for x in 0..=p.dx() as u64 {
        for y in 0..=p.dy() as u64 {
            if p.eval_mod(x, y, w) != 0 {
                return false;
            }
        }
    }
    true
}

/// Mixed-difference verdict: full difference tensor over the box, all zero
/// mod 2^w. Written independently of 84's implementation.
fn diff_verdict2(p: &Poly2, w: u32) -> bool {
    let m = mask_of(w);
    let (nx, ny) = (p.dx() + 1, p.dy() + 1);
    let mut t = vec![vec![0u64; ny]; nx];
    for x in 0..nx {
        for y in 0..ny {
            t[x][y] = p.eval_mod(x as u64, y as u64, w);
        }
    }
    // difference transform along x
    for j in 1..nx {
        for i in (j..nx).rev() {
            for y in 0..ny {
                t[i][y] = t[i][y].wrapping_sub(t[i - 1][y]) & m;
            }
        }
    }
    // then along y
    for j in 1..ny {
        for i in (j..ny).rev() {
            for x in 0..nx {
                t[x][i] = t[x][i].wrapping_sub(t[x][i - 1]) & m;
            }
        }
    }
    t.iter().all(|row| row.iter().all(|&v| v == 0))
}

/// Exact threshold from grid VALUES: min v2 of p(point) over the box, over Z.
fn wstar_from_grid2(p: &Poly2) -> u32 {
    let mut best = u32::MAX;
    for x in 0..=p.dx() as i128 {
        for y in 0..=p.dy() as i128 {
            best = best.min(v2_i128(p.eval_exact(x, y)));
        }
    }
    best
}

/// Exact threshold from the DIFFERENCE tensor over Z: min v2 of Delta^J p(0).
fn wstar_from_diffs2(p: &Poly2) -> u32 {
    let (nx, ny) = (p.dx() + 1, p.dy() + 1);
    let mut t = vec![vec![0i128; ny]; nx];
    for x in 0..nx {
        for y in 0..ny {
            t[x][y] = p.eval_exact(x as i128, y as i128);
        }
    }
    for j in 1..nx {
        for i in (j..nx).rev() {
            for y in 0..ny {
                t[i][y] -= t[i - 1][y];
            }
        }
    }
    for j in 1..ny {
        for i in (j..ny).rev() {
            for x in 0..nx {
                t[x][i] -= t[x][i - 1];
            }
        }
    }
    t.iter().flatten().map(|&v| v2_i128(v)).min().unwrap()
}

// ---------- arity-3 black-box polynomials over coefficient tensors ----------

#[derive(Clone)]
struct Poly3 {
    // c[i][j][k] coefficient of x^i y^j z^k
    c: Vec<Vec<Vec<i128>>>,
}

impl Poly3 {
    fn degs(&self) -> (usize, usize, usize) {
        (
            self.c.len() - 1,
            self.c[0].len() - 1,
            self.c[0][0].len() - 1,
        )
    }
    fn eval_mod(&self, x: u64, y: u64, z: u64, w: u32) -> u64 {
        let m = mask_of(w);
        let md = 1i128 << w;
        let mut acc = 0u64;
        for plane in self.c.iter().rev() {
            let mut py = 0u64;
            for row in plane.iter().rev() {
                let mut pz = 0u64;
                for &cij in row.iter().rev() {
                    let cr = (((cij % md) + md) % md) as u64;
                    pz = pz.wrapping_mul(z).wrapping_add(cr) & m;
                }
                py = py.wrapping_mul(y).wrapping_add(pz) & m;
            }
            acc = acc.wrapping_mul(x).wrapping_add(py) & m;
        }
        acc
    }
}

fn sweep3(p: &Poly3, w: u32) -> bool {
    let n = 1u64 << w;
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                if p.eval_mod(x, y, z, w) != 0 {
                    return false;
                }
            }
        }
    }
    true
}

fn grid_verdict3(p: &Poly3, w: u32) -> bool {
    let (dx, dy, dz) = p.degs();
    for x in 0..=dx as u64 {
        for y in 0..=dy as u64 {
            for z in 0..=dz as u64 {
                if p.eval_mod(x, y, z, w) != 0 {
                    return false;
                }
            }
        }
    }
    true
}

// ---------- black-box falling-factorial tensors (no coefficients needed) ----------

/// (x)_a * (y)_b evaluated through the width-w wrapping map. Never overflows,
/// at any a, b, because every multiply is through the map.
fn falling_tensor_mod(a: u64, b: u64, x: u64, y: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let mut acc = 1u64;
    for i in 0..a {
        acc = acc.wrapping_mul(x.wrapping_sub(i)) & m;
    }
    for i in 0..b {
        acc = acc.wrapping_mul(y.wrapping_sub(i)) & m;
    }
    acc
}

/// Grid verdict for (x)_a (y)_b at width w: box (a+1) x (b+1) evaluations.
fn falling_tensor_grid_verdict(a: u64, b: u64, w: u32) -> (bool, u64) {
    let mut evals = 0u64;
    for x in 0..=a {
        for y in 0..=b {
            evals += 1;
            if falling_tensor_mod(a, b, x, y, w) != 0 {
                return (false, evals);
            }
        }
    }
    (true, evals)
}

fn falling_tensor_sweep(a: u64, b: u64, w: u32) -> bool {
    let n = 1u64 << w;
    for x in 0..n {
        for y in 0..n {
            if falling_tensor_mod(a, b, x, y, w) != 0 {
                return false;
            }
        }
    }
    true
}

/// Legendre: v2(k!) = k - s2(k).
fn v2_factorial(k: u64) -> u32 {
    (k - k.count_ones() as u64) as u32
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

fn main() {
    println!("p1: the multivariate criterion at genuine per-variable degree\n");
    let mut rng = Xorshift(0x8686_8686_8686);

    // ---------------- arity-2 random battery, degrees to 6 per variable ----------------
    let mut battery2: Vec<Poly2> = Vec::new();
    for _ in 0..200 {
        let dx = 1 + (rng.next() % 6) as usize; // 1..=6
        let dy = 1 + (rng.next() % 6) as usize;
        let c: Vec<Vec<i128>> = (0..=dx)
            .map(|_| {
                (0..=dy)
                    .map(|_| {
                        let base = (rng.next() % 65) as i128 - 32;
                        let shift = rng.next() % 8; // extra powers of two
                        base << shift
                    })
                    .collect()
            })
            .collect();
        battery2.push(Poly2 { c });
    }
    // structured members with genuine mixed quadratic-and-higher content:
    // exactly-representable tensor falling factorials (x)_a (y)_b, a,b <= 6
    let falling_coefs = |k: usize| -> Vec<i128> {
        let mut c = vec![1i128];
        for i in 0..k as i128 {
            let mut next = vec![0i128; c.len() + 1];
            for (j, &cj) in c.iter().enumerate() {
                next[j + 1] += cj;
                next[j] += cj * (-i);
            }
            c = next;
        }
        c
    };
    for a in 2..=6usize {
        for b in 2..=6usize {
            let fa = falling_coefs(a);
            let fb = falling_coefs(b);
            let c: Vec<Vec<i128>> = fa
                .iter()
                .map(|&ca| fb.iter().map(|&cb| ca * cb).collect())
                .collect();
            battery2.push(Poly2 { c });
        }
    }

    let wmax2 = 7u32;
    let (mut checked, mut sweep_vs_grid, mut sweep_vs_diff, mut grid_vs_diff) =
        (0u64, 0u64, 0u64, 0u64);
    let mut wstar_mismatch = 0u64;
    let mut nontrivial = 0u64;
    let mut true_deg_ge2 = 0u64;
    for p in &battery2 {
        // true per-variable degree >= 2 count (guard against a multilinear battery)
        let dx_true =
            p.c.iter()
                .rposition(|row| row.iter().any(|&v| v != 0))
                .unwrap();
        let dy_true = (0..p.c[0].len())
            .rev()
            .find(|&j| p.c.iter().any(|row| row[j] != 0))
            .unwrap();
        if dx_true >= 2 && dy_true >= 2 {
            true_deg_ge2 += 1;
        }
        let wg = wstar_from_grid2(p);
        let wd = wstar_from_diffs2(p);
        if wg != wd {
            wstar_mismatch += 1;
        }
        if wg >= 1 && wg < wmax2 {
            nontrivial += 1;
        }
        for w in 1..=wmax2 {
            let s = sweep2(p, w);
            let g = grid_verdict2(p, w);
            let d = diff_verdict2(p, w);
            checked += 1;
            if s != g {
                sweep_vs_grid += 1;
            }
            if s != d {
                sweep_vs_diff += 1;
            }
            if g != d {
                grid_vs_diff += 1;
            }
        }
    }
    println!(
        "arity-2 battery: {} laws x widths 1..={}",
        battery2.len(),
        wmax2
    );
    println!(
        "  laws with TRUE per-variable degree >= 2 in both vars: {}",
        true_deg_ge2
    );
    println!(
        "  laws whose threshold sits inside the swept range:     {}",
        nontrivial
    );
    println!(
        "  (law, width) pairs checked:                           {}",
        checked
    );
    println!(
        "  sweep vs grid-verdict mismatches:                     {}",
        sweep_vs_grid
    );
    println!(
        "  sweep vs difference-verdict mismatches:               {}",
        sweep_vs_diff
    );
    println!(
        "  grid-verdict vs difference-verdict mismatches:        {}",
        grid_vs_diff
    );
    println!(
        "  threshold-from-grid vs threshold-from-diffs mismatch: {}",
        wstar_mismatch
    );
    assert!(sweep_vs_grid == 0 && sweep_vs_diff == 0 && grid_vs_diff == 0 && wstar_mismatch == 0);
    assert!(
        true_deg_ge2 > 100,
        "battery too easy: not enough genuine degree"
    );
    assert!(
        nontrivial > 100,
        "battery too easy: thresholds not exercised"
    );

    // ---------------- structured: Legendre-predicted thresholds ----------------
    println!("\ntensor falling factorials (x)_a (y)_b: threshold prediction");
    let mut pred_mismatch = 0u64;
    for a in 2..=6u64 {
        for b in 2..=6u64 {
            let fa = falling_coefs(a as usize);
            let fb = falling_coefs(b as usize);
            let c: Vec<Vec<i128>> = fa
                .iter()
                .map(|&ca| fb.iter().map(|&cb| ca * cb).collect())
                .collect();
            let p = Poly2 { c };
            let predicted = v2_factorial(a) + v2_factorial(b);
            let computed = wstar_from_grid2(&p);
            if computed != predicted {
                pred_mismatch += 1;
                println!(
                    "  MISMATCH at a={}, b={}: predicted {}, computed {}",
                    a, b, predicted, computed
                );
            }
        }
    }
    println!(
        "  predicted (a - s2(a)) + (b - s2(b)) vs computed, 25 members: {} mismatches",
        pred_mismatch
    );
    assert!(pred_mismatch == 0);

    // ---------------- arity-3 random battery, degrees to 4 per variable ----------------
    let mut battery3: Vec<Poly3> = Vec::new();
    for _ in 0..100 {
        let d: Vec<usize> = (0..3).map(|_| 1 + (rng.next() % 4) as usize).collect();
        let c: Vec<Vec<Vec<i128>>> = (0..=d[0])
            .map(|_| {
                (0..=d[1])
                    .map(|_| {
                        (0..=d[2])
                            .map(|_| {
                                let base = (rng.next() % 33) as i128 - 16;
                                let shift = rng.next() % 5;
                                base << shift
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        battery3.push(Poly3 { c });
    }
    let wmax3 = 4u32;
    let (mut checked3, mut mismatch3) = (0u64, 0u64);
    let mut deg2plus3 = 0u64;
    for p in &battery3 {
        let (dx, dy, dz) = p.degs();
        if dx >= 2 && dy >= 2 && dz >= 2 {
            deg2plus3 += 1;
        }
        for w in 1..=wmax3 {
            let s = sweep3(p, w);
            let g = grid_verdict3(p, w);
            checked3 += 1;
            if s != g {
                mismatch3 += 1;
            }
        }
    }
    println!(
        "\narity-3 battery: {} laws x widths 1..={}",
        battery3.len(),
        wmax3
    );
    println!(
        "  laws with true per-variable degree >= 2 in all three: {}",
        deg2plus3
    );
    println!(
        "  (law, width) pairs checked:                           {}",
        checked3
    );
    println!(
        "  sweep vs grid-verdict mismatches:                     {}",
        mismatch3
    );
    assert!(mismatch3 == 0);
    assert!(deg2plus3 > 20, "battery too easy");

    // ---------------- negative control: degree bound understated in one variable ----------------
    // (x)_3 (y)_1 with the x-degree claimed as 2: the grid misses x = 3 where
    // the value is 3! * y, so the truncated grid reports true while the law is
    // false from width 2 up (v2(3! * 1!) = 1).
    let fa = falling_coefs(3);
    let c: Vec<Vec<i128>> = fa.iter().map(|&ca| vec![0, ca]).collect(); // (x)_3 * y
    let p_ctrl = Poly2 { c };
    let w_ctrl = 3u32;
    let truncated_grid_true = {
        let mut ok = true;
        for x in 0..=2u64 {
            // claimed degree 2 in x: grid 0..=2 only
            for y in 0..=1u64 {
                if p_ctrl.eval_mod(x, y, w_ctrl) != 0 {
                    ok = false;
                }
            }
        }
        ok
    };
    let swept = sweep2(&p_ctrl, w_ctrl);
    println!(
        "\nnegative control: (x)_3 * y with x-degree claimed 2, width {}:",
        w_ctrl
    );
    println!(
        "  truncated grid says {}, sweep says {} => {}",
        truncated_grid_true,
        swept,
        if truncated_grid_true != swept {
            "wrong, as it must be (bound understated in x)"
        } else {
            "UNEXPECTED AGREEMENT"
        }
    );
    assert!(truncated_grid_true && !swept);

    // ---------------- width-64 verdicts at genuine degree, through the map ----------------
    println!("\nwidth-64 verdicts by grid evaluation (no sweep, no band, no transfer):");
    let (v33, e33) = falling_tensor_grid_verdict(33, 33, 64);
    println!("  (x)_33 (y)_33: {} in {} evaluations (Legendre threshold {} => false at 64, true through {})",
        v33, e33, v2_factorial(33) + v2_factorial(33), v2_factorial(33) + v2_factorial(33));
    assert!(!v33);
    let (v34, e34) = falling_tensor_grid_verdict(34, 34, 64);
    println!(
        "  (x)_34 (y)_34: {} in {} evaluations (Legendre threshold {} => TRUE at 64, false at 65)",
        v34,
        e34,
        v2_factorial(34) + v2_factorial(34)
    );
    assert!(v34);
    let (v34_65, _) = falling_tensor_grid_verdict(34, 34, 65);
    // width 65 exceeds u64; emulate: the value at (34, 34) is 34!^2 which has
    // v2 exactly 64, so mod 2^65 it is 2^64 * odd, nonzero. Check via u128.
    let mut acc = 1u128;
    for i in 0..34u128 {
        acc = acc.wrapping_mul(34 - i) & ((1u128 << 65) - 1);
    }
    for i in 0..34u128 {
        acc = acc.wrapping_mul(34 - i) & ((1u128 << 65) - 1);
    }
    println!("  (x)_34 (y)_34 at width 65 via u128 at the witness (34, 34): residue = {:#x} (must be 1 << 64)", acc);
    assert!(acc == 1u128 << 64);
    let _ = v34_65; // width-65 through the u64 map saturates the mask; the u128 check above is the honest one
                    // sanity: the small members agree with sweeps where sweeps exist
    let mut tensor_sweep_mismatch = 0u64;
    for a in 2..=5u64 {
        for b in 2..=5u64 {
            for w in 1..=5u32 {
                let (g, _) = falling_tensor_grid_verdict(a, b, w);
                if g != falling_tensor_sweep(a, b, w) {
                    tensor_sweep_mismatch += 1;
                }
            }
        }
    }
    println!(
        "  black-box tensor grid vs sweep, a,b in 2..=5, widths 1..=5: {} mismatches",
        tensor_sweep_mismatch
    );
    assert!(tensor_sweep_mismatch == 0);

    println!("\nall checks passed");
}
