#![allow(dead_code)]

// Probe A. Does a name denote one operation? Count where the candidate
// readings of a name disagree, over the whole representable domain.
//
// Build and run:
//   rustc -O a_pairwise_disagreement.rs -o /tmp/a && /tmp/a > a_output.txt
//
// The controls are stated before the sweep and asserted after it. Two of them
// are cases that MUST FAIL to disagree and two are cases that MUST disagree; a
// run where all four come out the same way is an instrument that measures
// nothing, whatever numbers it prints.
//
// When `half_down` was added to the mode set for probe C, the exhaustive match
// in `short` below refused to compile until it was handled, which is the type
// system reporting that a table claiming to cover the modes did not. Worth
// recording, because the same table in a language without the check would have
// printed a column short and looked fine.

include!("modes.rs");

fn disagreements(a: Mode, b: Mode, w: u32, f: u32, signed: bool) -> u64 {
    let mut n = 0u64;
    for k in domain(w, signed) {
        if round(a, k, f) != round(b, k, f) {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("PROBE A: pairwise disagreement between candidate readings");
    println!("value model: k / 2^F rounded to an integer, k over the whole");
    println!("representable domain of a W-bit format.");
    println!();

    println!("== FIXTURE (textbook values at F = 1) ==");
    let (ok, bad) = check_fixture();
    println!("  {} of {} fixture rows correct", ok, ok + bad);
    if bad != 0 {
        println!("  INSTRUMENT INVALID: the operations are misimplemented.");
        std::process::exit(1);
    }
    println!();

    println!("== CONTROL 1 (positive, against the canon) ==");
    println!("  probe::the_two_toward_zero_spellings_differ_and_by_how_much");
    println!("  records bit-drop against toward-zero at W = 8 as 64 at F = 1,");
    println!("  96 at F = 2, 120 at F = 4 and 127 at F = 7, signed.");
    let expect = [(1u32, 64u64), (2, 96), (4, 120), (7, 127)];
    let mut c1_ok = true;
    for (f, want) in expect {
        let got = disagreements(Mode::BitDrop, Mode::TowardZero, 8, f, true);
        let verdict = if got == want { "match" } else { "MISMATCH" };
        if got != want {
            c1_ok = false;
        }
        println!(
            "    F = {}: this instrument {}, canon {} -> {}",
            f, got, want, verdict
        );
    }
    println!();

    println!("== CONTROL 2 (positive, against the ratified note) ==");
    println!("  ruling::the_ambiguous_rounding_word_is_retired_for_six_explicit_names");
    println!("  records that bit truncation of a two's complement value is floor.");
    println!("  So bit_drop against floor must be 0 on every row.");
    let mut c2_max = 0u64;
    for w in [4u32, 6, 8, 10] {
        for f in 0..w {
            for signed in [true, false] {
                let n = disagreements(Mode::BitDrop, Mode::Floor, w, f, signed);
                if n > c2_max {
                    c2_max = n;
                }
            }
        }
    }
    println!(
        "    worst disagreement over W in {{4,6,8,10}}, F in 0..W, both signedness: {}",
        c2_max
    );
    println!();

    println!("== CONTROL 3 (the case that must fail) ==");
    println!("  The two readings of `half_up` must DISAGREE somewhere on a");
    println!("  signed domain with F >= 1. A zero here means the instrument is");
    println!("  comparing one operation with itself and every number below is void.");
    let c3 = disagreements(
        Mode::HalfUpTowardPosInf,
        Mode::HalfUpAwayFromZero,
        8,
        1,
        true,
    );
    println!("    W = 8, F = 1, signed: {}", c3);
    println!();

    println!("== CONTROL 4 (the case that must not fail) ==");
    println!("  The same two readings must AGREE on every unsigned row and on");
    println!("  every F = 0 row, because neither negatives nor ties exist there.");
    let mut c4_max = 0u64;
    for w in [4u32, 6, 8, 10] {
        for f in 0..w {
            let n = disagreements(
                Mode::HalfUpTowardPosInf,
                Mode::HalfUpAwayFromZero,
                w,
                f,
                false,
            );
            if n > c4_max {
                c4_max = n;
            }
        }
        let n = disagreements(
            Mode::HalfUpTowardPosInf,
            Mode::HalfUpAwayFromZero,
            w,
            0,
            true,
        );
        if n > c4_max {
            c4_max = n;
        }
    }
    println!("    worst disagreement over those rows: {}", c4_max);
    println!();

    println!("== SWEEP: half_up, the two readings, signed ==");
    println!("  W    F   disagreeing values   of domain   predicted 2^(W-1-F)");
    for w in [4u32, 6, 8, 10, 12] {
        for f in 0..w {
            let n = disagreements(
                Mode::HalfUpTowardPosInf,
                Mode::HalfUpAwayFromZero,
                w,
                f,
                true,
            );
            let total = 1u64 << w;
            let predicted = if f == 0 { 0 } else { 1u64 << (w - 1 - f) };
            let flag = if n == predicted {
                ""
            } else {
                "   <- MODEL BROKEN"
            };
            println!(
                "  {:2}   {:2}   {:18}   {:9}   {:9}{}",
                w, f, n, total, predicted, flag
            );
        }
    }
    println!();

    println!("== SWEEP: full pairwise table, W = 8, F = 4 ==");
    for signed in [true, false] {
        println!(
            "  signedness = {}",
            if signed { "signed" } else { "unsigned" }
        );
        print!("  {:24}", "");
        for b in ALL_MODES {
            print!("{:>7}", short(b));
        }
        println!();
        for a in ALL_MODES {
            print!("  {:24}", mode_name(a));
            for b in ALL_MODES {
                print!("{:>7}", disagreements(a, b, 8, 4, signed));
            }
            println!();
        }
        println!();
    }

    println!("== SWEEP: every pair that agrees on unsigned and differs on signed ==");
    println!("  the class the ratified ruling retired a word for. W = 8, F = 4.");
    for (i, a) in ALL_MODES.iter().enumerate() {
        for b in ALL_MODES.iter().skip(i + 1) {
            let u = disagreements(*a, *b, 8, 4, false);
            let s = disagreements(*a, *b, 8, 4, true);
            if u == 0 && s > 0 {
                println!(
                    "    {:24} vs {:24}  unsigned 0, signed {}",
                    mode_name(*a),
                    mode_name(*b),
                    s
                );
            }
        }
    }
    println!();

    println!("== VERDICTS ==");
    println!(
        "  control 1 (canon numbers reproduced): {}",
        if c1_ok { "PASS" } else { "FAIL" }
    );
    println!(
        "  control 2 (bit_drop == floor):        {}",
        if c2_max == 0 { "PASS" } else { "FAIL" }
    );
    println!(
        "  control 3 (half_up readings differ):  {}",
        if c3 > 0 { "PASS" } else { "FAIL" }
    );
    println!(
        "  control 4 (and agree where they must):{}",
        if c4_max == 0 { " PASS" } else { " FAIL" }
    );
    let all = c1_ok && c2_max == 0 && c3 > 0 && c4_max == 0;
    println!("  instrument: {}", if all { "sound" } else { "INVALID" });
    if !all {
        std::process::exit(1);
    }
}

fn short(m: Mode) -> &'static str {
    match m {
        Mode::Floor => "flr",
        Mode::Ceil => "ceil",
        Mode::TowardZero => "tz",
        Mode::AwayFromZero => "afz",
        Mode::BitDrop => "drop",
        Mode::HalfUpTowardPosInf => "hu+inf",
        Mode::HalfUpAwayFromZero => "hu-afz",
        Mode::HalfDownTowardNegInf => "hd-inf",
        Mode::HalfEven => "heven",
    }
}
