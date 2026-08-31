// The value path through the public surface. `Integer<3>` is the slot range
// [-4, 3], span 8. Wrapping i64::MAX into it: (i64::MAX + 4) mod 8 = 3, so the
// correct answer is -4 + 3 = -1.
use arvo_format::apply::{adapt, Dither, Exact};
use arvo_format::overflow::Wrap;
use arvo_format::points::Integer;
use arvo_format::rounding::Floor;
use arvo_format::{Adapt, Signature};
type S = Signature<Integer<3>, Adapt<Floor, Wrap>>;
fn main() {
    println!(
        "in range   {}",
        adapt::<S>(Exact::on_grid(2), Dither::UNUSED)
    );
    println!(
        "ordinary   {}",
        adapt::<S>(Exact::on_grid(9), Dither::UNUSED)
    );
    println!(
        "i64::MAX   {}",
        adapt::<S>(Exact::on_grid(i64::MAX), Dither::UNUSED)
    );
    println!(
        "i64::MIN   {}",
        adapt::<S>(Exact::on_grid(i64::MIN), Dither::UNUSED)
    );
}
