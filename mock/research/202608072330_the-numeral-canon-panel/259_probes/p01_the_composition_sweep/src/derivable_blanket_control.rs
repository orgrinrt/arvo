//! E3. The control for E2, and it must build.
//!
//! The same blanket impl over the same const-generic format, with the same
//! associated type and the same associated const, differing only in that the
//! right-hand side computes nothing on the parameters. If this builds and E2 does
//! not, E2's refusal is about the arithmetic rather than about a blanket impl
//! over a const-generic format, which is the thing that had to be separated.
//!
//! The wide format here is the narrow one, which is useless as a widening and is
//! exactly the point: it makes the impl legal without changing its shape.

use arvo_format::format::Format;
use arvo_format::slots::{Signed, Slots};
use arvo_format::standards::Fi;

trait Widens: Format {
    type Wide: Format;
    const FRACTION: u32;
}

impl<const W: u32, const F: i32> Widens for Fi<W, F>
where
    Signed<W>: Slots,
{
    type Wide = Fi<W, F>;

    const FRACTION: u32 = F as u32;
}

fn main() {
    let fraction = <Fi<6, 3> as Widens>::FRACTION;
    assert_eq!(fraction, 3);
    println!("E3 control: the blanket impl builds and FRACTION is {fraction}");
}
