// Probe 2, seat 225. Rounding through a wider intermediate is observable, so a
// carrier-stated arithmetic cannot meet a declared-width rounding contract.
//
// The container premise at the rounding axis: if behaviour were stated over the
// container, an implementation would be licensed to round onto the container's
// finer grid first (it holds more fraction bits) and onto the declared grid at
// the store. IEEE 754's x87 history says that diverges; this measures the same
// divergence in fixed point, small enough to sweep exhaustively.
//
// Setup: unsigned values on a source grid of F_src fraction bits, quantised
// half-up onto a target grid F_dst, either directly or via an intermediate grid
// F_mid with F_dst < F_mid < F_src. round(round(x, mid), dst) vs round(x, dst).
//
// Three arms:
//   arm A (measurement): witness counts for all (F_src, F_mid, F_dst) with
//     F_dst < F_mid < F_src <= 8, values exhaustive over one unit interval.
//   arm B (control, must agree): F_mid = F_src (exact intermediate, nothing
//     discarded). Any witness here means the instrument is broken.
//   arm C (must fail, negative control): asserts zero witnesses at
//     (F_src, F_mid, F_dst) = (4, 3, 2), where the classic witness is x = 5/16:
//     direct gives 1/4, via eighths gives 2/4.
//
// holds for (arm A result): F_src in 3..=8, F_dst < F_mid < F_src,
// signedness = unsigned, rounding = half_up, operation = quantise, arity = 1,
// overflow policy = none reached (one unit interval), threads = 1,
// toolchain = the committed rustc in probe2_out.txt.

/// Half-up quantise of `x` (an integer count of 1/2^f_src units) onto f_dst.
/// Returns an integer count of 1/2^f_dst units.
fn quantise_half_up(x: u64, f_src: u32, f_dst: u32) -> u64 {
    assert!(f_dst <= f_src);
    let shift = f_src - f_dst;
    if shift == 0 {
        return x;
    }
    let half = 1u64 << (shift - 1);
    (x + half) >> shift
}

fn witnesses(f_src: u32, f_mid: u32, f_dst: u32) -> (u64, Vec<u64>) {
    let mut found = Vec::new();
    let n = 1u64 << f_src; // one unit interval, exhaustive
    for x in 0..n {
        let direct = quantise_half_up(x, f_src, f_dst);
        let mid = quantise_half_up(x, f_src, f_mid);
        let via = quantise_half_up(mid, f_mid, f_dst);
        if direct != via {
            found.push(x);
        }
    }
    (n, found)
}

fn main() {
    println!("arm A: witness counts, half-up, F_dst < F_mid < F_src <= 8, exhaustive per cell");
    println!("{:>6} {:>6} {:>6} {:>8} {:>10}", "F_src", "F_mid", "F_dst", "values", "witnesses");
    let mut any = false;
    for f_src in 3..=8u32 {
        for f_mid in 1..f_src {
            for f_dst in 0..f_mid {
                let (n, w) = witnesses(f_src, f_mid, f_dst);
                if !w.is_empty() {
                    any = true;
                }
                println!("{f_src:>6} {f_mid:>6} {f_dst:>6} {n:>8} {:>10}", w.len());
            }
        }
    }
    if !any {
        println!("  UNEXPECTED: no cell diverges; instrument cannot support the claim either way");
        std::process::exit(2);
    }

    println!();
    println!("arm B (control, must agree): F_mid = F_src, nothing discarded at the intermediate");
    for f_src in 3..=8u32 {
        for f_dst in 0..f_src {
            let (_, w) = witnesses(f_src, f_src, f_dst);
            if !w.is_empty() {
                println!("  INSTRUMENT BROKEN at F_src={f_src} F_dst={f_dst}: exact intermediate diverged");
                std::process::exit(2);
            }
        }
    }
    println!("  zero witnesses in every cell, as required");

    println!();
    println!("arm C (negative control, MUST FAIL): assert zero witnesses at (4, 3, 2)");
    let (_, w) = witnesses(4, 3, 2);
    if w.is_empty() {
        println!("  UNEXPECTED PASS: the classic double-rounding witness is absent");
        std::process::exit(2);
    }
    let x = w[0];
    println!("  FAILED AS REQUIRED: {} witnesses; first x = {x}/16:", w.len());
    println!(
        "    direct to quarters: {}/4, via eighths: {}/4",
        quantise_half_up(x, 4, 2),
        quantise_half_up(quantise_half_up(x, 4, 3), 3, 2)
    );
    println!("  a carrier free to round onto its own finer grid first is observably a");
    println!("  different arithmetic from the declared-grid one.");
}
