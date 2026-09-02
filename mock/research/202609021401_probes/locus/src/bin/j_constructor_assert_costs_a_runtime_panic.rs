//! Arm J. What a const refusal on the constructor actually costs.
//!
//! The alternative the agenda row names is "a const refusal on the
//! constructor". A `const fn` carrying an `assert!` refuses at const evaluation
//! and *panics at run time*, because a `const fn` is also an ordinary function.
//! `Phase::of` is `pub`, so the run-time call site exists whether or not anyone
//! means it to.
//!
//! That is the thing `ruling::never_a_runtime_check_and_one_lowered_path`
//! forbids in the words "never any runtime checks, ever", so this arm is not a
//! complaint about ergonomics: it is the canon-grounded reason the constructor
//! is the wrong locus for this condition as a plain `const fn`.
//!
//! Predicted: the const site refuses at `cargo check`; the run-time site
//! compiles clean and panics when run.
//!
//! The control is the second call, at a denominator of two, which must return
//! rather than panic, or the panic says nothing about the denominator.

struct PhaseJ {
    num: i64,
    den: i64,
}

impl PhaseJ {
    const fn of(num: i64, den: i64) -> Self {
        assert!(den != 0, "a phase denominator of zero names no position");
        Self { num, den }
    }
}

// Uncomment to see the const site refuse at `cargo check`:
// const REFUSED: PhaseJ = PhaseJ::of(1, 0);

fn main() {
    // The control first, so a panic below cannot be read as the arm being broken.
    let ok = PhaseJ::of(1, 2);
    println!("J control: {}/{} returned", ok.num, ok.den);

    let den: i64 = core::hint::black_box(0);
    let bad = PhaseJ::of(1, den);
    println!("J: unreachable, {}/{}", bad.num, bad.den);
}
