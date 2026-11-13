//! The same consumer against the alias with the strategy added, and two new
//! lines that reach the default and name the strategy.

use arvo_strategy::presets::Warm;
use value_later as value;

include!("../written_today.rs");

fn main() {
    written_today();
    let _: value::Uint<8> = value::Num::new();
    let _: value::Uint<8, Warm> = value::Num::new();
    println!("later: the consumer written today builds and runs, and `Uint<8>` is `Warm`");
}
