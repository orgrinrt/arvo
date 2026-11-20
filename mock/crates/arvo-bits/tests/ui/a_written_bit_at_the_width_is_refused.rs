// Writing a bit takes the same index as reading one, and is refused at the
// same place: `with_bit` forces the assertion itself rather than through `bit`.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).with_bit::<8>(arvo_format::Bool::TRUE);
};

fn main() {}
