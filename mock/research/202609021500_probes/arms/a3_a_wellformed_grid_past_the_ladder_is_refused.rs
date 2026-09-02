// ARM 3, must FAIL.
//
// A two's complement grid of 63 bits. Mathematically it is exactly as well formed
// as the 62-bit one below it: contiguous, ordered, symmetric to within the usual
// one, and the width addresses it exactly. Nothing about it is not a number
// system. It is refused, and the refusal message names a signed 64-bit integer.
use arvo_format::slots::declared_slot_width;
use q31_probes::Grid;

type SixtyThree = Grid<-4611686018427387904, 4611686018427387903, 63>;

fn main() {
    let _ = declared_slot_width::<SixtyThree>();
}
