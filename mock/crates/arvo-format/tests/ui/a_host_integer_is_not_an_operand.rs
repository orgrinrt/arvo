// A host integer where an operand goes, refused.
//
// The operands are slots, the coordinate the format is stated in. A host
// integer carries no statement of which coordinate it is, and accepting one
// would be the one place addition said a slot is a number.

use arvo_format::adapt::{Adapt, Signature};
use arvo_format::addition::add;
use arvo_format::apply::Dither;
use arvo_format::overflow::Saturate;
use arvo_format::points::Integer;
use arvo_format::rounding::Floor;

const _REFUSED: () = {
    let _ = add::<Signature<Integer<4>, Adapt<Floor, Saturate>>>(3, 4, Dither::UNUSED);
};

fn main() {}
