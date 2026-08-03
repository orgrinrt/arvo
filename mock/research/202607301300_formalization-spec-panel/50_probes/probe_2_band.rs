//! Probe 2: the overflow band, for the float member that was struck, and a closed form for
//! the dividing-quantum condition file 44 left as "a proof owed, not yet built".
//!
//! rustc --edition 2021 -O probe_2_band.rs -o /tmp/p2 && /tmp/p2
//!
//! The band is the region where round-first and classify-first disagree: exact results
//! strictly above the largest representable value but strictly below the midpoint between it
//! and the next point of the unbounded-above grid. Round-first delivers max; classify-first
//! would refuse or saturate on range before rounding ever happens.
//!
//! The claim under test is a single criterion covering every member of the original sentence:
//!
//!   For an operation whose exact results lie on a lattice L (a subgroup of the rationals),
//!   the band is empty iff  q_result <= 2 * L,  where L is the lattice's generator.
//!
//! Division's exact results are not lattice-valued, so the criterion has nothing to say about
//! it and reachability decides, which is why file 43 had to compile that case.

#[path = "model.rs"]
mod model;
use model::*;

// ---------------------------------------------------------------------------
// part 1: the float member, exhaustive at the model width
// ---------------------------------------------------------------------------

fn float_band(f: &Fmt, label: &str) {
    let pos = f.positives();
    let mut all: Vec<Dyadic> = Vec::new();
    for d in &pos {
        all.push(*d);
        all.push(Dyadic { neg: true, ..*d });
    }
    all.push(Dyadic::zero(false));
    all.push(Dyadic::zero(true));

    let (maxm, maxq) = f.max_finite();
    let mut add_band = 0u64;
    let mut mul_band = 0u64;
    let mut div_band = 0u64;
    let mut add_pairs = 0u64;
    let mut add_witness: Option<(Dyadic, Dyadic)> = None;

    for &a in &all {
        for &b in &all {
            // add
            let r = exact_add(a, b);
            let g = quantize(f, &r, Dir::Nearest);
            add_pairs += 1;
            if in_band(&r, maxm, maxq, &g) {
                add_band += 1;
                if add_witness.is_none() {
                    add_witness = Some((a, b));
                }
            }
            // mul
            let r = exact_mul(a, b);
            let g = quantize(f, &r, Dir::Nearest);
            if in_band(&r, maxm, maxq, &g) {
                mul_band += 1;
            }
            // div
            if !b.is_zero() && !a.is_zero() {
                let r = exact_div(a, b);
                let g = quantize(f, &r, Dir::Nearest);
                if in_band(&r, maxm, maxq, &g) {
                    div_band += 1;
                }
            }
        }
    }
    println!(
        "{label}: p={} emin={} emax={} | {} finite values, {add_pairs} ordered pairs",
        f.p,
        f.emin,
        f.emax,
        all.len()
    );
    println!("  band inhabited: add {add_band}, mul {mul_band}, div {div_band}");
    if let Some((a, b)) = add_witness {
        println!(
            "  first add witness: {} + {} = exact {} > max {}",
            a.as_f64(),
            b.as_f64(),
            a.as_f64() + b.as_f64(),
            (maxm as f64) * (2f64).powi(maxq)
        );
    }
}

/// The exact result exceeds max finite in magnitude, yet the delivered result is finite.
fn in_band(r: &Rat, maxm: u128, maxq: i32, g: &Graded) -> bool {
    if r.num == 0 {
        return false;
    }
    match g.out {
        Outcome::Finite(d) => d.mag == maxm && d.scale == maxq && exceeds(r, maxm, maxq),
        _ => false,
    }
}

fn exceeds(r: &Rat, mag: u128, scale: i32) -> bool {
    let k = r.scale - scale;
    let (lhs, rhs) = if k >= 0 {
        (r.num << (k as u32), mag * r.den)
    } else {
        (r.num, (mag * r.den) << ((-k) as u32))
    };
    lhs > rhs
}

// ---------------------------------------------------------------------------
// part 2: fixed-point, rational quanta, predicted against measured
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q {
    n: i128,
    d: i128,
}

fn g(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        g(b, a % b)
    }
}

impl Q {
    fn new(n: i128, d: i128) -> Q {
        let k = g(n.abs(), d.abs());
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
    fn cmp(self, o: Q) -> std::cmp::Ordering {
        (self.n * o.d).cmp(&(o.n * self.d))
    }
    /// gcd of two positive rationals: the generator of the group they jointly generate.
    fn gcd(self, o: Q) -> Q {
        Q::new(g(self.n * o.d, o.n * self.d), self.d * o.d)
    }
}

/// Exhaustively: does any sum of two in-range grid points land strictly inside
/// (max_r, max_r + q_r/2)?
fn measured_band_add(q1: Q, n1: i128, q2: Q, n2: i128, qr: Q, nr: i128) -> bool {
    let maxr = qr.scale(nr);
    let top = maxr.add(qr.scale(1).mul(Q::new(1, 2)));
    for i in 0..=n1 {
        for j in 0..=n2 {
            let s = q1.scale(i).add(q2.scale(j));
            if s.cmp(maxr) == std::cmp::Ordering::Greater && s.cmp(top) == std::cmp::Ordering::Less
            {
                return true;
            }
        }
    }
    false
}

fn measured_band_mul(q1: Q, n1: i128, q2: Q, n2: i128, qr: Q, nr: i128) -> bool {
    let maxr = qr.scale(nr);
    let top = maxr.add(qr.mul(Q::new(1, 2)));
    for i in 0..=n1 {
        for j in 0..=n2 {
            let s = q1.scale(i).mul(q2.scale(j));
            if s.cmp(maxr) == std::cmp::Ordering::Greater && s.cmp(top) == std::cmp::Ordering::Less
            {
                return true;
            }
        }
    }
    false
}

/// predicted: band empty iff q_result <= 2 * lattice
fn predicted_empty(lattice: Q, qr: Q) -> bool {
    qr.cmp(lattice.scale(2)) != std::cmp::Ordering::Greater
}

fn main() {
    println!("== part 1: the float member, exhaustive at model width ==");
    float_band(&MODEL, "MODEL");
    float_band(
        &Fmt {
            p: 3,
            emin: -2,
            emax: 3,
            u: Underflow::Gradual,
        },
        "tiny",
    );
    float_band(
        &Fmt {
            p: 5,
            emin: -4,
            emax: 5,
            u: Underflow::Gradual,
        },
        "wider",
    );
    // a format with one binade is a fixed-point numeral wearing a float's clothes
    float_band(
        &Fmt {
            p: 4,
            emin: 0,
            emax: 0,
            u: Underflow::Abrupt,
        },
        "single-binade",
    );

    println!("\n== part 2: fixed-point, predicted against measured ==");
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
    ];
    let mut checked = 0;
    let mut agree = 0;
    let mut disagree = Vec::new();
    for &q1 in &quanta {
        for &q2 in &quanta {
            for &qr in &quanta {
                let (n1, n2, nr) = (15i128, 15, 15);
                // addition: exact sums lie on gcd(q1, q2)
                let lat = q1.gcd(q2);
                let pred = predicted_empty(lat, qr);
                let meas = !measured_band_add(q1, n1, q2, n2, qr, nr);
                checked += 1;
                if pred == meas {
                    agree += 1;
                } else {
                    disagree.push((q1, q2, qr, lat, pred, meas));
                }
            }
        }
    }
    println!("addition: {agree}/{checked} triples agree with the criterion");
    for d in disagree.iter().take(10) {
        println!(
            "  DISAGREE q1={:?} q2={:?} qr={:?} lat={:?} pred_empty={} meas_empty={}",
            d.0, d.1, d.2, d.3, d.4, d.5
        );
    }

    let mut checked = 0;
    let mut agree = 0;
    let mut dis2 = Vec::new();
    for &q1 in &quanta {
        for &q2 in &quanta {
            for &qr in &quanta {
                // multiplication: exact products lie on q1*q2
                let lat = q1.mul(q2);
                let pred = predicted_empty(lat, qr);
                let meas = !measured_band_mul(q1, 15, q2, 15, qr, 15);
                checked += 1;
                if pred == meas {
                    agree += 1;
                } else {
                    dis2.push((q1, q2, qr, lat, pred, meas));
                }
            }
        }
    }
    println!("multiplication: {agree}/{checked} triples agree with the criterion");
    for d in dis2.iter().take(10) {
        println!(
            "  DISAGREE q1={:?} q2={:?} qr={:?} lat={:?} pred_empty={} meas_empty={}",
            d.0, d.1, d.2, d.3, d.4, d.5
        );
    }

    println!("\n== part 3: the criterion read on the four members of the struck sentence ==");
    let q = Q::new(1, 8);
    println!(
        "same-format add:   lattice {:?}, q_r {:?} -> empty {}",
        q,
        q,
        predicted_empty(q, q)
    );
    println!(
        "same-format mul:   lattice {:?}, q_r {:?} -> empty {}",
        q.mul(q),
        q,
        predicted_empty(q.mul(q), q)
    );
    let (qa, qb) = (Q::new(1, 8), Q::new(1, 4));
    println!(
        "mixed add, dividing quanta: lattice {:?}, q_r {:?} -> empty {}",
        qa.gcd(qb),
        qa,
        predicted_empty(qa.gcd(qb), qa)
    );
    let (qa, qb) = (Q::new(1, 3), Q::new(1, 8));
    println!(
        "mixed add, non-dividing:    lattice {:?}, q_r {:?} -> empty {}",
        qa.gcd(qb),
        qa,
        predicted_empty(qa.gcd(qb), qa)
    );
    let (qf, qc) = (Q::new(1, 8), Q::new(1, 1));
    println!(
        "float add across binades:   lattice {:?}, q_r {:?} -> empty {}",
        qf.gcd(qc),
        qc,
        predicted_empty(qf.gcd(qc), qc)
    );
}
