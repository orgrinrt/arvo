// A declared width past sixty-four bits has nowhere to be masked into: the
// value itself is carried in a u64, which runs out of capacity there.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<65>::masked(0);
};

fn main() {}
