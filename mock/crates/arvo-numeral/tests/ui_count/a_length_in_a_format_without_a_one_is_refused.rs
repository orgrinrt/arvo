// A count steps by the format's one, so a format whose members are multiples of eight has none.
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::UFix;
use notko_hlist::{Cons, Empty, Length};

type Count = UFix<6, 3, Adapt<TowardZero, Wrap>>;
type One = Cons<(), Empty>;

fn main() {
    let _ = <One as Length<Count>>::LEN;
}
