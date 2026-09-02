// ARM 5, negative control, must FAIL.
//
// An inverted range at an admitted width. This is refused for a reason that is
// about mathematics: a range whose low index exceeds its high one admits nothing
// and is not a range. It sits in the same const, behind the same door, with the
// same shape of message as arm 3's, which is the whole finding: one obligation
// carries both kinds of refusal and nothing in the surface tells them apart.
use arvo_format::slots::declared_slot_width;
use q31_probes::Grid;

type Inverted = Grid<8, -8, 8>;

fn main() {
    let _ = declared_slot_width::<Inverted>();
}
