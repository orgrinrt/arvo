// The width the tier gives back is refused past sixty-four bits, as the type is.

use arvo_bits::{BitWidth, Bits};

fn main() {
    let _ = <Bits<65> as BitWidth>::WIDTH;
}
