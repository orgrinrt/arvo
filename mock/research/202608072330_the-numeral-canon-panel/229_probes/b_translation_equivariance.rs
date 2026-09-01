#![allow(dead_code)]

// Probe B. Translation equivariance, which is the property two canon law rows
// count positions by, and which discriminates the two readings of `half_up`.
//
//   law::fusing_a_multiply_add_preserves_the_answer_under_unsigned
//   law::fusing_a_multiply_add_preserves_the_answer_under_signed_wrapping
//
// The first says five of six swept positions are equivariant under unsigned and
// puts `nearest-half-even` alone in its failing region. The second says three of
// six under signed, holding for `{floor, ceiling, nearest-half-up}` and failing
// for `{toward zero, away from zero, nearest-half-even}`.
//
// A mode is equivariant when R(x + t) == R(x) + t for every integer t, with both
// x and x + t inside the domain.
//
// Build and run:
//   rustc -O b_translation_equivariance.rs -o /tmp/b && /tmp/b > b_output.txt

include!("modes.rs");

/// How many (k, t) pairs break R(x + t) == R(x) + t. Both endpoints are
/// required to be representable, so the count is about the operation and not
/// about the range policy.
fn equivariance_failures(m: Mode, w: u32, f: u32, signed: bool) -> u64 {
    let s: i64 = 1i64 << f;
    let d = domain(w, signed);
    let (lo, hi) = (*d.start(), *d.end());
    let mut n = 0u64;
    for k in lo..=hi {
        let base = round(m, k, f);
        let mut t = 1i64;
        loop {
            let shifted = k + t * s;
            if shifted > hi {
                break;
            }
            if round(m, shifted, f) != base + t {
                n += 1;
            }
            t += 1;
        }
        let mut t = -1i64;
        loop {
            let shifted = k + t * s;
            if shifted < lo {
                break;
            }
            if round(m, shifted, f) != base + t {
                n += 1;
            }
            t -= 1;
        }
    }
    n
}

/// The six positions the canon's fusion rows swept, in the canon's own words,
/// with `nearest-half-up` left to be filled by each reading in turn.
const CANON_SWEEP_FIXED: [Mode; 5] = [
    Mode::Floor,
    Mode::Ceil,
    Mode::TowardZero,
    Mode::AwayFromZero,
    Mode::HalfEven,
];

fn equivariant_count(half_up: Mode, w: u32, f: u32, signed: bool) -> (usize, Vec<&'static str>) {
    let mut n = 0usize;
    let mut names = Vec::new();
    for m in CANON_SWEEP_FIXED
        .iter()
        .copied()
        .chain(std::iter::once(half_up))
    {
        if equivariance_failures(m, w, f, signed) == 0 {
            n += 1;
            names.push(mode_name(m));
        }
    }
    (n, names)
}

fn main() {
    println!("PROBE B: translation equivariance of each candidate operation");
    println!();

    println!("== FIXTURE ==");
    let (ok, bad) = check_fixture();
    println!("  {} of {} fixture rows correct", ok, ok + bad);
    if bad != 0 {
        std::process::exit(1);
    }
    println!();

    println!("== CONTROL 1 (must hold) ==");
    println!("  floor is translation equivariant by construction, on every row.");
    let mut c1 = 0u64;
    for w in [4u32, 6, 8] {
        for f in 0..w {
            for signed in [true, false] {
                c1 += equivariance_failures(Mode::Floor, w, f, signed);
            }
        }
    }
    println!(
        "    total failures for floor over W in {{4,6,8}}, F in 0..W, both: {}",
        c1
    );
    println!();

    println!("== CONTROL 2 (the case that must fail) ==");
    println!("  law::fusing_a_multiply_add_preserves_the_answer_under_unsigned");
    println!("  puts nearest-half-even alone in its failing region, at F in 1..=5,");
    println!("  W = 6, unsigned. So half_even must NOT be equivariant there.");
    let c2 = equivariance_failures(Mode::HalfEven, 6, 1, false);
    println!("    half_even failures at W = 6, F = 1, unsigned: {}", c2);
    println!("    an instrument reporting 0 here contradicts a canon law row and");
    println!("    is measuring nothing; every count below would be void.");
    println!();

    println!("== PER-MODE, W = 6, F in 1..=5, the canon's own sweep widths ==");
    for signed in [true, false] {
        println!(
            "  signedness = {}",
            if signed { "signed" } else { "unsigned" }
        );
        for m in ALL_MODES {
            let mut tot = 0u64;
            let mut per = String::new();
            for f in 1..=5u32 {
                let n = equivariance_failures(m, 6, f, signed);
                tot += n;
                per.push_str(&format!("{:>6}", n));
            }
            println!(
                "    {:22} F=1..5:{}   total {:>6}  {}",
                mode_name(m),
                per,
                tot,
                if tot == 0 {
                    "EQUIVARIANT"
                } else {
                    "not equivariant"
                }
            );
        }
        println!();
    }

    println!("== THE COUNT THE CANON RECORDS, under each reading of half_up ==");
    println!("  law rows say: unsigned five of six, signed three of six.");
    for (label, hu) in [
        (
            "half_up = toward +inf, i.e. floor(x + 1/2)",
            Mode::HalfUpTowardPosInf,
        ),
        (
            "half_up = away from zero, i.e. roundTiesToAway",
            Mode::HalfUpAwayFromZero,
        ),
    ] {
        println!("  reading: {}", label);
        for signed in [false, true] {
            // A position counts as equivariant when it is equivariant at every
            // F the canon row swept, which is 1..=5 at W = 6.
            let mut all: Option<Vec<&'static str>> = None;
            let mut count = usize::MAX;
            for f in 1..=5u32 {
                let (n, names) = equivariant_count(hu, 6, f, signed);
                if n < count {
                    count = n;
                    all = Some(names);
                }
            }
            println!(
                "    {:8}: {} of 6 equivariant -> {:?}",
                if signed { "signed" } else { "unsigned" },
                count,
                all.unwrap()
            );
        }
        println!();
    }

    println!("== VERDICTS ==");
    println!(
        "  control 1 (floor equivariant):     {}",
        if c1 == 0 { "PASS" } else { "FAIL" }
    );
    println!(
        "  control 2 (half_even is not):      {}",
        if c2 > 0 { "PASS" } else { "FAIL" }
    );
    let sound = c1 == 0 && c2 > 0;
    println!("  instrument: {}", if sound { "sound" } else { "INVALID" });
    if !sound {
        std::process::exit(1);
    }
}
