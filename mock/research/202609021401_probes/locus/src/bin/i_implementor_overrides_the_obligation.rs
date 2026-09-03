//! Arm I. `ADMITTED` is a defaulted associated const
//! (`arvo-format/DESIGN.md.tmpl:300`), so the implementor may write it.
//!
//! The question is whether the party the obligation constrains can disarm it
//! inside the same impl block, which is the shape this registry already names at
//! `retirement::a_fidelity_licence_witness_constant` : "the implementor writing
//! the lie also controls the check for the lie inside the same impl block".
//!
//! Predicted, if the obligation is disarmable: `cargo build` succeeds and
//! `has_additive_identity` answers out of a produced binary for a declaration the
//! contract says is inadmissible.
//!
//! The control is arm B, which is this arm verbatim minus the overriding line.
//! If B refuses and I does not, the override is what did it.

use arvo_format::ambient::BinaryRationals;
use arvo_format::format::{Format, Phase, has_additive_identity, is_admissible_format};
use arvo_format::quantum::Constant;
use arvo_format::slots::Signed;

struct Disarmed;

impl Format for Disarmed {
    type Ambient = BinaryRationals;
    type Quantum = Constant<0>;
    type Slots = Signed<8>;

    // The one line arm B does not have.
    const ADMITTED: () = ();
    const PHASE: Phase = Phase::of(1, 0);
}

fn main() {
    println!(
        "I: verdict={} has_additive_identity={}",
        is_admissible_format::<Disarmed>().get(),
        has_additive_identity::<Disarmed>().get()
    );
}
