//! Probe p4: the accumulator grade of a SIGNED multiplicative fold, the last
//! empty measurement in `59` P4's cell ("induced structure, absorption,
//! accumulator grade").
//!
//! WHAT IS MEASURED. The mirror of `58_probes/p2` section 1 (read at source
//! before this was written, and re-derived rather than copied), moved to the
//! signed domain: format elements are raw integers in Q = [-8, 7] denoting
//! r / 2^F at F = 3 (a signed 1.3 fixed-point format), folded by
//! multiplication with w guard fraction bits kept between steps, narrowing F
//! bits per step with NO intermediate range clamp, then narrowing off the
//! guard and clamping to Q exactly once at the end. LEFT against RIGHT
//! association, exhaustively over Q^n, n = 2 through 5. Both rescale
//! spellings from p2 (trunc toward zero, floor via arithmetic shift) are
//! run, because they differ on negative intermediates and the unsigned probe
//! could not see the difference.
//!
//! PREDICTIONS, stated before running:
//!
//!   1. Full guard width w = (n-1)*F is clean for both spellings: with the
//!      full guard, every per-step narrow drops only zeros (the accumulator
//!      still holds the exact product), so LEFT == RIGHT == exact-once.
//!
//!   2. Both spellings self-compose (trunc of trunc is trunc, floor of floor
//!      is floor, for divisor products of powers of two), so `60`'s fusion
//!      argument predicts a saving of at least F at n = 3 and 4 for BOTH
//!      spellings, signed operands notwithstanding.
//!
//!   3. At n = 5, `60`'s probe D found the unsigned truncating saving grows
//!      past F (4 bits against F = 3), attributed to the final adaptation
//!      absorbing interior differences. Whether the signed cell reproduces
//!      that, and whether the two spellings agree on it, is the open
//!      measurement; no prediction either way.
//!
//!   4. The linear-growth headline is untouched: whatever the saving, the
//!      absolute guard needed grows with n, and there is no signed analogue
//!      of addition's fold-length-independent closed form.
//!
//! INSTRUMENT VALIDATION. w = 0 must diverge for n >= 3 (the coarsen-only
//! mechanism, measured signed in p2 section 3); w = full must be clean for
//! every n and both spellings; and the with-clamp variant (58's section 2
//! bug-turned-finding, here with the TWO-SIDED signed clamp) must diverge at
//! full fractional precision, showing the checker can fail and the second
//! mechanism transplants to the signed domain.
//!
//! Build and run:
//!   rustc +nightly-2026-05-28 -O --edition 2021 \
//!       -o p4 p4_signed_multiplicative_accumulator.rs && ./p4

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rescale {
    /// toward zero
    Trunc,
    /// toward negative infinity (arithmetic shift)
    Floor,
}

fn narrow(x: i128, shift: u32, r: Rescale) -> i128 {
    if shift == 0 {
        return x;
    }
    match r {
        Rescale::Trunc => x / (1i128 << shift),
        Rescale::Floor => x >> shift,
    }
}

const LO: i64 = -8;
const HI: i64 = 7;

/// no intermediate range clamp: pure fractional axis, range adapted once at
/// the end, mirroring 58_probes/p2's eager_no_clamp with sign and a spelling.
fn eager_no_clamp(ops: &[i64], f: u32, w: u32, r: Rescale, right_assoc: bool) -> i64 {
    let seq: Vec<i64> = if right_assoc {
        ops.iter().rev().copied().collect()
    } else {
        ops.to_vec()
    };
    let mut acc: i128 = (seq[0] as i128) << w;
    for &a in &seq[1..] {
        acc = narrow(acc * (a as i128), f, r);
    }
    narrow(acc, w, r).clamp(LO as i128, HI as i128) as i64
}

/// the with-clamp variant: two-sided signed range clamp at every step, at
/// the guard-scaled bounds. this is 58 section 2's second mechanism with the
/// signed geometry (a clamp face on BOTH sides, and sign-flipping products
/// that can carry a clamped value toward the opposite face).
fn eager_with_clamp(ops: &[i64], f: u32, w: u32, r: Rescale, right_assoc: bool) -> i64 {
    let seq: Vec<i64> = if right_assoc {
        ops.iter().rev().copied().collect()
    } else {
        ops.to_vec()
    };
    let lo_w = (LO as i128) << w;
    let hi_w = (HI as i128) << w;
    let mut acc: i128 = (seq[0] as i128) << w;
    for &a in &seq[1..] {
        acc = narrow(acc * (a as i128), f, r).clamp(lo_w, hi_w);
    }
    narrow(acc, w, r).clamp(LO as i128, HI as i128) as i64
}

fn exact_once(ops: &[i64], f: u32, r: Rescale) -> i64 {
    let mut prod: i128 = 1;
    for &a in ops {
        prod *= a as i128;
    }
    let extra = (ops.len() as u32 - 1) * f;
    narrow(prod, extra, r).clamp(LO as i128, HI as i128) as i64
}

fn for_each_tuple(n: usize, mut visit: impl FnMut(&[i64])) {
    let card = (HI - LO + 1) as u64;
    let mut xs = vec![0i64; n];
    let combos = card.pow(n as u32);
    for code in 0..combos {
        let mut c = code;
        for i in 0..n {
            xs[i] = LO + (c % card) as i64;
            c /= card;
        }
        visit(&xs);
    }
}

fn divergence(
    fold: fn(&[i64], u32, u32, Rescale, bool) -> i64,
    n: usize,
    f: u32,
    w: u32,
    r: Rescale,
) -> u64 {
    let mut d = 0u64;
    for_each_tuple(n, |t| {
        if fold(t, f, w, r, false) != fold(t, f, w, r, true) {
            d += 1;
        }
    });
    d
}

fn main() {
    let mut ok = true;
    let f = 3u32;

    println!("=== section 1: signed multiplicative guard sweep, no intermediate clamp ===");
    println!();
    println!(
        "  format: Q = [{}, {}], F = {} (signed 1.3). exhaustive over Q^n.",
        LO, HI, f
    );
    println!();
    println!(
        "  {:>2} {:>8} {:>6} {:>10} {:>12} {:>10} {:>8}",
        "n", "tuples", "spell", "div at w=0", "full w", "min w", "saving"
    );

    let mut w0_diverges = true;
    let mut full_clean = true;
    let mut saving_at_least_f_n34 = true;
    for n in 2usize..=5 {
        let full = (n as u32 - 1) * f;
        for (rn, r) in [("trunc", Rescale::Trunc), ("floor", Rescale::Floor)] {
            let div0 = divergence(eager_no_clamp, n, f, 0, r);
            let mut minw = full + 1;
            for w in 0..=full {
                if divergence(eager_no_clamp, n, f, w, r) == 0 {
                    minw = w;
                    break;
                }
            }
            let tuples = ((HI - LO + 1) as u64).pow(n as u32);
            let saving = full as i64 - minw as i64;
            println!(
                "  {:>2} {:>8} {:>6} {:>10} {:>12} {:>10} {:>8}",
                n, tuples, rn, div0, full, minw, saving
            );
            if n >= 3 {
                w0_diverges &= div0 > 0;
                if minw > full {
                    full_clean = false;
                }
                if n <= 4 && saving < f as i64 {
                    saving_at_least_f_n34 = false;
                }
            }
            // cross-check at full precision against exact-once
            let mut mism = 0u64;
            for_each_tuple(n, |t| {
                if eager_no_clamp(t, f, full, r, false) != exact_once(t, f, r) {
                    mism += 1;
                }
            });
            if mism != 0 {
                full_clean = false;
            }
        }
    }
    println!();
    println!(
        "  w=0 diverges for every n >= 3, both spellings: {}",
        w0_diverges
    );
    println!(
        "  full precision clean and equal to exact-once, both spellings: {}",
        full_clean
    );
    println!(
        "  saving >= F at n = 3 and 4 for both spellings (the fusion prediction): {}",
        saving_at_least_f_n34
    );
    ok &= w0_diverges && full_clean && saving_at_least_f_n34;
    println!();

    println!("=== section 2: the two-sided intermediate clamp, at full fractional precision ===");
    println!();
    let mut clamp_dirty_at_full = false;
    for n in 3usize..=4 {
        let full = (n as u32 - 1) * f;
        for (rn, r) in [("trunc", Rescale::Trunc), ("floor", Rescale::Floor)] {
            let no_c = divergence(eager_no_clamp, n, f, full, r);
            let with_c = divergence(eager_with_clamp, n, f, full, r);
            println!(
                "  n={} full w={} {}: no-clamp div {}  with-clamp div {}",
                n, full, rn, no_c, with_c
            );
            clamp_dirty_at_full |= with_c > 0;
        }
    }
    println!();
    println!(
        "  the two-sided intermediate clamp diverges at full fractional precision\n\
         (58's second mechanism, alive in the signed domain): {}",
        clamp_dirty_at_full
    );
    ok &= clamp_dirty_at_full;

    println!();
    println!("{}", if ok { "P4 WORKS" } else { "P4 FAILS" });
    std::process::exit(if ok { 0 } else { 1 });
}
