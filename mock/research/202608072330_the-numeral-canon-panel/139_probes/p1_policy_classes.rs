// p1: how many OBSERVATIONALLY DISTINCT policy assignments exist, and does the
// count depend on the numeral's shape?
//
// A "policy assignment" here is a point in a small candidate axis product:
//   overflow     in {Wrap, Sat}
//   rounding     in {Trunc, NearestEven, Floor}
//   intermediate in {Exact, Stepwise}
// = 12 syntactic points.
//
// Two points are OBSERVATIONALLY EQUAL at a shape when they agree on every
// output of every operation over the entire input domain. The question the
// probe answers is how many equivalence classes that leaves, per shape.
//
// PREDICTIONS, recorded before the first run:
//   P1  (W=6, F=0, unsigned): 3 classes. Rounding is dead with no fractional
//       bits to discard, and Wrap merges the two Intermediate values because
//       reduction mod 2^W is a ring homomorphism, so where you reduce cannot
//       matter. Sat is not a homomorphism, so it keeps both.
//   P2  (W=6, F=0, signed):   3 classes, same argument.
//   P3  (W=6, F=3, unsigned): 8 classes. Rounding is live, but Trunc and Floor
//       coincide with no negative values, so 2 x 2 x 2.
//   P4  (W=6, F=3, signed):  12 classes. All three rounding modes separate.
//
// CONTROLS, each of which must fail if the instrument is broken:
//   C1 a duplicate policy, written through a different construction, must land
//      in the SAME class as its twin. Catches a comparator that never merges.
//   C2 Wrap/Trunc/Exact and Sat/Trunc/Exact must land in DIFFERENT classes at
//      every shape. Catches a comparator that always merges.
//   C3 mutation: rerun the classification with the comparator replaced by
//      always-equal and by never-equal, and show the class count moves. A count
//      that does not move under a sabotaged comparator was not measuring the
//      comparator's subject.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overflow {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rounding {
    Trunc,
    NearestEven,
    Floor,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Intermediate {
    Exact,
    Stepwise,
}

#[derive(Clone, Copy, Debug)]
struct Policy {
    ov: Overflow,
    rd: Rounding,
    it: Intermediate,
    // Only set for the duplicate control: a second construction of an
    // already-present point, reached by a different code path.
    dup_of: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
struct Shape {
    w: u32,
    f: u32,
    signed: bool,
    // Stride on the three-argument sweep only. 1 means exhaustive. A stride
    // above 1 makes the reported class count a LOWER BOUND: a witness of
    // difference found on a subset is still a witness, but two policies that
    // agree on the subset might still differ off it, so classes can only
    // split further with more data, never merge.
    stride3: usize,
}

fn range(s: Shape) -> (i128, i128) {
    let m = 1i128 << s.w;
    if s.signed {
        (-(m >> 1), (m >> 1) - 1)
    } else {
        (0, m - 1)
    }
}

fn wrap(v: i128, s: Shape) -> i128 {
    let m = 1i128 << s.w;
    let r = v.rem_euclid(m);
    if s.signed && r >= (m >> 1) {
        r - m
    } else {
        r
    }
}

fn saturate(v: i128, s: Shape) -> i128 {
    let (lo, hi) = range(s);
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

fn reduce(v: i128, s: Shape, ov: Overflow) -> i128 {
    match ov {
        Overflow::Wrap => wrap(v, s),
        Overflow::Sat => saturate(v, s),
    }
}

// Divide p by 2^sh under the named rounding mode.
fn rshift(p: i128, sh: u32, rd: Rounding) -> i128 {
    if sh == 0 {
        return p;
    }
    let d = 1i128 << sh;
    match rd {
        Rounding::Floor => p.div_euclid(d),
        Rounding::Trunc => p / d, // Rust integer division truncates toward zero
        Rounding::NearestEven => {
            let q = p.div_euclid(d);
            let r = p.rem_euclid(d);
            let half = d >> 1;
            if r > half {
                q + 1
            } else if r < half {
                q
            } else if q.rem_euclid(2) == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

// The operation set. Binary ops make the overflow and rounding axes live; the
// two chained ops make the intermediate axis live, since they are the only
// place where "reduce once at the end" and "reduce at each step" can differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Mul3,
    Madd,
}

fn eval(op: Op, a: i128, b: i128, c: i128, s: Shape, p: Policy) -> i128 {
    let f = s.f;
    match op {
        // Same scale on both sides: no rounding, one reduction. The
        // intermediate axis cannot bite on a single operation.
        Op::Add => reduce(a + b, s, p.ov),
        Op::Sub => reduce(a - b, s, p.ov),
        // Product lands at scale 2^-2F and must come back to 2^-F.
        Op::Mul => reduce(rshift(a * b, f, p.rd), s, p.ov),
        Op::Mul3 => match p.it {
            Intermediate::Exact => {
                // one rounding, one reduction, over the exact triple product
                reduce(rshift(a * b * c, 2 * f, p.rd), s, p.ov)
            }
            Intermediate::Stepwise => {
                let t = reduce(rshift(a * b, f, p.rd), s, p.ov);
                reduce(rshift(t * c, f, p.rd), s, p.ov)
            }
        },
        Op::Madd => match p.it {
            Intermediate::Exact => {
                // a*b at scale 2^-2F, c lifted to the same scale, one rounding
                let acc = a * b + (c << f);
                reduce(rshift(acc, f, p.rd), s, p.ov)
            }
            Intermediate::Stepwise => {
                let t = reduce(rshift(a * b, f, p.rd), s, p.ov);
                reduce(t + c, s, p.ov)
            }
        },
    }
}

fn domain(s: Shape) -> Vec<i128> {
    let (lo, hi) = range(s);
    (lo..=hi).collect()
}

// The full behaviour of one policy at one shape: every output of every
// operation over the whole domain, in a fixed order.
fn behaviour(s: Shape, p: Policy) -> Vec<i64> {
    let d = domain(s);
    let mut out = Vec::new();
    for op in [Op::Add, Op::Sub, Op::Mul] {
        for &a in &d {
            for &b in &d {
                out.push(eval(op, a, b, 0, s, p) as i64);
            }
        }
    }
    let st = s.stride3;
    for op in [Op::Mul3, Op::Madd] {
        for a in d.iter().step_by(st) {
            for b in d.iter().step_by(st) {
                for c in d.iter().step_by(st) {
                    out.push(eval(op, *a, *b, *c, s, p) as i64);
                }
            }
        }
    }
    out
}

fn policies() -> Vec<Policy> {
    let mut v = Vec::new();
    for ov in [Overflow::Wrap, Overflow::Sat] {
        for rd in [Rounding::Trunc, Rounding::NearestEven, Rounding::Floor] {
            for it in [Intermediate::Exact, Intermediate::Stepwise] {
                v.push(Policy {
                    ov,
                    rd,
                    it,
                    dup_of: None,
                });
            }
        }
    }
    // C1: a thirteenth entry that is the same point as index 0
    // (Wrap/Trunc/Exact) reached by a separate construction.
    let twin = Policy {
        ov: Overflow::Wrap,
        rd: Rounding::Trunc,
        it: Intermediate::Exact,
        dup_of: Some(0),
    };
    v.push(twin);
    v
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Comparator {
    Real,
    AlwaysEqual,
    NeverEqual,
}

fn same(a: &[i64], b: &[i64], c: Comparator) -> bool {
    match c {
        Comparator::Real => a == b,
        Comparator::AlwaysEqual => true,
        Comparator::NeverEqual => false,
    }
}

fn classify(s: Shape, ps: &[Policy], tables: &[Vec<i64>], c: Comparator) -> Vec<usize> {
    let mut class_of = vec![usize::MAX; ps.len()];
    let mut reps: Vec<usize> = Vec::new();
    for i in 0..ps.len() {
        let mut found = None;
        for (k, &r) in reps.iter().enumerate() {
            if same(&tables[i], &tables[r], c) {
                found = Some(k);
                break;
            }
        }
        class_of[i] = match found {
            Some(k) => k,
            None => {
                reps.push(i);
                reps.len() - 1
            }
        };
    }
    let _ = s;
    class_of
}

fn name(p: Policy) -> String {
    let d = if p.dup_of.is_some() { " [dup]" } else { "" };
    format!("{:?}/{:?}/{:?}{}", p.ov, p.rd, p.it, d)
}

fn main() {
    let shapes = [
        Shape { w: 6, f: 0, signed: false, stride3: 1 },
        Shape { w: 6, f: 0, signed: true, stride3: 1 },
        Shape { w: 6, f: 3, signed: false, stride3: 1 },
        Shape { w: 6, f: 3, signed: true, stride3: 1 },
        // wider shapes, to check the counts are not an artifact of W=6.
        // stride3 > 1 makes these a lower bound; see the note on Shape.
        Shape { w: 8, f: 0, signed: true, stride3: 8 },
        Shape { w: 8, f: 4, signed: true, stride3: 8 },
        Shape { w: 8, f: 4, signed: false, stride3: 8 },
    ];
    let ps = policies();
    let mut failures = 0usize;

    for s in shapes {
        let d = domain(s);
        let tables: Vec<Vec<i64>> = ps.iter().map(|&p| behaviour(s, p)).collect();
        let obs = tables[0].len();
        let class_of = classify(s, &ps, &tables, Comparator::Real);
        let n_classes = class_of.iter().copied().max().unwrap() + 1;

        println!(
            "shape W={} F={} signed={} stride3={}  domain={} observations/policy={}",
            s.w,
            s.f,
            s.signed,
            s.stride3,
            d.len(),
            obs
        );
        println!("  distinct classes among 12 syntactic policies (+1 duplicate): {n_classes}");

        // group listing
        for k in 0..n_classes {
            let members: Vec<String> = ps
                .iter()
                .enumerate()
                .filter(|(i, _)| class_of[*i] == k)
                .map(|(_, &p)| name(p))
                .collect();
            println!("    class {k}: {}", members.join("  |  "));
        }

        // C1: the duplicate must share its twin's class.
        let dup_idx = ps.iter().position(|p| p.dup_of.is_some()).unwrap();
        let twin_idx = ps[dup_idx].dup_of.unwrap();
        if class_of[dup_idx] == class_of[twin_idx] {
            println!("  C1 duplicate merges with its twin: PASS");
        } else {
            println!("  C1 duplicate merges with its twin: FAIL");
            failures += 1;
        }

        // C2: Wrap/Trunc/Exact (index 0) and Sat/Trunc/Exact must differ.
        let sat_te = ps
            .iter()
            .position(|p| {
                p.ov == Overflow::Sat
                    && p.rd == Rounding::Trunc
                    && p.it == Intermediate::Exact
                    && p.dup_of.is_none()
            })
            .unwrap();
        if class_of[0] != class_of[sat_te] {
            println!("  C2 Wrap and Sat separate: PASS");
        } else {
            println!("  C2 Wrap and Sat separate: FAIL");
            failures += 1;
        }

        // C3: mutation. The count must move when the comparator is sabotaged.
        let n_always = classify(s, &ps, &tables, Comparator::AlwaysEqual)
            .iter()
            .copied()
            .max()
            .unwrap()
            + 1;
        let n_never = classify(s, &ps, &tables, Comparator::NeverEqual)
            .iter()
            .copied()
            .max()
            .unwrap()
            + 1;
        println!("  C3 mutation: real={n_classes} always-equal={n_always} never-equal={n_never}");
        if n_always == 1 && n_never == ps.len() && n_classes != n_always && n_classes != n_never {
            println!("  C3 count is sensitive to the comparator: PASS");
        } else {
            println!("  C3 count is sensitive to the comparator: FAIL");
            failures += 1;
        }
        println!();
    }

    println!("control failures: {failures}");
    if failures > 0 {
        std::process::exit(1);
    }
}
