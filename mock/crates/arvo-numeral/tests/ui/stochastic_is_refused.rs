// Stochastic rounding reads a dither, and a value has nowhere to carry one.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::Stochastic;
use arvo_numeral::Int;

fn main() {
    let _ = Int::<8, Adapt<Stochastic, Wrap>>::constant::<0>();
}
