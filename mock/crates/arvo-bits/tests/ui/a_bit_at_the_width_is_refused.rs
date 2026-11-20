// Bits are counted from zero at the low bit, so the last index of `Bits<8>` is
// seven. An index of eight is refused at build, by the assertion `bit` forces.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).bit::<8>();
};

fn main() {}
