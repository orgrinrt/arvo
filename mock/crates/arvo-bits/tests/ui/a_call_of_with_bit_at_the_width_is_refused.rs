// `with_bit` at the width, called at run time through the inherent method, is refused at build.

use arvo_bits::Bits;

fn main() {
    let b = Bits::<8>::masked(1);
    let _ = b.with_bit::<8>(arvo_format::Bool::TRUE);
}
