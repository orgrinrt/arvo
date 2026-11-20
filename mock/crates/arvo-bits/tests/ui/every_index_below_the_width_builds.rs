// Every method at the last index or amount of the width builds and runs, at run
// time through the inherent method and through the traits, and the width reads
// back at one bit and at sixty-four. Its being here makes trybuild build the
// refusal cases rather than check them.

use arvo_bits::{BitAccess, BitSequence, BitWidth, Bits};
use arvo_format::{Bool, Width};

fn at_the_last<B: BitAccess + BitSequence>(b: B) -> B {
    let _ = b.bit::<7>();
    b.with_bit::<7>(Bool::TRUE)
        .shift_up::<7>()
        .shift_down::<7>()
        .rotate_up::<7>()
        .rotate_down::<7>()
}

fn main() {
    let b = Bits::<8>::masked(1);
    let _ = b.bit::<7>();
    let inherent = b
        .with_bit::<7>(Bool::TRUE)
        .shift_up::<7>()
        .shift_down::<7>()
        .rotate_up::<7>()
        .rotate_down::<7>();
    assert_eq!(at_the_last(b), inherent);
    assert_eq!(<Bits<1> as BitWidth>::WIDTH, Width::bits(1));
    assert_eq!(<Bits<64> as BitWidth>::WIDTH, Width::bits(64));
}
