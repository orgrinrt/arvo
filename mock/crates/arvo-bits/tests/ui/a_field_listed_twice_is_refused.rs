// A field listed twice, whose widths tile the word, is refused where it is
// read: its place in the list is ambiguous.

use arvo_bits::{Bits, Field, Layout, Word};
use notko_hlist::{Cons, Empty};

#[derive(Clone, Copy)]
struct Half;

impl Field for Half {
    type Value = Bits<16>;
}

struct Twice;

impl Layout for Twice {
    type Container = Bits<32>;
    type Fields = Cons<Half, Cons<Half, Empty>>;
}

fn main() {
    let word = Word::<Twice>::of(Bits::masked(0));
    let _ = word.get(Half);
}
