// Stochastic rounding reads a dither, and a value has nowhere to carry one.
use arvo_format::Adapt;
use arvo_format::rounding::Stochastic;
use arvo_numeral::{Int, Wrapping};

fn main() {
    let _ = Int::<8, Adapt<Stochastic, Wrapping>>::constant::<0>();
}
