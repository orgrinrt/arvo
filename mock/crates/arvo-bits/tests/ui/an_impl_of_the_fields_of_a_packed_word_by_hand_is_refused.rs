// A list of fields of a consumer's own, claiming a sum, a union and no overlap
// its two 20-bit fields do not have, is refused: `Fields` is sealed.

#![feature(const_trait_impl)]

use arvo_bits::{Bits, Field, Fields, Layout, Word};
use arvo_format::{Bool, Width};

struct Handle;
struct Low;
struct High;
struct Liar;

impl Fields for Liar {
    const SUM: Width = Width::bits(32);
    const COVER: Bits<64> = Bits::masked(0xffff_ffff);
    const OVERLAP: Bool = Bool::FALSE;
}

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Liar;
}

impl Field for Low {
    type Layout = Handle;
    type Value = Bits<20>;
    const BELOW: Width = Width::bits(0);
}

impl Field for High {
    type Layout = Handle;
    type Value = Bits<20>;
    const BELOW: Width = Width::bits(8);
}

fn main() {
    let _ = Word::<Handle>::of(Bits::masked(0)).bits();
}
