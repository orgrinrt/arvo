// `shift_down` by nine on a `Bits<8>`, past the width rather than at it, is refused at build.

use arvo_bits::Bits;

fn main() {
    let _ = Bits::<8>::masked(1).shift_down::<9>();
}
