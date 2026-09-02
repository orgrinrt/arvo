//! Arm A. What `Phase::of` does with a zero denominator, at run time.
//!
//! The positive control for the whole set: if this does not build and run, the
//! crate is broken for reasons that have nothing to do with the question.
//!
//! What must fail for this arm to mean anything: the three assertions are over
//! three different predicates, and a `Phase` that normalised, reinterpreted or
//! refused the pair would break at least one of them. `of(1, 0)` reinterpreted
//! as `of(1, 1)` breaks the second and the third; a refusal breaks all three by
//! not compiling.

use arvo_format::format::Phase;

fn main() {
    let p = Phase::of(1, 0);

    // Stored unchanged, both halves.
    assert_eq!(p.numerator(), 1, "the numerator did not come back");
    assert_eq!(p.denominator(), 0, "the denominator was reinterpreted");

    // The predicates answer rather than diverging.
    assert!(!p.denotes().get(), "a zero denominator was said to denote");
    assert!(
        !p.is_whole_multiple().get(),
        "a zero denominator was said to be a whole multiple"
    );
    assert!(!p.is_zero().get(), "1/0 was said to be a zero phase");

    // The control: the same numerator over one is a different position, and the
    // two are distinguishable, so the storage is not collapsing them.
    assert_ne!(Phase::of(1, 0), Phase::of(1, 1), "1/0 and 1/1 compared equal");

    println!("A ok: of(1, 0) stores (1, 0); denotes=false is_whole_multiple=false");
}
