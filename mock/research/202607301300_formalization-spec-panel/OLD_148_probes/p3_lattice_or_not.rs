// p3: does the numeral family, ordered by value-set inclusion, have binary meets and joins?
//
// The naive way to answer this is to search a finite grid of numerals and ask whether the set
// of lower bounds has a unique maximum inside that grid.  That answer is worthless in both
// directions: a unique maximum inside the grid may be beaten by a numeral outside it, and an
// absent maximum inside the grid may be supplied by one.  So this probe does not do that.
//
// It uses the one structural fact that makes the question decidable.  A lower bound of N1 and N2
// has a value set contained in the finite set V1 cap V2, so the lower bounds can be enumerated
// COMPLETELY, over every radix, precision, exponent, adjustment, bias and sign domain, because
// each of those is pinned by the finite set it has to sit inside.  The same enumerator settles
// the join once two upper bounds are in hand, since a least upper bound must sit inside their
// intersection.
//
// The design statements this is built from, all at 124 section 1.2:
//   Radix is `Rad<P>` over the sealed `Pos`, bounded by `AtLeastTwo`, so every integer radix
//     from two upward is nameable and nothing below two is.
//   `Bias` and `Adjustment` are signed, gcd-normalised rationals, value-unique and sealed.
//   The `Adjustment` constructors are `Unit` and `FullRange<F>` (124:1638).
//   `Domain` is `NonNegative | Symmetric | AsymmetricLow`.
//
// Run: rustc +nightly-2026-05-28 -O p3_lattice_or_not.rs -o /tmp/p3 && /tmp/p3

use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
        Rat::new(self.n * o.d, self.d * o.n)
    }
    fn key(self) -> (i128, i128) {
        (self.n, self.d)
    }
    fn lt(self, o: Rat) -> bool {
        self.n * o.d < o.n * self.d
    }
    fn cmp_num(self, o: Rat) -> core::cmp::Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dom {
    NonNegative,
    Symmetric,
    AsymmetricLow,
}

#[derive(Clone, Copy, Debug)]
struct Num {
    r: i128,
    p: u32,
    e: i32,
    a: Rat,
    b: Rat,
    dom: Dom,
}

fn ipow(r: i128, p: u32) -> i128 {
    r.checked_pow(p).unwrap()
}

impl Num {
    fn count(&self) -> i128 {
        let m = ipow(self.r, self.p);
        match self.dom {
            Dom::NonNegative => m,
            Dom::Symmetric => 2 * m - 1,
            Dom::AsymmetricLow => 2 * m,
        }
    }
    fn kmin(&self) -> i128 {
        let m = ipow(self.r, self.p);
        match self.dom {
            Dom::NonNegative => 0,
            Dom::Symmetric => -(m - 1),
            Dom::AsymmetricLow => -m,
        }
    }
    fn q(&self) -> Rat {
        let mut v = self.a;
        if self.e >= 0 {
            v = v.mul(Rat::int(ipow(self.r, self.e as u32)));
        } else {
            v = v.div(Rat::int(ipow(self.r, (-self.e) as u32)));
        }
        v
    }
    fn values(&self) -> Vec<Rat> {
        let q = self.q();
        let k0 = self.kmin();
        (0..self.count())
            .map(|i| q.mul(Rat::int(k0 + i)).add(self.b))
            .collect()
    }
    fn value_key(&self) -> BTreeSet<(i128, i128)> {
        self.values().into_iter().map(|r| r.key()).collect()
    }
}

// Is `q` expressible as Adjustment * radix^exponent for this radix?
// Adjustment constructors: Unit, and FullRange<F> = r^F / (r^F - 1).
fn express(q: Rat, r: i128, fmax: u32) -> Option<(Rat, i32)> {
    let mut adjustments = vec![Rat::int(1)];
    for f in 1..=fmax {
        let rf = ipow(r, f);
        adjustments.push(Rat::new(rf, rf - 1));
    }
    for a in adjustments {
        // q / a must be r^e for an integer e
        let mut t = q.div(a);
        if t.n <= 0 {
            continue;
        }
        let mut e: i32 = 0;
        // divide down
        while t.n % r == 0 && t.d == 1 {
            t = Rat::new(t.n / r, 1);
            e += 1;
        }
        // multiply up
        while t.d % r == 0 {
            t = Rat::new(t.n, t.d / r);
            e -= 1;
        }
        if t == Rat::int(1) {
            return Some((a, e));
        }
    }
    None
}

// Every numeral whose value set is a subset of `s`.  Complete: the finiteness of `s`
// pins the count (hence the radix and precision), the quantum, and the bias.
fn numerals_inside(s: &[Rat], fmax: u32) -> Vec<Num> {
    let set: BTreeSet<(i128, i128)> = s.iter().map(|r| r.key()).collect();
    let n_max = s.len() as i128;
    let mut out = Vec::new();
    for &dom in &[Dom::NonNegative, Dom::Symmetric, Dom::AsymmetricLow] {
        for r in 2..=(n_max.max(2)) {
            for p in 0u32..=32 {
                let m = match r.checked_pow(p) {
                    Some(v) if v <= n_max * 2 + 2 => v,
                    _ => break,
                };
                let n = match dom {
                    Dom::NonNegative => m,
                    Dom::Symmetric => 2 * m - 1,
                    Dom::AsymmetricLow => 2 * m,
                };
                if n > n_max || n < 1 {
                    continue;
                }
                for &v0 in s.iter() {
                    if n == 1 {
                        // a single point; quantum is unconstrained by the set, so pin it to
                        // the smallest expressible one and record the numeral once
                        if let Some((a, e)) = express(Rat::int(1), r, fmax) {
                            let k0 = match dom {
                                Dom::NonNegative => 0,
                                Dom::Symmetric => -(m - 1),
                                Dom::AsymmetricLow => -m,
                            };
                            let q = Rat::int(1);
                            let b = v0.sub(q.mul(Rat::int(k0)));
                            out.push(Num { r, p, e, a, b, dom });
                            let _ = b;
                        }
                        continue;
                    }
                    for &v1 in s.iter() {
                        if !v0.lt(v1) {
                            continue;
                        }
                        let q = v1.sub(v0);
                        // the whole progression must lie in the set
                        let mut ok = true;
                        for i in 0..n {
                            let v = v0.add(q.mul(Rat::int(i)));
                            if !set.contains(&v.key()) {
                                ok = false;
                                break;
                            }
                        }
                        if !ok {
                            continue;
                        }
                        if let Some((a, e)) = express(q, r, fmax) {
                            let k0 = match dom {
                                Dom::NonNegative => 0,
                                Dom::Symmetric => -(m - 1),
                                Dom::AsymmetricLow => -m,
                            };
                            let b = v0.sub(q.mul(Rat::int(k0)));
                            let cand = Num { r, p, e, a, b, dom };
                            debug_assert_eq!(cand.count(), n);
                            out.push(cand);
                        }
                    }
                }
            }
        }
    }
    out
}

fn subset(a: &BTreeSet<(i128, i128)>, b: &BTreeSet<(i128, i128)>) -> bool {
    a.iter().all(|x| b.contains(x))
}

fn report(tag: &str, n1: &Num, n2: &Num, fmax: u32) {
    let v1 = n1.value_key();
    let v2 = n2.value_key();
    let inter: Vec<Rat> = v1.intersection(&v2).map(|&(n, d)| Rat { n, d }).collect();
    let union: Vec<Rat> = v1.union(&v2).map(|&(n, d)| Rat { n, d }).collect();
    let mut inter_s = inter.clone();
    inter_s.sort_by(|a, b| a.cmp_num(*b));
    let mut union_s = union.clone();
    union_s.sort_by(|a, b| a.cmp_num(*b));

    println!("\n== {} ==", tag);
    println!("  N1 |V|={} values {:?}", v1.len(), fmt(&v1));
    println!("  N2 |V|={} values {:?}", v2.len(), fmt(&v2));
    println!(
        "  intersection |{}|  union |{}|",
        inter_s.len(),
        union_s.len()
    );

    // MEET: every lower bound sits inside the intersection, so enumerate them all.
    let lowers = numerals_inside(&inter_s, fmax);
    let mut lkeys: Vec<BTreeSet<(i128, i128)>> = lowers.iter().map(|n| n.value_key()).collect();
    lkeys.sort();
    lkeys.dedup();
    // maximal ones
    let maximal: Vec<&BTreeSet<(i128, i128)>> = lkeys
        .iter()
        .filter(|x| !lkeys.iter().any(|y| *y != **x && subset(x, y)))
        .collect();
    println!(
        "  lower bounds: {} distinct value sets, {} maximal",
        lkeys.len(),
        maximal.len()
    );
    if maximal.len() == 1 {
        println!("  MEET EXISTS, |V| = {}", maximal[0].len());
    } else {
        println!("  NO MEET. the maximal lower bounds are incomparable:");
        for m in maximal.iter().take(4) {
            println!("    {:?}", fmt(m));
        }
    }

    // JOIN: enumerate every numeral containing the union, bounded, and take the minimal ones.
    let uppers = numerals_outside(&union_s, fmax, 64, 6);
    let minimal: Vec<&BTreeSet<(i128, i128)>> = uppers
        .iter()
        .filter(|x| !uppers.iter().any(|y| *y != **x && subset(y, x)))
        .collect();
    println!(
        "  upper bounds found (count <= 64, quantum refined <= 6x): {}, minimal: {}",
        uppers.len(),
        minimal.len()
    );
    if minimal.len() == 1 {
        let m = minimal[0];
        println!(
            "  JOIN EXISTS, |V| = {}, overshoot = {} points beyond the union",
            m.len(),
            m.len() as i64 - union_s.len() as i64
        );
    } else if minimal.is_empty() {
        println!("  no upper bound inside the searched region; inconclusive on the join");
    } else {
        let mut sizes: Vec<usize> = minimal.iter().map(|m| m.len()).collect();
        sizes.sort();
        sizes.dedup();
        println!(
            "  NO JOIN: {} incomparable minimal upper bounds, sizes {:?}, union has {}",
            minimal.len(),
            sizes,
            union_s.len()
        );
        let smallest = minimal.iter().min_by_key(|m| m.len()).unwrap();
        println!(
            "    smallest minimal upper bound, {} points: {:?}",
            smallest.len(),
            fmt(smallest)
        );
        // a least upper bound would have to sit inside EVERY upper bound, hence inside the
        // smallest one, hence inside the searched region.  Two incomparable minimal ones
        // therefore settle it rather than merely suggesting it.
        let second = minimal
            .iter()
            .find(|m| !subset(smallest, m) && !subset(m, smallest))
            .unwrap();
        println!(
            "    an incomparable sibling, {} points: {:?}",
            second.len(),
            fmt(second)
        );
    }
}

// Every numeral whose value set CONTAINS the finite set `s`, with quantum a unit fraction of
// the lattice `s` generates and count at most `nmax`.  Complete for that bounded region: the
// quantum is pinned by divisibility, and the requirement to cover `s` pins the placement to
// finitely many offsets.
fn numerals_outside(
    s: &[Rat],
    fmax: u32,
    nmax: i128,
    refine_max: i128,
) -> Vec<BTreeSet<(i128, i128)>> {
    // g: the quantum of the affine lattice `s` generates
    let mut g = s[1].sub(s[0]);
    for i in 1..s.len() {
        g = rat_gcd(g, s[i].sub(s[0]));
    }
    let lo = s[0];
    let hi = *s.last().unwrap();
    let mut out = Vec::new();
    let sset: BTreeSet<(i128, i128)> = s.iter().map(|r| r.key()).collect();
    for refine in 1..=refine_max {
        let q = g.div(Rat::int(refine));
        // the covering AP must span at least [lo, hi] on quantum q
        let span = hi.sub(lo).div(q);
        if span.d != 1 {
            continue;
        }
        let need = span.n + 1;
        if need > nmax {
            continue;
        }
        for &dom in &[Dom::NonNegative, Dom::Symmetric, Dom::AsymmetricLow] {
            for r in 2..=nmax {
                let mut p = 0u32;
                loop {
                    let m = match r.checked_pow(p) {
                        Some(v) if v <= nmax * 2 => v,
                        _ => break,
                    };
                    let n = match dom {
                        Dom::NonNegative => m,
                        Dom::Symmetric => 2 * m - 1,
                        Dom::AsymmetricLow => 2 * m,
                    };
                    p += 1;
                    if n < need || n > nmax {
                        if n > nmax {
                            break;
                        }
                        continue;
                    }
                    let (a, e) = match express(q, r, fmax) {
                        Some(x) => x,
                        None => continue,
                    };
                    // slide the window: its least value v0 = lo - j*q for j in 0..=(n - need)
                    for j in 0..=(n - need) {
                        let v0 = lo.sub(q.mul(Rat::int(j)));
                        let k0 = match dom {
                            Dom::NonNegative => 0,
                            Dom::Symmetric => -(m - 1),
                            Dom::AsymmetricLow => -m,
                        };
                        let b = v0.sub(q.mul(Rat::int(k0)));
                        let cand = Num {
                            r,
                            p: p - 1,
                            e,
                            a,
                            b,
                            dom,
                        };
                        let vk = cand.value_key();
                        if subset(&sset, &vk) {
                            out.push(vk);
                        }
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn rat_gcd(a: Rat, b: Rat) -> Rat {
    // gcd of two positive rationals: gcd(n1,n2) / lcm(d1,d2)
    let (a, b) = (
        Rat {
            n: a.n.abs(),
            d: a.d,
        },
        Rat {
            n: b.n.abs(),
            d: b.d,
        },
    );
    if a.n == 0 {
        return b;
    }
    if b.n == 0 {
        return a;
    }
    let n = gcd(a.n * b.d, b.n * a.d);
    let d = a.d * b.d;
    Rat::new(n, d)
}

fn fmt(s: &BTreeSet<(i128, i128)>) -> Vec<String> {
    let mut v: Vec<Rat> = s.iter().map(|&(n, d)| Rat { n, d }).collect();
    v.sort_by(|a, b| a.cmp_num(*b));
    v.iter()
        .map(|r| {
            if r.d == 1 {
                format!("{}", r.n)
            } else {
                format!("{}/{}", r.n, r.d)
            }
        })
        .collect()
}

// A quiet version of the two questions, for sweeping.
fn meet_join(n1: &Num, n2: &Num, fmax: u32) -> (bool, usize) {
    let v1 = n1.value_key();
    let v2 = n2.value_key();
    let mut inter: Vec<Rat> = v1.intersection(&v2).map(|&(n, d)| Rat { n, d }).collect();
    inter.sort_by(|a, b| a.cmp_num(*b));
    let mut union_s: Vec<Rat> = v1.union(&v2).map(|&(n, d)| Rat { n, d }).collect();
    union_s.sort_by(|a, b| a.cmp_num(*b));

    let meet = if inter.is_empty() {
        false
    } else {
        let lowers = numerals_inside(&inter, fmax);
        let mut lk: Vec<BTreeSet<(i128, i128)>> = lowers.iter().map(|n| n.value_key()).collect();
        lk.sort();
        lk.dedup();
        lk.iter()
            .filter(|x| !lk.iter().any(|y| *y != **x && subset(x, y)))
            .count()
            == 1
    };
    let uppers = numerals_outside(&union_s, fmax, 48, 4);
    let nmin = uppers
        .iter()
        .filter(|x| !uppers.iter().any(|y| *y != **x && subset(y, x)))
        .count();
    // 1 = a least upper bound exists.  >1 = none exists, and this is decisive rather than
    // bounded-search noise: any lub would sit inside every upper bound, hence inside the
    // smallest one found, hence inside the searched region, hence would have been found.
    // 0 = the search found no upper bound at all, which is a bound on the search and not
    // a fact about the family.
    (meet, nmin)
}

fn sweep(tag: &str, g: &[Num], fmax: u32) {
    let mut n = 0;
    let mut no_meet = 0;
    let mut join_ok = 0;
    let mut no_join = 0;
    let mut join_unknown = 0;
    for i in 0..g.len() {
        for j in (i + 1)..g.len() {
            let (m, nmin) = meet_join(&g[i], &g[j], fmax);
            n += 1;
            if !m {
                no_meet += 1;
            }
            match nmin {
                0 => join_unknown += 1,
                1 => join_ok += 1,
                _ => no_join += 1,
            }
        }
    }
    println!(
        "\nSWEEP {}: {} unordered pairs. meet absent in {}. join present in {}, \
         absent in {} (decided), undetermined by the bounded search in {}.",
        tag, n, no_meet, join_ok, no_join, join_unknown
    );
}

fn main() {
    let fmax = 4;

    // A. the unbiased slice, which is what the two-condition reading was checked over.
    // Two fixed-point numerals sharing radix, adjustment and bias.
    let a1 = Num {
        r: 2,
        p: 3,
        e: 0,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    let a2 = Num {
        r: 2,
        p: 2,
        e: -1,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    report("A. unbiased, same radix, same sign domain", &a1, &a2, fmax);

    // B. unbiased, sign domains differing.
    let b1 = Num {
        r: 2,
        p: 3,
        e: 0,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    let b2 = Num {
        r: 2,
        p: 2,
        e: 0,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::AsymmetricLow,
    };
    report("B. unbiased, sign domains differing", &b1, &b2, fmax);

    // C. biased: two numerals on the same grid, offset by three quanta.
    let c1 = Num {
        r: 2,
        p: 3,
        e: 0,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    let c2 = Num {
        r: 2,
        p: 3,
        e: 0,
        a: Rat::int(1),
        b: Rat::int(-3),
        dom: Dom::NonNegative,
    };
    report(
        "C. biased, same grid, offset by three quanta",
        &c1,
        &c2,
        fmax,
    );

    // D. biased and off-phase: the phase condition's own witness shape.
    let d1 = Num {
        r: 2,
        p: 2,
        e: 0,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    let d2 = Num {
        r: 2,
        p: 2,
        e: 0,
        a: Rat::int(1),
        b: Rat::new(1, 3),
        dom: Dom::NonNegative,
    };
    report(
        "D. biased and off-phase, disjoint value sets",
        &d1,
        &d2,
        fmax,
    );

    // F. biased, on-phase, and the intersection has a cardinality no radix can name at that
    // quantum.  Six points at quantum one quarter: NonNegative needs r^p = 6 so r = 6, and
    // 1/4 is not 6^E nor FullRange at radix six; Symmetric needs 2r^p - 1 = 6, which is even;
    // AsymmetricLow needs r^p = 3 so r = 3, and 1/4 is not expressible at radix three.
    let f1 = Num {
        r: 2,
        p: 3,
        e: -2,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    let f2 = Num {
        r: 2,
        p: 3,
        e: -2,
        a: Rat::int(1),
        b: Rat::new(-1, 2),
        dom: Dom::NonNegative,
    };
    report(
        "F. biased, on-phase, unrealisable intersection cardinality",
        &f1,
        &f2,
        fmax,
    );

    // Sweeps.  The unbiased slice first, which is the one the two-condition reading was
    // checked over, then the same shapes with a bias applied.
    let mut unbiased = Vec::new();
    for p in 1u32..=3 {
        for e in -2i32..=0 {
            for dom in [Dom::NonNegative, Dom::Symmetric, Dom::AsymmetricLow] {
                unbiased.push(Num {
                    r: 2,
                    p,
                    e,
                    a: Rat::int(1),
                    b: Rat::int(0),
                    dom,
                });
            }
        }
    }
    sweep("unbiased, radix two, Unit adjustment", &unbiased, fmax);

    let mut biased = Vec::new();
    for p in 1u32..=3 {
        for e in -2i32..=0 {
            for b in [Rat::int(0), Rat::new(1, 2), Rat::new(1, 3)] {
                biased.push(Num {
                    r: 2,
                    p,
                    e,
                    a: Rat::int(1),
                    b,
                    dom: Dom::NonNegative,
                });
            }
        }
    }
    sweep("biased, radix two, NonNegative", &biased, fmax);

    // E. a pair whose intersection has a cardinality no radix can name at a usable quantum.
    let e1 = Num {
        r: 2,
        p: 3,
        e: -2,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    let e2 = Num {
        r: 3,
        p: 2,
        e: -2,
        a: Rat::int(1),
        b: Rat::int(0),
        dom: Dom::NonNegative,
    };
    report("E. radices differing", &e1, &e2, fmax);
}
