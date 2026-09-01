// k1. Are the mode NAMES the corpus writes distinct FUNCTIONS, and where?
//
// Every question in this seat's brief reduces to one thing: a predicate names a
// mode by a word, and a word is not a function. So this probe never asks what a
// probe's author called a mode. It implements each candidate function from its
// definition and compares them pointwise on the domains the corpus sweeps.
//
// The candidates are the six ratified names plus the two things that are NOT
// settled by those six:
//
//   floor, ceil, toward_zero, half_even        -- four of the ratified six
//   half_up_pos_inf                            -- `half_up` read as ties to +inf
//   half_up_away                               -- `half_up` read as ties away from 0
//   away_from_zero                             -- named by three predicates,
//                                                 not among the ratified six
//
// `stochastic` is deliberately absent: it is not a function, so pointwise
// equality is not the question to ask of it, and no entry in the nine names it.
//
// PREDICTIONS, written before the first run.
//
//   K1  On a NON-NEGATIVE domain there are exactly three coincidences:
//         floor == toward_zero, ceil == away_from_zero,
//         half_up_pos_inf == half_up_away.
//       Every other pair differs. So the five modes the unsigned fusion row
//       names denote three functions, and `away from zero` there is `ceil`.
//
//   K2  On a SIGNED domain all seven are pairwise distinct. In particular
//       `away_from_zero` differs from each of the six ratified names, so it is
//       not a spelling of one of them and the vocabulary is short a name there.
//
//   K3  `half_up` is ambiguous on a signed domain in exactly the way the
//       retired word was: its two readings are different functions, and both
//       readings are implemented in this panel's own committed probes.
//
//   K4  At f = 0 every candidate is the identity, so the rounding axis is
//       degenerate there and its honest value is `exact` rather than a mode set.
//
// CONTROLS. A comparison probe with no failing pair proves nothing.
//
//   C1  At least one pair must DIFFER on the non-negative domain, or the
//       comparison is vacuous and every "coincidence" below is an artifact.
//   C2  Each coincidence found on the non-negative domain must FAIL on the
//       signed domain, measured, or the instrument is insensitive to sign and
//       K1 is not evidence for anything.
//   C3  A deliberately false claim, floor == ceil, must be refuted on both
//       domains. If it survives, the equality test is not testing equality.
//   C4  The f = 0 identity claim must be checked against f >= 1 on the same
//       modes: if nothing differs at f >= 1 either, K4 is measuring the sweep
//       rather than the shift.
//
// Build and run:
//   rustc --edition 2024 -O -o k1 k1_the_modes_as_functions.rs && ./k1

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Mode {
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    HalfUpPosInf,
    HalfUpAway,
    HalfEven,
}

const MODES: [Mode; 7] = [
    Mode::Floor,
    Mode::Ceil,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::HalfUpPosInf,
    Mode::HalfUpAway,
    Mode::HalfEven,
];

fn name(m: Mode) -> &'static str {
    match m {
        Mode::Floor => "floor",
        Mode::Ceil => "ceil",
        Mode::TowardZero => "toward_zero",
        Mode::AwayFromZero => "away_from_zero",
        Mode::HalfUpPosInf => "half_up(+inf)",
        Mode::HalfUpAway => "half_up(away)",
        Mode::HalfEven => "half_even",
    }
}

/// Place the exact value `p / 2^f` on the integer grid under mode `m`.
///
/// Written from each mode's DEFINITION, not copied from any panel probe, so
/// agreement with those probes is a reproduction rather than a restatement.
/// `q` is the floor quotient and `r` the euclidean remainder in `[0, d)`, so
/// the exact value is `q + r/d` and every branch below is a choice about `r`.
fn rnd(p: i128, f: u32, m: Mode) -> i128 {
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    match m {
        Mode::Floor => q,
        Mode::Ceil => q + if r == 0 { 0 } else { 1 },
        // toward zero: floor above the origin, ceil below it
        Mode::TowardZero => {
            if p >= 0 {
                q
            } else {
                q + if r == 0 { 0 } else { 1 }
            }
        }
        // away from zero: ceil above the origin, floor below it
        Mode::AwayFromZero => {
            if p >= 0 {
                q + if r == 0 { 0 } else { 1 }
            } else {
                q
            }
        }
        // ties to the numerically greater neighbour
        Mode::HalfUpPosInf => q + if 2 * r >= d { 1 } else { 0 },
        // ties to the neighbour further from the origin
        Mode::HalfUpAway => {
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if p >= 0 {
                q + 1
            } else {
                q
            }
        }
        Mode::HalfEven => {
            if 2 * r > d {
                q + 1
            } else if 2 * r < d {
                q
            } else if q.rem_euclid(2) == 0 {
                q
            } else {
                q + 1
            }
        }
    }
}

/// Do two modes agree at every point of `lo..=hi` for every `f` in `fs`?
/// Returns the first disagreeing `(p, f)` if there is one.
fn first_disagreement(
    a: Mode,
    b: Mode,
    lo: i128,
    hi: i128,
    fs: &[u32],
) -> Option<(i128, u32, i128, i128)> {
    for &f in fs {
        for p in lo..=hi {
            let (x, y) = (rnd(p, f, a), rnd(p, f, b));
            if x != y {
                return Some((p, f, x, y));
            }
        }
    }
    None
}

fn matrix(label: &str, lo: i128, hi: i128, fs: &[u32]) -> Vec<(Mode, Mode)> {
    println!();
    println!(
        "-- pairwise equality on {label}, p in [{lo}, {hi}], f in {fs:?} --"
    );
    let mut same = Vec::new();
    let mut differ = 0usize;
    for i in 0..MODES.len() {
        for j in (i + 1)..MODES.len() {
            let (a, b) = (MODES[i], MODES[j]);
            match first_disagreement(a, b, lo, hi, fs) {
                None => {
                    println!("  {:<16} == {:<16}  SAME everywhere swept", name(a), name(b));
                    same.push((a, b));
                }
                Some((p, f, x, y)) => {
                    differ += 1;
                    if differ <= 3 {
                        println!(
                            "  {:<16} != {:<16}  first at p={p}, f={f}: {x} vs {y}",
                            name(a),
                            name(b)
                        );
                    }
                }
            }
        }
    }
    println!(
        "  {} coincident pair(s), {} distinct pair(s) of {} total",
        same.len(),
        differ,
        MODES.len() * (MODES.len() - 1) / 2
    );
    same
}

fn main() {
    // The non-negative domain is what every unsigned sweep in the corpus
    // reaches; the signed domain is what the signed ones reach. The widths are
    // small on purpose: a single counterexample settles an equality, so the
    // sweep only has to be wide enough to contain one.
    let fs = [1u32, 2, 3, 4];
    let (ulo, uhi) = (0i128, 255i128);
    let (slo, shi) = (-256i128, 255i128);

    println!("k1. are the mode names distinct functions, and on which domain?");

    let same_unsigned = matrix("a NON-NEGATIVE domain", ulo, uhi, &fs);
    let same_signed = matrix("a SIGNED domain", slo, shi, &fs);

    // ---------------------------------------------------------------- K1
    println!();
    println!("K1 (non-negative coincidences are exactly three, and are these three):");
    let want: [(Mode, Mode); 3] = [
        (Mode::Floor, Mode::TowardZero),
        (Mode::Ceil, Mode::AwayFromZero),
        (Mode::HalfUpPosInf, Mode::HalfUpAway),
    ];
    let k1 = same_unsigned.len() == 3
        && want
            .iter()
            .all(|(a, b)| same_unsigned.iter().any(|(x, y)| x == a && y == b));
    println!("  {}", if k1 { "HOLDS" } else { "REFUTED" });

    // ---------------------------------------------------------------- K2
    println!();
    println!("K2 (on a signed domain every pair is distinct, so `away_from_zero`");
    println!("    is not a spelling of any ratified name):");
    let k2 = same_signed.is_empty();
    println!("  {}", if k2 { "HOLDS" } else { "REFUTED" });
    if k2 {
        for m in MODES {
            if m == Mode::AwayFromZero {
                continue;
            }
            let d = first_disagreement(Mode::AwayFromZero, m, slo, shi, &fs)
                .or_else(|| first_disagreement(m, Mode::AwayFromZero, slo, shi, &fs));
            match d {
                Some((p, f, x, y)) => println!(
                    "    away_from_zero != {:<16} witness p={p}, f={f}: {x} vs {y}",
                    name(m)
                ),
                None => println!("    away_from_zero == {} (K2 refuted)", name(m)),
            }
        }
    }

    // ---------------------------------------------------------------- K3
    println!();
    println!("K3 (`half_up` names two functions on a signed domain):");
    match first_disagreement(Mode::HalfUpPosInf, Mode::HalfUpAway, slo, shi, &fs) {
        Some((p, f, x, y)) => println!(
            "  HOLDS. witness p={p}, f={f}: ties-to-+inf gives {x}, ties-away gives {y}"
        ),
        None => println!("  REFUTED: the two readings agree on the signed domain"),
    }

    // ---------------------------------------------------------------- K4
    println!();
    println!("K4 (at f = 0 every mode is the identity):");
    let mut k4 = true;
    for m in MODES {
        for p in slo..=shi {
            if rnd(p, 0, m) != p {
                k4 = false;
                println!("  REFUTED by {} at p={p}", name(m));
            }
        }
    }
    println!("  {}", if k4 { "HOLDS for all 7 modes" } else { "REFUTED" });

    // ---------------------------------------------------------------- controls
    println!();
    println!("== controls ==");

    let c1 = same_unsigned.len() < MODES.len() * (MODES.len() - 1) / 2;
    println!(
        "C1 some pair differs on the non-negative domain: {}",
        if c1 { "pass" } else { "FAIL (comparison vacuous)" }
    );

    let mut c2 = true;
    for (a, b) in &same_unsigned {
        match first_disagreement(*a, *b, slo, shi, &fs) {
            Some((p, f, x, y)) => println!(
                "C2 {:<16} == {:<16} on non-negative, differs at p={p}, f={f}: {x} vs {y}",
                name(*a),
                name(*b)
            ),
            None => {
                c2 = false;
                println!(
                    "C2 {} == {} on BOTH domains: the sign sensitivity is unproven",
                    name(*a),
                    name(*b)
                );
            }
        }
    }
    println!(
        "C2 every non-negative coincidence fails when negatives are admitted: {}",
        if c2 { "pass" } else { "FAIL" }
    );

    let c3u = first_disagreement(Mode::Floor, Mode::Ceil, ulo, uhi, &fs).is_some();
    let c3s = first_disagreement(Mode::Floor, Mode::Ceil, slo, shi, &fs).is_some();
    println!(
        "C3 the false claim floor == ceil is refuted on both domains: {}",
        if c3u && c3s { "pass" } else { "FAIL" }
    );

    let mut c4 = false;
    for m in MODES {
        for p in slo..=shi {
            if rnd(p, 1, m) != p {
                c4 = true;
                break;
            }
        }
    }
    println!(
        "C4 the modes are not the identity at f = 1, so K4 measures the shift: {}",
        if c4 { "pass" } else { "FAIL" }
    );

    let all = k1 && k2 && k4 && c1 && c2 && c3u && c3s && c4;
    println!();
    println!("k1 verdict: {}", if all { "every prediction and control holds" } else { "SOMETHING FAILED, read above" });
}
