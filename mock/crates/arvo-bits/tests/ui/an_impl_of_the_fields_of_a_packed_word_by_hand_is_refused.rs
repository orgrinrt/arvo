// A list of fields of a consumer's own, claiming a sum of thirty-two its one
// 20-bit field does not have, is refused: `Fields` is sealed.

use arvo_bits::{Bits, Fields, Layout, Word};
use arvo_format::Width;

struct Liar;

impl Fields for Liar {
    const SUM: Width = Width::bits(32);
}

struct Handle;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Liar;
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).bits();
}
