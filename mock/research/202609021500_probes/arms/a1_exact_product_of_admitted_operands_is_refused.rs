// ARM 1, must FAIL.
//
// Both operands are formats arvo admits: `Signed<32>` is in the shipped impl set.
// The ratified factoring says arithmetic on a format is an exact operation in the
// ambient domain composed with an adaptation onto the representable set. So the
// exact product of two admitted 32-bit grids is a real element of the ambient
// domain. This is the slot range it occupies, and arvo's own admission obligation
// refuses it.
//
// The endpoints both fit a signed 64-bit slot index and the span fits a count, so
// the only assertion that can fire is the width one, which is the assertion about
// the machine rather than about the grid.
use arvo_format::slots::{declared_slot_width, Signed, Slots};
use q31_probes::Grid;

// The operands, which arvo admits.
type Lhs = Signed<32>;
type Rhs = Signed<32>;

// The set the exact product lands in. MIN = -(2^31) * (2^31 - 1), MAX = (2^31)^2.
type ExactProduct = Grid<-4611686016279904256, 4611686018427387904, 63>;

fn main() {
    // Force the operands' obligations, which pass.
    let _ = declared_slot_width::<Lhs>();
    let _ = declared_slot_width::<Rhs>();
    assert_eq!(<Lhs as Slots>::WIDTH.count(), 32);
    // Force the product's, which does not.
    let _ = declared_slot_width::<ExactProduct>();
}
