// A field its layout does not list, read in a constant, is refused at build,
// though the layout itself tiles its word.

use arvo_bits::{Bits, Field, Layout, Word};
use arvo_format::Bool;
use notko_hlist::{Cons, Empty};

#[derive(Clone, Copy)]
struct Id;
#[derive(Clone, Copy)]
struct Flag;
#[derive(Clone, Copy)]
struct Other;

impl Field for Id {
    type Value = Bits<31>;
}

impl Field for Flag {
    type Value = Bool;
}

impl Field for Other {
    type Value = Bits<31>;
}

struct Handle;

impl Layout for Handle {
    type Container = Bits<32>;
    type Fields = Cons<Id, Cons<Flag, Empty>>;
}

const OTHER: Bits<31> = Word::<Handle>::of(Bits::masked(0)).get(Other);

fn main() {
    let _ = OTHER;
}
