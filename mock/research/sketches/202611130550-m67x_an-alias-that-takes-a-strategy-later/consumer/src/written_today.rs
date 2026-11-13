// Written against the alias as it ships today, and never edited after.

use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;

type U8Wrap = value::Uint<8, Adapt<TowardZero, Wrap>>;

/// Generic over what names the adaptation, as a consumer of the alias is.
fn generic<A: value::NamesAdaptation>(v: value::Uint<8, A>) -> value::Uint<8, A> {
    v
}

fn written_today() {
    let a: U8Wrap = value::Num::new();
    let _ = generic::<Adapt<TowardZero, Wrap>>(a);
}
