// `rotate_up` by sixty-four on a `Bits<64>`, called at run time, is refused at build.

use arvo_bits::Bits;

fn main() {
    let _ = Bits::<64>::masked(1).rotate_up::<64>();
}
