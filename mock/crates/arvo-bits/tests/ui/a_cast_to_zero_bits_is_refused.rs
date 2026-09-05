// `cast` is the second public door onto the obligation, and it forces it only
// by delegating to `masked` at the target width. A post-monomorphisation
// refusal reached through a delegation is exactly the kind that survives a
// refactor without anybody noticing, so the target width is pinned here too.

use arvo_bits::Bits;

const _REFUSED: () = {
    let _ = Bits::<8>::masked(1).cast::<0>();
};

fn main() {}
