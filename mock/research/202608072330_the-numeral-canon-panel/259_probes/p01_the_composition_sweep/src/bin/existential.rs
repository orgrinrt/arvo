//! Step 03. The existential reading, which is the one the ruling under review
//! uses. Not "does the natural composition agree" but "is there any composition
//! of a declared multiply and a declared add, at this format, whose answer is the
//! fused answer at every triple".
//!
//! The search space is every arm a consumer could actually write at one format.
//! A multiply is a declared signature and an add is a declared signature, and the
//! two need not be the same one, so an arm is a triple of choices: the multiply's
//! rounding mode, the multiply's overflow policy, and the add's overflow policy.
//! The add's rounding mode is not a choice, because a slot plus a slot is on the
//! grid and the add's rounding region is dead; C3 measures that rather than
//! assuming it. Five modes times two policies times two policies is twenty arms,
//! against ten targets, at every cell.
//!
//! Stated before the run:
//!
//! C1 must hold: at every signed saturating cell, all twenty arms must die. That
//!    is the ruling's exception under its own reading, and it is the case that
//!    must fail for the rest of the table to mean anything.
//! C2 must hold: at every cell that is not signed saturating, at least one target
//!    must be reachable, and the identity arm must be among the survivors
//!    wherever the natural composition agreed in step 01.
//! C3 must hold: varying the add's rounding mode must never change any answer.
//!    If it did, the twenty-arm space would be a hundred-arm space and this step
//!    would have searched a fifth of it.
//! C4 must hold: the search must kill arms. A run where every arm survives every
//!    target is a run where the comparison is not being made.
//!
//! Stated after C2 broke, and labelled as such because they were written knowing
//! the answer and are therefore worth less than the four above:
//!
//! C5: a target is reachable by some arm exactly when the identity arm reaches
//!     it, over every target including the signed saturating ones. If that holds,
//!     the nineteen non-identity arms buy nothing anywhere and the existential
//!     reading collapses onto the natural composition.
//! C6: `half_even` is unreachable at every fraction length above zero, at both
//!     signednesses and both policies.

use arvo_format::adapt::{Adapt, DeclaredSignature, Signature};
use arvo_format::apply::{Dither, Exact, Fraction, adapt};
use arvo_format::format::Format;
use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::rounding::{Ceil, Floor, HalfEven, HalfUp, TowardZero};
use arvo_format::slots::{Slot, Slots};
use arvo_format::standards::{Fi, Ufi};

/// The ten declared signatures over one format, in a fixed order: five modes,
/// each under wrap then saturate.
const NAMES: [(&str, &str); 10] = [
    ("floor", "wrap"),
    ("floor", "saturate"),
    ("ceil", "wrap"),
    ("ceil", "saturate"),
    ("toward_zero", "wrap"),
    ("toward_zero", "saturate"),
    ("half_up", "wrap"),
    ("half_up", "saturate"),
    ("half_even", "wrap"),
    ("half_even", "saturate"),
];

macro_rules! ten {
    ($fmt:ty, $call:expr) => {{
        let mut out = [0i64; 10];
        macro_rules! at {
            ($idx:literal, $m:ident, $o:ident) => {
                out[$idx] = ($call)(core::marker::PhantomData::<Signature<$fmt, Adapt<$m, $o>>>);
            };
        }
        at!(0, Floor, Wrap);
        at!(1, Floor, Saturate);
        at!(2, Ceil, Wrap);
        at!(3, Ceil, Saturate);
        at!(4, TowardZero, Wrap);
        at!(5, TowardZero, Saturate);
        at!(6, HalfUp, Wrap);
        at!(7, HalfUp, Saturate);
        at!(8, HalfEven, Wrap);
        at!(9, HalfEven, Saturate);
        out
    }};
}

fn fused_of<S: DeclaredSignature>(
    _: core::marker::PhantomData<S>,
    a: i64,
    b: i64,
    c: i64,
    den: i64,
) -> i64 {
    adapt::<S>(
        Exact::between(Slot::at(c), Fraction::of(a * b, den)),
        Dither::UNUSED,
    )
    .index()
}

fn mul_of<S: DeclaredSignature>(_: core::marker::PhantomData<S>, a: i64, b: i64, den: i64) -> i64 {
    adapt::<S>(
        Exact::between(Slot::ZERO, Fraction::of(a * b, den)),
        Dither::UNUSED,
    )
    .index()
}

fn add_of<S: DeclaredSignature>(_: core::marker::PhantomData<S>, p: i64, c: i64) -> i64 {
    adapt::<S>(Exact::on_grid(Slot::at(p + c)), Dither::UNUSED).index()
}

struct CellResult {
    /// For each of the ten targets, the arms that reproduced it everywhere.
    survivors: [Vec<usize>; 10],
    killed:    u64,
}

/// One cell of the search. Twenty arms against ten targets, exhaustively.
///
/// The arm index is `mul * 2 + add_policy`, where `mul` indexes `NAMES` and
/// `add_policy` is 0 for wrap and 1 for saturate.
fn search<F: Format>(fraction: u32) -> CellResult {
    let lo = <F::Slots as Slots>::MIN.index();
    let hi = <F::Slots as Slots>::MAX.index();
    let den = 1i64 << fraction;

    let mut alive = [[true; 20]; 10];
    for a in lo ..= hi {
        for b in lo ..= hi {
            let muls = ten!(F, |p| mul_of(p, a, b, den));
            for c in lo ..= hi {
                let fuseds = ten!(F, |p| fused_of(p, a, b, c, den));
                let mut steps = [0i64; 20];
                for m in 0 .. 10 {
                    steps[m * 2] = add_of(
                        core::marker::PhantomData::<Signature<F, Adapt<Floor, Wrap>>>,
                        muls[m],
                        c,
                    );
                    steps[m * 2 + 1] = add_of(
                        core::marker::PhantomData::<Signature<F, Adapt<Floor, Saturate>>>,
                        muls[m],
                        c,
                    );
                }
                for target in 0 .. 10 {
                    for arm in 0 .. 20 {
                        if alive[target][arm] && fuseds[target] != steps[arm] {
                            alive[target][arm] = false;
                        }
                    }
                }
            }
        }
    }

    let mut killed = 0u64;
    let survivors: [Vec<usize>; 10] = core::array::from_fn(|target| {
        let mut kept = Vec::new();
        for arm in 0 .. 20 {
            if alive[target][arm] {
                kept.push(arm);
            } else {
                killed += 1;
            }
        }
        kept
    });
    CellResult {
        survivors,
        killed,
    }
}

fn arm_name(arm: usize) -> String {
    let (mode, policy) = NAMES[arm / 2];
    let add = if arm.is_multiple_of(2) { "wrap" } else { "saturate" };
    format!("mul={mode}/{policy} add={add}")
}

fn main() {
    println!("# step 03: is the fused answer reachable by any arm a consumer could write");
    println!("# arm = (multiply mode, multiply policy, add policy); 20 arms, 10 targets");
    println!();

    let mut signed_saturating_targets = 0u64;
    let mut signed_saturating_reached = 0u64;
    let mut other_targets = 0u64;
    let mut other_targets_reached = 0u64;
    let mut identity_arm_missing = 0u64;
    let mut total_killed = 0u64;
    let mut all_targets = 0u64;
    let mut reachable_matches_identity = 0u64;
    let mut half_even_targets_above_zero = 0u64;
    let mut half_even_reached_above_zero = 0u64;

    macro_rules! cell {
        ($fmt:ty, $sign:literal, $w:literal, $f:literal) => {{
            let result = search::<$fmt>($f);
            total_killed += result.killed;
            for target in 0 .. 10 {
                let (mode, policy) = NAMES[target];
                let kept = &result.survivors[target];
                let names: Vec<String> = kept.iter().map(|a| arm_name(*a)).collect();
                println!(
                    "reach {} W={} F={} target={}/{} arms={} {:?}",
                    $sign,
                    $w,
                    $f,
                    mode,
                    policy,
                    kept.len(),
                    names
                );

                // The identity arm is the one with the target's own mode and the
                // target's own policy on both operations.
                let identity = target * 2 + if policy == "wrap" { 0 } else { 1 };
                all_targets += 1;
                if kept.is_empty() == !kept.contains(&identity) {
                    reachable_matches_identity += 1;
                }

                if mode == "half_even" && $f >= 1 {
                    half_even_targets_above_zero += 1;
                    if !kept.is_empty() {
                        half_even_reached_above_zero += 1;
                    }
                }

                if $sign == "signed" && policy == "saturate" {
                    signed_saturating_targets += 1;
                    if !kept.is_empty() {
                        signed_saturating_reached += 1;
                    }
                } else {
                    other_targets += 1;
                    if !kept.is_empty() {
                        other_targets_reached += 1;
                        if !kept.contains(&identity) {
                            identity_arm_missing += 1;
                        }
                    }
                }
            }
        }};
    }

    macro_rules! both {
        ($w:literal, $f:literal) => {{
            cell!(Ufi<$w, $f>, "unsigned", $w, $f);
            cell!(Fi<$w, $f>, "signed", $w, $f);
        }};
    }

    both!(3, 0);
    both!(3, 1);
    both!(3, 2);
    both!(4, 0);
    both!(4, 1);
    both!(4, 2);
    both!(4, 3);
    both!(5, 0);
    both!(5, 1);
    both!(5, 2);
    both!(5, 3);
    both!(5, 4);
    both!(6, 0);
    both!(6, 1);
    both!(6, 2);
    both!(6, 3);
    both!(6, 4);
    both!(6, 5);

    println!();
    println!("# --- the arms stated before the run ---");
    println!(
        "C1 detail: {signed_saturating_targets} signed saturating targets, {signed_saturating_reached} of them reached by some arm"
    );
    verdict(
        "C1",
        "no signed saturating target may be reached by any of the twenty arms",
        signed_saturating_reached == 0 && signed_saturating_targets > 0,
    );
    println!(
        "C2 detail: {other_targets} targets outside signed saturating, {other_targets_reached} reached; identity arm absent from a non-empty survivor set {identity_arm_missing} times"
    );
    verdict(
        "C2",
        "outside signed saturating every target must be reached, and by the identity arm",
        other_targets_reached == other_targets && identity_arm_missing == 0 && other_targets > 0,
    );
    println!(
        "C2 note: the half of C2 about the identity arm holds. The half about every target being reached does not, and that break is the finding of this step: {} targets outside signed saturating are reached by nothing.",
        other_targets - other_targets_reached
    );
    arm_c3();
    println!("C4 detail: arms killed across the search: {total_killed}");
    verdict("C4", "the search must kill arms", total_killed > 0);

    println!();
    println!("# --- the arms stated after C2 broke, and worth less for it ---");
    println!(
        "C5 detail: {all_targets} targets, {reachable_matches_identity} where reachability and the identity arm's survival agree"
    );
    verdict(
        "C5",
        "a target is reachable exactly when the identity arm reaches it, so the other nineteen arms buy nothing",
        reachable_matches_identity == all_targets && all_targets > 0,
    );
    println!(
        "C6 detail: {half_even_targets_above_zero} half_even targets at F >= 1, {half_even_reached_above_zero} reached"
    );
    verdict(
        "C6",
        "half_even is unreachable at every fraction length above zero",
        half_even_reached_above_zero == 0 && half_even_targets_above_zero > 0,
    );
}

fn verdict(name: &str, expectation: &str, held: bool) {
    println!(
        "{name}: {expectation} -> {}",
        if held { "HELD" } else { "BROKEN" }
    );
}

/// The add's rounding mode is not a choice, because the position it adapts is on
/// the grid. Measured rather than argued: five modes over the whole cell, and no
/// answer may move.
fn arm_c3() {
    type Fmt = Fi<6, 3>;
    let den = 1i64 << 3;
    let lo = -32i64;
    let hi = 31i64;
    let mut moved = 0u64;
    let mut checked = 0u64;
    for a in lo ..= hi {
        for b in lo ..= hi {
            let product = mul_of(
                core::marker::PhantomData::<Signature<Fmt, Adapt<HalfEven, Wrap>>>,
                a,
                b,
                den,
            );
            for c in lo ..= hi {
                let answers = ten!(Fmt, |p| add_of(p, product, c));
                // Indices 0, 2, 4, 6, 8 are the five modes under wrap and the odd
                // ones the same five under saturate. Within each policy the five
                // must agree.
                for group in [[0, 2, 4, 6, 8], [1, 3, 5, 7, 9]] {
                    let first = answers[group[0]];
                    for index in group {
                        checked += 1;
                        if answers[index] != first {
                            moved += 1;
                        }
                    }
                }
            }
        }
    }
    println!("C3 detail: {checked} add answers compared, {moved} moved with the mode");
    verdict(
        "C3",
        "the add's rounding mode must never change an answer",
        moved == 0 && checked > 0,
    );
}
