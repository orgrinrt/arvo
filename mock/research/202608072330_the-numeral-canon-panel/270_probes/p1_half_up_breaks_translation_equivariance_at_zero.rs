//! p1: does the shipped, away-from-zero `HalfUp` commute with translation
//! across zero, the property `law::fusing_a_multiply_add_preserves_the_answer_
//! under_signed_wrapping` currently claims of it?
//!
//! Seat 270. The law's `holds` region lists `rounding: in {floor, ceil,
//! half_up}` under signed wrapping as translation equivariant. Translation
//! equivariance is `round(x + k) == round(x) + k` for integer `k`. The rule
//! for each mode is transcribed directly from
//! `mock/crates/arvo-format/src/apply.rs`'s `round_slot` (the `Floor`, `Ceil`
//! and `HalfUp` arms, the latter with its `away from zero` tie rule, which is
//! also what `mock/crates/arvo-format/src/apply/tests/mod.rs`'s
//! `half_up_goes_away_from_zero_on_a_tie_and_half_even_goes_to_the_even_slot`
//! asserts against the shipped crate). `round(x)` here returns the rounded
//! integer as a rational numerator over a fixed denominator of 2, so a tie is
//! exactly numerator `1`.
//!
//! The cases that must fail, all run and reported:
//!   C1  floor must be reported equivariant across the same zero-crossing
//!       shift. Without it the instrument cannot tell equivariant from not.
//!   C2  ceil must be reported equivariant across the same shift, for the same
//!       reason as C1 with the opposite tie side.
//!   C3  a deliberately-broken "away from zero but only for floor" rule must
//!       be reported non-equivariant. One control that can only fail on the
//!       named case is not a control.
//!   C4  half_up (away from zero, as shipped) must be reported NON-
//!       equivariant at the tie -0.5 -> 0.5, which is the finding.
//!   C5  the alternate reading, half_up as ties-toward-positive-infinity, must
//!       be reported EQUIVARIANT at the same shift, showing the law's claim is
//!       true of that reading and false of the shipped one, not false of both.

// Slot value doubled, position = slot + 1/2 exactly, for the one case this
// probe needs. Returns the rounded integer.

fn floor_of(slot: i64) -> i64 {
    slot
}

fn ceil_of(slot: i64) -> i64 {
    slot + 1
}

// Shipped `HalfUp`, transcribed from `apply.rs`: at a tie, negative goes down
// (away from zero), non-negative goes up.
fn half_up_away_from_zero(slot: i64) -> i64 {
    if slot < 0 { slot } else { slot + 1 }
}

// The other reading: ties toward positive infinity, always resolves up.
fn half_up_toward_positive_infinity(_slot: i64) -> i64 {
    _slot + 1
}

// C3's deliberately broken control: floor except it adds one at slot == -1,
// which must be caught as non-equivariant by construction.
fn broken_floor(slot: i64) -> i64 {
    if slot == -1 { slot + 1 } else { slot }
}

/// Position is `slot + 1/2`. Shifting by integer `k` moves the position to
/// `slot + k + 1/2`, i.e. calls the same rule at `slot + k`. Equivariant means
/// `rule(slot + k) - k == rule(slot)` for the `k` this probe exercises.
fn equivariant_at_shift(rule: fn(i64) -> i64, slot: i64, k: i64) -> bool {
    rule(slot + k) - k == rule(slot)
}

fn report(name: &str, ok: bool, must_hold: bool) {
    let verdict = if ok == must_hold { "PASS" } else { "FAIL" };
    println!("{verdict}  {name}: equivariant={ok} (must be {must_hold})");
}

fn main() {
    // The tie at slot = -1 (position -0.5), shifted by k = 1 to slot = 0
    // (position 0.5). This is the exact zero-crossing tie the FMA law's
    // `holds` region names via `signedness: signed, overflow_policy: wrap`.
    let slot = -1i64;
    let k = 1i64;

    report("C1 floor", equivariant_at_shift(floor_of, slot, k), true);
    report("C2 ceil", equivariant_at_shift(ceil_of, slot, k), true);
    report(
        "C3 broken_floor (control, must be caught non-equivariant)",
        equivariant_at_shift(broken_floor, slot, k),
        false,
    );
    report(
        "C4 half_up (shipped, away from zero)",
        equivariant_at_shift(half_up_away_from_zero, slot, k),
        false,
    );
    report(
        "C5 half_up (alternate reading, toward positive infinity)",
        equivariant_at_shift(half_up_toward_positive_infinity, slot, k),
        true,
    );

    // Concrete values, so the finding is readable without re-deriving it.
    println!();
    println!("position -0.5 (slot {slot}):");
    println!(
        "  floor {}   ceil {}   half_up(away) {}   half_up(+inf) {}",
        floor_of(slot),
        ceil_of(slot),
        half_up_away_from_zero(slot),
        half_up_toward_positive_infinity(slot)
    );
    println!("position 0.5 (slot {}, shifted by {k}):", slot + k);
    println!(
        "  floor {}   ceil {}   half_up(away) {}   half_up(+inf) {}",
        floor_of(slot + k),
        ceil_of(slot + k),
        half_up_away_from_zero(slot + k),
        half_up_toward_positive_infinity(slot + k)
    );
    println!();
    println!(
        "half_up(away): round(-0.5)={}, round(-0.5)+1={}, round(0.5)={} -> {}",
        half_up_away_from_zero(slot),
        half_up_away_from_zero(slot) + k,
        half_up_away_from_zero(slot + k),
        if half_up_away_from_zero(slot) + k == half_up_away_from_zero(slot + k) {
            "equivariant"
        } else {
            "NOT equivariant"
        }
    );
}
