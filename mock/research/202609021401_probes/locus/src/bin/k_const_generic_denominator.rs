//! Arm K. The denominator as a const generic parameter, so the refusal has no
//! run-time path to fall through to.
//!
//! An inline `const` block inside a generic function is evaluated once per
//! instantiation, at monomorphisation, and there is no run-time value of `DEN`
//! for a caller to smuggle past it. So the refusal reaches the run-time call
//! site that arm J leaves open, without adding the run-time check that
//! `ruling::never_a_runtime_check_and_one_lowered_path` forbids.
//!
//! Predicted: the good instantiation builds and runs; the bad one refuses at
//! `cargo check`, from a call site that is not const.
//!
//! The control is the good instantiation, which must build, or the refusal is
//! about the construct rather than about the denominator.

struct PhaseK {
    num: i64,
    den: i64,
}

impl PhaseK {
    const fn of<const DEN: i64>(num: i64) -> Self {
        const { assert!(DEN != 0, "a phase denominator of zero names no position") }
        Self {
            num,
            den: DEN,
        }
    }
}

// Uncomment to see a NON-const, run-time call site refuse:
// fn refused() -> PhaseK { PhaseK::of::<0>(1) }

fn main() {
    let ok = PhaseK::of::<2>(1);
    println!("K control: {}/{} built and ran", ok.num, ok.den);
}
