// ARM 2, control, must BUILD.
//
// The identical construction at sixteen bits. The exact product needs 31 bits,
// which is inside the admitted set, so the whole shape builds and arm 1's refusal
// is about the width rather than about `Grid`, about the const parameters having
// arithmetic in them, or about anything else in the program.
use arvo_format::slots::{declared_slot_width, Signed};
use q31_probes::Grid;

type Lhs = Signed<16>;
type ExactProduct = Grid<-1073709056, 1073741824, 31>;

fn main() {
    let _ = declared_slot_width::<Lhs>();
    assert_eq!(declared_slot_width::<ExactProduct>().count(), 31);
    println!("arm 2 built and ran: the 16-bit exact product is admitted at width 31");
}
