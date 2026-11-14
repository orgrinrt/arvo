// A member constant at the bottom of `UFixed<8, 0>`, admitted where the
// constant is named. It differs from
// `a_member_constant_below_the_bottom_is_refused` in the constant alone.

use arvo_format::Member;
use arvo_format::points::UFixed;

const BOTTOM: Member<UFixed<8, 0>> = Member::constant::<0>();

fn main() {
    let _ = BOTTOM.slot();
}
