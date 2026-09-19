// A declared width past what the slot ladder admits. There is no `Slots` impl at
// 65, so this is a trait-bound error rather than a value nobody checked: nothing
// compiles that names a width outside the impl set.

use arvo_format::slots::{Signed, Slots};

fn main() {
    let _ = <Signed<65> as Slots>::MIN;
}
