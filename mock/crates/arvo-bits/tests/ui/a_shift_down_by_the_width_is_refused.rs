// A shift down by the whole width is no move within it, and is refused at build.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).shift_down::<8>();
};

fn main() {}
