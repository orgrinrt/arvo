// p2: the same derivation for the `Ranged` family.
//
// The design's own statement of the value set, quoted at 124 section 1.5 from `58:220-224`:
//   "A `Ranged` numeral denotes the union, over `e` in `[EMIN, EMAX]`, of the grids with quantum
//    `radix^(e - p + 1)` restricted to `[radix^e, radix^(e+1))`, together with the bottom grid
//    extended down to zero when `Underflow = Gradual`, omitted when `Underflow = Abrupt`."
//
// Note what `Ranged<EMIN, EMAX, U, S>` does NOT carry: an Adjustment and a Bias. Those are
// members of `Implicit<E, A, B>`. So the phase condition that p1 finds for `Implicit` has no
// `Ranged` instance, and the condition set is keyed on the exponent form.
//
// Specials are left out of the value set here; they are a separate containment condition
// (S1 subset of S2) that does not interact with the grid, and folding them in would only
// add a conjunct.
//
// Run: rustc +nightly-2026-05-28 -O p2_ranged_inclusion.rs -o /tmp/p2 && /tmp/p2

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rat {
    n: i128,
    d: i128,
}
fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    if a == 0 {
        1
    } else {
        a
    }
}
impl Rat {
    fn new(n: i128, d: i128) -> Rat {
        let s = if d < 0 { -1 } else { 1 };
        let (n, d) = (n * s, d * s);
        let g = gcd(n, d);
        Rat { n: n / g, d: d / g }
    }
    fn int(n: i128) -> Rat {
        Rat { n, d: 1 }
    }
    fn mul(self, o: Rat) -> Rat {
        Rat::new(self.n * o.n, self.d * o.d)
    }
    fn neg(self) -> Rat {
        Rat {
            n: -self.n,
            d: self.d,
        }
    }
    fn cmp_r(self, o: Rat) -> core::cmp::Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
}
fn powr(r: i128, e: i32) -> Rat {
    if e >= 0 {
        Rat::int(r.pow(e as u32))
    } else {
        Rat::new(1, r.pow((-e) as u32))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Under {
    Gradual,
    Abrupt,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dom {
    NonNegative,
    Symmetric,
}

#[derive(Clone, Copy, Debug)]
struct RNum {
    r: i128,
    p: u32,
    emin: i32,
    emax: i32,
    u: Under,
    dom: Dom,
}

impl RNum {
    fn values(&self) -> Vec<Rat> {
        let mut v = vec![Rat::int(0)];
        let base = self.r.pow(self.p - 1); // r^(p-1)
        let top = self.r.pow(self.p) - 1; // r^p - 1
        for e in self.emin..=self.emax {
            let q = powr(self.r, e - self.p as i32 + 1);
            for m in base..=top {
                v.push(q.mul(Rat::int(m)));
            }
        }
        if self.u == Under::Gradual {
            let q = powr(self.r, self.emin - self.p as i32 + 1);
            for m in 1..base {
                v.push(q.mul(Rat::int(m)));
            }
        }
        if self.dom == Dom::Symmetric {
            let neg: Vec<Rat> = v.iter().map(|x| x.neg()).collect();
            v.extend(neg);
        }
        v.sort_by(|a, b| a.cmp_r(*b));
        v.dedup();
        v
    }
    // the exponent of the finest quantum anywhere in the format
    fn finest(&self) -> i32 {
        self.emin - self.p as i32 + 1
    }
    // the exponent of the coarsest quantum, at the top binade
    fn coarsest(&self) -> i32 {
        self.emax - self.p as i32 + 1
    }
    fn smallest_positive_exp_ok(&self) -> i32 {
        // magnitude exponent below which the format has no nonzero value
        match self.u {
            Under::Gradual => self.finest(),
            Under::Abrupt => self.emin,
        }
    }
}

fn oracle(src: &RNum, tgt: &RNum) -> bool {
    let sv = src.values();
    let tv = tgt.values();
    sv.iter()
        .all(|x| tv.binary_search_by(|y| y.cmp_r(*x)).is_ok())
}

fn dom_rank(d: Dom) -> u8 {
    match d {
        Dom::NonNegative => 0,
        Dom::Symmetric => 1,
    }
}

// R_naive: the product order on the members `Ranged` declares, each on its own axis.
fn r_naive(s: &RNum, t: &RNum) -> bool {
    s.r == t.r
        && s.p <= t.p
        && t.emin <= s.emin
        && t.emax >= s.emax
        && dom_rank(s.dom) <= dom_rank(t.dom)
        && (s.u == Under::Abrupt || t.u == Under::Gradual)
}

// R_basis: the same product order after the change of basis that replaces EMIN by
// the finest-quantum exponent EMIN - p + 1, which is the coordinate the grid actually
// lives in.  This is the reading under which "adding a significand digit buys an exponent".
fn r_basis(s: &RNum, t: &RNum) -> bool {
    s.r == t.r
        && s.p <= t.p
        && t.finest() <= s.finest()
        && t.emax >= s.emax
        && dom_rank(s.dom) <= dom_rank(t.dom)
        && (s.u == Under::Abrupt || t.u == Under::Gradual)
}

// R_exact: the candidate exact condition set, derived rather than guessed.
//   (G) within every binade the source occupies, the target's local quantum divides the
//       source's.  Because both grids are powers of the same radix this reduces to a
//       comparison of exponent functions, phi(e) = max(e - p + 1, finest), pointwise.
//   (T) the top: the largest source value is at most the largest target value.
//   (B) the bottom: no source value falls below where the target's grid stops.
fn phi(n: &RNum, e: i32) -> i32 {
    let normal = e - n.p as i32 + 1;
    match n.u {
        Under::Gradual => {
            if normal < n.finest() {
                n.finest()
            } else {
                normal
            }
        }
        Under::Abrupt => normal,
    }
}
fn r_exact(s: &RNum, t: &RNum) -> bool {
    if s.r != t.r {
        return false;
    }
    if dom_rank(s.dom) > dom_rank(t.dom) {
        return false;
    }
    // pointwise on every binade the source can inhabit, including its subnormal band
    let lo = if s.u == Under::Gradual {
        s.finest()
    } else {
        s.emin
    };
    for e in lo..=s.emax {
        if phi(t, e) > phi(s, e) {
            return false;
        }
    }
    // the target's grid must reach down at least as far as the source's smallest nonzero
    if t.smallest_positive_exp_ok() > s.smallest_positive_exp_ok() {
        return false;
    }
    // and up at least as far as the source's largest
    if t.emax < s.emax {
        return false;
    }
    true
}

fn grid() -> Vec<RNum> {
    let mut v = Vec::new();
    for &r in &[2i128, 3] {
        for p in 1u32..=3 {
            for emin in -3i32..=0 {
                for emax in emin..=(emin + 3) {
                    for u in [Under::Gradual, Under::Abrupt] {
                        for dom in [Dom::NonNegative, Dom::Symmetric] {
                            v.push(RNum {
                                r,
                                p,
                                emin,
                                emax,
                                u,
                                dom,
                            });
                        }
                    }
                }
            }
        }
    }
    v
}

fn main() {
    // The specific pair `146` reports as its third instance, checked directly against the
    // design's own value-set sentence rather than against a re-derived model.
    // `146`: "R. first counterexample: p2 [-3,0] into p3 [-3,0]: componentwise says true,
    //         inclusion is false" and "adding a significand digit while holding the exponent
    //         window loses values off the bottom".
    for u in [Under::Gradual, Under::Abrupt] {
        let src = RNum {
            r: 2,
            p: 2,
            emin: -3,
            emax: 0,
            u,
            dom: Dom::NonNegative,
        };
        let tgt = RNum {
            r: 2,
            p: 3,
            emin: -3,
            emax: 0,
            u,
            dom: Dom::NonNegative,
        };
        let sv = src.values();
        let tv = tgt.values();
        let missing: Vec<(i128, i128)> = sv
            .iter()
            .filter(|x| tv.binary_search_by(|y| y.cmp_r(**x)).is_err())
            .map(|r| (r.n, r.d))
            .collect();
        println!(
            "146's third-instance pair, Underflow={:?}: source has {} values, target has {}, \
             source values NOT in target: {} {:?}",
            u,
            sv.len(),
            tv.len(),
            missing.len(),
            missing
        );
        println!(
            "  under the design's sentence the binade quantum is r^(e-p+1), so at e={} it is \
             r^{} for the source and r^{} for the target: the target grid is FINER everywhere",
            src.emin,
            src.emin - src.p as i32 + 1,
            tgt.emin - tgt.p as i32 + 1
        );
    }
    println!();

    let g = grid();
    println!("Ranged numerals: {}", g.len());
    println!("ordered pairs: {}", g.len() * g.len());

    let mut n = 0usize;
    let mut naive = (0usize, 0usize);
    let mut basis = (0usize, 0usize);
    let mut exact = (0usize, 0usize);
    let mut w_naive: Option<(usize, usize)> = None;
    let mut w_basis: Option<(usize, usize)> = None;
    let mut w_exact_fp: Option<(usize, usize)> = None;
    let mut w_exact_fn: Option<(usize, usize)> = None;
    // same-radix restriction, and the cross-radix inclusions R_exact declines to decide
    let mut same_radix_pairs = 0usize;
    let mut same_radix_exact_fn = 0usize;
    let mut cross_radix_incl = 0usize;
    let mut cross_radix_incl_nondegenerate = 0usize;
    let mut w_cross: Option<(usize, usize)> = None;
    // the specific shape: precision rises, EMIN rises with it, inclusion still holds
    let mut digit_buys_exponent = 0usize;
    let mut w_dbe: Option<(usize, usize)> = None;

    for (i, s) in g.iter().enumerate() {
        for (j, t) in g.iter().enumerate() {
            let o = oracle(s, t);
            if o {
                n += 1;
            }
            for (f, c, w) in [
                (r_naive(s, t), &mut naive, &mut w_naive),
                (r_basis(s, t), &mut basis, &mut w_basis),
            ] {
                if f && !o {
                    c.0 += 1;
                    if w.is_none() {
                        *w = Some((i, j));
                    }
                }
                if !f && o {
                    c.1 += 1;
                }
            }
            let e = r_exact(s, t);
            if e && !o {
                exact.0 += 1;
                if w_exact_fp.is_none() {
                    w_exact_fp = Some((i, j));
                }
            }
            if !e && o {
                exact.1 += 1;
                if w_exact_fn.is_none() {
                    w_exact_fn = Some((i, j));
                }
            }
            if s.r == t.r {
                same_radix_pairs += 1;
                if !e && o {
                    same_radix_exact_fn += 1;
                }
            } else if o {
                cross_radix_incl += 1;
                if s.values().len() >= 4 {
                    cross_radix_incl_nondegenerate += 1;
                    if w_cross.is_none() {
                        w_cross = Some((i, j));
                    }
                }
            }
            if o && t.p > s.p && t.emin > s.emin {
                digit_buys_exponent += 1;
                if w_dbe.is_none() {
                    w_dbe = Some((i, j));
                }
            }
        }
    }

    println!(
        "\noracle: {} of {} ordered pairs include",
        n,
        g.len() * g.len()
    );
    println!("\ncondition set                      false-positive  false-negative");
    println!(
        "R_naive (p, EMIN, EMAX, U, dom)   {:>15}  {:>14}",
        naive.0, naive.1
    );
    println!(
        "R_basis (p, EMIN-p+1, EMAX, ...)  {:>15}  {:>14}",
        basis.0, basis.1
    );
    println!(
        "R_exact (pointwise phi + range)   {:>15}  {:>14}",
        exact.0, exact.1
    );

    if let Some((i, j)) = w_naive {
        println!("\nR_naive false positive:");
        show(&g[i], "  source");
        show(&g[j], "  target");
    }
    if let Some((i, j)) = w_basis {
        println!("\nR_basis false positive:");
        show(&g[i], "  source");
        show(&g[j], "  target");
    }
    if let Some((i, j)) = w_exact_fp {
        println!("\nR_exact false positive:");
        show(&g[i], "  source");
        show(&g[j], "  target");
    }
    if let Some((i, j)) = w_exact_fn {
        println!("\nR_exact false negative:");
        show(&g[i], "  source");
        show(&g[j], "  target");
    }

    println!(
        "\nsame-radix pairs: {}, R_exact false negatives among them: {}",
        same_radix_pairs, same_radix_exact_fn
    );
    println!(
        "cross-radix inclusions the oracle finds: {}, of which the source has at least 4 values: {}",
        cross_radix_incl, cross_radix_incl_nondegenerate
    );
    if let Some((i, j)) = w_cross {
        println!("cross-radix witness:");
        show(&g[i], "  source");
        show(&g[j], "  target");
        println!(
            "  source values: {:?}",
            g[i].values().iter().map(|r| (r.n, r.d)).collect::<Vec<_>>()
        );
    }
    println!(
        "\ninclusions where the target's precision is strictly larger AND its EMIN is strictly \
         larger (a significand digit paying for an exponent): {}",
        digit_buys_exponent
    );
    if let Some((i, j)) = w_dbe {
        show(&g[i], "  source");
        show(&g[j], "  target");
        println!(
            "  source finest-quantum exponent {}, target finest-quantum exponent {}",
            g[i].finest(),
            g[j].finest()
        );
    }
}

fn show(n: &RNum, tag: &str) {
    let v = n.values();
    println!(
        "{}: r={} p={} EMIN={} EMAX={} U={:?} dom={:?}  finest=r^{} coarsest=r^{}  |V|={}",
        tag,
        n.r,
        n.p,
        n.emin,
        n.emax,
        n.u,
        n.dom,
        n.finest(),
        n.coarsest(),
        v.len()
    );
}
