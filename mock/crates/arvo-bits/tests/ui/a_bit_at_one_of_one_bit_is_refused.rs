// `bit` at index one of a `Bits<1>`, called at run time, is refused at build.

use arvo_bits::Bits;

fn main() {
    let _ = Bits::<1>::masked(1).bit::<1>();
}
