// The twin of `a_width_past_sixty_four_bits_is_refused`.
use arvo_format::Adapt;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Int, Uint, Wrapping};

fn main() {
    let _ = Int::<64, Adapt<TowardZero, Wrapping>>::constant::<0>();
}
