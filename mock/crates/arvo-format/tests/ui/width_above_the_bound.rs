// A declared width past what the slot ladder admits. There is no `Slots` impl at
// 65, so this is a trait-bound error rather than a value nobody checked.
//
// Before the bound became the impl set, a width past the ladder compiled: it
// panicked in debug and in release returned a derived width of zero.

use arvo_format::slots::{Signed, Slots};

fn main() {
    let _ = <Signed<65> as Slots>::MIN;
}
