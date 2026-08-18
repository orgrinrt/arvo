// Probe F. Is association order an accuracy lever in fixed point, or only a
// speed lever?
//
// The reflex imported from floating-point numerics says pairwise summation beats
// sequential summation, O(log n) error against O(n). That argument turns on
// RELATIVE precision: a float's absolute rounding error grows with the magnitude
// of the running sum, so keeping partial magnitudes balanced keeps errors small.
//
// A fixed-point accumulator has a fixed absolute LSB. This probe asks whether
// the argument survives that change.
//
// THE CASE THAT MUST FAIL, and it is the whole instrument here:
//   NC12 The same comparison run on a RELATIVE-precision accumulator (f32) must
//        show pairwise beating sequential. If it does not, this probe cannot
//        detect an ordering effect at all and its fixed-point verdict means
//        nothing.
//   NC13 The truncation count must be identical for both orders. If it is not,
//        any difference is a difference in how much rounding happened rather
//        than in where it happened.
//
// Exact oracle: i128 for fixed point, f64 pairwise for the f32 arm. No timing.

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

const F: u32 = 12;

// ---------------------------------------------------------------------------
// Fixed point. Every partial sum is truncated back to the accumulator's Q(.F).
// Values are Q(.F) integers; the running sum is itself Q(.F), so the truncation
// that matters is the one on each product.
// ---------------------------------------------------------------------------

static mut TRUNCS: u64 = 0;

fn trunc(p: i128) -> i64 {
    unsafe { TRUNCS += 1 };
    (p >> F) as i64
}

fn seq_fixed(a: &[i64], b: &[i64]) -> i64 {
    let mut acc = 0i64;
    for i in 0..a.len() {
        acc += trunc((a[i] as i128) * (b[i] as i128));
    }
    acc
}

fn tree_fixed(a: &[i64], b: &[i64]) -> i64 {
    // Each leaf is one truncated product; the adds above are exact.
    let mut level: Vec<i64> = (0..a.len())
        .map(|i| trunc((a[i] as i128) * (b[i] as i128)))
        .collect();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i + 1 < level.len() {
            next.push(level[i] + level[i + 1]);
            i += 2;
        }
        if i < level.len() {
            next.push(level[i]);
        }
        level = next;
    }
    level[0]
}

fn exact_fixed(a: &[i64], b: &[i64]) -> i128 {
    let mut s: i128 = 0;
    for i in 0..a.len() {
        s += (a[i] as i128) * (b[i] as i128);
    }
    s
}

// ---------------------------------------------------------------------------
// Relative precision, for NC12.
// ---------------------------------------------------------------------------

fn seq_f32(x: &[f32]) -> f32 {
    let mut acc = 0f32;
    for &v in x {
        acc += v;
    }
    acc
}

fn tree_f32(x: &[f32]) -> f32 {
    let mut level: Vec<f32> = x.to_vec();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut i = 0;
        while i + 1 < level.len() {
            next.push(level[i] + level[i + 1]);
            i += 2;
        }
        if i < level.len() {
            next.push(level[i]);
        }
        level = next;
    }
    level[0]
}

fn exact_f64(x: &[f32]) -> f64 {
    // Kahan in f64: far beyond f32's precision, so it is an oracle here.
    let mut s = 0f64;
    let mut c = 0f64;
    for &v in x {
        let y = v as f64 - c;
        let t = s + y;
        c = (t - s) - y;
        s = t;
    }
    s
}

fn main() {
    println!("== Fixed point, Q(.{F}) accumulate, error in LSBs, worst of 32 seeds ==");
    println!(
        "{:>9}  {:>14}  {:>14}  {:>14}  {:>16}",
        "n", "sequential", "pairwise tree", "difference", "truncs equal?"
    );
    let mut nc13_ok = true;
    let mut fixed_tree_better = 0;
    let mut fixed_seq_better = 0;
    for &n in &[16usize, 64, 256, 1024, 4096, 16384, 65536] {
        let (mut ms, mut mt) = (0.0f64, 0.0f64);
        let mut trunc_mismatch = false;
        for s in 0..32u64 {
            let mut r = Lcg(0xF00D ^ (s << 24) ^ n as u64);
            let one = 1i64 << F;
            let (mut a, mut b) = (Vec::with_capacity(n), Vec::with_capacity(n));
            for _ in 0..n {
                a.push((r.next() % (8 * one as u64)) as i64 - 4 * one);
                b.push((r.next() % (8 * one as u64)) as i64 - 4 * one);
            }
            let e = exact_fixed(&a, &b);
            unsafe { TRUNCS = 0 };
            let vs = seq_fixed(&a, &b);
            let ts = unsafe { TRUNCS };
            unsafe { TRUNCS = 0 };
            let vt = tree_fixed(&a, &b);
            let tt = unsafe { TRUNCS };
            if ts != tt {
                trunc_mismatch = true;
            }
            let es = (((vs as i128) << F) - e) as f64 / (1u64 << F) as f64;
            let et = (((vt as i128) << F) - e) as f64 / (1u64 << F) as f64;
            ms = ms.max(es.abs());
            mt = mt.max(et.abs());
        }
        if trunc_mismatch {
            nc13_ok = false;
        }
        if mt < ms {
            fixed_tree_better += 1;
        } else if ms < mt {
            fixed_seq_better += 1;
        }
        println!(
            "{:>9}  {:>14.3}  {:>14.3}  {:>14.3}  {:>16}",
            n,
            ms,
            mt,
            mt - ms,
            if trunc_mismatch { "NO" } else { "yes" }
        );
    }
    println!(
        "  NC13 (identical truncation count both orders): {}",
        if nc13_ok { "ok" } else { "FAIL" }
    );
    println!("  sizes where the tree is strictly better: {fixed_tree_better}");
    println!("  sizes where the sequential is strictly better: {fixed_seq_better}");

    println!();
    println!("== NC12: the same comparison on a relative-precision accumulator (f32) ==");
    println!(
        "{:>9}  {:>16}  {:>16}  {:>10}",
        "n", "sequential |err|", "tree |err|", "seq/tree"
    );
    let mut f32_tree_better = 0;
    for &n in &[16usize, 64, 256, 1024, 4096, 16384, 65536] {
        let (mut ms, mut mt) = (0.0f64, 0.0f64);
        for s in 0..32u64 {
            let mut r = Lcg(0xBEE5 ^ (s << 24) ^ n as u64);
            let x: Vec<f32> = (0..n)
                .map(|_| ((r.next() >> 11) as f64 / (1u64 << 53) as f64) as f32)
                .collect();
            let e = exact_f64(&x);
            ms = ms.max(((seq_f32(&x) as f64) - e).abs());
            mt = mt.max(((tree_f32(&x) as f64) - e).abs());
        }
        if mt < ms {
            f32_tree_better += 1;
        }
        println!(
            "{:>9}  {:>16.6e}  {:>16.6e}  {:>10.2}",
            n,
            ms,
            mt,
            if mt > 0.0 { ms / mt } else { f64::INFINITY }
        );
    }
    println!(
        "  NC12 (tree must beat sequential on a relative-precision accumulator at some size): {}",
        if f32_tree_better > 0 {
            "ok"
        } else {
            "FAIL, the instrument cannot detect an ordering effect"
        }
    );
    println!("  sizes where the tree is strictly better, f32: {f32_tree_better} of 7");
}
