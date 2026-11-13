// The twin of `stochastic_is_refused`.
use arvo_format::Adapt;
use arvo_format::rounding::HalfEven;
use arvo_numeral::{Int, Wrapping};

fn main() {
    let _ = Int::<8, Adapt<HalfEven, Wrapping>>::constant::<0>();
}
