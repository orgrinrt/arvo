// A4. The offending declaration through `adapt`, the applied map, which is the
// public path a value actually takes. `adapt` forces `Slots::ADMITTED` and not
// `Format::ADMITTED`, so this arm asks whether the obligation is reachable from
// where arithmetic happens.
mod shared;
use arvo_format::adapt::{Adapt, Signature};
use arvo_format::apply::{adapt, Dither, Exact};
use arvo_format::overflow::Saturate;
use arvo_format::rounding::HalfEven;
use arvo_format::slots::Slot;

type Broken = Signature<shared::Broken, Adapt<HalfEven, Saturate>>;

fn main() {
    let out = adapt::<Broken>(Exact::on_grid(Slot::at(3)), Dither::UNUSED);
    println!("A4 broken/adapt = {}", out.index());
}
