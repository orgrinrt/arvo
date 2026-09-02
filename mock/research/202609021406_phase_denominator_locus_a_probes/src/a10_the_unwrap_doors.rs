// A10. Not the assigned question either. The design says every coordinate type
// "has exactly one declared unwrap accessor". `Phase` has two, and the source
// argues for the second in its own rustdoc. This arm calls both, so the count is
// a compiled fact rather than a reading of the source.
use arvo_format::format::Phase;

fn main() {
    const P: Phase = Phase::of(7, 3);
    // Two distinct declared unwrap doors on one coordinate type.
    println!("A10 Phase unwrap doors: numerator={} denominator={}", P.numerator(), P.denominator());
    assert_eq!((P.numerator(), P.denominator()), (7, 3));
}
