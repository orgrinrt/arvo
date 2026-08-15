// p2: the declared-slack mechanism 139 proposes buys nothing in any cell.
//
// 139 section 4 argues that because fusing a multiply-add changes the answer,
// a policy must declare a SET of acceptable answers rather than one, and its p3
// prices the required slack: 0 raw units unsigned everywhere, 1 raw unit for
// signed wrapping at F >= 1, and 32 of 63 for signed saturating. The signed
// wrapping row is the mechanism's entire positive case: it is the only place a
// small declaration buys a real win.
//
// p1b refuted my first hypothesis about why 139's signed wrapping row is nonzero
// and mine is zero, so I stop guessing at 139's model and prove what fusion alone
// can do.
//
// THEOREM (absorption). Let R be reduction modulo 2^W, in either the unsigned or
// the two's-complement signed reading. For all integers x and c,
//
//     R(R(x) + c) = R(x + c).
//
// Proof. R(x) = x - k*2^W for some integer k, so R(x) + c = (x + c) - k*2^W,
// which is congruent to x + c mod 2^W. R is a function of the residue class
// alone, so it agrees on congruent arguments. QED.
//
// COROLLARY. If the fused and the stepwise arm feed the same value x to the final
// add, and the overflow policy is wrapping, they compute the same answer at every
// F, every width, and both signednesses. Fusion under wrapping is answer-
// preserving by construction, not by measurement, and the required slack is zero.
//
// CONSEQUENTLY a nonzero fusion-difference rate under WRAPPING is evidence that
// the two arms being compared differ in something other than where the reduction
// sits. Whatever else 139's arms differ in, the rate it reports for signed
// wrapping is not a rate for fusion alone, and the slack its p3 derives from that
// row is a price for a bundle rather than for fusion.
//
// This probe therefore does three things:
//   (a) checks the theorem exhaustively, with a control that can fail it;
//   (b) recomputes the minimum slack admitting fusion, per cell, on my model;
//   (c) measures what the required slack actually declares, by counting how many
//       distinct answers the declared set permits for a single input.
//
// (c) is the attack on the repair itself. 139 writes that the set formulation
// "keeps the property my firewall exists for, which is that nothing outside the
// declaration can move an answer". A declaration that permits k answers for one
// input determines the answer only when k = 1. Section 4's own objection, that
// two builds of one program produce different results with no predicate naming
// the difference, survives its own repair whenever k > 1, and the size of k is
// how much of the objection survives.
//
// PREDICTIONS, before running:
//   S1. Minimum slack admitting fusion is 0 for every unsigned cell and for every
//       signed WRAPPING cell, at every F, both truncation modes. This contradicts
//       139's p3, which reports 1 for signed wrapping at F >= 1.
//   S2. Minimum slack for signed saturating at F = 0 is 32, agreeing with 139.
//   S3. At slack 32 on a 64-value range the mean conforming set exceeds 50% of the
//       range, so the declaration determines almost nothing.
//   S4. At slack 0 the conforming set has cardinality exactly 1 at every input.
//
// CONTROLS:
//   C1 (must fail): run the absorption check with R replaced by SATURATION. It
//      must report mismatches, or the check cannot detect a non-homomorphism and
//      the zero it reports for wrapping is worthless.
//   C2 (minimality): at slack = required - 1 there must exist a non-conforming
//      input. Otherwise "minimum" is not minimum and the number is an upper bound.
//   C3 (reach): the slack search must see cells where fusion actually differs.
//      A cell whose fused and stepwise arms never differ trivially needs 0 slack,
//      and is printed as such rather than counted as a mechanism win.
//
// Run: rustc -O -o /tmp/p2 p2_the_slack_buys_nothing.rs && /tmp/p2

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sign {
    U,
    S,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Ovf {
    Wrap,
    Sat,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Trunc {
    TowardZero,
    Floor,
}

fn lo(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => 0,
        Sign::S => -(1i128 << (w - 1)),
    }
}
fn hi(s: Sign, w: u32) -> i128 {
    match s {
        Sign::U => (1i128 << w) - 1,
        Sign::S => (1i128 << (w - 1)) - 1,
    }
}

fn wrap(v: i128, s: Sign, w: u32) -> i128 {
    let m = 1i128 << w;
    let r = v.rem_euclid(m);
    match s {
        Sign::U => r,
        Sign::S => {
            if r >= (1i128 << (w - 1)) {
                r - m
            } else {
                r
            }
        }
    }
}
fn sat(v: i128, s: Sign, w: u32) -> i128 {
    v.clamp(lo(s, w), hi(s, w))
}
fn reduce(v: i128, s: Sign, o: Ovf, w: u32) -> i128 {
    match o {
        Ovf::Wrap => wrap(v, s, w),
        Ovf::Sat => sat(v, s, w),
    }
}
fn shift(p: i128, f: u32, t: Trunc) -> i128 {
    if f == 0 {
        return p;
    }
    match t {
        Trunc::TowardZero => p / (1i128 << f),
        Trunc::Floor => p >> f,
    }
}

fn stepwise(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    reduce(reduce(shift(a * b, f, t), s, o, w) + c, s, o, w)
}
fn fused(a: i128, b: i128, c: i128, s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> i128 {
    reduce(shift(a * b, f, t) + c, s, o, w)
}

// ---------- (a) the absorption theorem, checked ----------

fn absorption_mismatches(s: Sign, w: u32, use_sat_instead: bool) -> u64 {
    let (l, h) = (lo(s, w), hi(s, w));
    // x ranges well outside the declared range so the reduction has work to do.
    let span = 4 * (1i128 << w);
    let mut bad = 0u64;
    for x in -span..=span {
        for c in l..=h {
            let (left, right) = if use_sat_instead {
                (sat(sat(x, s, w) + c, s, w), sat(x + c, s, w))
            } else {
                (wrap(wrap(x, s, w) + c, s, w), wrap(x + c, s, w))
            };
            if left != right {
                bad += 1;
            }
        }
    }
    bad
}

// ---------- (b) the minimum slack admitting fusion ----------

struct Slack {
    required: i128,
    /// C3: does fusion ever differ in this cell at all?
    fusion_differs: u64,
    /// C2: at required-1, is some input non-conforming?
    minimal: bool,
}

fn min_slack(s: Sign, o: Ovf, w: u32, f: u32, t: Trunc) -> Slack {
    let (l, h) = (lo(s, w), hi(s, w));
    let mut worst = 0i128;
    let mut differs = 0u64;
    for a in l..=h {
        for b in l..=h {
            for c in l..=h {
                let st = stepwise(a, b, c, s, o, w, f, t);
                let fu = fused(a, b, c, s, o, w, f, t);
                if st != fu {
                    differs += 1;
                }
                let d = (st - fu).abs();
                if d > worst {
                    worst = d;
                }
            }
        }
    }
    // C2: minimality. At worst-1 some input must be non-conforming, which is
    // exactly the input achieving the maximum, so minimality holds iff worst > 0
    // and that maximum is attained. Check it rather than argue it.
    let mut minimal = worst == 0;
    if worst > 0 {
        'outer: for a in l..=h {
            for b in l..=h {
                for c in l..=h {
                    let d = (stepwise(a, b, c, s, o, w, f, t) - fused(a, b, c, s, o, w, f, t)).abs();
                    if d > worst - 1 {
                        minimal = true;
                        break 'outer;
                    }
                }
            }
        }
    }
    Slack {
        required: worst,
        fusion_differs: differs,
        minimal,
    }
}

// ---------- (c) what the declared set actually declares ----------

/// For each input, the number of representable values within `slack` of the
/// declared answer. That is how many different answers two conforming builds of
/// the same program may produce.
fn conforming_set_sizes(s: Sign, o: Ovf, w: u32, f: u32, t: Trunc, slack: i128) -> (f64, i128, i128) {
    let (l, h) = (lo(s, w), hi(s, w));
    let mut total = 0u128;
    let mut n = 0u128;
    let mut mn = i128::MAX;
    let mut mx = 0i128;
    for a in l..=h {
        for b in l..=h {
            for c in l..=h {
                let d = stepwise(a, b, c, s, o, w, f, t);
                let k = (d + slack).min(h) - (d - slack).max(l) + 1;
                total += k as u128;
                n += 1;
                mn = mn.min(k);
                mx = mx.max(k);
            }
        }
    }
    (total as f64 / n as f64, mn, mx)
}

fn main() {
    let w = 6u32;
    println!("p2: what the declared-slack mechanism actually buys\n");

    println!("=== (a) absorption theorem, checked exhaustively ===");
    for wq in [4u32, 5, 6] {
        for s in [Sign::U, Sign::S] {
            let sn = if s == Sign::U { "unsigned" } else { "signed" };
            let bad = absorption_mismatches(s, wq, false);
            println!("W={wq} {sn:<9} wrapping : R(R(x)+c) == R(x+c) mismatches = {bad}");
        }
    }
    println!("\n--- C1: the same check with SATURATION substituted, which must FAIL ---");
    for wq in [4u32, 5, 6] {
        for s in [Sign::U, Sign::S] {
            let sn = if s == Sign::U { "unsigned" } else { "signed" };
            let bad = absorption_mismatches(s, wq, true);
            let verdict = if bad > 0 { "control FIRES" } else { "control TOOTHLESS" };
            println!("W={wq} {sn:<9} saturating: mismatches = {bad:<8} {verdict}");
        }
    }

    println!("\n=== (b) minimum slack admitting fusion, W={w}, my model ===");
    for t in [Trunc::TowardZero, Trunc::Floor] {
        let tn = match t {
            Trunc::TowardZero => "toward zero",
            Trunc::Floor => "floor",
        };
        println!("-- rounding = {tn} --");
        println!("{:<22} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}", "cell", "F=0", "F=1", "F=2", "F=3", "F=4", "F=5");
        for s in [Sign::U, Sign::S] {
            for o in [Ovf::Wrap, Ovf::Sat] {
                let name = format!(
                    "{}, {}",
                    if s == Sign::U { "unsigned" } else { "signed" },
                    if o == Ovf::Wrap { "wrapping" } else { "saturating" }
                );
                let mut row = format!("{name:<22}");
                let mut notes: Vec<String> = Vec::new();
                for f in 0..=5u32 {
                    let sl = min_slack(s, o, w, f, t);
                    row.push_str(&format!(" {:>6}", sl.required));
                    if !sl.minimal {
                        notes.push(format!("F={f} C2 FAILED: not minimal"));
                    }
                    if sl.fusion_differs == 0 {
                        notes.push(format!("F={f} C3: fusion never differs, 0 is trivial"));
                    }
                }
                println!("{row}");
                for n in notes {
                    println!("      . {n}");
                }
            }
        }
    }

    println!("\n=== (c) how many answers the declaration permits, signed saturating, floor ===");
    println!("(range holds {} representable values)", hi(Sign::S, w) - lo(Sign::S, w) + 1);
    for f in [0u32, 3, 5] {
        let sl = min_slack(Sign::S, Ovf::Sat, w, f, Trunc::Floor);
        let (mean, mn, mx) = conforming_set_sizes(Sign::S, Ovf::Sat, w, f, Trunc::Floor, sl.required);
        let (mean0, mn0, mx0) = conforming_set_sizes(Sign::S, Ovf::Sat, w, f, Trunc::Floor, 0);
        println!(
            "F={f}: slack {:>2} -> conforming set per input: mean {mean:.2}, min {mn}, max {mx}  ({:.1}% of range)",
            sl.required,
            100.0 * mean / 64.0
        );
        println!(
            "      S4 control, slack 0 -> mean {mean0:.2}, min {mn0}, max {mx0} (must be exactly 1/1/1)"
        );
    }
}
