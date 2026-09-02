//! Seat 242. `has_additive_identity` is a free `pub const fn`, so it is also an
//! ordinary function. Seat 237's proposed body uses `%` by `PHASE_DEN`. A const
//! context catches a zero divisor; a runtime call does not.
//!
//! This is the distinction that decides whether the proposed repair satisfies
//! `ruling::never_a_runtime_check_and_one_lowered_path` or violates it.
//!
//! The case that must fail, stated before the run: the const arm must refuse at
//! compile time AND the runtime arm must reach execution, or the two contexts
//! are not being distinguished and the probe says nothing.
//!
//! Build: `rustc --edition 2024 -O runtime_call.rs -o /tmp/rc`

const fn proposed(phase_num: i64, phase_den: i64) -> bool {
    phase_num % phase_den == 0
}

// A generic stand-in for the `F::PHASE_DEN` the real function reads: the value
// comes from a trait impl, so it is not a literal at the call site.
trait Fmt {
    const PHASE_NUM: i64;
    const PHASE_DEN: i64;
}
struct ZeroDen;
impl Fmt for ZeroDen {
    const PHASE_NUM: i64 = 1;
    const PHASE_DEN: i64 = 0;
}

const fn has_additive_identity<F: Fmt>() -> bool {
    proposed(F::PHASE_NUM, F::PHASE_DEN)
}

fn main() {
    println!("control: reached main, so nothing refused at compile time.");
    // Runtime call. Not a const context, so const-eval never looks at it.
    let verdict = has_additive_identity::<ZeroDen>();
    println!("runtime call returned {verdict}");
}
