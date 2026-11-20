// `with_bit` at index sixty-four of a `Bits<64>`, where the host shift by the carrier's width lives, is refused at build.

use arvo_bits::Bits;
use arvo_format::Bool;

fn main() {
    let _ = Bits::<64>::masked(1).with_bit::<64>(Bool::TRUE);
}
