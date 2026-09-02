//! Probe 2b: the corrected closed form for the overflow band.
//!
//! rustc --edition 2021 -O probe_2b_band_closed_form.rs -o /tmp/p2b && /tmp/p2b
//!
//! Probe 2 proposed `q_r <= 2 * lattice` and measurement refused it, 753/1000 for addition
//! and 639/1000 for multiplication, with failures in both directions. The two failure modes
//! were informative:
//!
//!   ALIGNMENT. q1=1, q2=1/2, q_r=3/4. Lattice 1/2, so the criterion says empty. But
//!   max_r = 15 * 3/4 = 11.25 is not on the lattice, and 11.5 sits inside
//!   (11.25, 11.625). The interval's POSITION matters, not only its width. The original
//!   criterion silently assumed max_r is a lattice point, which is true for every dyadic
//!   case the review had compiled and false in general.
//!
//!   REACHABILITY. q1=1, q2=1/4, q_r=1, multiplication. 15.25 is on the lattice and inside
//!   the interval, but 15.25 = 61/4 needs a product of indices equal to 61, a prime above
//!   15*15's reachable set. Products do not cover their lattice.
//!
//! The corrected statement is two clauses, and only the first has a closed form:
//!
//!   LATTICE CLAUSE. The band is empty unless some point of the exact-result set's lattice
//!   lies strictly inside (max_r, max_r + q_r/2). For an operation whose exact results form
//!   a subgroup, this is decidable in closed form from the three quanta alone.
//!
//!   REACHABILITY CLAUSE. That point must be an actual exact result of two in-range
//!   operands. Addition's results fill their lattice up to max1+max2, so the lattice clause
//!   decides. Multiplication's and division's do not, so enumeration decides, which is
//!   exactly why files 43 and 44 had to compile those two members rather than derive them.

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q {
    n: i128,
    d: i128,
}

fn gcd_i(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        gcd_i(b, a % b)
    }
}

impl Q {
    fn new(n: i128, d: i128) -> Q {
        let k = gcd_i(n.abs(), d.abs()).max(1);
        Q { n: n / k, d: d / k }
    }
    fn mul(self, o: Q) -> Q {
        Q::new(self.n * o.n, self.d * o.d)
    }
    fn scale(self, k: i128) -> Q {
        Q::new(self.n * k, self.d)
    }
    fn add(self, o: Q) -> Q {
        Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    fn cmp(self, o: Q) -> Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
    fn gcd(self, o: Q) -> Q {
        Q::new(gcd_i(self.n * o.d, o.n * self.d), self.d * o.d)
    }
    fn f(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

/// Is there a multiple of `lat` strictly inside (lo, hi), and at most `cap`?
fn lattice_point_in(lat: Q, lo: Q, hi: Q, cap: Q) -> bool {
    // smallest k with k*lat > lo  is  floor(lo/lat) + 1
    let num = lo.n * lat.d;
    let den = lo.d * lat.n;
    let k = num.div_euclid(den) + 1;
    let p = lat.scale(k);
    p.cmp(hi) == Ordering::Less && p.cmp(cap) != Ordering::Greater
}

fn predicted_add(q1: Q, n1: i128, q2: Q, n2: i128, qr: Q, nr: i128) -> bool {
    let maxr = qr.scale(nr);
    let hi = maxr.add(qr.mul(Q::new(1, 2)));
    let cap = q1.scale(n1).add(q2.scale(n2));
    lattice_point_in(q1.gcd(q2), maxr, hi, cap)
}

fn measured_add(q1: Q, n1: i128, q2: Q, n2: i128, qr: Q, nr: i128) -> bool {
    let maxr = qr.scale(nr);
    let hi = maxr.add(qr.mul(Q::new(1, 2)));
    for i in 0..=n1 {
        for j in 0..=n2 {
            let s = q1.scale(i).add(q2.scale(j));
            if s.cmp(maxr) == Ordering::Greater && s.cmp(hi) == Ordering::Less {
                return true;
            }
        }
    }
    false
}

fn measured_mul(q1: Q, n1: i128, q2: Q, n2: i128, qr: Q, nr: i128) -> bool {
    let maxr = qr.scale(nr);
    let hi = maxr.add(qr.mul(Q::new(1, 2)));
    for i in 0..=n1 {
        for j in 0..=n2 {
            let s = q1.scale(i).mul(q2.scale(j));
            if s.cmp(maxr) == Ordering::Greater && s.cmp(hi) == Ordering::Less {
                return true;
            }
        }
    }
    false
}

fn predicted_mul_upper(q1: Q, n1: i128, q2: Q, n2: i128, qr: Q, nr: i128) -> bool {
    let maxr = qr.scale(nr);
    let hi = maxr.add(qr.mul(Q::new(1, 2)));
    let cap = q1.scale(n1).mul(q2.scale(n2));
    lattice_point_in(q1.mul(q2), maxr, hi, cap)
}

fn main() {
    let quanta = [
        Q::new(1, 1),
        Q::new(1, 2),
        Q::new(1, 4),
        Q::new(1, 8),
        Q::new(1, 3),
        Q::new(1, 6),
        Q::new(2, 3),
        Q::new(3, 4),
        Q::new(1, 5),
        Q::new(5, 6),
        Q::new(7, 8),
        Q::new(2, 5),
    ];
    let sizes = [7i128, 15, 31];

    let (mut n, mut ok) = (0u32, 0u32);
    let mut unders = 0u32;
    let mut bad = Vec::new();
    for &q1 in &quanta {
        for &q2 in &quanta {
            for &qr in &quanta {
                for &sz in &sizes {
                    let p = predicted_add(q1, sz, q2, sz, qr, sz);
                    let m = measured_add(q1, sz, q2, sz, qr, sz);
                    n += 1;
                    if p == m {
                        ok += 1;
                    } else {
                        if !p && m {
                            unders += 1;
                        }
                        bad.push((q1, q2, qr, sz, p, m));
                    }
                }
            }
        }
    }
    println!("ADDITION, lattice clause against exhaustive measurement: {ok}/{n} agree");
    println!(
        "  of the {} disagreements, {unders} are under-predictions (claimed empty, was inhabited)",
        bad.len()
    );
    for b in bad.iter().take(8) {
        println!(
            "  DISAGREE q1={} q2={} qr={} n={} predicted={} measured={}",
            b.0.f(),
            b.1.f(),
            b.2.f(),
            b.3,
            b.4,
            b.5
        );
    }

    let (mut n, mut ok, mut over, mut under) = (0u32, 0u32, 0u32, 0u32);
    for &q1 in &quanta {
        for &q2 in &quanta {
            for &qr in &quanta {
                for &sz in &sizes {
                    let p = predicted_mul_upper(q1, sz, q2, sz, qr, sz);
                    let m = measured_mul(q1, sz, q2, sz, qr, sz);
                    n += 1;
                    if p == m {
                        ok += 1;
                    } else if p && !m {
                        over += 1;
                    } else {
                        under += 1;
                    }
                }
            }
        }
    }
    println!(
        "MULTIPLICATION, lattice clause as an upper bound: {ok}/{n} exact, \
         {over} over-predicted (lattice point unreachable), {under} under-predicted"
    );
    println!("  under-predicted must be 0 for the clause to be a sound upper bound: {under}");

    println!("\n== the criterion read on every member of the sentence ==");
    let rows: [(&str, Q, Q, Q, i128); 6] = [
        (
            "fixed same-format add",
            Q::new(1, 8),
            Q::new(1, 8),
            Q::new(1, 8),
            15,
        ),
        (
            "fixed same-format mul",
            Q::new(1, 8),
            Q::new(1, 8),
            Q::new(1, 8),
            15,
        ),
        (
            "fixed mixed add, dividing quanta",
            Q::new(1, 8),
            Q::new(1, 4),
            Q::new(1, 8),
            15,
        ),
        (
            "fixed mixed add, non-dividing",
            Q::new(1, 3),
            Q::new(1, 8),
            Q::new(1, 8),
            15,
        ),
        (
            "float add, top binade only",
            Q::new(1, 1),
            Q::new(1, 1),
            Q::new(1, 1),
            15,
        ),
        (
            "float add, one operand 3 binades down",
            Q::new(1, 8),
            Q::new(1, 1),
            Q::new(1, 1),
            15,
        ),
    ];
    for (name, q1, q2, qr, sz) in rows {
        let lat = if name.contains("mul") {
            q1.mul(q2)
        } else {
            q1.gcd(q2)
        };
        let maxr = qr.scale(sz);
        let hi = maxr.add(qr.mul(Q::new(1, 2)));
        let inhabited = if name.contains("mul") {
            measured_mul(q1, sz, q2, sz, qr, sz)
        } else {
            measured_add(q1, sz, q2, sz, qr, sz)
        };
        println!(
            "{name:34} lattice {:6.4}  band ({:.4}, {:.4})  inhabited {}",
            lat.f(),
            maxr.f(),
            hi.f(),
            inhabited
        );
    }
}
