// Probe A2. Where the one-rounding guarantee is reachable by carrying the
// residual and NOT by widening the accumulator, at a fixed container width.
//
// `widened` needs an accumulator holding the 2F-scaled exact sum: I + 2F +
// ceil(log2 n) bits. `comp` needs I + F + ceil(log2 n) for the accumulator plus
// F bits of carry, and the carry never grows. So at a fixed container width
// there is a region where one fits and the other does not.
//
// THE CASE THAT MUST FAIL.
//   NC4  At a geometry where BOTH fit, the two arms must agree exactly. If they
//        disagree there, the comparison below is measuring a coding error rather
//        than a representability boundary.
//   NC5  The i128 oracle must disagree with the overflowing arm. If the
//        overflowing arm happens to be right, the geometry does not actually
//        overflow and nothing is demonstrated.
//
// Container for both arms: i64. Oracle: i128.

const CONTAINER_BITS: u32 = 64;

#[derive(Clone, Copy)]
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// `signed = true` draws in [-4, 4); `signed = false` draws in [0, 4).
///
/// The distinction turned out to be the whole finding of v1 of this probe: on
/// decorrelated signed data the accumulated sum is a random walk and grows as
/// sqrt(n), so the worst-case bit count I + 2F + log2(n) is never approached and
/// nothing overflows however large the geometry. The worst case needs terms that
/// do not cancel, which is what `signed = false` produces.
fn build(n: usize, seed: u64, f: u32, signed: bool) -> (Vec<i64>, Vec<i64>) {
    let mut r = Lcg(seed);
    let one = 1i64 << f;
    let (mut a, mut b) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for _ in 0..n {
        if signed {
            a.push((r.next() % (8 * one as u64)) as i64 - 4 * one);
            b.push((r.next() % (8 * one as u64)) as i64 - 4 * one);
        } else {
            a.push((r.next() % (4 * one as u64)) as i64);
            b.push((r.next() % (4 * one as u64)) as i64);
        }
    }
    (a, b)
}

/// Wide accumulation in an i64 container: wraps if the 2F-scaled sum does not fit.
fn widened_i64(a: &[i64], b: &[i64], f: u32) -> i64 {
    let mut acc: i64 = 0;
    for i in 0..a.len() {
        acc = acc.wrapping_add(a[i].wrapping_mul(b[i]));
    }
    acc >> f
}

/// Residual carried, accumulator at scale F in the same i64 container.
fn comp_i64(a: &[i64], b: &[i64], f: u32) -> i64 {
    let mask: i64 = (1i64 << f) - 1;
    let mut acc: i64 = 0;
    let mut carry: i64 = 0;
    for i in 0..a.len() {
        // the product itself still needs 2F bits; it is consumed immediately and
        // never accumulated, so it lives in one temporary rather than in state.
        let p = (a[i] as i128) * (b[i] as i128) + carry as i128;
        acc = acc.wrapping_add((p >> f) as i64);
        carry = (p & mask as i128) as i64;
    }
    acc
}

fn oracle(a: &[i64], b: &[i64], f: u32) -> i128 {
    let mut s: i128 = 0;
    for i in 0..a.len() {
        s += (a[i] as i128) * (b[i] as i128);
    }
    s >> f
}

fn bits_needed_widened(f: u32, n: usize, ival: u32) -> u32 {
    ival + 2 * f + (usize::BITS - (n - 1).leading_zeros())
}
fn bits_needed_comp(f: u32, n: usize, ival: u32) -> u32 {
    ival + f + (usize::BITS - (n - 1).leading_zeros())
}

fn main() {
    // Values are in [-4, 4), so the integer part costs 3 bits including sign.
    const IVAL: u32 = 3;

    println!("== NC4: a geometry where both fit; the arms must agree ==");
    {
        let (f, n) = (8u32, 1usize << 10);
        let (a, b) = build(n, 7, f, false);
        let w = widened_i64(&a, &b, f);
        let c = comp_i64(&a, &b, f);
        let o = oracle(&a, &b, f);
        println!(
            "  f={f} n={n} widened_bits={} comp_bits={} : widened={w} comp={c} oracle={o}",
            bits_needed_widened(f, n, IVAL),
            bits_needed_comp(f, n, IVAL)
        );
        println!(
            "  NC4 {}",
            if w == c && (w as i128) == o { "ok" } else { "FAIL" }
        );
    }

    println!();
    println!("== The boundary: container = {CONTAINER_BITS} bits ==");
    println!(
        "{:>4} {:>9} {:>9} {:>8} {:>22} {:>22} {:>8}",
        "F", "n", "wid_bits", "cmp_bits", "widened", "comp", "oracle=?"
    );
    let mut demonstrated = 0;
    for &f in &[8u32, 16, 20, 24, 26] {
        for &lg in &[10usize, 14, 18] {
            let n = 1usize << lg;
            let wb = bits_needed_widened(f, n, IVAL);
            let cb = bits_needed_comp(f, n, IVAL);
            let (a, b) = build(n, 0x51 ^ (f as u64) ^ (lg as u64), f, false);
            let w = widened_i64(&a, &b, f);
            let c = comp_i64(&a, &b, f);
            let o = oracle(&a, &b, f);
            let wok = (w as i128) == o;
            let cok = (c as i128) == o;
            let tag = match (wok, cok) {
                (true, true) => "both",
                (false, true) => "COMP ONLY",
                (true, false) => "wid only",
                (false, false) => "neither",
            };
            if !wok && cok && wb > CONTAINER_BITS && cb <= CONTAINER_BITS {
                demonstrated += 1;
            }
            println!("{f:>4} {n:>9} {wb:>9} {cb:>8} {w:>22} {c:>22} {tag:>8}");
        }
    }
    println!();
    println!("  geometries where the predicted overflow region is confirmed by the oracle: {demonstrated}");
    println!("  NC5: a row tagged COMP ONLY is one where the widened arm is wrong, which is");
    println!("       the disagreement the demonstration needs; 0 such rows would mean nothing shown.");
}
