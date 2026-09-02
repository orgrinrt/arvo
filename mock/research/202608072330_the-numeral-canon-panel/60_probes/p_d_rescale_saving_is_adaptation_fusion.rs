// Probe D (PHASE TWO; built after reading the panel, unlike probes A, B, C).
//
// Hypothesis, registered before running: `58_probes/p2`'s finding that the
// multiplicative fold's guard saves exactly one rescale (`min_w == full_w - F`,
// constant in fold length) is an instance of ADAPTATION FUSION in the schedule:
// at w = full - F every per-step narrow except the last is exact, and the last
// per-step narrow is adjacent to the final narrow, so the two compose into one.
// Truncation composes exactly (trunc by F then by w equals trunc by F + w), so
// the saving exists under truncation. Round-to-nearest-even does not compose
// (double rounding), so under RNE the saving should shrink or vanish. `07`'s
// narrowing-composition result already in the register (directed modes compose,
// nearest never does) predicts the same split from the other side.
//
// Secondary job: extend `58`'s truncation pattern past n = 4, which `58`
// section 7 names as wanted ("I would want n = 5 and n = 6 before trusting the
// pattern"). n = 5 is swept here; n = 6 is not (cost) and is said so.
//
// Construction mirrors `58_probes/p2` section 1 exactly (no intermediate range
// clamp; guard width w; final narrow by w; clamp once at the end), with the
// narrowing rule a parameter. Oracle: exact integer product, narrowed once by
// (n-1)*F WITH THE SAME RULE, clamped once. Same rule on both sides so the
// comparison isolates the intermediate schedule, never the final rounding rule.
//
// Instrument checks: at w = 0 both modes must diverge somewhere (the checker
// can fail); at w = full both modes must agree everywhere (the construction is
// exact there for any rule, since every narrow but the final is a no-op in
// information terms). Either assertion failing panics.
//
// Shortcuts (spike): unsigned operands in [0, M] only, one (M, F) point
// matching `58`'s, left fold only for min-w (association agreement is `58`'s
// job, not this probe's), bare primitives, std.

const M: i64 = 15;
const F: u32 = 3;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
    Trunc,
    Rne,
}

fn narrow(x: i128, shift: u32, mode: Mode) -> i128 {
    if shift == 0 {
        return x;
    }
    match mode {
        Mode::Trunc => x >> shift,
        Mode::Rne => {
            let q = x >> shift;
            let rem = x & ((1i128 << shift) - 1);
            let half = 1i128 << (shift - 1);
            if rem > half || (rem == half && (q & 1) == 1) {
                q + 1
            } else {
                q
            }
        }
    }
}

fn eager_no_clamp(ops: &[i64], w: u32, mode: Mode) -> i64 {
    let mut acc: i128 = (ops[0] as i128) << w;
    for &a in &ops[1..] {
        acc = narrow(acc * (a as i128), F, mode);
    }
    narrow(acc, w, mode).clamp(0, M as i128) as i64
}

fn exact_once(ops: &[i64], mode: Mode) -> i64 {
    let mut prod: i128 = 1;
    for &a in ops {
        prod *= a as i128;
    }
    let extra = (ops.len() as u32 - 1) * F;
    narrow(prod, extra, mode).clamp(0, M as i128) as i64
}

fn divergences(n: usize, w: u32, mode: Mode) -> u64 {
    let mut count = 0u64;
    let mut idx = vec![0i64; n];
    loop {
        let d = eager_no_clamp(&idx, w, mode) != exact_once(&idx, mode);
        if d {
            count += 1;
        }
        let mut i = 0;
        loop {
            idx[i] += 1;
            if idx[i] <= M {
                break;
            }
            idx[i] = 0;
            i += 1;
            if i == n {
                break;
            }
        }
        if i == n {
            break;
        }
    }
    count
}

fn main() {
    println!(
        "M={M} F={F}; count = tuples over [0,{M}]^n; full_w = (n-1)*F; 58's pattern: min_w = full_w - F"
    );
    for mode in [Mode::Trunc, Mode::Rne] {
        for n in [3usize, 4, 5] {
            let full = (n as u32 - 1) * F;
            let mut min_w = None;
            let mut per_w = Vec::new();
            for w in 0..=full {
                let d = divergences(n, w, mode);
                per_w.push((w, d));
                if d == 0 && min_w.is_none() {
                    min_w = Some(w);
                }
            }
            // instrument: w = 0 must diverge, w = full must not
            assert!(
                per_w[0].1 > 0,
                "HARNESS BROKEN: {mode:?} n={n} w=0 shows no divergence"
            );
            assert_eq!(
                per_w[full as usize].1, 0,
                "HARNESS BROKEN: {mode:?} n={n} w=full diverges"
            );
            let min_w = min_w.unwrap();
            let saving = full - min_w;
            println!(
                "{mode:?} n={n}: divergence by w {per_w:?}; min_w={min_w}, full_w={full}, saving={saving} bits"
            );
        }
    }
    println!("OUTCOME: see counts; hypothesis is judged in 60's phase two, not here");
}
