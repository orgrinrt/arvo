// A signed `fi` at word length 64, fraction length 32. MATLAB admits it and
// nothing about the declaration is unusual, which is what makes it the right
// case to pin: the refusal is the floor's width ladder rather than anything the
// alias chose.
//
// The alias carries the slot range's own bound, so this fails where the type is
// named rather than at a use site further away, and the diagnostic is the one
// `arvo-format` supplies about the ladder.

use arvo_format::format::Format;
use arvo_format::standards::Fi;

fn main() {
    let _ = <Fi<64, 32> as Format>::PHASE;
}
