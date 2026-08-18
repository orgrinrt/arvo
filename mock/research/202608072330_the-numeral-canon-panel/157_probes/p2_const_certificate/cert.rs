// 157 P2. A separation certificate checked at const time, at real widths.
//
// CLAIM UNDER TEST
//   111:507-573 says adequacy is "checkable the way 110 checks a congruence: at model
//   widths, exhaustively, with the transfer argument named rather than assumed."
//   This file tests whether the completeness half needs a model width at all.
//
//   Completeness is a conjunction of INEQUALITIES between denotations. An inequality is
//   discharged by ONE witness. So the check is: evaluate both realisation maps at one
//   argument and compare. Cost is O(1) per pair, at any width, with no enumeration and
//   therefore nothing to transfer.
//
// NEGATIVE CONTROL, stated before the run
//   `separates_rounding_at_f0` must be FALSE at every width, so the assertion over it
//   must FAIL TO COMPILE. If it compiles, this probe proves nothing: it would mean the
//   witness scheme accepts an axis that denotes nothing, which is exactly the spurious
//   parameter 112 F112-2 says must not exist.
//   The failing assertion is behind `cfg(control)` so the file builds both ways and the
//   difference between the two builds is the evidence.

#![allow(dead_code)]

const fn max_of(w: u32) -> u64 {
    if w >= 64 { u64::MAX } else { (1u64 << w) - 1 }
}

// R(p) for policy = wrap, over an exact result held in u128.
const fn realise_wrap(exact: u128, w: u32) -> u64 {
    let span: u128 = 1u128 << w;
    (exact % span) as u64
}

// R(p) for policy = saturate.
const fn realise_sat(exact: u128, w: u32) -> u64 {
    let hi = max_of(w) as u128;
    if exact > hi { hi as u64 } else { exact as u64 }
}

// The uniform witness for the overflow-policy axis: add the maximum to one.
// Closed form, independent of w. This is the whole certificate.
const fn policy_witness(w: u32) -> (u128, u128) {
    (max_of(w) as u128, 1u128)
}

const fn separates_policy(w: u32) -> bool {
    let (a, b) = policy_witness(w);
    let exact = a + b;
    realise_wrap(exact, w) != realise_sat(exact, w)
}

// The control axis. At F = 0 with an integer-valued signature, both rounding modes are
// the identity on the exact result, so no argument can separate them. The search below
// is exhaustive over a large sample of arguments and must find nothing.
const fn round_trunc(exact: u128) -> u128 { exact }
const fn round_near(exact: u128) -> u128 { exact }

const fn separates_rounding_at_f0(w: u32) -> bool {
    let mut a: u128 = 0;
    while a < 4096 {
        let e = a;
        if realise_wrap(round_trunc(e), w) != realise_wrap(round_near(e), w) {
            return true;
        }
        a += 1;
    }
    // and the two extremes, which is where a range region would bite if it were read
    let hi = max_of(w) as u128;
    realise_sat(round_trunc(hi + 1), w) != realise_sat(round_near(hi + 1), w)
}

// ---- the certificate, discharged at real widths, at compile time ----
const _: () = assert!(separates_policy(1));
const _: () = assert!(separates_policy(3));
const _: () = assert!(separates_policy(13));
const _: () = assert!(separates_policy(47));
const _: () = assert!(separates_policy(63));
const _: () = assert!(separates_policy(64));

// The whole family in one const loop: 1..=64, no enumeration over values.
const fn policy_separates_every_width() -> bool {
    let mut w = 1u32;
    while w <= 64 {
        if !separates_policy(w) { return false; }
        w += 1;
    }
    true
}
const _: () = assert!(policy_separates_every_width());

// ---- the control ----
#[cfg(control)]
const _: () = assert!(separates_rounding_at_f0(64));

fn main() {
    println!("separates_policy(64) = {}", separates_policy(64));
    println!("separates_policy(13) = {}", separates_policy(13));
    println!("policy_separates_every_width() = {}", policy_separates_every_width());
    println!("separates_rounding_at_f0(64) = {}", separates_rounding_at_f0(64));
    // The arity-1 mask signature 154 P4 used, at the same point, for comparison.
    let w = 64u32;
    let mask_wrap = |x: u64| x & max_of(w);
    let mask_clamp = |x: u64| if x > max_of(w) { max_of(w) } else { x };
    let mut differ = 0u32;
    for x in 0..100_000u64 { if mask_wrap(x) != mask_clamp(x) { differ += 1; } }
    println!("arity-1 mask at W=64: differing inputs in 0..100000 = {differ}  (154 P4's collapse)");
}
