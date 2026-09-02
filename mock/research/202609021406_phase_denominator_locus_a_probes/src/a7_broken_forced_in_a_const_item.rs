// A7. The same obligation forced in a `const` item rather than from a runtime
// call. The design says where an obligation is forced decides which tool sees
// the refusal, and the shipped `a_law_over_no_magnitudes_is_refused.rs` uses
// exactly this shape for the magnitude condition. This arm asks whether the
// shape is available for the phase condition too.
mod shared;
use arvo_format::format::has_additive_identity;
use arvo_format::width::Bool;

const _REFUSED: Bool = has_additive_identity::<shared::Broken>();

fn main() {
    println!("A7 reached main, which it should not have");
}
