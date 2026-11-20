// The width the tier gives back is refused at a width the type refuses, here zero.

use arvo_bits::{BitWidth, Bits};

fn main() {
    let _ = <Bits<0> as BitWidth>::WIDTH;
}
