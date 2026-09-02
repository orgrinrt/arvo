// A3. The offending declaration reaches a produced binary and is never passed to
// a function that forces the obligation. The design says an inadmissible
// declaration cannot reach a produced binary; this arm is that sentence's test.
mod shared;
use arvo_format::format::Format;

fn main() {
    // Reads the coordinate, so the impl is genuinely live rather than dropped as
    // an unused item before anything could refuse it.
    println!(
        "A3 broken PHASE = {}/{}",
        <shared::Broken as Format>::PHASE.numerator(),
        <shared::Broken as Format>::PHASE.denominator()
    );
}
