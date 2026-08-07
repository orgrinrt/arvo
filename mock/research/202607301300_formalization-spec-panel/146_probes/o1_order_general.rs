//! The order, derived independently and then generalised.
//!
//! `145` states the order as componentwise on `(I, F)` and compiles it for the
//! unbiased dyadic family at one sign domain at a time. This rebuilds it from the
//! only thing a numeral's value set actually is, a finite arithmetic progression of
//! rationals, and asks where the componentwise reading stops holding.
//!
//! A numeral's value set: { b + q*j : 0 <= j < n }, b the low endpoint, q the
//! quantum (adjustment times radix to the exponent), n the count. Bias moves b off
//! zero; adjustment scales q.
//!
//!   rustc -O o1_order_general.rs -o o1_order_general && ./o1_order_general

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Rat {
    n: i128,
    d: i128,
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

impl Rat {
    fn new(n: i128, d: i128) -> Rat {
        assert!(d != 0);
        let s = if d < 0 { -1 } else { 1 };
        let (n, d) = (n * s, d * s);
        let g = gcd(n, d).max(1);
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
    fn le(self, o: Rat) -> bool {
        self.n * o.d <= o.n * self.d
    }
    fn is_int(self) -> bool {
        self.d == 1
    }
    fn pow2(e: i32) -> Rat {
        if e >= 0 {
            Rat::int(1i128 << e)
        } else {
            Rat::new(1, 1i128 << (-e))
        }
    }
}

/// A numeral, as its value set. Nothing about how it is stored.
#[derive(Clone, Copy, Debug)]
struct Num {
    b: Rat,
    q: Rat,
    n: i128,
}

impl Num {
    fn top(&self) -> Rat {
        self.b.add(self.q.mul(Rat::int(self.n - 1)))
    }
    fn values(&self) -> Vec<Rat> {
        (0..self.n)
            .map(|j| self.b.add(self.q.mul(Rat::int(j))))
            .collect()
    }
}

/// Inclusion, stated as the four conditions the value sets impose.
/// grid: the target's quantum divides the source's.
/// phase: the target's grid passes through the source's low endpoint.
/// low, high: the target's range covers the source's.
fn subset_by_conditions(a: &Num, b: &Num) -> bool {
    let grid = a.n < 2 || a.q.div(b.q).is_int();
    let phase = a.b.sub(b.b).div(b.q).is_int();
    let low = b.b.le(a.b);
    let high = a.top().le(b.top());
    grid && phase && low && high
}

/// The oracle: element by element.
fn subset_by_elements(a: &Num, b: &Num) -> bool {
    let bs: std::collections::HashSet<(i128, i128)> =
        b.values().into_iter().map(|r| (r.n, r.d)).collect();
    a.values().into_iter().all(|r| bs.contains(&(r.n, r.d)))
}

// ---------------------------------------------------------------- sign domains
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sign {
    NonNegative,
    Symmetric,
    AsymmetricLow,
}

/// A fixed-point numeral in the design's declared coordinates.
fn fixed(i: i32, f: i32, s: Sign) -> Num {
    let q = Rat::pow2(-f);
    let mag = 1i128 << (i + f); // count of steps in [0, 2^i)
    match s {
        Sign::NonNegative => Num {
            b: Rat::int(0),
            q,
            n: mag,
        },
        // [-(2^i - q), 2^i - q]
        Sign::Symmetric => Num {
            b: Rat::int(0).sub(Rat::pow2(i)).add(q),
            q,
            n: 2 * mag - 1,
        },
        // [-2^i, 2^i - q]
        Sign::AsymmetricLow => Num {
            b: Rat::int(0).sub(Rat::pow2(i)),
            q,
            n: 2 * mag,
        },
    }
}

fn sign_rank(s: Sign) -> u8 {
    match s {
        Sign::NonNegative => 0,
        Sign::Symmetric => 1,
        Sign::AsymmetricLow => 2,
    }
}

fn main() {
    // ---- A. the conditions agree with the elements, over every shape ----------
    let mut shapes = Vec::new();
    for i in 0..=5i32 {
        for f in 0..=5i32 {
            if i + f <= 5 {
                for s in [Sign::NonNegative, Sign::Symmetric, Sign::AsymmetricLow] {
                    shapes.push((i, f, s, fixed(i, f, s)));
                }
            }
        }
    }
    let mut oracle_mismatch = 0usize;
    let mut pairs = 0usize;
    for (_, _, _, a) in &shapes {
        for (_, _, _, b) in &shapes {
            pairs += 1;
            if subset_by_conditions(a, b) != subset_by_elements(a, b) {
                oracle_mismatch += 1;
            }
        }
    }
    println!("A. shapes {} ordered pairs {}", shapes.len(), pairs);
    println!("A. condition-vs-element mismatches {}", oracle_mismatch);

    // ---- B. is inclusion componentwise in the DECLARED coordinates? ----------
    // two sign domains only, which is the slice `145` compiled
    let two: Vec<_> = shapes
        .iter()
        .filter(|(_, _, s, _)| *s != Sign::Symmetric)
        .collect();
    let mut cw_fail_two = 0usize;
    for (i1, f1, s1, a) in &two {
        for (i2, f2, s2, b) in &two {
            let cw = i1 <= i2 && f1 <= f2 && sign_rank(*s1) <= sign_rank(*s2);
            if cw != subset_by_elements(a, b) {
                cw_fail_two += 1;
            }
        }
    }
    println!(
        "B. componentwise failures, two sign domains: {}",
        cw_fail_two
    );

    // all three sign domains
    let mut cw_fail_three = 0usize;
    let mut first: Option<String> = None;
    for (i1, f1, s1, a) in &shapes {
        for (i2, f2, s2, b) in &shapes {
            let cw = i1 <= i2 && f1 <= f2 && sign_rank(*s1) <= sign_rank(*s2);
            let real = subset_by_elements(a, b);
            if cw != real {
                cw_fail_three += 1;
                if first.is_none() && a.n >= 3 && b.n >= 3 {
                    first = Some(format!(
                        "Q{}.{} {:?} into Q{}.{} {:?}: componentwise says {}, inclusion is {}",
                        i1, f1, s1, i2, f2, s2, cw, real
                    ));
                }
            }
        }
    }
    println!(
        "B. componentwise failures, three sign domains: {}",
        cw_fail_three
    );
    if let Some(s) = first {
        println!("B. first counterexample: {}", s);
    }

    // ---- C. equal cardinality forces equality (the antichain, generally) -----
    // over a family with arbitrary bias and adjustment, not only fixed point
    let mut gen = Vec::new();
    for qn in 1..=3i128 {
        for qd in 1..=4i128 {
            for bn in -3..=3i128 {
                for bd in 1..=3i128 {
                    for n in 2..=5i128 {
                        gen.push(Num {
                            b: Rat::new(bn, bd),
                            q: Rat::new(qn, qd),
                            n,
                        });
                    }
                }
            }
        }
    }
    let mut eq_card_violations = 0usize;
    let mut eq_card_pairs = 0usize;
    for a in &gen {
        for b in &gen {
            if a.n == b.n {
                eq_card_pairs += 1;
                if subset_by_elements(a, b) {
                    let same = a.values() == b.values();
                    if !same {
                        eq_card_violations += 1;
                    }
                }
            }
        }
    }
    println!("C. numerals in the biased/adjusted family: {}", gen.len());
    println!(
        "C. equal-cardinality ordered pairs {} inclusion-without-equality {}",
        eq_card_pairs, eq_card_violations
    );

    // ---- D. does the meet exist in the family once bias is admitted? ---------
    let mut empty_meets = 0usize;
    let mut meet_pairs = 0usize;
    let mut first_empty: Option<String> = None;
    for a in &gen {
        for b in &gen {
            meet_pairs += 1;
            let av: std::collections::HashSet<(i128, i128)> =
                a.values().into_iter().map(|r| (r.n, r.d)).collect();
            let bv: std::collections::HashSet<(i128, i128)> =
                b.values().into_iter().map(|r| (r.n, r.d)).collect();
            if av.intersection(&bv).count() == 0 {
                empty_meets += 1;
                if first_empty.is_none() {
                    first_empty = Some(format!(
                        "b={}/{} q={}/{} n={}  meet  b={}/{} q={}/{} n={}  is empty",
                        a.b.n, a.b.d, a.q.n, a.q.d, a.n, b.b.n, b.b.d, b.q.n, b.q.d, b.n
                    ));
                }
            }
        }
    }
    println!(
        "D. ordered pairs {} with empty intersection {}",
        meet_pairs, empty_meets
    );
    if let Some(s) = first_empty {
        println!("D. first: {}", s);
    }

    // the same question restricted to the unbiased dyadic slice
    let unb: Vec<Num> = shapes
        .iter()
        .filter(|(_, _, s, _)| *s == Sign::NonNegative)
        .map(|(_, _, _, n)| *n)
        .collect();
    let mut unb_empty = 0usize;
    for a in &unb {
        for b in &unb {
            let av: std::collections::HashSet<(i128, i128)> =
                a.values().into_iter().map(|r| (r.n, r.d)).collect();
            let bv: std::collections::HashSet<(i128, i128)> =
                b.values().into_iter().map(|r| (r.n, r.d)).collect();
            if av.intersection(&bv).count() == 0 {
                unb_empty += 1;
            }
        }
    }
    println!(
        "D. unbiased dyadic slice: pairs {} with empty intersection {}",
        unb.len() * unb.len(),
        unb_empty
    );

    // ---- E. equal precision, comparable: the Ranged counterexample -----------
    // significand precision p = 3 (mantissas 4..7), exponent ranges nested.
    let f = |elo: i32, ehi: i32| -> std::collections::HashSet<(i128, i128)> {
        let mut s = std::collections::HashSet::new();
        s.insert((0, 1));
        for m in 4..8i128 {
            for e in elo..=ehi {
                let v = Rat::int(m).mul(Rat::pow2(e));
                s.insert((v.n, v.d));
                let w = Rat::int(-m).mul(Rat::pow2(e));
                s.insert((w.n, w.d));
            }
        }
        s
    };
    let narrow = f(-2, 2);
    let wide = f(-4, 4);
    println!("E. equal significand precision 3, exponent ranges [-2,2] and [-4,4]");
    println!(
        "E. narrow has {} values, wide has {}",
        narrow.len(),
        wide.len()
    );
    println!("E. narrow is a subset of wide: {}", narrow.is_subset(&wide));
    println!("E. the two are equal: {}", narrow == wide);

    // ---- F. the phase condition, which only bias can violate ----------------
    // same quantum, same count, biases differing by half a quantum.
    let a = Num {
        b: Rat::new(0, 1),
        q: Rat::new(1, 1),
        n: 4,
    };
    let b = Num {
        b: Rat::new(-1, 4),
        q: Rat::new(1, 2),
        n: 20,
    };
    println!("F. a: b=0 q=1 n=4, values 0 1 2 3");
    println!("F. b: b=-1/4 q=1/2 n=20, values -0.25 0.25 0.75 ... 9.25");
    println!("F. grid refines: {}", a.q.div(b.q).is_int());
    println!(
        "F. range covers: {} and {}",
        b.b.le(a.b),
        a.top().le(b.top())
    );
    println!("F. phase aligns: {}", a.b.sub(b.b).div(b.q).is_int());
    println!("F. a is a subset of b: {}", subset_by_elements(&a, &b));

    // ---- G. meets in the two-sign-domain fixed-point slice ------------------
    let two_shapes: Vec<_> = shapes
        .iter()
        .filter(|(_, _, s, _)| *s != Sign::Symmetric)
        .cloned()
        .collect();
    let mut meet_exact = 0usize;
    let mut meet_wrong = 0usize;
    for (i1, f1, s1, a) in &two_shapes {
        for (i2, f2, s2, b) in &two_shapes {
            let mi = *i1.min(i2);
            let mf = *f1.min(f2);
            let ms = if sign_rank(*s1) <= sign_rank(*s2) {
                *s1
            } else {
                *s2
            };
            let m = fixed(mi, mf, ms);
            let av: std::collections::HashSet<(i128, i128)> =
                a.values().into_iter().map(|r| (r.n, r.d)).collect();
            let bv: std::collections::HashSet<(i128, i128)> =
                b.values().into_iter().map(|r| (r.n, r.d)).collect();
            let mv: std::collections::HashSet<(i128, i128)> =
                m.values().into_iter().map(|r| (r.n, r.d)).collect();
            let inter: std::collections::HashSet<(i128, i128)> =
                av.intersection(&bv).cloned().collect();
            if inter == mv {
                meet_exact += 1;
            } else {
                meet_wrong += 1;
            }
        }
    }
    println!(
        "G. two-sign-domain slice: componentwise-min IS the meet in {} pairs, is NOT in {}",
        meet_exact, meet_wrong
    );
}
