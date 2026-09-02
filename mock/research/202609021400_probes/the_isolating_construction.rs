// Seat q31a. The test that is missing, written out and shown to fire.
//
// `does_any_test_isolate_the_62_bit_clause.sh` establishes that deleting
// `Self::WIDTH.count() <= 62` from `Slots::ADMITTED` leaves all 134 tests green.
// Naming a gap is half a finding, so this is the other half: the construction
// that closes it, plus the run showing it separates the real crate from the
// mutant.
//
// The construction. Every other clause of the obligation holds:
//   MIN <= MAX                       0 <= 255                  holds
//   WIDTH >= 1                       63 >= 1                   holds
//   WIDTH <= 62                      63 <= 62                  FAILS, alone
//   MAX - MIN < i64::MAX             255 < i64::MAX            holds
//   MAX - MIN < 1 << WIDTH           255 < 2^63                holds
//
// A width wider than the span it addresses is a shape the crate admits on
// purpose: `the_inventory.rs` ships `WiderThanItsSpan` at width 13 over a span
// of a few hundred and asserts it admissible. So this construction differs from
// an admitted one in exactly one coordinate, which is what makes it isolating.
//
// This belongs in `mock/crates/arvo-format/src/tests/the_inventory.rs` beside
// `SpanTooWide`, as a `#[test]`. It is here rather than there because this seat
// is in TOPIC phase and may not edit crate source. It is ready to paste.

use arvo_format::slots::{is_admissible, Slot, Slots, Unsigned};
use arvo_format::width::Width;

/// A range that meets every obligation but the width bound.
///
/// Nothing about it is inverted, its span is tiny, and its width addresses that
/// span many times over. The only thing wrong with it is that this crate counts
/// slots in a signed 64-bit integer and `2^63` does not fit one, which is a fact
/// about arvo and not about the 63-bit two's complement integers it refuses.
struct SixtyThreeAndOtherwiseFine;

impl Slots for SixtyThreeAndOtherwiseFine {
    const MIN: Slot = Slot::ZERO;
    const MAX: Slot = Slot::at(255);
    const WIDTH: Width = Width::bits(63);
}

/// The control, differing in the one coordinate under test.
struct SixtyTwoAndOtherwiseFine;

impl Slots for SixtyTwoAndOtherwiseFine {
    const MIN: Slot = Slot::ZERO;
    const MAX: Slot = Slot::at(255);
    const WIDTH: Width = Width::bits(62);
}

fn main() {
    let sixty_three = is_admissible::<SixtyThreeAndOtherwiseFine>().get();
    let sixty_two = is_admissible::<SixtyTwoAndOtherwiseFine>().get();

    println!("  width 63, everything else fine : admissible = {sixty_three}");
    println!("  width 62, everything else fine : admissible = {sixty_two}");
    println!();

    // What the pasted test would assert.
    let would_pass = !sixty_three && sixty_two;
    println!(
        "  the missing assertion (!63 && 62) : {}",
        if would_pass { "PASSES" } else { "FAILS" }
    );
    println!();
    println!("  Run against the crate as it stands, this passes. Run against the");
    println!("  mutant with the width clause deleted, `sixty_three` becomes true");
    println!("  and it fails, which is the whole point: it is the first test in");
    println!("  the crate whose verdict depends on that clause existing.");

    if !would_pass {
        std::process::exit(1);
    }
}
