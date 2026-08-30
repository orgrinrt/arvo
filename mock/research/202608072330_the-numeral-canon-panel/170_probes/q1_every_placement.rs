//! q1. The pointwise-optimality claim, re-run over EVERY placement, on my own
//! instrument rather than on the attacker's.
//!
//! `169` section 2 found that `168`'s `eager_wins` compares two placements,
//! `full_mask` and `0`, while `168` 7.1's claim quantifies over all `2^(n-1)`.
//! I confirmed that by opening my own function. This probe is the repair, and
//! it is deliberately a rewrite of my own Rust rather than a reading of `169`'s
//! Python, so that agreement between us is two instruments rather than one.
//!
//! It also answers three things `169` asked for and one it left open:
//!
//!   R-5 / R-6. The clamp row had no matched non-nearest partner. `Pi::Clamp`
//!     projects onto all 256 in-range values; `Pi::Wrap` projects onto the SAME
//!     256 values and is not nearest. That is the matched pair clamp lacked.
//!   O-169-1. A COARSER grid, `RoundTo(5)` and `TruncTo(5)`, multiples of 32,
//!     where most exact values sit far from every representable point and a
//!     placement has the most room to buy something.
//!   The exercise count per resolution, so a zero is qualified by how often
//!     placement changes the answer at all.
//!
//! THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
//!   C1. Arm 1 must reproduce 168's published 0 / 0 / 91 and 1330 truncate
//!       inputs exactly, or this is a neighbouring experiment and nothing it
//!       says bears on the claim.
//!   C2. Arm 2 must find STRICTLY MORE truncate counterexamples than arm 1, or
//!       the widening found nothing and 169's defect is cosmetic.
//!   C3. Each non-nearest row must be nonzero, or that row is not a control.
//!   C4. Each resolution's exercise count must be nonzero, or a zero in its
//!       win column is a fact about the workload rather than about the claim.
//!
//! Run: rustc -O q1_every_placement.rs -o /tmp/q1 && /tmp/q1

const W: u32 = 8;
const DOMAIN: u128 = 1 << W;
const LIMIT: u128 = (1 << W) - 1;

#[derive(Clone, Copy, Debug)]
enum Step {
    AddK(u128),
    MulK(u128),
    ShrK(u32),
    XorK(u128),
    SatSubK(u128),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pi {
    Wrap,
    Clamp,
    RoundTo(u32),
    TruncTo(u32),
}

fn apply(s: Step, v: u128) -> u128 {
    match s {
        Step::AddK(k) => v + k,
        Step::MulK(k) => v * k,
        Step::ShrK(g) => v >> g,
        Step::XorK(k) => v ^ k,
        Step::SatSubK(k) => v.saturating_sub(k),
    }
}

fn resolve(p: Pi, v: u128) -> u128 {
    match p {
        Pi::Wrap => v & LIMIT,
        Pi::Clamp => {
            if v > LIMIT {
                LIMIT
            } else {
                v
            }
        }
        Pi::RoundTo(g) => {
            if g == 0 {
                return v;
            }
            let step = 1u128 << g;
            ((v + (step >> 1)) / step) * step
        }
        Pi::TruncTo(g) => {
            if g == 0 {
                return v;
            }
            let step = 1u128 << g;
            (v / step) * step
        }
    }
}

fn run(x: u128, steps: &[Step], p: Pi, mask: u32) -> u128 {
    let mut v = x;
    for (i, &s) in steps.iter().enumerate() {
        v = apply(s, v);
        if i + 1 < steps.len() && (mask >> i) & 1 == 1 {
            v = resolve(p, v);
        }
    }
    resolve(p, v)
}

fn exact(x: u128, steps: &[Step]) -> u128 {
    let mut v = x;
    for &s in steps {
        v = apply(s, v);
    }
    v
}

fn dist(a: u128, b: u128) -> u128 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// The deferred placement is mask 0. Does ANY other placement beat it, at any
/// input? `all` selects whether "any other" means every mask or only the fully
/// eager one, which is exactly the difference 169 found.
fn beats_deferred(steps: &[Step], p: Pi, all: bool) -> (usize, usize) {
    let interior = steps.len().saturating_sub(1);
    let top = 1u32 << interior;
    let mut win_inputs = 0usize;
    let mut any = 0usize;
    for x in 0..DOMAIN {
        let want = exact(x, steps);
        let dd = dist(run(x, steps, p, 0), want);
        let mut hit = false;
        if all {
            for mask in 1..top {
                if dist(run(x, steps, p, mask), want) < dd {
                    hit = true;
                    break;
                }
            }
        } else if interior > 0 && dist(run(x, steps, p, top - 1), want) < dd {
            hit = true;
        }
        if hit {
            win_inputs += 1;
            any = 1;
        }
    }
    (any, win_inputs)
}

/// How often placement changes the output at all, for this chain. A row whose
/// exercise count is zero cannot report a meaningful zero in the win column.
fn placement_matters(steps: &[Step], p: Pi) -> bool {
    let interior = steps.len().saturating_sub(1);
    let top = 1u32 << interior;
    for x in 0..DOMAIN {
        let base = run(x, steps, p, 0);
        for mask in 1..top {
            if run(x, steps, p, mask) != base {
                return true;
            }
        }
    }
    false
}

fn chains() -> Vec<Vec<Step>> {
    // Regenerated exactly as 168_probes/p3 does: same alphabet, same xorshift
    // seed, same order of draws. C1 is what checks that it really is the same.
    let k = 97u128;
    let alphabet = [
        Step::AddK(k),
        Step::MulK(3),
        Step::ShrK(2),
        Step::XorK(0b1011_0110),
        Step::SatSubK(LIMIT / 2),
        Step::MulK(5),
        Step::ShrK(1),
        Step::AddK(13),
    ];
    let mut rng: u64 = 0xA5A5_1234_DEAD_BEEF;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut out = Vec::new();
    for _ in 0..3000 {
        let d = 2 + (next() % 4) as usize;
        out.push((0..d).map(|_| alphabet[(next() % 8) as usize]).collect());
    }
    out
}

fn sweep(cs: &[Vec<Step>], p: Pi, all: bool) -> (usize, usize, usize) {
    let mut win_chains = 0;
    let mut win_inputs = 0;
    let mut exercised = 0;
    for c in cs {
        let (any, wi) = beats_deferred(c, p, all);
        if any > 0 {
            win_chains += 1;
        }
        win_inputs += wi;
        if placement_matters(c, p) {
            exercised += 1;
        }
    }
    (win_chains, win_inputs, exercised)
}

fn main() {
    let cs = chains();
    println!(
        "W = {W}, domain 0..{DOMAIN} exhaustive, {} chains, depth 2..=5",
        cs.len()
    );

    // How much wider arm 2 actually is, counted rather than assumed. Arm 1
    // tries exactly one alternative placement per chain; arm 2 tries
    // 2^(depth-1) - 1 of them.
    let mut hist = [0usize; 6];
    let mut alts_narrow = 0usize;
    let mut alts_full = 0usize;
    for c in cs.iter() {
        hist[c.len()] += 1;
        alts_narrow += 1;
        alts_full += (1usize << (c.len() - 1)) - 1;
    }
    println!(
        "depth histogram (2..=5): {:?}   alternative placements tried: arm 1 {}, arm 2 {} ({:.2}x)",
        &hist[2..=5],
        alts_narrow,
        alts_full,
        alts_full as f64 / alts_narrow as f64
    );
    println!();

    let rows: [(&str, Pi, bool); 6] = [
        ("clamp            [256 vals]", Pi::Clamp, true),
        ("wrap  CONTROL    [256 vals]", Pi::Wrap, false),
        ("round to 2^3      [32 vals]", Pi::RoundTo(3), true),
        ("trunc CONTROL 2^3 [32 vals]", Pi::TruncTo(3), false),
        ("round to 2^5       [8 vals]", Pi::RoundTo(5), true),
        ("trunc CONTROL 2^5  [8 vals]", Pi::TruncTo(5), false),
    ];

    println!("=== ARM 1: two placements, as 168 ran it ===");
    println!(
        "{:<30} {:>10} {:>11} {:>10}",
        "resolution", "win_chains", "win_inputs", "exercised"
    );
    let mut a1 = Vec::new();
    for (name, p, _) in rows.iter() {
        let r = sweep(&cs, *p, false);
        println!("{name:<30} {:>10} {:>11} {:>10}", r.0, r.1, r.2);
        a1.push(r);
    }
    println!();

    println!("=== ARM 2: every placement, as the claim states it ===");
    println!(
        "{:<30} {:>10} {:>11} {:>10}",
        "resolution", "win_chains", "win_inputs", "exercised"
    );
    let mut a2 = Vec::new();
    for (name, p, _) in rows.iter() {
        let r = sweep(&cs, *p, true);
        println!("{name:<30} {:>10} {:>11} {:>10}", r.0, r.1, r.2);
        a2.push(r);
    }
    println!();

    // ---- controls --------------------------------------------------------
    println!("=== CONTROLS ===");
    let c1 = a1[2].0 == 0 && a1[0].0 == 0 && a1[3].0 == 91 && a1[3].1 == 1330;
    println!("C1 arm 1 reproduces 168's round 0, clamp 0, trunc 91 chains / 1330 inputs : {c1}");
    assert!(
        c1,
        "arm 1 does not reproduce 168's published numbers, so this is a different experiment"
    );

    let c2 = a2[3].0 > a1[3].0;
    println!(
        "C2 widening finds strictly more truncate counterexamples ({} -> {})       : {c2}",
        a1[3].0, a2[3].0
    );
    assert!(
        c2,
        "widening found nothing, so the scope defect was cosmetic"
    );

    let c3 = a2[1].0 > 0 && a2[3].0 > 0 && a2[5].0 > 0;
    println!(
        "C3 every non-nearest row is nonzero (wrap {}, trunc3 {}, trunc5 {})        : {c3}",
        a2[1].0, a2[3].0, a2[5].0
    );
    assert!(
        c3,
        "a non-nearest row reported zero, so it is not a control for its partner"
    );

    let c4 = a2.iter().all(|r| r.2 > 0);
    println!("C4 every resolution is exercised on some chain                             : {c4}");
    assert!(
        c4,
        "a resolution is never exercised, so its zero says nothing"
    );

    let c5 = a2[0].0 == 0 && a2[2].0 == 0 && a2[4].0 == 0;
    println!("C5 every NEAREST-POINT row is still zero over all placements               : {c5}");
    println!();
    if c5 {
        println!("RESULT: the claim survives at the strength it asserts. Every nearest-point");
        println!("resolution reports zero over EVERY placement, including a grid coarse enough");
        println!("that most exact values sit far from any representable point; and each has a");
        println!("matched non-nearest partner onto the identical representable set that fires.");
    } else {
        println!("RESULT: a nearest-point resolution admits a strictly closer placement, so");
        println!("168 7.1 is REFUTED at the widths and chains swept here.");
    }
}
