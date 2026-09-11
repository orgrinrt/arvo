// Addition over the magnitude-indexed family, refused where it is forced.
//
// A slot names a different value at each magnitude, so two members may sit at
// two quanta and their sum may belong at a third. The applied map carries no
// magnitude coordinate to land it at, so the obligation refuses the format,
// and the call is bound in a const item so the refusal is reached at check
// time rather than only at codegen.

use arvo_format::addition::sum_position;
use arvo_format::points::Floating;
use arvo_format::slots::Slot;

const _REFUSED: () = {
    let _ = sum_position::<Floating<4, -3, 4>>(Slot::ZERO, Slot::ZERO);
};

fn main() {}
