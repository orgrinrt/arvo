// Two bits count to three, and under the default check a longer list has no length there.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::Uint;
use notko_hlist::{Cons, Empty, Length};

type Count = Uint<2, Adapt<TowardZero, Wrap>>;
type Three = Cons<(), Cons<(), Cons<(), Empty>>>;
type Four = Cons<(), Three>;

fn main() {
    let _ = <Four as Length<Count>>::LEN;
}
