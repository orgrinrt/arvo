// Probe A. The residual is the chain-level fact, in fixed point.
//
// QUESTION. A multiply of two Q(I.F) values produces 2F fraction bits. A design
// whose only unit is the operation must return a Q(I.F) value from that
// multiply, so it discards F bits at every step. This probe asks what that
// discard costs over a chain of n such steps, and whether anything recovers it.
//
// THE CASE THAT MUST FAIL. Three negative controls, each of which would fire if
// the instrument were measuring something other than what it claims:
//
//   NC1  At F = 0 no product has fraction bits to discard, so every arm must be
//        exact. A nonzero error at F = 0 means the harness itself is lossy.
//   NC2  `fake_comp` computes the residual and throws it away. It must equal
//        `naive` bit for bit at every n. If it does not, `comp`'s advantage is
//        coming from something other than feeding the residual forward.
//   NC3  On a workload where every product is exactly representable in Q(I.F),
//        `naive` must be exact too. If `naive` shows error there, the workload
//        was rigged to make truncation look bad.
//
// ARMS.
//   naive        acc += (a*b) >> F                        floor at each step
//   naive_round  acc += (a*b + half) >> F                 round-to-nearest at each step,
//                                                         the best a per-op design can do
//   widened      accumulate at 2F, truncate once at end    one rounding for the whole chain
//   comp         floor at each step, residual fed forward  error feedback
//   fake_comp    residual computed, discarded              NC2
//
// Error is reported in LSBs of the Q(I.F) result: err = (arm << F) - exact_2F,
// divided by 2^F.
//
// Not a benchmark. No timing is taken and none is claimed. Counts and exact
// error magnitudes only.

const F: u32 = 12;

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

/// Inputs at scale 2^f. Values roughly in [-4, 4) so products stay small.
fn build(n: usize, seed: u64, f: u32, exact_products: bool) -> (Vec<i64>, Vec<i64>) {
    let mut r = Lcg(seed);
    let one = 1i64 << f;
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        let av = (r.next() % (8 * one as u64)) as i64 - 4 * one;
        let bv = if exact_products {
            // b a whole multiple of 1.0, so a*b has no bits below 2^-f and the
            // shift by f is exact. NC3.
            ((r.next() % 7) as i64 - 3) * one
        } else {
            (r.next() % (8 * one as u64)) as i64 - 4 * one
        };
        a.push(av);
        b.push(bv);
    }
    (a, b)
}

fn exact_2f(a: &[i64], b: &[i64]) -> i128 {
    let mut s: i128 = 0;
    for i in 0..a.len() {
        s += (a[i] as i128) * (b[i] as i128);
    }
    s
}

fn naive(a: &[i64], b: &[i64], f: u32) -> i64 {
    let mut acc = 0i64;
    for i in 0..a.len() {
        let p = (a[i] as i128) * (b[i] as i128);
        acc += (p >> f) as i64;
    }
    acc
}

fn naive_round(a: &[i64], b: &[i64], f: u32) -> i64 {
    let half: i128 = if f == 0 { 0 } else { 1i128 << (f - 1) };
    let mut acc = 0i64;
    for i in 0..a.len() {
        let p = (a[i] as i128) * (b[i] as i128);
        acc += ((p + half) >> f) as i64;
    }
    acc
}

fn widened(a: &[i64], b: &[i64], f: u32) -> i64 {
    let mut acc: i128 = 0;
    for i in 0..a.len() {
        acc += (a[i] as i128) * (b[i] as i128);
    }
    (acc >> f) as i64
}

fn comp(a: &[i64], b: &[i64], f: u32) -> i64 {
    let mask: i128 = if f == 0 { 0 } else { (1i128 << f) - 1 };
    let mut acc = 0i64;
    let mut carry: i128 = 0;
    for i in 0..a.len() {
        let p = (a[i] as i128) * (b[i] as i128) + carry;
        acc += (p >> f) as i64;
        carry = p & mask; // the bits the per-op truncation would have destroyed
    }
    acc
}

fn fake_comp(a: &[i64], b: &[i64], f: u32) -> i64 {
    let mask: i128 = if f == 0 { 0 } else { (1i128 << f) - 1 };
    let mut acc = 0i64;
    let mut _carry: i128 = 0;
    for i in 0..a.len() {
        let p = (a[i] as i128) * (b[i] as i128);
        acc += (p >> f) as i64;
        _carry = p & mask; // computed and never read: NC2
    }
    acc
}

fn err_lsb(arm: i64, exact: i128, f: u32) -> f64 {
    let d = ((arm as i128) << f) - exact;
    (d as f64) / ((1u64 << f) as f64)
}

fn main() {
    println!("== NC1: F = 0, every arm must be exact ==");
    let mut nc1_bad = 0;
    for &n in &[1usize, 16, 1024, 65536] {
        let (a, b) = build(n, 0xC0FFEE ^ n as u64, 0, false);
        let e = exact_2f(&a, &b);
        for (name, v) in [
            ("naive", naive(&a, &b, 0)),
            ("naive_round", naive_round(&a, &b, 0)),
            ("widened", widened(&a, &b, 0)),
            ("comp", comp(&a, &b, 0)),
            ("fake_comp", fake_comp(&a, &b, 0)),
        ] {
            let er = err_lsb(v, e, 0);
            if er != 0.0 {
                println!("  FAIL n={n} {name} err={er}");
                nc1_bad += 1;
            }
        }
    }
    println!("  NC1 violations: {nc1_bad}  (expected 0)");

    println!();
    println!("== NC3: products exactly representable, naive must be exact ==");
    let mut nc3_bad = 0;
    for &n in &[16usize, 1024, 65536] {
        let (a, b) = build(n, 0xBEEF ^ n as u64, F, true);
        let e = exact_2f(&a, &b);
        let er = err_lsb(naive(&a, &b, F), e, F);
        println!("  n={n:<7} naive err = {er} LSB");
        if er != 0.0 {
            nc3_bad += 1;
        }
    }
    println!("  NC3 violations: {nc3_bad}  (expected 0)");

    println!();
    println!("== The chain, F = {F}, error in LSBs of the Q(.{F}) result ==");
    println!(
        "{:>9}  {:>14}  {:>14}  {:>14}  {:>14}  {:>10}",
        "n", "naive", "naive_round", "widened", "comp", "NC2 ok"
    );
    let mut nc2_bad = 0;
    for &n in &[
        1usize, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144, 1048576,
    ] {
        let (a, b) = build(n, 0x1234_5678 ^ n as u64, F, false);
        let e = exact_2f(&a, &b);
        let vn = naive(&a, &b, F);
        let vr = naive_round(&a, &b, F);
        let vw = widened(&a, &b, F);
        let vc = comp(&a, &b, F);
        let vf = fake_comp(&a, &b, F);
        let nc2 = vf == vn;
        if !nc2 {
            nc2_bad += 1;
        }
        println!(
            "{:>9}  {:>14.4}  {:>14.4}  {:>14.4}  {:>14.4}  {:>10}",
            n,
            err_lsb(vn, e, F),
            err_lsb(vr, e, F),
            err_lsb(vw, e, F),
            err_lsb(vc, e, F),
            if nc2 { "yes" } else { "NO" }
        );
    }
    println!("  NC2 violations: {nc2_bad}  (expected 0)");

    println!();
    println!("== Growth: max |err| over 32 seeds, per arm, per n ==");
    println!(
        "{:>9}  {:>12}  {:>12}  {:>12}  {:>12}",
        "n", "naive", "naive_round", "widened", "comp"
    );
    for &n in &[16usize, 256, 4096, 65536, 1048576] {
        let (mut mn, mut mr, mut mw, mut mc) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for s in 0..32u64 {
            let (a, b) = build(n, 0xABCD_0000 ^ (s << 20) ^ n as u64, F, false);
            let e = exact_2f(&a, &b);
            mn = mn.max(err_lsb(naive(&a, &b, F), e, F).abs());
            mr = mr.max(err_lsb(naive_round(&a, &b, F), e, F).abs());
            mw = mw.max(err_lsb(widened(&a, &b, F), e, F).abs());
            mc = mc.max(err_lsb(comp(&a, &b, F), e, F).abs());
        }
        println!("{n:>9}  {mn:>12.4}  {mr:>12.4}  {mw:>12.4}  {mc:>12.4}");
    }
}
