//! p1. The two things that let a chain drop an interior resolution are
//! independent, and neither alone covers the other.
//!
//! A chain of operations on a declared W-bit numeral is realised in some
//! container. The honest realisation applies the declared resolution `pi`
//! after every step ("eager"). A cheaper realisation applies it once, at the
//! chain boundary ("deferred"). The two agree only under a licence, and this
//! probe establishes that there are at least two distinct licences:
//!
//!   (A) RANGE. Every intermediate provably lies in the subset of the
//!       container where `pi` is the identity, so every interior `pi` is a
//!       no-op. Depends on the widths and on a static bound. Does not depend
//!       on which operations the chain contains.
//!
//!   (B) ALGEBRA. `pi` commutes with, or is absorbed by, the composition, so
//!       interior applications may be deleted whatever the values are.
//!       Depends on which operations and which resolution. Does not depend on
//!       any bound.
//!
//! Independence is shown by exhibiting a case where each holds and the other
//! fails.
//!
//! THE CASES THAT MUST FAIL. A probe whose deletion is always safe measures
//! nothing, so two negative controls are asserted to DISAGREE:
//!   - clamping with a mixed add/subtract chain (B fails: clamping is a
//!     retraction only for monotone accumulation),
//!   - rounding-to-a-grid with an intermediate off the grid (B fails: rounding
//!     is idempotent but is not a homomorphism).
//! If either of those agreed, this probe would be proving nothing and its
//! positive results would be worthless.
//!
//! Run: rustc -O p1_two_licences_are_independent.rs -o /tmp/p1 && /tmp/p1

// ---------------------------------------------------------------------------
// Resolutions. Each is `pi` for one declared meaning.
// ---------------------------------------------------------------------------

const fn wrap(v: u128, w: u32) -> u128 {
    if w >= 128 { v } else { v & ((1u128 << w) - 1) }
}
const fn clamp(v: u128, w: u32) -> u128 {
    let l = if w >= 128 { u128::MAX } else { (1u128 << w) - 1 };
    if v > l { l } else { v }
}
/// Round-half-up to a grid of `2^g`, then keep the value. This is what a
/// narrowing from F+g fraction bits to F fraction bits does.
const fn round_grid(v: u128, g: u32) -> u128 {
    if g == 0 { return v; }
    let step = 1u128 << g;
    let half = step >> 1;
    ((v + half) / step) * step
}

// ---------------------------------------------------------------------------
// Steps. A step is a unary function on the container value.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum Step { AddK(u128), WSubK(u128), SatSubK(u128), Mul3 }

fn apply(s: Step, v: u128) -> u128 {
    match s {
        Step::AddK(k) => v.wrapping_add(k),
        Step::WSubK(k) => v.wrapping_sub(k),
        Step::SatSubK(k) => v.saturating_sub(k),
        Step::Mul3 => v.wrapping_mul(3),
    }
}

fn run<F: Fn(u128) -> u128>(x: u128, steps: &[Step], pi: F, eager: bool) -> u128 {
    let mut v = x;
    for &s in steps {
        v = apply(s, v);
        if eager { v = pi(v); }
    }
    pi(v)
}

fn disagreement<F: Fn(u128) -> u128 + Copy>(
    steps: &[Step], pi: F, xs: &[u128],
) -> Option<(u128, u128, u128)> {
    for &x in xs {
        let e = run(x, steps, pi, true);
        let d = run(x, steps, pi, false);
        if e != d { return Some((x, e, d)); }
    }
    None
}

fn main() {
    let w: u32 = 13;
    let l = (1u128 << w) - 1;
    let k = l / 3 | 1;

    // A spread of inputs including ones that certainly overflow the declared
    // width under the chain below, which is what makes the RANGE licence fail.
    let wide: Vec<u128> = (0..4096u128).map(|i| (i * 2654435761) % (l + 1)).collect();
    // Every step ring-affine, so the wrapping homomorphism covers the chain.
    let affine = [Step::AddK(k), Step::Mul3, Step::WSubK(k)];
    // The same chain with ONE step swapped for a saturating subtraction, which
    // is not a ring homomorphism. It is here to show the licence is a property
    // of every step rather than of the endpoints.
    let one_foreign = [Step::AddK(k), Step::Mul3, Step::SatSubK(k)];
    let nonneg = [Step::AddK(k), Step::AddK(k), Step::AddK(k)];
    // Two clamping additions then a large saturating subtraction. The order
    // matters: an earlier version put the subtraction in the middle and the
    // final clamp absorbed the difference, so the control could not fire even
    // though the retraction lemma genuinely fails for this chain. A control
    // that cannot fire is the defect this probe exists to avoid, so the case
    // is kept in its firing form and the near miss is recorded here.
    let mixed  = [Step::AddK(k), Step::AddK(k), Step::SatSubK(l / 2)];

    println!("W = {w}, limit = {l}, k = {k}");
    println!();

    // ---- B holds where A fails ------------------------------------------
    // Wrapping is a ring homomorphism, so eager and deferred agree on an
    // affine chain even when every intermediate leaves [0, 2^W).
    let over = wide.iter().filter(|&&x| {
        let mut v = x; for &s in affine.iter() { v = apply(s, v); }
        v > l
    }).count();
    assert!(over > 0, "no input overflows W, so the RANGE licence is not being denied");
    let d = disagreement(&affine, |v| wrap(v, w), &wide);
    println!("B-holds-A-fails  wrap, affine chain, {over}/{} inputs exceed 2^W", wide.len());
    println!("  eager vs deferred: {}", match d { None => "AGREE".to_string(), Some(t) => format!("DISAGREE {t:?}") });
    assert!(d.is_none(), "wrapping is a homomorphism; this must agree");

    // ---- NEGATIVE CONTROL 0: one foreign step revokes the licence -------
    // Identical chain with a single saturating subtraction in place of the
    // wrapping one. Nothing about the endpoints changed.
    let d = disagreement(&one_foreign, |v| wrap(v, w), &wide);
    println!("CONTROL  wrap, same chain with ONE saturating step");
    println!("  eager vs deferred: {}", match d { None => "AGREE".to_string(), Some(t) => format!("DISAGREE {t:?}") });
    assert!(d.is_some(), "CONTROL FAILED: a non-homomorphic step did not revoke the licence, so the licence is not being tested");

    // ---- A holds where B fails ------------------------------------------
    // Rounding to a grid of 2^g. Every value in `narrow` is a multiple of the
    // grid and every step preserves that, so `pi` is the identity throughout
    // and the deletion is licensed by RANGE (here: exactness) alone.
    let g: u32 = 3;
    let grid: Vec<u128> = (0..4096u128).map(|i| ((i * 7919) % 64) << g).collect();
    let kg = (k >> g) << g;
    let on_grid = [Step::AddK(kg), Step::Mul3, Step::AddK(kg)];
    let d = disagreement(&on_grid, |v| round_grid(v, g), &grid);
    println!("A-holds-B-fails  round to 2^{g}, every intermediate on the grid");
    println!("  eager vs deferred: {}", match d { None => "AGREE".to_string(), Some(t) => format!("DISAGREE {t:?}") });
    assert!(d.is_none(), "every intermediate is on the grid; rounding is the identity");

    // ---- NEGATIVE CONTROL 1: rounding is not a homomorphism -------------
    // Same rounding, same chain shape, operands off the grid. If this agreed,
    // the positive result above would be proving nothing.
    let off_grid = [Step::AddK(kg | 1), Step::Mul3, Step::AddK(kg | 1)];
    let d = disagreement(&off_grid, |v| round_grid(v, g), &grid);
    println!("CONTROL  round to 2^{g}, operand off the grid");
    println!("  eager vs deferred: {}", match d { None => "AGREE".to_string(), Some(t) => format!("DISAGREE {t:?}") });
    assert!(d.is_some(), "CONTROL FAILED: rounding deletion looked safe, so this probe measures nothing");

    // ---- B holds: clamping is a retraction under monotone accumulation ---
    let d = disagreement(&nonneg, |v| clamp(v, w), &wide);
    println!("B-holds  clamp, non-negative additions only");
    println!("  eager vs deferred: {}", match d { None => "AGREE".to_string(), Some(t) => format!("DISAGREE {t:?}") });
    assert!(d.is_none(), "the retraction lemma must hold here");

    // ---- NEGATIVE CONTROL 2: the retraction lemma is not general --------
    let d = disagreement(&mixed, |v| clamp(v, w), &wide);
    println!("CONTROL  clamp, mixed add/subtract chain");
    println!("  eager vs deferred: {}", match d { None => "AGREE".to_string(), Some(t) => format!("DISAGREE {t:?}") });
    assert!(d.is_some(), "CONTROL FAILED: clamp deletion looked safe under subtraction, so the retraction claim is untested here");

    // ---- Neither licence: wrapping does not survive a non-affine step ----
    // A shift right is not a ring homomorphism's friend: (a>>1) mod 2^W is not
    // ((a mod 2^W) >> 1) when a has bits at or above W.
    let with_shift = |x: u128, eager: bool| -> u128 {
        let mut v = x;
        v = v.wrapping_mul(3); if eager { v = wrap(v, w); }
        v >>= 1;               if eager { v = wrap(v, w); }
        wrap(v, w)
    };
    let mut found = None;
    for &x in wide.iter() {
        if with_shift(x, true) != with_shift(x, false) { found = Some(x); break; }
    }
    println!("CONTROL  wrap, chain containing a right shift");
    println!("  eager vs deferred: {}", match found { None => "AGREE".to_string(), Some(x) => format!("DISAGREE at x={x}") });
    assert!(found.is_some(), "CONTROL FAILED: wrapping looked safe past a non-affine step");

    println!();
    println!("RESULT: the two licences are independent, and both negative controls fired.");
}
