// Seat 253. Does the shipped admission contract refuse a candidate that is
// coherent as a declaration, on a capacity bound of this crate's own carrier?
//
// The candidate: an outside slot range declaring a width of 63 bits over the
// index window [0, 255]. Every condition that is about the declaration agreeing
// with itself holds. The only condition it fails is `WIDTH.count() <= 62`, whose
// stated reason is that "the count of slots is 2^width and 2^63 does not fit the
// signed 64-bit integer a slot index is carried in".
//
// The control is `Narrow62`, identical but for one bit of declared width, which
// is admitted. Without it a refusal here would say nothing, because a law stuck
// at false refuses everything.
//
// The second control is `slot_count` on the control arm: the count this crate
// actually computes is `MAX - MIN + 1`, so the quantity the refused bound names
// is not the quantity anything computes.
//
// The arm that must fail is behind `--features force`, which forces `ADMITTED`
// through a runtime call. It is a separate compilation on purpose: a codegen
// refusal aborts the build, so it cannot share a binary with the arm that prints.

use arvo_format::slots::{is_admissible, slot_count, Slot, Slots};
use arvo_format::width::Width;

/// 63 bits of declared width over a 256-index window. Coherent with itself.
struct Wide63;

impl Slots for Wide63 {
    const MIN: Slot = Slot::at(0);
    const MAX: Slot = Slot::at(255);
    const WIDTH: Width = Width::bits(63);
}

/// The control. One bit narrower, everything else identical.
struct Narrow62;

impl Slots for Narrow62 {
    const MIN: Slot = Slot::at(0);
    const MAX: Slot = Slot::at(255);
    const WIDTH: Width = Width::bits(62);
}

/// The five conditions of the shipped contract, evaluated one at a time so the
/// cause of a refusal is named rather than summarised.
fn report<S: Slots>(name: &str) {
    let min = S::MIN.index() as i128;
    let max = S::MAX.index() as i128;
    let w = S::WIDTH.count();

    let ordered = min <= max;
    let at_least_one_bit = w >= 1;
    let within_the_carrier_bound = w <= 62;
    let span_counts = (max - min) < i64::MAX as i128;
    let width_covers_the_span = (max - min) < (1i128 << w);

    println!("{name}: MIN={min} MAX={max} WIDTH={w}");
    println!("  MIN <= MAX                         : {ordered}");
    println!("  WIDTH >= 1                         : {at_least_one_bit}");
    println!("  WIDTH <= 62   (carrier capacity)   : {within_the_carrier_bound}");
    println!("  span < i64::MAX                    : {span_counts}");
    println!("  span < 2^WIDTH                     : {width_covers_the_span}");
    println!("  is_admissible                      : {}", is_admissible::<S>().get());
    println!(
        "  conditions failed                  : {}",
        [
            ordered,
            at_least_one_bit,
            within_the_carrier_bound,
            span_counts,
            width_covers_the_span
        ]
        .iter()
        .filter(|c| !**c)
        .count()
    );
}

#[cfg(not(feature = "force"))]
fn main() {
    report::<Wide63>("Wide63  (the candidate)");
    println!();
    report::<Narrow62>("Narrow62 (the control)");
    println!();

    // The quantity the refused bound is named after, against the quantity the
    // crate computes. `slot_count` forces `ADMITTED`, so it is reachable only on
    // the arm that is admitted, which is itself part of the finding.
    println!(
        "slot_count(Narrow62) = {:?}   (this is MAX - MIN + 1, not 2^WIDTH)",
        slot_count::<Narrow62>()
    );
    println!("2^62 as the bound names it = {}", 1u128 << 62);
    println!("2^63 as the bound names it = {}", 1u128 << 63);
    println!("i64::MAX                   = {}", i64::MAX);
}

// The case that must fail. `ADMITTED` is a `const ()` and is evaluated where it
// is forced, which is at codegen, so this arm is built rather than checked.
#[cfg(feature = "force")]
fn main() {
    let w: Width = arvo_format::slots::declared_slot_width::<Wide63>();
    println!("built, which would be the refutation: {}", w.count());
}
