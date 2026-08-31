// Probe 1, seat 225. Does wrap-at-declared-width agree with wrap-at-container-width?
//
// The question this answers: for a declared unsigned numeral of width W carried in a
// container of width C > W, under a wrap overflow policy, is the multiplication result
// the same whether the wrap is taken at 2^W (declared) or at 2^C (container)?
//
// Shape: exhaustive over all operand pairs for W <= 10, deterministic LCG sample of
// 40_000 pairs for wider W. Carrier is u32 throughout (C = 32), except the control,
// which sets W = C so the two wraps coincide by construction.
//
// Three arms:
//   arm A (measurement): count disagreeing pairs per W, W in 1..=31, C = 32.
//   arm B (control, must agree): W = 32, C = 32, sampled. If this reports any
//     disagreement the instrument is broken.
//   arm C (must fail, negative control): asserts agreement at W = 13, C = 32,
//     which arm A shows is false. Printed as MUST-FAIL with its first witness so
//     the instrument is shown able to fail before arm A's numbers count.
//
// holds for (arm A result): W in 1..=31, C = 32, signedness = unsigned,
// operation = mul, arity = 2, overflow policy = wrap, F = 0, rounding = exact,
// threads = 1, toolchain = the committed rustc in probe1_out.txt.

fn mask(w: u32) -> u64 {
    if w >= 64 { u64::MAX } else { (1u64 << w) - 1 }
}

// deterministic LCG so the run is reproducible without any input
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0
    }
}

fn disagreements(w: u32, c: u32, exhaustive_limit: u32, samples: u32) -> (u64, u64, Option<(u64, u64)>) {
    let mw = mask(w);
    let mc = mask(c);
    let mut total = 0u64;
    let mut diff = 0u64;
    let mut first: Option<(u64, u64)> = None;
    let mut check = |a: u64, b: u64| {
        let declared = (a.wrapping_mul(b)) & mw;
        let container = (a.wrapping_mul(b)) & mc;
        total += 1;
        if declared != container {
            diff += 1;
            if first.is_none() {
                first = Some((a, b));
            }
        }
    };
    if w <= exhaustive_limit {
        for a in 0..=mw {
            for b in 0..=mw {
                check(a, b);
            }
        }
    } else {
        let mut lcg = Lcg(0x2255_2255);
        for _ in 0..samples {
            let a = lcg.next() & mw;
            let b = lcg.next() & mw;
            check(a, b);
        }
    }
    (total, diff, first)
}

fn main() {
    println!("arm A: W in 1..=31, C = 32, wrap mul, declared vs container");
    println!("{:>3} {:>12} {:>12} {:>10}  first witness", "W", "pairs", "disagree", "mode");
    for w in 1..=31u32 {
        let (total, diff, first) = disagreements(w, 32, 10, 40_000);
        let mode = if w <= 10 { "exhaustive" } else { "sampled" };
        let witness = match first {
            Some((a, b)) => format!("a={a} b={b}"),
            None => "none".to_string(),
        };
        println!("{w:>3} {total:>12} {diff:>12} {mode:>10}  {witness}");
    }

    println!();
    println!("arm B (control, must agree): W = 32, C = 32, sampled");
    let (total, diff, _) = disagreements(32, 32, 10, 200_000);
    println!("  pairs={total} disagree={diff}");
    if diff != 0 {
        println!("  INSTRUMENT BROKEN: control disagrees where the two wraps coincide by construction");
        std::process::exit(2);
    }

    println!();
    println!("arm C (negative control, MUST FAIL): assert declared == container at W = 13, C = 32");
    let (_, diff, first) = disagreements(13, 32, 13, 0);
    if diff == 0 {
        println!("  UNEXPECTED PASS: instrument cannot detect the divergence it exists to measure");
        std::process::exit(2);
    }
    let (a, b) = first.unwrap();
    let declared = (a * b) & mask(13);
    let container = (a * b) & mask(32);
    println!("  FAILED AS REQUIRED: {diff} disagreeing pairs of 2^26.");
    println!("  first witness: a={a} b={b}: declared-wrap {declared}, container-wrap {container}");
    println!("  (matlab fi with word length 13 and wrap documents the declared value; a");
    println!("  container-stated numeral returns the other one, so at most one branch can");
    println!("  pass a fi parity suite)");
}
