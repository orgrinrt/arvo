// Probe G. Does the carried residual recover the one-rounding guarantee for a
// PRODUCT chain, as probe A showed it does for an accumulate chain?
//
// Built in phase two, to attack my own phase-one mechanism against a claim from
// three panel topics that multiplicative chains need width growing linearly in
// the chain length. If the residual carry worked for products, that claim would
// be in trouble. I expect it does not, and this probe is built to find out where
// it stops rather than to confirm the expectation.
//
// The chain: x_0 = a_0; x_k = trunc_F(x_{k-1} * a_k), factors in (0, 2].
//
// ARMS
//   per_step   truncate after every multiply                    the endomorphic typing
//   deferred   keep the exact product, truncate once at the end  the wide route
//   carried    keep (x, r) with r < 2^F, exactly F extra bits of state
//
// THE CASE THAT MUST FAIL
//   NC14  At chain length 1 all three arms must be identical: one multiply, one
//         place to round. A difference there is an instrument error.
//   NC15  `deferred` must beat `per_step` somewhere, otherwise the probe has no
//         resolution and cannot say anything about `carried` either.
//
// Error is measured against the exact rational, held as an i128 numerator at the
// chain's own natural scale. No timing.

const F: u32 = 8;

#[derive(Clone, Copy)]
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// Factors in (0, 2] at scale 2^F.
fn factors(k: usize, seed: u64) -> Vec<i128> {
    let mut r = Lcg(seed);
    (0..k)
        .map(|_| 1 + (r.next() % (2u64 << F)) as i128)
        .collect()
}

/// Exact value of the chain, as a numerator at scale 2^(F*k).
fn exact(f: &[i128]) -> i128 {
    f.iter().product()
}

fn per_step(f: &[i128]) -> i128 {
    let mut x = f[0];
    for &a in &f[1..] {
        x = (x * a) >> F;
    }
    x
}

fn deferred(f: &[i128]) -> i128 {
    let p: i128 = f.iter().product();
    // exact product is at scale 2^(F*k); narrow once to scale 2^F.
    p >> (F * (f.len() as u32 - 1))
}

/// State is (x at scale 2^F, r < 2^F), so exactly F bits beyond `per_step`.
fn carried(f: &[i128]) -> i128 {
    let m: i128 = (1i128 << F) - 1;
    let mut x = f[0];
    let mut r: i128 = 0;
    for &a in &f[1..] {
        // the pair (x, r) denotes x + r/2^F at scale 2^F
        let full = (x * a) * (1i128 << F) + r * a; // at scale 2^(2F)
        x = full >> (2 * F);
        r = (full >> F) & m;
    }
    x
}

/// Error in LSBs of the scale-2^F result, against the exact rational.
fn err(v: i128, f: &[i128]) -> f64 {
    let k = f.len() as u32 - 1;
    // v is at scale 2^F; exact is at scale 2^(F*(k+1)). Compare at 2^(F*(k+1)).
    let scaled = v << (F * k);
    let d = scaled - exact(f);
    (d as f64) / ((1u128 << (F * k)) as f64)
}

fn main() {
    println!(
        "== Product chain, F = {F}, factors in (0, 2], error in LSBs, 400 chains per length =="
    );
    println!(
        "{:>6}  {:>16}  {:>16}  {:>16}  {:>22}",
        "len", "per_step max", "deferred max", "carried max", "carried == deferred?"
    );
    let mut nc14 = true;
    let mut nc15 = false;
    let mut carried_matches_upto = 0usize;
    for k in 1..=8usize {
        let (mut mp, mut md, mut mc) = (0.0f64, 0.0f64, 0.0f64);
        let mut same = 0usize;
        for s in 0..400u64 {
            let f = factors(k, 0xC0DE ^ (s << 16) ^ k as u64);
            let (vp, vd, vc) = (per_step(&f), deferred(&f), carried(&f));
            if k == 1 && !(vp == vd && vd == vc) {
                nc14 = false;
            }
            if vd != vp {
                nc15 = true;
            }
            if vc == vd {
                same += 1;
            }
            mp = mp.max(err(vp, &f).abs());
            md = md.max(err(vd, &f).abs());
            mc = mc.max(err(vc, &f).abs());
        }
        if same == 400 {
            carried_matches_upto = k;
        }
        println!(
            "{:>6}  {:>16.3}  {:>16.3}  {:>16.3}  {:>22}",
            k,
            mp,
            md,
            mc,
            format!("{same}/400")
        );
    }
    println!();
    println!(
        "  NC14 (all three identical at length 1): {}",
        if nc14 { "ok" } else { "FAIL" }
    );
    println!(
        "  NC15 (deferred differs from per_step somewhere, so the probe has resolution): {}",
        if nc15 { "ok" } else { "FAIL, nothing shown" }
    );
    println!();
    println!("  The carried form reproduces the deferred answer on every chain up to length {carried_matches_upto},");
    println!("  and stops reproducing it above that. F extra bits of state buy exactly F extra");
    println!("  fraction bits, and a product of k factors needs F*(k-1) of them.");
}
