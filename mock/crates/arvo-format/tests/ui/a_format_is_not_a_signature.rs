// A format where a declared signature goes, refused.
//
// Addition is a function of the declared signature, the format together with
// its adaptation. A format alone names no rounding and no overflow policy, so
// there is nothing to adapt the sum with.

use arvo_format::addition::add;
use arvo_format::apply::Dither;
use arvo_format::points::Integer;
use arvo_format::slots::Slot;

const _REFUSED: () = {
    let _ = add::<Integer<4>>(Slot::ZERO, Slot::ZERO, Dither::UNUSED);
};

fn main() {}
