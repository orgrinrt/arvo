#![allow(dead_code)]

// Probe E. The mean error each operation carries over the whole domain.
//
// Probe B showed the two readings of `half_up` sit on opposite sides of
// translation equivariance. This one shows they sit on opposite sides of bias
// as well, and in the other direction, so the choice between them is a real
// trade rather than a spelling.
//
// err(x) = R(x) - x, summed over every representable x, reported as an exact
// rational. Two domains: the whole one, and the symmetric one that drops the
// most negative value, whose negation is not representable. An odd operation
// has a zero sum on the symmetric domain and generally not on the full one, so
// reporting both separates a property of the operation from a property of
// two's complement.
//
// Build and run:
//   rustc -O e_mean_error.rs -o /tmp/e && /tmp/e > e_output.txt

include!("modes.rs");

/// The summed error over a domain, as a numerator over 2^f.
fn error_sum(m: Mode, w: u32, f: u32, signed: bool, symmetric: bool) -> i64 {
    let s: i64 = 1i64 << f;
    let d = domain(w, signed);
    let (mut lo, hi) = (*d.start(), *d.end());
    if symmetric && signed {
        lo += 1;
    }
    let mut num: i64 = 0;
    for k in lo..=hi {
        // err = round(k/s) - k/s, so s * err = s * round - k.
        num += round(m, k, f) * s - k;
    }
    num
}

/// How many ties the domain holds, which is what a nearest mode's bias is
/// carried by.
fn tie_count(w: u32, f: u32, signed: bool) -> u64 {
    if f == 0 {
        return 0;
    }
    let s: i64 = 1i64 << f;
    let mut n = 0u64;
    for k in domain(w, signed) {
        if 2 * frem(k, s) == s {
            n += 1;
        }
    }
    n
}

fn main() {
    println!("PROBE E: mean error per operation");
    println!();

    println!("== FIXTURE ==");
    let (ok, bad) = check_fixture();
    println!("  {} of {} fixture rows correct", ok, ok + bad);
    if bad != 0 {
        std::process::exit(1);
    }
    println!();

    println!("== CONTROL 1 (the case that must fail) ==");
    println!("  floor is downward on every non-grid value, so its error sum must");
    println!("  be strictly negative. A zero here means the sum is not being");
    println!("  accumulated and every number below is void.");
    let c1 = error_sum(Mode::Floor, 8, 2, true, false);
    println!("    floor at W = 8, F = 2, signed, full domain: {}/4", c1);
    println!();

    println!("== CONTROL 2 (must hold) ==");
    println!("  floor and ceil are conjugate under negation, so on the symmetric");
    println!("  domain their error sums must be exact negatives of one another.");
    let a = error_sum(Mode::Floor, 8, 2, true, true);
    let b = error_sum(Mode::Ceil, 8, 2, true, true);
    println!("    floor {}/4 against ceil {}/4", a, b);
    let c2_ok = a == -b;
    println!();

    println!("== ERROR SUM, W = 8, F = 2, signed. numerator over 4 ==");
    println!("  mode                       full domain   symmetric domain");
    for m in ALL_MODES {
        let full = error_sum(m, 8, 2, true, false);
        let sym = error_sum(m, 8, 2, true, true);
        println!(
            "  {:24}{:>12}{:>19}   {}",
            mode_name(m),
            full,
            sym,
            if sym == 0 {
                "UNBIASED on the symmetric domain"
            } else {
                "biased"
            }
        );
    }
    println!();

    println!("== THE NEAREST MODES ONLY, across widths, symmetric domain ==");
    println!("  the non-tie errors cancel, so what is left is the tie handling.");
    println!("  ties in the domain are counted beside it.");
    for w in [6u32, 8, 10] {
        for f in 1..=3u32 {
            let ties = tie_count(w, f, true);
            print!("  W = {:2} F = {} ties {:>5}: ", w, f, ties);
            for m in [
                Mode::HalfUpTowardPosInf,
                Mode::HalfUpAwayFromZero,
                Mode::HalfDownTowardNegInf,
                Mode::HalfEven,
            ] {
                print!(
                    "{} {:>6}/{:<4}  ",
                    match m {
                        Mode::HalfUpTowardPosInf => "hu+inf",
                        Mode::HalfUpAwayFromZero => "hu-afz",
                        Mode::HalfDownTowardNegInf => "hd-inf",
                        _ => "heven ",
                    },
                    error_sum(m, w, f, true, true),
                    1i64 << f
                );
            }
            println!();
        }
    }
    println!();

    println!("== THE SAME ON AN UNSIGNED DOMAIN ==");
    println!("  where the two readings of half_up are one operation, so the two");
    println!("  columns must be identical. This is the negative control on the");
    println!("  claim that the readings differ: they must not differ here.");
    for f in 1..=3u32 {
        println!(
            "  W = 8 F = {}:  hu+inf {:>6}/{:<3}   hu-afz {:>6}/{:<3}   heven {:>6}/{}",
            f,
            error_sum(Mode::HalfUpTowardPosInf, 8, f, false, false),
            1i64 << f,
            error_sum(Mode::HalfUpAwayFromZero, 8, f, false, false),
            1i64 << f,
            error_sum(Mode::HalfEven, 8, f, false, false),
            1i64 << f
        );
    }
    println!();

    println!("== VERDICTS ==");
    println!(
        "  control 1 (floor sum is negative): {}",
        if c1 < 0 { "PASS" } else { "FAIL" }
    );
    println!(
        "  control 2 (floor and ceil mirror): {}",
        if c2_ok { "PASS" } else { "FAIL" }
    );
    let sound = c1 < 0 && c2_ok;
    println!("  instrument: {}", if sound { "sound" } else { "INVALID" });
    if !sound {
        std::process::exit(1);
    }
}
