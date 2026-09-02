// ARM 4, control, must BUILD.
//
// The identical shape one bit narrower. If this also refused, arm 3 would be
// about `Grid` or about the hand-written impl rather than about the width.
use arvo_format::slots::declared_slot_width;
use q31_probes::Grid;

type SixtyTwo = Grid<-2305843009213693952, 2305843009213693951, 62>;

fn main() {
    assert_eq!(declared_slot_width::<SixtyTwo>().count(), 62);
    println!("arm 4 built and ran: the 62-bit two's complement grid is admitted");
}
