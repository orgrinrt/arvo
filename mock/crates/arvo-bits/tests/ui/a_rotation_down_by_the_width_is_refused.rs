// A rotation down by the whole width is refused at build like the rotation up.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).rotate_down::<8>();
};

fn main() {}
