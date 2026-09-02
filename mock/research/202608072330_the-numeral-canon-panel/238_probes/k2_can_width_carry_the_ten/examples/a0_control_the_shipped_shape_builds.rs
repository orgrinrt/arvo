// Control. The shipped shape, written from outside `arvo-format`, must build.
//
// Without this an arm that fails proves nothing: every later failure could be
// a broken path dependency, a missing feature or a toolchain problem rather
// than a fact about `Width`. `Slots::WIDTH: Width` is here too, so the control
// also shows an associated constant does take an arvo type, which is the half
// of the first option that is true.

use arvo_format::slots::Slots;
use arvo_format::width::Width;

pub struct MyRange;

impl Slots for MyRange {
    const MIN: i64 = -128;
    const MAX: i64 = 127;
    const WIDTH: Width = Width::bits(8);
}

fn main() {
    let _ = <MyRange as Slots>::WIDTH;
}
