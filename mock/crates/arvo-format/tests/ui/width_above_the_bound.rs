// A declared width past what a slot index carries. There is no `Slots` impl at
// 63, so this is a trait-bound error rather than a value nobody checked.
//
// Before the bound became the impl set, this compiled: it panicked in debug and
// in release returned a derived width of zero for a declared 63-bit numeral.

use arvo_format::slots::{Signed, Slots};

fn main() {
    let _ = <Signed<63> as Slots>::MIN;
}
