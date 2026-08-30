//! The order on dyadic fixed-point numerals, checked exhaustively rather than argued.
//!
//! Claim O: for unsigned dyadic numerals with zero bias, V(I1,F1) subset-of V(I2,F2)
//!          exactly when I1 <= I2 and F1 <= F2.
//! Claim S: the same, for signed.
//! Claim A: every equal-precision family {(I,F) : I+F = P} is an antichain.
//! Claim M: V(meet(A,B)) == V(A) intersect V(B), where meet = (min I, min F).
//! Claim J: V(join(A,B)) is the least numeral value set containing V(A) union V(B),
//!          where join = (max I, max F), and it is strictly larger than the union
//!          whenever A and B are incomparable, so V preserves meets and not joins.
//!
//! The matrix is the whole matrix at the checked bound, not a sample: every ordered
//! pair of (I,F) with I+F <= LIM, both signs.
//!
//! rustc 1.98.0-nightly (57d06900f 2026-05-27), plain std probe, enumeration not const-eval.

const LIM: u32 = 8; // I + F <= 8, so at most 256 values per numeral
const FMAX: u32 = 8; // common denominator exponent

/// The value set of UFixed<I,F> as integers scaled by 2^FMAX.
fn vset_u(i: u32, f: u32) -> Vec<i64> {
    let step = 1i64 << (FMAX - f);
    let n = 1i64 << (i + f);
    (0..n).map(|k| k * step).collect()
}

/// The value set of IFixed<I,F>, i.e. one sign bit plus I integer and F fraction digits.
fn vset_i(i: u32, f: u32) -> Vec<i64> {
    let step = 1i64 << (FMAX - f);
    let n = 1i64 << (i + f);
    (-n..n).map(|k| k * step).collect()
}

fn is_subset(a: &[i64], b: &[i64]) -> bool {
    use std::collections::BTreeSet;
    let bs: BTreeSet<i64> = b.iter().copied().collect();
    a.iter().all(|x| bs.contains(x))
}

fn intersect(a: &[i64], b: &[i64]) -> Vec<i64> {
    use std::collections::BTreeSet;
    let bs: BTreeSet<i64> = b.iter().copied().collect();
    let mut v: Vec<i64> = a.iter().copied().filter(|x| bs.contains(x)).collect();
    v.sort_unstable();
    v.dedup();
    v
}

fn shapes() -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    for i in 0..=LIM {
        for f in 0..=(LIM - i) {
            v.push((i, f));
        }
    }
    v
}

fn main() {
    let sh = shapes();
    let mut pairs = 0usize;
    let mut fail_o = 0usize;
    let mut fail_s = 0usize;
    let mut fail_m = 0usize;
    let mut fail_j = 0usize;
    let mut incomparable = 0usize;
    let mut join_strictly_bigger = 0usize;

    for &(i1, f1) in &sh {
        for &(i2, f2) in &sh {
            pairs += 1;
            let predicted = i1 <= i2 && f1 <= f2;

            // Claim O
            let au = vset_u(i1, f1);
            let bu = vset_u(i2, f2);
            let actual_u = is_subset(&au, &bu);
            if actual_u != predicted {
                fail_o += 1;
                println!(
                    "O FAIL U({i1},{f1}) <= U({i2},{f2}): predicted {predicted} actual {actual_u}"
                );
            }

            // Claim S
            let ai = vset_i(i1, f1);
            let bi = vset_i(i2, f2);
            let actual_s = is_subset(&ai, &bi);
            if actual_s != predicted {
                fail_s += 1;
                println!(
                    "S FAIL I({i1},{f1}) <= I({i2},{f2}): predicted {predicted} actual {actual_s}"
                );
            }

            // Claim M: meet is exact
            let (mi, mf) = (i1.min(i2), f1.min(f2));
            let meet = vset_u(mi, mf);
            let inter = intersect(&au, &bu);
            if meet != inter {
                fail_m += 1;
                println!(
                    "M FAIL U({i1},{f1}) meet U({i2},{f2}): meet={} inter={}",
                    meet.len(),
                    inter.len()
                );
            }

            // Claim J: join contains both, is least among numerals that do,
            // and is strictly larger than the union when the pair is incomparable.
            let (ji, jf) = (i1.max(i2), f1.max(f2));
            let join = vset_u(ji, jf);
            if !is_subset(&au, &join) || !is_subset(&bu, &join) {
                fail_j += 1;
                println!("J FAIL containment U({i1},{f1}) U({i2},{f2})");
            }
            // leastness: no other shape whose set contains both is a proper subset of join
            for &(ki, kf) in &sh {
                let k = vset_u(ki, kf);
                if is_subset(&au, &k)
                    && is_subset(&bu, &k)
                    && is_subset(&k, &join)
                    && k.len() < join.len()
                {
                    fail_j += 1;
                    println!("J FAIL leastness: U({ki},{kf}) beats join U({ji},{jf})");
                }
            }
            let comparable = predicted || (i2 <= i1 && f2 <= f1);
            if !comparable {
                incomparable += 1;
                let mut uni = au.clone();
                uni.extend_from_slice(&bu);
                uni.sort_unstable();
                uni.dedup();
                if join.len() > uni.len() {
                    join_strictly_bigger += 1;
                }
            }
        }
    }

    // Claim A: every equal-precision family is an antichain.
    let mut fail_a = 0usize;
    let mut families = 0usize;
    for p in 0..=LIM {
        let fam: Vec<(u32, u32)> = (0..=p).map(|i| (i, p - i)).collect();
        families += 1;
        for &a in &fam {
            for &b in &fam {
                if a == b {
                    continue;
                }
                let sub = a.0 <= b.0 && a.1 <= b.1;
                if sub {
                    fail_a += 1;
                    println!("A FAIL ({},{}) <= ({},{}) at P={p}", a.0, a.1, b.0, b.1);
                }
            }
        }
    }

    println!("pairs checked          {pairs}");
    println!("O failures (unsigned)  {fail_o}");
    println!("S failures (signed)    {fail_s}");
    println!("M failures (meet)      {fail_m}");
    println!("J failures (join)      {fail_j}");
    println!("families checked       {families}");
    println!("A failures (antichain) {fail_a}");
    println!("incomparable pairs     {incomparable}");
    println!("  of which join strictly exceeds the union: {join_strictly_bigger}");
}
