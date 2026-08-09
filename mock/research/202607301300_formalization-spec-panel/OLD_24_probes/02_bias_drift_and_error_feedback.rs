// Probe 02: where the error actually comes from in a long-running system.
//
// Hypothesis: the choice of in-range rounding rule, which fires on half of all
// multiplication pairs (probe 01), is not a per-operation nicety; over a long
// accumulation it decides whether the system drifts. A two's-complement `>> F`
// rounds toward negative infinity, injecting a mean error of about -q/2 per
// quantisation (uniform fraction), so a running per-op-truncated accumulation
// acquires a DC ramp linear in the operation count. Round-to-nearest is
// zero-mean and drifts as a random walk (~ q*sqrt(K/12)). First-order error
// feedback (add the previous quantisation residual back before quantising, the
// fixed-point field's "fraction saving") keeps total error bounded by one
// quantum, forever, using one word of state.
//
// The measurement: accumulate K = 2^k quantised products (zero-mean random Q2.9
// products quantised to F=9-bit fraction... model below), tracking the exact
// running sum alongside, and report accumulated error in quanta at increasing K,
// per discipline.
//
// Model: products carry EXTRA = 6 fractional bits beyond the accumulator's own
// fraction; accumulator wide enough (i64) that range recovery never fires, so
// the in-range rule is isolated from the range rule entirely (the same
// isolation 17_probes/03's unbounded-range rows used).

const EXTRA: u32 = 6;

fn main() {
    let mut seed: u64 = 0x9E3779B97F4A7C15;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };

    let ks = [256usize, 1024, 4096, 16384, 65536];
    println!("K | floor-drift (quanta) | rne-drift (quanta) | feedback-drift (quanta)");
    for &kmax in &ks {
        // three disciplines share one product stream
        let mut acc_floor = 0i64; // accumulates floor-quantised products
        let mut acc_rne = 0i64;
        let mut acc_fb = 0i64;
        let mut fb_resid = 0i64; // error-feedback state, in raw 2^-EXTRA units
        let mut exact = 0i64; // exact running sum in 2^-EXTRA units

        let mut s2 = rnd();
        let mut r2 = move || {
            s2 ^= s2 << 13;
            s2 ^= s2 >> 7;
            s2 ^= s2 << 17;
            s2
        };

        for _ in 0..kmax {
            // zero-mean product with a full-width fraction: uniform in
            // [-2048, 2047], units 2^-EXTRA.
            let p = ((r2() & 4095) as i64) - 2048;
            exact += p;

            // floor: arithmetic shift toward -inf
            acc_floor += p >> EXTRA;

            // round-to-nearest-even
            let unit = 1i64 << EXTRA;
            let half = unit >> 1;
            let fl = p >> EXTRA;
            let rem = p - (fl << EXTRA);
            acc_rne += if rem > half {
                fl + 1
            } else if rem < half {
                fl
            } else if fl & 1 == 0 {
                fl
            } else {
                fl + 1
            };

            // error feedback (fraction saving): quantise p + residual by floor,
            // carry the new residual. Total error stays bounded by one quantum.
            let t = p + fb_resid;
            let q = t >> EXTRA;
            fb_resid = t - (q << EXTRA);
            acc_fb += q;
        }

        // exact sum expressed in accumulator quanta (2^-0 relative to the
        // quantised stream's unit): exact/2^EXTRA as a real number.
        let exact_q = exact as f64 / (1i64 << EXTRA) as f64;
        println!(
            "{kmax} | {:+.1} | {:+.1} | {:+.3}",
            acc_floor as f64 - exact_q,
            acc_rne as f64 - exact_q,
            acc_fb as f64 - exact_q
        );
    }
    println!();
    println!("floor drifts linearly at about -K/2 quanta (the -q/2 per-op bias);");
    println!("rne random-walks at about sqrt(K/12); feedback stays inside one quantum.");
    println!("feedback needs one word of carried state, which no zero-sized resolution");
    println!("marker can hold: it is an accumulator-object discipline, not a type-level one.");
}
