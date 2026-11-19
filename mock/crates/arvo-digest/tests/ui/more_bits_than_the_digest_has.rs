// A 32 bit digest asked for 40 bits is refused when `bits` is instantiated.
// The call sits in a const item so the refusal is reached at `cargo check`,
// the way `arvo-bits` holds its own.

use arvo_bits::Bytes;
use arvo_digest::Fnv1a;

const _REFUSED: () = {
    let _ = Fnv1a::<32>::of(Bytes::of(b"foobar")).bits::<40>();
};

fn main() {}
