// A5. The offending declaration through `contains`, the membership predicate the
// ratified spine names. It reads the slot and magnitude coordinates and does not
// force the obligation.
mod shared;
use arvo_format::format::contains;
use arvo_format::quantum::Magnitude;
use arvo_format::slots::Slot;

fn main() {
    let out = contains::<shared::Broken>(Slot::at(3), Magnitude::at(0));
    println!("A5 broken/contains = {}", out.get());
}
