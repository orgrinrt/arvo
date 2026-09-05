// The other end of the same delegation: a cast to a width past what a u64
// carries is refused at the target width rather than at the source, which is
// where the obligation lives and where a reader would go looking for it.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).cast::<65>();
};

fn main() {}
