//! Probe p2: the grid p1 refutes the clause on is a format the shipped crate
//! already ships, and the crate's own doc comment states the false clause.
//!
//! p1 is arithmetic and could be dismissed as a geometry nobody would declare.
//! This one closes that: it compiles against `arvo-format` as committed and
//! instantiates the counterexample out of the crate's own inventory.
//!
//! `arvo_format::points::Biased<BITS, EXP, PHASE>` fixes `PHASE_DEN = 2`, so it
//! IS the half-step family, and leaves `EXP` free. `Biased<4, 1, 1>` is then a
//! half-step-biased grid whose quantum is `radix^1 = 2` and whose phase is
//! therefore the value one. `Biased<4, -2, 1>` is the same family at a quantum
//! below one, which is where the finding was measured.
//!
//! What must fail, stated before the run:
//!
//!   1. The two instantiations must disagree about whether one is on the grid.
//!      If they agree, the instrument is reporting a property of the code rather
//!      than of the format and establishes nothing.
//!   2. `has_additive_identity` must be false for both. It is the crate's own
//!      statement of the clause that does survive, and if it came out true the
//!      instrument would be reading a different phase than the one declared.
//!   3. A phase-zero format from the same inventory (`points::Integer<4>`) must
//!      report `has_additive_identity` true, or the false results above are
//!      facts about the function rather than about the phase.
//!
//! The crate has no function that asks whether a VALUE is in the set: `contains`
//! is over `(slot, magnitude)` coordinates and never computes
//! `phase + slot * quantum`. So the value arithmetic below is done here, from
//! the crate's own exported coordinates (`PHASE_NUM`, `PHASE_DEN`,
//! `step_exponent`, `radix`), and that is itself the finding p2 reports second.
//!
//! Exact arithmetic in integer units of 1/`SCALE`, so nothing rounds.

use arvo_format::format::{has_additive_identity, radix, step_exponent, Format};
use arvo_format::points::{Biased, Integer};

/// Everything below is an integer count of 1/SCALE.
const SCALE: i64 = 64;

/// The quantum at magnitude zero, in units of 1/SCALE.
fn quantum_at_zero<F: Format>() -> i64 {
    let e = step_exponent::<F>(0);
    let r = i64::from(radix::<F>());
    if e >= 0 {
        SCALE * r.pow(e as u32)
    } else {
        SCALE / r.pow((-e) as u32)
    }
}

/// The phase, in units of 1/SCALE. `PHASE_NUM / PHASE_DEN` of the quantum at
/// magnitude zero, which is what `Format::PHASE_NUM`'s own doc says it is.
fn phase<F: Format>() -> i64 {
    quantum_at_zero::<F>() * F::PHASE_NUM / F::PHASE_DEN
}

/// Whether a value lies on the format's grid at magnitude zero, ignoring the
/// slot bounds. Closure and identity membership are lattice questions.
fn on_grid<F: Format>(value_scaled: i64) -> bool {
    (value_scaled - phase::<F>()).rem_euclid(quantum_at_zero::<F>()) == 0
}

fn report<F: Format>(name: &str) -> (bool, bool, bool) {
    let q = quantum_at_zero::<F>();
    let p = phase::<F>();
    let has_zero = on_grid::<F>(0);
    let has_one = on_grid::<F>(SCALE);
    let identity = has_additive_identity::<F>();
    println!("{name}");
    println!("    quantum at magnitude zero   {}/{}", q, SCALE);
    println!("    phase                       {}/{}", p, SCALE);
    println!("    value one on the grid       {has_one}");
    println!("    value zero on the grid      {has_zero}");
    println!("    has_additive_identity()     {identity}");
    println!();
    (has_one, has_zero, identity)
}

fn main() {
    // The measured geometry's family: quantum 2^-2 = 1/4, phase an eighth.
    let (a_one, a_zero, a_id) = report::<Biased<4, -2, 1>>("Biased<4, -2, 1>  quantum below one");
    // The counterexample: quantum 2^1 = 2, phase exactly the value one.
    let (b_one, b_zero, b_id) = report::<Biased<4, 1, 1>>("Biased<4,  1, 1>  quantum above one");
    // Control 3: a phase-zero format from the same inventory.
    let (c_one, c_zero, c_id) = report::<Integer<4>>("Integer<4>        phase zero (control)");
    // A phase of a whole quantum, which is the same lattice as phase zero.
    // `PHASE_NUM = 2` over `PHASE_DEN = 2` is one whole step, so the grid is
    // shifted onto itself and zero stays representable.
    let (_d_one, d_zero, d_id) =
        report::<Biased<4, 0, 2>>("Biased<4,  0, 2>  phase a whole quantum");

    println!(
        "CONTROL 1  the two Biased formats disagree about one   {}",
        a_one != b_one
    );
    println!(
        "CONTROL 2  neither Biased format has an additive id    {}",
        !a_id && !b_id
    );
    println!(
        "CONTROL 3  the phase-zero control does have one        {}",
        c_id
    );
    println!(
        "CONTROL 4  arm D carries zero while the crate says no  {}",
        d_zero && !d_id
    );
    assert!(a_one != b_one, "CONTROL 1 FAILED");
    assert!(!a_id && !b_id, "CONTROL 2 FAILED");
    assert!(c_id, "CONTROL 3 FAILED");
    assert!(
        !a_zero && !b_zero,
        "a biased grid carried zero, which contradicts p1"
    );
    assert!(c_zero && c_one, "the phase-zero control lost zero or one");
    assert!(
        d_zero,
        "CONTROL 4 FAILED: a whole-quantum phase moved the lattice"
    );
    assert!(
        !d_id,
        "CONTROL 4 FAILED: the crate agreed, so there is nothing to report"
    );

    println!();
    println!("VERDICT");
    println!("  `Biased<4, 1, 1>` is a shipped format of the shipped inventory,");
    println!("  it is half-step-biased by construction (PHASE_DEN = 2),");
    println!("  it carries no additive identity, and it carries the value one.");
    println!("  So `format::has_additive_identity`'s doc clause \"and takes one");
    println!("  off with it\" is false of a format this crate ships.");
    println!();
    println!("  And arm D is a second, independent defect in the same function.");
    println!("  `has_additive_identity` tests `PHASE_NUM == 0` rather than whether");
    println!("  the phase is a whole multiple of the quantum, so at a phase of one");
    println!("  whole step it reports no additive identity while zero is on the");
    println!("  grid. `PHASE_DEN` is read by no function in the crate at all.");
    println!();
    println!("P2 WORKS");
}
