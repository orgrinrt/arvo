// `bit` at the largest index a `u32` holds, far past the width, is refused at build.

use arvo_bits::Bits;

fn main() {
    let _ = Bits::<8>::masked(1).bit::<{ u32::MAX }>();
}
