// `with_bit` at index one of a `Bits<1>`, called at run time, is refused at build.

use arvo_bits::Bits;
use arvo_format::Bool;

fn main() {
    let _ = Bits::<1>::masked(1).with_bit::<1>(Bool::TRUE);
}
