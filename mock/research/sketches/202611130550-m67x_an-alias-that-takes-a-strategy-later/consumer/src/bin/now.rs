//! The consumer against the alias as it ships.

use value_now as value;

include!("../written_today.rs");

fn main() {
    written_today();
    println!("now: the consumer written today builds and runs");
}
