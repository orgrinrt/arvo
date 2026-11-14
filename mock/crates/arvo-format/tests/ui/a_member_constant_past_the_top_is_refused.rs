// A member constant one past the top of `UFixed<8, 0>`, refused where the
// constant is named. It differs from
// `a_member_constant_at_the_top_is_admitted` in the constant alone.

use arvo_format::Member;
use arvo_format::points::UFixed;

const TOP: Member<UFixed<8, 0>> = Member::constant::<256>();

fn main() {
    let _ = TOP.slot();
}
