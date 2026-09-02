//! Driver for s3. std-linked scaffolding so the checks print. The definition itself is
//! `no_std` and lives in the included file; that file compiles as a `--crate-type lib`
//! on its own, which is the constraint check that matters.

use s3::*;

fn main() {
    println!("s3: one definition of a numeral, projected to both vocabularies");
    println!();

    println!("check 1: the general derivation, at the constant case, recovers 2^W");
    let ok = check_width_pair_recovers_two_to_the_w();
    println!(
        "         width box (12 rows, W in 0..12, F in 0..32): {}",
        if ok { "PASS" } else { "FAIL" }
    );
    println!(
        "         compile-time asserts: W=4 F=2 -> {}, W=4 F=0 -> {}, W=1 F=32 -> {}",
        WIDTH_COUNT_4_2, WIDTH_COUNT_4_0, WIDTH_COUNT_1_32
    );
    println!();

    println!("check 2: the SAME derivation on families the width pair cannot name");
    let (f4, f3, t4) = float_counts();
    println!(
        "         float p=4, binades -3..3 : {} magnitudes (7 binades * 8 + zero = 57)",
        f4
    );
    println!(
        "         float p=3, binades -2..2 : {} magnitudes (5 binades * 4 + zero = 21)",
        f3
    );
    println!("         tapered slope-two p=4    : {} magnitudes", t4);
    println!(
        "         float counts as expected : {}",
        if f4 == 57 && f3 == 21 { "PASS" } else { "FAIL" }
    );
    println!();

    println!("check 3: the discriminating fact, which is why there is no fraction width");
    let ((fx_lo, fx_hi), (fl_lo, fl_hi)) = step_exponent_spread();
    println!(
        "         constant grid F=2: f(-2) = {}, f(5) = {}  -> one step exponent, so F names it",
        fx_lo, fx_hi
    );
    println!(
        "         slope-one grid p=4: f(-3) = {}, f(3) = {}  -> {} distinct, so no F exists",
        fl_lo,
        fl_hi,
        fl_hi - fl_lo + 1
    );
    println!("         constant has a fraction width: {}", fx_lo == fx_hi);
    println!("         slope-one has one           : {}", fl_lo == fl_hi);
    println!();

    println!("check 4: the knee, carried by the same definition with one more integer");
    let (a, b, c, d, e) = knee_profile();
    println!(
        "         KneeGrid<p=4, knee at binade -6>: f(-9)={} f(-7)={} f(-6)={} f(-4)={} f(0)={}",
        a, b, c, d, e
    );
    println!(
        "         constant below the knee: {}   sloped above: {}",
        a == b && b == c,
        (d - c) == 2 && (e - d) == 4
    );
    println!(
        "         magnitude count through the same walk: {}",
        knee_count()
    );
    let knee_ok = a == b && b == c && d > c && e > d;
    println!("         knee behaves as gradual underflow: {}", knee_ok);
    println!();

    let all = ok && f4 == 57 && f3 == 21 && fx_lo == fx_hi && fl_lo != fl_hi && knee_ok;
    println!("overall: {}", if all { "PASS" } else { "FAIL" });
    std::process::exit(if all { 0 } else { 1 });
}
