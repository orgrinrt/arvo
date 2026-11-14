// A member constant at the top of `UFixed<8, 0>`, admitted where the
// constant is named. It differs from
// `a_member_constant_past_the_top_is_refused` in the constant alone.

use arvo_format::Member;
use arvo_format::points::UFixed;

const TOP: Member<UFixed<8, 0>> = Member::constant::<255>();

fn main() {
    let _ = TOP.slot();
}
