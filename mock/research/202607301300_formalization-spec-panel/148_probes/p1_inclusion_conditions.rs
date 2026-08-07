// p1: derive the conditions under which one Implicit numeral's value set includes another's.
//
// The design's own statements this is built from:
//   124:1.1  "The value of a stored integer `k` under a numeral is `Adjustment * radix^exponent * k + Bias`."
//   124:1.2  `Domain: SignDomain` is `NonNegative | Symmetric | AsymmetricLow`, a value fact.
//   124:1.2  the SC_SAT_SYM cell: AsymmetricLow's floor is -8 where Symmetric's is -7, so
//            NonNegative  k in [0, r^p - 1]
//            Symmetric    k in [-(r^p - 1), r^p - 1]
//            AsymmetricLow k in [-r^p, r^p - 1]
//
// No condition set is assumed. The oracle is elementwise set inclusion over exact rationals.
// The candidate conditions are then checked against the oracle over the whole grid, in both
// directions (no false positives, no false negatives).
//
// Run: rustc +nightly-2026-05-28 -O p1_inclusion_conditions.rs -o /tmp/p1 && /tmp/p1

// ---------- exact rationals ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rat {
    n: i128,
    d: i128,
} // d > 0, gcd(|n|,d) = 1

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
        assert!(d != 0);
        let s = if d < 0 { -1 } else { 1 };
        let (n, d) = (n * s, d * s);
        let g = gcd(n, d);
        Rat { n: n / g, d: d / g }
    }
    fn int(n: i128) -> Rat {
        Rat { n, d: 1 }
    }
    fn zero() -> Rat {
        Rat { n: 0, d: 1 }
    }
    fn add(self, o: Rat) -> Rat {
        Rat::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    fn sub(self, o: Rat) -> Rat {
        Rat::new(self.n * o.d - o.n * self.d, self.d * o.d)
    }
    fn mul(self, o: Rat) -> Rat {
        Rat::new(self.n * o.n, self.d * o.d)
    }
    fn div(self, o: Rat) -> Rat {
        assert!(o.n != 0);
        Rat::new(self.n * o.d, self.d * o.n)
    }
    fn is_int(self) -> bool {
        self.d == 1
    }
    fn cmp_r(self, o: Rat) -> core::cmp::Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
    fn le(self, o: Rat) -> bool {
        self.cmp_r(o) != core::cmp::Ordering::Greater
    }
    fn ge(self, o: Rat) -> bool {
        self.cmp_r(o) != core::cmp::Ordering::Less
    }
}

fn powr(r: i128, e: i32) -> Rat {
    if e >= 0 {
        Rat::int(r.pow(e as u32))
    } else {
        Rat::new(1, r.pow((-e) as u32))
    }
}

// ---------- numerals ----------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dom {
    NonNegative,
    Symmetric,
    AsymmetricLow,
}

#[derive(Clone, Copy, Debug)]
struct Num {
    r: i128, // Radix, >= 2
    p: u32,  // Precision, significand digit count
    e: i32,  // Implicit exponent
    a: Rat,  // Adjustment
    b: Rat,  // Bias
    dom: Dom,
}

impl Num {
    // index interval [k-, k+]
    fn kmin(&self) -> i128 {
        let m = self.r.pow(self.p);
        match self.dom {
            Dom::NonNegative => 0,
            Dom::Symmetric => -(m - 1),
            Dom::AsymmetricLow => -m,
        }
    }
    fn kmax(&self) -> i128 {
        self.r.pow(self.p) - 1
    }
    // the quantum: the spacing between adjacent representable values
    fn q(&self) -> Rat {
        self.a.mul(powr(self.r, self.e))
    }
    fn val(&self, k: i128) -> Rat {
        self.q().mul(Rat::int(k)).add(self.b)
    }
    // the value set, sorted ascending
    fn values(&self) -> Vec<Rat> {
        let mut v: Vec<Rat> = (self.kmin()..=self.kmax()).map(|k| self.val(k)).collect();
        v.sort_by(|x, y| x.cmp_r(*y));
        v.dedup();
        v
    }
    fn vmin(&self) -> Rat {
        let (x, y) = (self.val(self.kmin()), self.val(self.kmax()));
        if x.le(y) {
            x
        } else {
            y
        }
    }
    fn vmax(&self) -> Rat {
        let (x, y) = (self.val(self.kmin()), self.val(self.kmax()));
        if x.le(y) {
            y
        } else {
            x
        }
    }
    fn card(&self) -> i128 {
        self.kmax() - self.kmin() + 1
    }
}

// ---------- the oracle: elementwise inclusion ----------

fn oracle_includes(src: &Num, tgt: &Num) -> bool {
    let sv = src.values();
    let tv = tgt.values();
    sv.iter()
        .all(|x| tv.binary_search_by(|y| y.cmp_r(*x)).is_ok())
}

// ---------- candidate condition sets ----------

// C_two: the two-condition reading. Under A=1, B=0, same radix, same sign domain,
// this is exactly "target fraction width >= source fraction width" (grid) and
// "target integer width >= source integer width" (range), restated in the general
// coordinates so it can be evaluated at every numeral rather than only at that slice.
fn cond_two(src: &Num, tgt: &Num) -> bool {
    let grid = src.q().div(tgt.q());
    let grid_ok = grid.is_int() || Rat::zero().sub(grid).is_int();
    let top_ok = src.vmax().le(tgt.vmax());
    grid_ok && top_ok
}

// C_four: grid refinement, phase alignment, lower range, upper range.
fn cond_four(src: &Num, tgt: &Num) -> (bool, bool, bool, bool) {
    let (q1, q2) = (src.q(), tgt.q());
    // grid: |q1| / |q2| is a positive integer
    let ratio = q1.div(q2);
    let ratio = if ratio.n < 0 {
        Rat::zero().sub(ratio)
    } else {
        ratio
    };
    let grid = ratio.is_int();
    // phase: the source's offset sits on the target's lattice.
    // Anchor at the source's own least value rather than at its bias, so the
    // condition is stated without assuming the index interval contains 0.
    let phase = src.vmin().sub(tgt.b).div(q2).is_int();
    let low = src.vmin().ge(tgt.vmin());
    let high = src.vmax().le(tgt.vmax());
    (grid, phase, low, high)
}

fn cond_four_all(src: &Num, tgt: &Num) -> bool {
    let (g, p, l, h) = cond_four(src, tgt);
    if src.card() == 1 {
        // degenerate: a single point needs only membership
        return p && l && h;
    }
    g && p && l && h
}

// C_lat: the two-coordinate reading.
//   coordinate 1, the affine lattice  L(N) = b + q Z, ordered by inclusion
//   coordinate 2, the range interval  [vmin, vmax], ordered by inclusion
fn lattice_included(src: &Num, tgt: &Num) -> bool {
    let ratio = src.q().div(tgt.q());
    let ratio = if ratio.n < 0 {
        Rat::zero().sub(ratio)
    } else {
        ratio
    };
    ratio.is_int() && src.b.sub(tgt.b).div(tgt.q()).is_int()
}

fn cond_lat(src: &Num, tgt: &Num) -> bool {
    lattice_included(src, tgt) && src.vmin().ge(tgt.vmin()) && src.vmax().le(tgt.vmax())
}

// C_decl: the declared-coordinate product order.  p, e, dom each compared on its own,
// with dom ordered NonNegative < Symmetric < AsymmetricLow, holding radix/adjustment/bias equal.
fn dom_rank(d: Dom) -> u8 {
    match d {
        Dom::NonNegative => 0,
        Dom::Symmetric => 1,
        Dom::AsymmetricLow => 2,
    }
}
// C_pure: the product order on the members the numeral actually declares,
// Precision and Exponent and Domain, each on its own axis and nothing mixed.
fn cond_pure(src: &Num, tgt: &Num) -> bool {
    src.r == tgt.r
        && src.a == tgt.a
        && src.b == tgt.b
        && src.p <= tgt.p
        && tgt.e <= src.e
        && dom_rank(src.dom) <= dom_rank(tgt.dom)
}

// C_decl: the product order after a change of basis, on (top magnitude, grid, domain).
fn cond_decl(src: &Num, tgt: &Num) -> bool {
    src.r == tgt.r
        && src.a == tgt.a
        && src.b == tgt.b
        && tgt.e <= src.e
        && (src.p as i32 + src.e) <= (tgt.p as i32 + tgt.e)
        && dom_rank(src.dom) <= dom_rank(tgt.dom)
}

// ---------- the grid ----------

fn grid() -> Vec<Num> {
    let mut v = Vec::new();
    for &r in &[2i128, 3] {
        for p in 1u32..=3 {
            for e in -2i32..=1 {
                // Adjustment: Unit, and FullRange at this shape (r^p / (r^p - 1)),
                // which is the design's own second constructor.
                let full = Rat::new(r.pow(p), r.pow(p) - 1);
                for a in [Rat::int(1), full] {
                    for b in [Rat::zero(), Rat::new(1, 2), Rat::new(1, 3), Rat::int(1)] {
                        for dom in [Dom::NonNegative, Dom::Symmetric, Dom::AsymmetricLow] {
                            v.push(Num { r, p, e, a, b, dom });
                        }
                    }
                }
            }
        }
    }
    v
}

fn main() {
    let g = grid();
    println!("numerals: {}", g.len());
    println!("ordered pairs: {}", g.len() * g.len());

    let mut n_incl = 0usize;
    // agreement counters: (false positive, false negative)
    let mut two = (0usize, 0usize);
    let mut four = (0usize, 0usize);
    let mut lat = (0usize, 0usize);
    let mut decl = (0usize, 0usize);
    let mut pure = (0usize, 0usize);
    let mut first_pure_fp: Option<(usize, usize)> = None;

    // which single condition of the four is the one that saves C_four where C_two fails
    let mut sole_phase = 0usize;
    let mut sole_low = 0usize;
    let mut first_phase_witness: Option<(usize, usize)> = None;
    let mut first_low_witness: Option<(usize, usize)> = None;

    // same-radix, same-adjustment, same-bias slice, to isolate the sign-domain finding
    let mut decl_slice_fp = 0usize;
    let mut decl_slice_fn = 0usize;
    let mut first_signdom_witness: Option<(usize, usize)> = None;

    for (i, s) in g.iter().enumerate() {
        for (j, t) in g.iter().enumerate() {
            let o = oracle_includes(s, t);
            if o {
                n_incl += 1;
            }
            let c2 = cond_two(s, t);
            let c4 = cond_four_all(s, t);
            let cl = cond_lat(s, t);
            let cd = cond_decl(s, t);

            if c2 && !o {
                two.0 += 1;
            }
            if !c2 && o {
                two.1 += 1;
            }
            if c4 && !o {
                four.0 += 1;
            }
            if !c4 && o {
                four.1 += 1;
            }
            if cl && !o {
                lat.0 += 1;
            }
            if !cl && o {
                lat.1 += 1;
            }
            if cd && !o {
                decl.0 += 1;
            }
            if !cd && o {
                decl.1 += 1;
            }
            let cp = cond_pure(s, t);
            if cp && !o {
                pure.0 += 1;
                if first_pure_fp.is_none() {
                    first_pure_fp = Some((i, j));
                }
            }
            if !cp && o {
                pure.1 += 1;
            }

            // where C_two claims inclusion and the oracle refuses, which of the two
            // conditions C_two lacks is the one doing the work
            if c2 && !o {
                let (_gg, pp, ll, _hh) = cond_four(s, t);
                if !pp && ll {
                    sole_phase += 1;
                    if first_phase_witness.is_none() {
                        first_phase_witness = Some((i, j));
                    }
                }
                if pp && !ll {
                    sole_low += 1;
                    if first_low_witness.is_none() {
                        first_low_witness = Some((i, j));
                    }
                }
            }

            // sign-domain isolation: same radix, adjustment, bias; oracle says yes,
            // product order on (precision, exponent, domain) says no, or vice versa
            if s.r == t.r && s.a == t.a && s.b == t.b {
                if cd && !o {
                    decl_slice_fp += 1;
                    if first_signdom_witness.is_none() && s.dom != t.dom {
                        first_signdom_witness = Some((i, j));
                    }
                }
                if !cd && o {
                    decl_slice_fn += 1;
                }
            }
        }
    }

    println!(
        "\noracle: {} of {} ordered pairs include",
        n_incl,
        g.len() * g.len()
    );
    println!("\ncondition set            false-positive  false-negative");
    println!("C_two  (grid, top)       {:>14}  {:>14}", two.0, two.1);
    println!("C_four (grid,phase,lo,hi){:>14}  {:>14}", four.0, four.1);
    println!("C_lat  (lattice, range)  {:>14}  {:>14}", lat.0, lat.1);
    println!("C_decl (top,grid,dom)     {:>14}  {:>14}", decl.0, decl.1);
    println!("C_pure (prec,exp,dom)     {:>14}  {:>14}", pure.0, pure.1);
    if let Some((i, j)) = first_pure_fp {
        println!(
            "\npure-product-order false positive (it claims an inclusion the oracle refuses):"
        );
        show(&g[i], "  source");
        show(&g[j], "  target");
    }

    println!(
        "\nof C_two's false positives: {} are sole phase failures, {} are sole lower-range failures",
        sole_phase, sole_low
    );

    if let Some((i, j)) = first_phase_witness {
        let (s, t) = (&g[i], &g[j]);
        println!("\nphase witness (finer grid, containing range, no shared value):");
        show(s, "  source");
        show(t, "  target");
        let sv = s.values();
        let tv = t.values();
        let shared = sv
            .iter()
            .filter(|x| tv.binary_search_by(|y| y.cmp_r(**x)).is_ok())
            .count();
        println!(
            "  source quantum {:?}  target quantum {:?}  ratio integral: {}",
            s.q(),
            t.q(),
            s.q().div(t.q()).is_int()
        );
        println!(
            "  source range [{:?},{:?}] target range [{:?},{:?}]",
            s.vmin(),
            s.vmax(),
            t.vmin(),
            t.vmax()
        );
        println!(
            "  source has {} values, {} of them are in the target",
            sv.len(),
            shared
        );
    }

    if let Some((i, j)) = first_low_witness {
        println!("\nlower-range witness (C_two checks only the top):");
        show(&g[i], "  source");
        show(&g[j], "  target");
    }

    println!(
        "\nsame radix/adjustment/bias slice: product order has {} false positives, {} false negatives",
        decl_slice_fp, decl_slice_fn
    );
    if let Some((i, j)) = first_signdom_witness {
        let (s, t) = (&g[i], &g[j]);
        println!("\nsign-domain witness (product order says yes, oracle says no):");
        show(s, "  source");
        show(t, "  target");
        println!("  source vmin {:?}  target vmin {:?}", s.vmin(), t.vmin());
    }

    // the sign domain is NOT independent of precision: exhibit a pair where the
    // domain goes down and the inclusion still holds because precision went up
    let mut cross = 0usize;
    let mut first_cross: Option<(usize, usize)> = None;
    for (i, s) in g.iter().enumerate() {
        for (j, t) in g.iter().enumerate() {
            if s.r == t.r && s.a == t.a && s.b == t.b && dom_rank(s.dom) > dom_rank(t.dom) {
                if oracle_includes(s, t) {
                    cross += 1;
                    if first_cross.is_none() {
                        first_cross = Some((i, j));
                    }
                }
            }
        }
    }
    println!(
        "\npairs where the sign domain strictly DECREASES and inclusion still holds: {}",
        cross
    );
    if let Some((i, j)) = first_cross {
        show(&g[i], "  source");
        show(&g[j], "  target");
    }
}

fn show(n: &Num, tag: &str) {
    println!(
        "{}: r={} p={} e={} A={}/{} B={}/{} dom={:?}  values [{:?} .. {:?}] step {:?}",
        tag,
        n.r,
        n.p,
        n.e,
        n.a.n,
        n.a.d,
        n.b.n,
        n.b.d,
        n.dom,
        n.vmin(),
        n.vmax(),
        n.q()
    );
}
