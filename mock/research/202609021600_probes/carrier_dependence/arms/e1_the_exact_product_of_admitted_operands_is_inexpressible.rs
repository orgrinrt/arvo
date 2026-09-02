// ARM E1, must FAIL TO BUILD.
//
// The repair of the leading argument, which the other route got wrong by asking
// the wrong contract.
//
// `Signed<62>` is the widest slot range arvo admits, and it admits it. The
// ratified factoring says arithmetic on a format is an exact operation in an
// ambient domain composed with a total adaptation onto the representable set, and
// `apply::adapt` is that adaptation: it takes the exact value as an `Exact` and a
// target signature, and it never asks for the exact value to be a declared
// format. So the question is not whether a 63-bit `Slots` impl is admitted. It is
// whether the exact value can be written down at all.
//
// The exact product of two operands at the top of `Signed<62>` sits at slot
// 2^61 * 2^61 = 2^122 in the target's own units. `Slot` carries an `i64`. There is
// no declaration to refuse here and no obligation to fire: the value cannot be
// spelled.
use arvo_format::slots::Slot;

// 2^122, the slot the exact product occupies.
const EXACT_PRODUCT: Slot = Slot::at(5316911983139663491615228241121378304);

fn main() {
    let _ = EXACT_PRODUCT;
}
