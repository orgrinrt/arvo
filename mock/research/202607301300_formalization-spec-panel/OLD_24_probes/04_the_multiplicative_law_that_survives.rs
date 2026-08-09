// Probe 04: which multiplicative law survives quantisation.
//
// Files 13/14 found that for addition the law the algorithm crates need is
// monotonicity (distributivity over max/min), not associativity, and that the
// presets sort by it: clamp monotone, wrap not. Hypothesis: the same is true
// multiplicatively. Scaling by a nonnegative constant, x -> Q(c*x), is a
// composition of a monotone map (multiplication by c >= 0), a monotone
// quantiser (floor and nearest are both monotone), and a monotone range
// recovery (clamp), so it is monotone, and therefore commutes with max and
// min. Under wrap it is not. And for c < 0 the clamped form is
// order-REVERSING, the ordered-ring positive-cone axiom's other half.
//
// Model: signed Q2.2 raw [-8, 7]; scaling by a Q2.2 constant c; product
// carries 2 extra fractional bits; quantisers floor / rne; recovery clamp /
// wrap (wrap = mod 16 into [-8,7]).

const LO: i64 = -8;
const HI: i64 = 7;

fn rne(x: i64, drop: u32) -> i64 {
    let half = 1i64 << (drop - 1);
    let fl = x >> drop;
    let rem = x - (fl << drop);
    if rem > half {
        fl + 1
    } else if rem < half {
        fl
    } else if fl & 1 == 0 {
        fl
    } else {
        fl + 1
    }
}

fn clamp(x: i64) -> i64 {
    x.clamp(LO, HI)
}

fn wrap(x: i64) -> i64 {
    ((x - LO).rem_euclid(16)) + LO
}

fn main() {
    let quants: [(&str, fn(i64, u32) -> i64); 2] = [("floor", |x, d| x >> d), ("rne", rne)];
    let recovs: [(&str, fn(i64) -> i64); 2] = [("clamp", clamp), ("wrap", wrap)];

    println!(
        "rule | recovery | monotone for all c>=0 | commutes with max | order-reversing for c<0"
    );
    for (qn, q) in quants {
        for (rn, r) in recovs {
            let scale = |c: i64, x: i64| r(q(c * x, 2));
            // monotone in x for every c >= 0
            let mut mono = true;
            let mut cmax = true;
            for c in 0..=HI {
                for x in LO..HI {
                    if scale(c, x) > scale(c, x + 1) {
                        mono = false;
                    }
                }
                for a in LO..=HI {
                    for b in LO..=HI {
                        let lhs = scale(c, a.max(b));
                        let rhs = scale(c, a).max(scale(c, b));
                        if lhs != rhs {
                            cmax = false;
                        }
                    }
                }
            }
            // order-reversing for c < 0
            let mut rev = true;
            for c in LO..0 {
                for x in LO..HI {
                    if scale(c, x) < scale(c, x + 1) {
                        rev = false;
                    }
                }
            }
            println!("{qn} | {rn} | {mono} | {cmax} | {rev}");
        }
    }
    println!();
    println!("monotone quantisation composed with clamp preserves the ordered-ring");
    println!("positive-cone structure exactly; wrap destroys it, at every rule.");
}
