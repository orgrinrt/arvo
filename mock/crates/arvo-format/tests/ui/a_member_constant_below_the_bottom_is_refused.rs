// A member constant one below the bottom of `UFixed<8, 0>`, refused where the
// constant is named. It differs from
// `a_member_constant_at_the_bottom_is_admitted` in the constant alone.

use arvo_format::Member;
use arvo_format::points::UFixed;

const BOTTOM: Member<UFixed<8, 0>> = Member::constant::<-1>();

fn main() {
    let _ = BOTTOM.slot();
}
