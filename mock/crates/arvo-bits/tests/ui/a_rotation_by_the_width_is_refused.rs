// A rotation by the whole width would be the identity, and is refused at build
// with the shifts, so an amount is always below the width.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).rotate_down::<8>();
};

fn main() {}
