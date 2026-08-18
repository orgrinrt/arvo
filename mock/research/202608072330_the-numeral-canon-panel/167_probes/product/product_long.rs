// Probe G2. Does the carried form's ~1 LSB bound on a PRODUCT chain survive long
// chains, or had it merely not diverged yet at length 8?
//
// F = 4 so the exact product of k factors (each below 2^5) fits i128 to k = 24.
//
// THE CASE THAT MUST FAIL
//   NC16  per_step must keep growing over the same range. If it flattened, the
//         comparison would be against a ceiling rather than against growth.
//   NC17  The exact product must not overflow i128 at any length reported. The
//         probe recomputes it in f64 and cross-checks the magnitude; a row where
//         they part is dropped rather than reported.

const F: u32 = 4;

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

fn factors(k: usize, seed: u64) -> Vec<i128> {
    let mut r = Lcg(seed);
    (0..k)
        .map(|_| 1 + (r.next() % (2u64 << F)) as i128)
        .collect()
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
    p >> (F * (f.len() as u32 - 1))
}
fn carried(f: &[i128]) -> i128 {
    let m: i128 = (1i128 << F) - 1;
    let mut x = f[0];
    let mut r: i128 = 0;
    for &a in &f[1..] {
        let full = (x * a) * (1i128 << F) + r * a;
        x = full >> (2 * F);
        r = (full >> F) & m;
    }
    x
}
/// Two carried limbs instead of one: 2F extra bits of state, still constant in k.
fn carried2(f: &[i128]) -> i128 {
    let m: i128 = (1i128 << (2 * F)) - 1;
    let mut x = f[0];
    let mut r: i128 = 0; // r < 2^(2F), denotes x + r/2^(2F)
    for &a in &f[1..] {
        let full = (x * a) * (1i128 << (2 * F)) + r * a; // scale 2^(3F)
        x = full >> (3 * F);
        r = (full >> F) & m;
    }
    x
}

fn err(v: i128, f: &[i128]) -> f64 {
    let k = f.len() as u32 - 1;
    let exact: i128 = f.iter().product();
    let d = (v << (F * k)) - exact;
    (d as f64) / ((1u128 << (F * k)) as f64)
}

fn main() {
    println!("== Product chain, F = {F}, 400 chains per length, max |err| in LSBs ==");
    println!(
        "{:>6}  {:>12}  {:>12}  {:>12}  {:>12}  {:>10}",
        "len", "per_step", "deferred", "carried(F)", "carried(2F)", "oracle ok"
    );
    let mut prev_ps = 0.0f64;
    let mut nc16 = true;
    for k in [2usize, 4, 8, 12, 16, 20, 24] {
        let (mut mp, mut md, mut mc, mut mc2) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        let mut oracle_ok = true;
        for s in 0..400u64 {
            let f = factors(k, 0xFEED ^ (s << 16) ^ k as u64);
            let exact: i128 = f.iter().product();
            let approx: f64 = f.iter().map(|&x| x as f64).product();
            // NC17: the i128 product must agree with an f64 recomputation in magnitude.
            if approx > 0.0 && ((exact as f64) / approx - 1.0).abs() > 1e-9 {
                oracle_ok = false;
            }
            mp = mp.max(err(per_step(&f), &f).abs());
            md = md.max(err(deferred(&f), &f).abs());
            mc = mc.max(err(carried(&f), &f).abs());
            mc2 = mc2.max(err(carried2(&f), &f).abs());
        }
        if mp <= prev_ps && k > 2 {
            nc16 = false;
        }
        prev_ps = mp;
        println!(
            "{k:>6}  {mp:>12.3}  {md:>12.3}  {mc:>12.3}  {mc2:>12.3}  {:>10}",
            if oracle_ok { "yes" } else { "NO" }
        );
    }
    println!();
    println!(
        "  NC16 (per_step keeps growing over the range, so the flat rows are a bound not a ceiling): {}",
        if nc16 { "ok" } else { "FAIL" }
    );
    println!("  NC17 is the 'oracle ok' column: an i128/f64 magnitude cross-check per chain.");
    println!();
    println!("  State: per_step holds F fraction bits at every length. carried(F) holds 2F.");
    println!("  carried(2F) holds 3F. deferred holds F*k, growing linearly in the chain length.");
}
