// 21_probes/02: the accumulator is not a free dimension. Its contribution to a law's key is a
// threshold, and the threshold is predicted by the recovery map's structural class.
//
// Question. File 18 section 6 measured that signed saturating addition goes from regrouping
// diameter 7 to Kleene associativity with no axis changed, purely by widening the accumulator,
// and left two readings open: the accumulator becomes a combinator parameter, or it becomes an
// eleventh Policy axis. Both put a free dimension in the key. This probe asks whether there is
// a third reading in which the accumulator is in the key but is not searched over, because the
// smallest sufficient accumulator is computable from facts the design already derives.
//
// Measured, exhaustively, no timing and no performance claim:
//
//   1. For each of four recovery maps, the structural class file 18 section 4 defines
//      (homomorphism, partial identity, retraction) plus monotonicity, at each accumulator
//      scale.
//   2. The THRESHOLD: the smallest accumulator scale at which every grouping of an n-element
//      fold agrees, per arity.
//   3. Whether the class predicts the threshold, and what the residual is.
//
// Model: signed 3-bit numeral, values [-4, 3]. Accumulator at scale K holds [K*-4, K*3].
// Elements are drawn from the numeral; the accumulator holds the running value. This is file
// 18's `18_probes/01` instrument with the sweep run over class as well as verdict.

const NLO: i64 = -4;
const NHI: i64 = 3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Res {
    Wrap,
    Saturate,
    Refuse,
    SubZero,
}
use Res::*;

const ALL: [Res; 4] = [Wrap, Saturate, Refuse, SubZero];

fn label(r: Res) -> &'static str {
    match r {
        Wrap => "Wrap     (Hot)",
        Saturate => "Saturate (Warm/Cold)",
        Refuse => "Refuse   (Precise)",
        SubZero => "SubZero",
    }
}

fn phi(r: Res, x: i64, lo: i64, hi: i64) -> Option<i64> {
    if x >= lo && x <= hi {
        return Some(x);
    }
    match r {
        Wrap => {
            let m = hi - lo + 1;
            let mut v = (x - lo) % m;
            if v < 0 {
                v += m;
            }
            Some(v + lo)
        }
        Saturate => Some(if x > hi { hi } else { lo }),
        Refuse => None,
        SubZero => Some(0),
    }
}

fn step(r: Res, a: Option<i64>, b: Option<i64>, lo: i64, hi: i64) -> Option<i64> {
    match (a, b) {
        (Some(x), Some(y)) => phi(r, x + y, lo, hi),
        _ => None,
    }
}

/// Every grouping of the slice, as a multiset of results.
fn groupings(r: Res, v: &[i64], lo: i64, hi: i64, out: &mut Vec<Option<i64>>) {
    if v.len() == 1 {
        out.push(Some(v[0]));
        return;
    }
    for k in 1..v.len() {
        let mut ls = Vec::new();
        groupings(r, &v[..k], lo, hi, &mut ls);
        let mut rs = Vec::new();
        groupings(r, &v[k..], lo, hi, &mut rs);
        for a in &ls {
            for b in &rs {
                out.push(step(r, *a, *b, lo, hi));
            }
        }
    }
}

/// Kleene agreement (both refuse, or both return and agree) across every grouping, over every
/// tuple of `n` numeral values.
fn kleene_agrees(r: Res, n: usize, lo: i64, hi: i64) -> bool {
    let span = (NHI - NLO + 1) as usize;
    let total = span.pow(n as u32);
    let mut v = vec![0i64; n];
    for mut idx in 0..total {
        for slot in v.iter_mut() {
            *slot = NLO + (idx % span) as i64;
            idx /= span;
        }
        let mut out = Vec::new();
        groupings(r, &v, lo, hi, &mut out);
        let first = out[0];
        for g in &out[1..] {
            let same = match (first, *g) {
                (None, None) => true,
                (Some(a), Some(b)) => a == b,
                _ => false,
            };
            if !same {
                return false;
            }
        }
    }
    true
}

// --- structural classification of `phi` at a given accumulator, file 18 section 4's three ---

/// The exact window a fold of `n` numeral values can reach, one step past the accumulator so
/// the boundary behaviour is exercised.
fn exact_window(lo: i64, hi: i64) -> (i64, i64) {
    (lo + 2 * NLO, hi + 2 * NHI)
}

fn is_homomorphism(r: Res, lo: i64, hi: i64) -> bool {
    let (elo, ehi) = exact_window(lo, hi);
    for x in elo..=ehi {
        for y in elo..=ehi {
            let direct = phi(r, x + y, lo, hi);
            let staged = match (phi(r, x, lo, hi), phi(r, y, lo, hi)) {
                (Some(a), Some(b)) => phi(r, a + b, lo, hi),
                _ => None,
            };
            let same = match (direct, staged) {
                (None, None) => true,
                (Some(a), Some(b)) => a == b,
                _ => false,
            };
            if !same {
                return false;
            }
        }
    }
    true
}

fn is_partial_identity(r: Res, lo: i64, hi: i64) -> bool {
    let (elo, ehi) = exact_window(lo, hi);
    (elo..=ehi).all(|x| match phi(r, x, lo, hi) {
        Some(v) => v == x,
        None => true,
    })
}

fn is_retraction(r: Res, lo: i64, hi: i64) -> bool {
    let (elo, ehi) = exact_window(lo, hi);
    let total = (elo..=ehi).all(|x| phi(r, x, lo, hi).is_some());
    let fixes = (lo..=hi).all(|x| phi(r, x, lo, hi) == Some(x));
    total && fixes && is_monotone(r, lo, hi)
}

/// Monotone in the flat order: refusal compares with nothing, and where both are defined the
/// order is preserved. This is the generous reading file 18 section 7.1 used.
fn is_monotone(r: Res, lo: i64, hi: i64) -> bool {
    let (elo, ehi) = exact_window(lo, hi);
    for x in elo..ehi {
        if let (Some(a), Some(b)) = (phi(r, x, lo, hi), phi(r, x + 1, lo, hi)) {
            if a > b {
                return false;
            }
        }
    }
    true
}

/// Does `phi` ever fire at all over the reachable window of an n-element fold? When it never
/// fires, the accumulator's addition is total on the reachable set and associativity is free
/// for any recovery map whatsoever.
fn never_fires(n: usize, lo: i64, hi: i64) -> bool {
    let reach_lo = (n as i64) * NLO;
    let reach_hi = (n as i64) * NHI;
    reach_lo >= lo && reach_hi <= hi
}

fn class_of(r: Res, lo: i64, hi: i64) -> &'static str {
    if is_homomorphism(r, lo, hi) {
        "homomorphism"
    } else if is_partial_identity(r, lo, hi) {
        "partial identity"
    } else if is_retraction(r, lo, hi) {
        "retraction"
    } else {
        "none of three"
    }
}

fn main() {
    println!("=== 1. structural class and monotonicity of each recovery map ===");
    println!("(measured at accumulator scale 1; the class does not move with the scale, checked below)\n");
    println!("{:<22} {:<18} {:>9}", "resolution", "class", "monotone");
    for r in ALL {
        println!(
            "{:<22} {:<18} {:>9}",
            label(r),
            class_of(r, NLO, NHI),
            is_monotone(r, NLO, NHI)
        );
    }

    println!("\nclass stability across accumulator scales 1..8:");
    for r in ALL {
        let classes: Vec<&str> = (1..=8).map(|k| class_of(r, NLO * k, NHI * k)).collect();
        let stable = classes.iter().all(|c| *c == classes[0]);
        println!("  {:<22} stable = {:<6} ({})", label(r), stable, classes[0]);
    }

    println!(
        "\n=== 2. the threshold: smallest accumulator scale at which every grouping agrees ==="
    );
    println!("(K = scale; the accumulator holds [K*-4, K*3]; `-` means not reached by scale 8)\n");
    print!("{:<22}", "resolution");
    for n in 3..=6 {
        print!("{:>8}", format!("n={}", n));
    }
    println!("{:>26}", "never-fires scale, n=6");
    for r in ALL {
        print!("{:<22}", label(r));
        for n in 3..=6 {
            let t = (1..=8).find(|k| kleene_agrees(r, n, NLO * k, NHI * k));
            match t {
                Some(k) => print!("{:>8}", format!("K{}", k)),
                None => print!("{:>8}", "-"),
            }
        }
        let nf = (1..=8).find(|k| never_fires(6, NLO * k, NHI * k));
        println!("{:>26}", format!("K{}", nf.unwrap()));
    }

    section_three();
    section_four();
}

/// The interior-safety condition, which mentions no recovery map at all: does the accumulator
/// cover every partial sum of at most `n-1` numeral values? If it does, no interior node of any
/// grouping can leave the accumulator, so `phi` is applied at most once per grouping, at the
/// root, to the exact sum.
fn interior_safe(n: usize, lo: i64, hi: i64) -> bool {
    let m = (n as i64) - 1;
    m * NLO >= lo && m * NHI <= hi
}

fn section_three() {
    println!("\n=== 3. two candidate predictors, one refuted ===\n");
    println!("candidate A: the structural class / monotonicity of the recovery map.");
    println!("candidate B: interior safety, acc covers (n-1) * numeral, which names no map.\n");
    println!(
        "{:<22} {:>9} {:>10} {:>12} {:>14}",
        "resolution", "class", "monotone", "threshold n=5", "interior-safe K"
    );
    for r in ALL {
        let t = (1..=8)
            .find(|k| kleene_agrees(r, 5, NLO * k, NHI * k))
            .unwrap();
        let isafe = (1..=8)
            .find(|k| interior_safe(5, NLO * k, NHI * k))
            .unwrap();
        println!(
            "{:<22} {:>9} {:>10} {:>12} {:>14}",
            label(r),
            if class_of(r, NLO, NHI) == "homomorphism" {
                "hom"
            } else {
                "other"
            },
            is_monotone(r, NLO, NHI),
            format!("K{}", t),
            format!("K{}", isafe)
        );
    }
    println!(
        "\ncandidate A is REFUTED. `SubZero` is neither monotone nor any of the three classes and\n\
         reaches agreement at exactly the scale `Saturate` (monotone retraction) and `Refuse`\n\
         (monotone partial identity) do. Monotonicity separates nothing here.\n\
         candidate B holds for every non-homomorphism row, at every arity measured below."
    );
}

fn section_four() {
    println!("\n=== 4. candidate B against the whole sweep ===\n");
    print!("{:<22}", "resolution");
    for n in 3..=6 {
        print!("{:>12}", format!("n={}", n));
    }
    println!();
    for r in ALL {
        print!("{:<22}", label(r));
        for n in 3..=6 {
            let t = (1..=9).find(|k| kleene_agrees(r, n, NLO * k, NHI * k));
            let b = (1..=9)
                .find(|k| interior_safe(n, NLO * k, NHI * k))
                .unwrap();
            let cell = match t {
                Some(k) if k == b => format!("K{} = B", k),
                Some(k) if k < b => format!("K{} < B{}", k, b),
                Some(k) => format!("K{} > B{}", k, b),
                None => "-".to_string(),
            };
            print!("{:>12}", cell);
        }
        println!();
    }
    println!(
        "\ntwo sufficient conditions, and they are the two ends of one axis:\n\
         \n\
         (i)  the recovery map COMMUTES with the operation (homomorphism). it may be applied at\n\
              every step and the answer is unchanged, so the accumulator is free. `Wrap` only.\n\
         (ii) the recovery map is DEFERRED TO THE ROOT (acc covers (n-1) * numeral). it is\n\
              applied once, to the exact sum, so the answer cannot depend on grouping, and the\n\
              map itself drops out of the fact entirely. every map, at K = n-1.\n\
         \n\
         nothing was measured strictly between the two. that is the whole content of the\n\
         accumulator dimension: it decides WHETHER THE RECOVERY MAP IS IN THE KEY AT ALL."
    );
}
