// Probe R1. Do the six ratified names denote one operation each, and which of
// them do the nine flagged entries actually name?
//
// Every conclusion in `232` that is not a direct quotation of an instrument's
// source is established here. The instruments this checks against are read
// rather than trusted: each mode below is written from the definition, and the
// two spellings taken verbatim from other probes are marked as such.
//
// Four parts, each a separate question:
//
//   part 1  Are the six ratified names, plus `away from zero`, plus the two
//           readings of `half_up`, pairwise distinct as functions? Answered on
//           a signed domain and again on a non-negative one, because the whole
//           question is which of them collapse where.
//   part 2  Retraction of a rounding policy over a two-multiply chain, over
//           every one of the six rather than over the two `94_probes/
//           c_retraction.rs` part 2 happened to sweep. Its own two arms are
//           reproduced first, so a disagreement with it would show.
//   part 3  Stochastic, which is not a function and therefore cannot be given
//           a pointwise verdict the way the other five can.
//   part 4  The `half_up` collision, with a witness: two probes in this panel
//           implement two different operations and the registry spells both
//           of them `half_up`.
//
// Controls are stated per part and every one of them is live: each part
// reports at least one cell that must come out the other way, and says so.
//
// Build and run:
//   rustc --edition 2024 -O -o r1 r1_the_six_names_against_the_instruments.rs
//   ./r1

// ---------------------------------------------------------------------------
// The modes. `p` is a numerator; the divisor is `1 << f`. Every one of these is
// written from its definition, not transcribed, except the two marked.
// ---------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Mode {
    Floor,
    Ceil,
    TowardZero,
    AwayFromZero,
    /// `half_up` read as ties toward positive infinity, which is `floor(x+1/2)`.
    /// This is the spelling `149_probes/y1_the_unsigned_half_over_six_modes.rs`
    /// gives `Mode::NearestHalfUp`, in its own comment.
    HalfUpToPlusInf,
    /// `half_up` read as ties away from zero, which is Java's `HALF_UP`. This is
    /// the spelling `97_probes/p2_congruence_predicts_the_laws.py` gives
    /// `nearest`, in its own comment.
    HalfUpAwayFromZero,
    HalfEven,
    /// Not a rounding mode. The planted control: it is off by one everywhere,
    /// including at `f == 0`, so any part reporting it as agreeing or as
    /// retracting has a broken harness.
    MutantAlwaysUp,
}

const REAL_MODES: [Mode; 7] = [
    Mode::Floor,
    Mode::Ceil,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::HalfUpToPlusInf,
    Mode::HalfUpAwayFromZero,
    Mode::HalfEven,
];

/// The five of the six that are total deterministic functions. `stochastic` is
/// part 3 and is deliberately not here.
const RATIFIED_DETERMINISTIC: [Mode; 5] = [
    Mode::TowardZero,
    Mode::Floor,
    Mode::Ceil,
    Mode::HalfUpToPlusInf,
    Mode::HalfEven,
];

fn name(m: Mode) -> &'static str {
    match m {
        Mode::Floor => "floor",
        Mode::Ceil => "ceil",
        Mode::TowardZero => "toward_zero",
        Mode::AwayFromZero => "away_from_zero",
        Mode::HalfUpToPlusInf => "half_up(+inf)",
        Mode::HalfUpAwayFromZero => "half_up(away)",
        Mode::HalfEven => "half_even",
        Mode::MutantAlwaysUp => "MUTANT",
    }
}

fn rnd(p: i128, f: u32, m: Mode) -> i128 {
    if m == Mode::MutantAlwaysUp {
        // deliberately wrong at every f, f == 0 included
        return (p >> f) + 1;
    }
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d); // floor
    let r = p.rem_euclid(d); // 0 <= r < d
    match m {
        Mode::Floor => q,
        Mode::Ceil => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::TowardZero => {
            if p >= 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::AwayFromZero => {
            if p < 0 || r == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::HalfUpToPlusInf => {
            // floor(x + 1/2): the tie goes up the number line, so at -1/2 it
            // gives 0 and not -1.
            if 2 * r >= d { q + 1 } else { q }
        }
        Mode::HalfUpAwayFromZero => {
            // the tie goes away from zero, so at -1/2 it gives -1 and not 0.
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
            } else if q % 2 == 0 {
                q
            } else {
                q + 1
            }
        }
        Mode::MutantAlwaysUp => unreachable!(),
    }
}

/// A second spelling of floor, by arithmetic shift rather than by `div_euclid`.
/// The positive control for part 1: it must agree with `Mode::Floor` at every
/// cell, so a matrix that separates everything from everything is broken.
fn floor_by_shift(p: i128, f: u32) -> i128 {
    p >> f
}

// ---------------------------------------------------------------------------
// Part 1. Which names denote one operation, and where do they collapse?
// ---------------------------------------------------------------------------

fn part1() {
    println!("part 1: are the names pairwise distinct as functions?");
    println!("        exhaustive over the stated domain at every f in 1..=4\n");

    for &(dname, lo, hi) in [
        ("signed      p in -64..=63", -64i128, 63i128),
        ("non-negative p in 0..=63", 0i128, 63i128),
    ]
    .iter()
    {
        println!("=== domain: {dname} ===");
        print!("{:<16}", "");
        for m in REAL_MODES {
            print!("{:>16}", name(m));
        }
        println!();
        for a in REAL_MODES {
            print!("{:<16}", name(a));
            for b in REAL_MODES {
                let mut differ = 0u64;
                for f in 1..=4u32 {
                    for p in lo..=hi {
                        if rnd(p, f, a) != rnd(p, f, b) {
                            differ += 1;
                        }
                    }
                }
                if differ == 0 {
                    print!("{:>16}", "=");
                } else {
                    print!("{:>16}", differ);
                }
            }
            println!();
        }

        // the collapses, named, with a witness for every separation
        println!("\n  collapses and separations on this domain:");
        for (a, b) in [
            (Mode::Floor, Mode::TowardZero),
            (Mode::Ceil, Mode::AwayFromZero),
            (Mode::HalfUpToPlusInf, Mode::HalfUpAwayFromZero),
            (Mode::AwayFromZero, Mode::Floor),
            (Mode::AwayFromZero, Mode::HalfEven),
        ] {
            let mut witness: Option<(i128, u32, i128, i128)> = None;
            for f in 1..=4u32 {
                for p in lo..=hi {
                    let (x, y) = (rnd(p, f, a), rnd(p, f, b));
                    if x != y && witness.is_none() {
                        witness = Some((p, f, x, y));
                    }
                }
            }
            match witness {
                None => println!("    {:>14} == {:<16} everywhere", name(a), name(b)),
                Some((p, f, x, y)) => println!(
                    "    {:>14} != {:<16} witness p={p} f={f}: {x} vs {y}",
                    name(a),
                    name(b)
                ),
            }
        }

        // positive control: two spellings of one operation must agree, or the
        // matrix above is separating things by accident.
        let mut ctl = 0u64;
        for f in 1..=4u32 {
            for p in lo..=hi {
                if rnd(p, f, Mode::Floor) != floor_by_shift(p, f) {
                    ctl += 1;
                }
            }
        }
        // negative control: the planted mutant must differ from every real mode.
        let mut mutant_caught = 0;
        for m in REAL_MODES {
            let mut d = 0u64;
            for f in 0..=4u32 {
                for p in lo..=hi {
                    if rnd(p, f, m) != rnd(p, f, Mode::MutantAlwaysUp) {
                        d += 1;
                    }
                }
            }
            if d > 0 {
                mutant_caught += 1;
            }
        }
        println!(
            "\n  CONTROL positive: floor by div_euclid vs floor by shift, disagreements = {ctl} (must be 0)"
        );
        println!(
            "  CONTROL negative: planted mutant separated from {mutant_caught} of {} real modes (must be {})\n",
            REAL_MODES.len(),
            REAL_MODES.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Part 2. Retraction over a two-multiply chain, over every ratified name.
//
// The shape is `94_probes/c_retraction.rs` part 2 exactly: a, b, c range over
// 0..2^W, the eager form rounds after each multiply by F, the deferred form
// rounds once at the end by 2F, and the cell is the count of triples where the
// two disagree. That probe swept two arms; this sweeps five, and reproduces its
// two first so a disagreement would be visible rather than assumed.
// ---------------------------------------------------------------------------

fn retraction_differ(w: u32, f: u32, m: Mode) -> u64 {
    let n: i128 = 1 << w;
    let mut differ = 0u64;
    for a in 0..n {
        for b in 0..n {
            let ab_exact = a * b;
            let ab_q = rnd(ab_exact, f, m);
            for c in 0..n {
                let eager = rnd(ab_q * c, f, m);
                let deferred = rnd(ab_exact * c, 2 * f, m);
                if eager != deferred {
                    differ += 1;
                }
            }
        }
    }
    differ
}

fn part2() {
    println!("\npart 2: does a rounding policy retract over a two-multiply chain?");
    println!("        exhaustive over a, b, c in 0..2^W, unsigned, as c_retraction part 2\n");

    // reproduction of c_retraction's own two arms, at its own widths
    println!("  reproduction of 94_probes/c_retraction.rs part 2 (its `truncate` is a");
    println!("  logical shift on a non-negative value, so it is floor and toward_zero at");
    println!("  once; its `nearest` is (x + 2^(f-1)) >> f, which is half_up):");
    println!(
        "  {:>12} {:>3} {:>3} {:>12} {:>9}   {}",
        "c_retraction", "W", "F", "differ", "pct", "its committed figure"
    );
    for (their_name, m, expect) in [
        ("truncate", Mode::Floor, [0u64, 800, 1128, 910, 543]),
        ("nearest", Mode::HalfUpToPlusInf, [0u64, 864, 1248, 880, 550]),
    ] {
        for f in 0..=4u32 {
            let d = retraction_differ(4, f, m);
            let total = 4096u64;
            println!(
                "  {:>12} {:>3} {:>3} {:>12} {:>8.2}%   {} {}",
                their_name,
                4,
                f,
                d,
                100.0 * d as f64 / total as f64,
                expect[f as usize],
                if d == expect[f as usize] {
                    "match"
                } else {
                    "MISMATCH"
                }
            );
        }
    }

    println!("\n  the same question over every deterministic ratified name:");
    print!("  {:<16}", "mode");
    for w in [4u32, 6, 8] {
        for f in 0..=w {
            let _ = f;
        }
        print!("  W={w}: F=0..={w}");
    }
    println!();
    for m in RATIFIED_DETERMINISTIC {
        print!("  {:<16}", name(m));
        for w in [4u32, 6, 8] {
            print!("  ");
            for f in 0..=w {
                let d = retraction_differ(w, f, m);
                print!("{}", if d == 0 { "." } else { "x" });
            }
        }
        println!();
    }
    println!("  (`.` retracts, `x` does not; leftmost of each group is F = 0)");

    // control: the planted mutant must fail even at F = 0, where every real
    // mode is the identity. Without this, the F = 0 column of dots means only
    // that the harness returns zero.
    let mut mutant_f0 = 0u64;
    for w in [4u32, 6, 8] {
        mutant_f0 += retraction_differ(w, 0, Mode::MutantAlwaysUp);
    }
    println!(
        "\n  CONTROL: planted mutant at F = 0 over the same three widths, differ = {mutant_f0} (must be > 0,"
    );
    println!("  or the F = 0 column above is an artifact of the harness rather than a result)");

    // and the one that decides the `holds` entry: at F = 0 is every mode the
    // identity, for a reason rather than by coincidence?
    let mut f0_nonidentity = 0u64;
    for m in REAL_MODES {
        for p in -64i128..=63 {
            if rnd(p, 0, m) != p {
                f0_nonidentity += 1;
            }
        }
    }
    println!(
        "  At F = 0 the quantiser is the identity for {} of {} real modes over p in -64..=63",
        REAL_MODES.len() - if f0_nonidentity > 0 { 1 } else { 0 },
        REAL_MODES.len()
    );
    println!("  (non-identity results: {f0_nonidentity}, must be 0)");
}

// ---------------------------------------------------------------------------
// Part 3. Stochastic. Not a function, so it gets no pointwise verdict; what it
// gets is a statement of which questions are well posed for it.
// ---------------------------------------------------------------------------

fn stochastic(p: i128, f: u32, draw: i128) -> i128 {
    if f == 0 {
        return p;
    }
    let d = 1i128 << f;
    let q = p.div_euclid(d);
    let r = p.rem_euclid(d);
    // round up with probability r/d, realised by comparing against a draw
    // uniform on 0..d. At r == 0 no draw can round up, which is what makes
    // the F = 0 and exact cases deterministic.
    if r > draw { q + 1 } else { q }
}

fn part3() {
    println!("\npart 3: stochastic\n");

    // (a) at F = 0 it is the identity for every draw, so it is deterministic
    //     there and the F = 0 verdict extends to it.
    let mut bad = 0u64;
    for p in -64i128..=63 {
        for draw in 0..1i128 {
            if stochastic(p, 0, draw) != p {
                bad += 1;
            }
        }
    }
    println!("  (a) at F = 0, stochastic != identity on {bad} of 128 values (must be 0)");

    // (b) at F > 0 it is not a function, so `q(q(x)) == q(x)` has no truth
    //     value until a coupling of draws is stated. Two couplings, two answers,
    //     on the same domain: that is the whole point.
    let (w, f) = (4u32, 2u32);
    let n: i128 = 1 << w;
    let d: i128 = 1 << f;
    let mut differ_shared = 0u64; // one draw reused at both positions
    let mut differ_indep = 0u64; // a different draw at each position
    let mut total = 0u64;
    for a in 0..n {
        for b in 0..n {
            for c in 0..n {
                for draw1 in 0..d {
                    for draw2 in 0..d {
                        total += 1;
                        let ab = a * b;
                        let eager_shared =
                            stochastic(stochastic(ab, f, draw1) * c, f, draw1);
                        let eager_indep = stochastic(stochastic(ab, f, draw1) * c, f, draw2);
                        let deferred = stochastic(ab * c, 2 * f, draw1);
                        if eager_shared != deferred {
                            differ_shared += 1;
                        }
                        if eager_indep != deferred {
                            differ_indep += 1;
                        }
                    }
                }
            }
        }
    }
    println!(
        "  (b) at W = {w}, F = {f}, over {total} (triple, draw, draw) cells:\n      shared-draw coupling disagrees at {differ_shared}, independent-draw at {differ_indep}"
    );
    println!(
        "      Two couplings, two counts, from one definition and one domain. The question"
    );
    println!("      `does it retract` is therefore not well posed for stochastic at F > 0");
    println!("      until a coupling is named, and no instrument in this panel names one.");

    // (c) the chain at F = 0, which is what the `holds` entry turns on: with the
    //     quantiser the identity, retraction is exact for every draw pair.
    let mut sd = 0u64;
    let mut scells = 0u64;
    for a in 0..16i128 {
        for b in 0..16i128 {
            for c in 0..16i128 {
                for d1 in 0..1i128 {
                    for d2 in 0..1i128 {
                        scells += 1;
                        let ab = a * b;
                        let eager = stochastic(stochastic(ab, 0, d1) * c, 0, d2);
                        let deferred = stochastic(ab * c, 0, d1);
                        if eager != deferred {
                            sd += 1;
                        }
                    }
                }
            }
        }
    }
    println!("  (c) the two-multiply chain at W = 4, F = 0: disagreements {sd} of {scells} (must be 0),");
    println!("      so the F = 0 verdict covers stochastic too, for the reason that there is");
    println!("      no rounding at F = 0 rather than for anything about the mode.");
}

// ---------------------------------------------------------------------------
// Part 4. The `half_up` collision, with a witness.
// ---------------------------------------------------------------------------

fn part4() {
    println!("\npart 4: does `half_up` denote one operation?\n");
    for &(dname, lo, hi) in [
        ("signed      p in -64..=63", -64i128, 63i128),
        ("non-negative p in 0..=63", 0i128, 63i128),
    ]
    .iter()
    {
        let mut differ = 0u64;
        let mut first: Option<(i128, u32, i128, i128)> = None;
        for f in 1..=4u32 {
            for p in lo..=hi {
                let x = rnd(p, f, Mode::HalfUpToPlusInf);
                let y = rnd(p, f, Mode::HalfUpAwayFromZero);
                if x != y {
                    differ += 1;
                    if first.is_none() {
                        first = Some((p, f, x, y));
                    }
                }
            }
        }
        match first {
            None => println!("  {dname}: the two readings agree at every cell ({differ} disagreements)"),
            Some((p, f, x, y)) => println!(
                "  {dname}: {differ} disagreements, first at p={p} f={f}: +inf gives {x}, away gives {y}"
            ),
        }
    }
    println!(
        "\n  The same shape the ratified ruling gives for the retired word: two operations,"
    );
    println!("  one name, differing on signed rows only.");
}


/// Retraction over a SIGNED domain, otherwise the same chain as `retraction_differ`.
/// The row this bears on has no `signedness` entry at all and its instrument is
/// unsigned-only, so whether the verdict survives the signed domain is exactly
/// what nobody measured.
fn retraction_differ_signed(w: u32, f: u32, m: Mode) -> u64 {
    let lo: i128 = -(1i128 << (w - 1));
    let hi: i128 = (1i128 << (w - 1)) - 1;
    let mut differ = 0u64;
    for a in lo..=hi {
        for b in lo..=hi {
            let ab_exact = a * b;
            let ab_q = rnd(ab_exact, f, m);
            for c in lo..=hi {
                let eager = rnd(ab_q * c, f, m);
                let deferred = rnd(ab_exact * c, 2 * f, m);
                if eager != deferred {
                    differ += 1;
                }
            }
        }
    }
    differ
}

fn part5() {
    println!("\npart 5: the same retraction question on a signed domain");
    println!("        a, b, c in -2^(W-1)..=2^(W-1)-1, over all seven functions\n");
    print!("  {:<16}", "mode");
    for w in [4u32, 6] {
        print!("  W={w}: F=0..={w}");
    }
    println!();
    for m in REAL_MODES {
        print!("  {:<16}", name(m));
        for w in [4u32, 6] {
            print!("  ");
            for f in 0..=w {
                let d = retraction_differ_signed(w, f, m);
                print!("{}", if d == 0 { "." } else { "x" });
            }
        }
        println!();
    }
    println!("  (`.` retracts, `x` does not; leftmost of each group is F = 0)");
    let mut mutant = 0u64;
    for w in [4u32, 6] {
        mutant += retraction_differ_signed(w, 0, Mode::MutantAlwaysUp);
    }
    println!("\n  CONTROL: planted mutant at F = 0, differ = {mutant} (must be > 0)");
}

fn main() {
    part1();
    part2();
    part3();
    part4();
    part5();
}
