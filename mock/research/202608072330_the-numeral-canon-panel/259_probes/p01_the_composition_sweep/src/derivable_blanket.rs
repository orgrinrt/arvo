//! E2. The shape a reader reaches for first, and it must be refused.
//!
//! The widening relation written as one blanket impl that computes the wide
//! width and the wide fraction length from the narrow ones on its own right-hand
//! side. That is arithmetic over a still-generic const parameter in type
//! position, which wants `generic_const_exprs`, which this workspace forbids.
//!
//! Compiled on its own by `run` and its stderr committed beside it. The refusal
//! is the result; nothing here is meant to build.

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
    Signed<{ 2 * W + 1 }>: Slots,
{
    type Wide = Fi<{ 2 * W + 1 }, { 2 * F }>;

    const FRACTION: u32 = F as u32;
}

fn main() {
    let _ = <Fi<6, 3> as Widens>::FRACTION;
}
