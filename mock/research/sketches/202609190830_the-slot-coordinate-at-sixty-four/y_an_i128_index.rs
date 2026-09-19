// Option Y: carry the slot index and the count in a signed 128-bit integer, and
// keep the ladder of impls as the bound, stopping at 64.
//
// The ends are written as shifts of the 64-bit host integers rather than of the
// 128-bit carrier, so a width past 64 fails where it is written: `64 - 65`
// underflows the `u32` the width is spelled in. A shift of the carrier would
// have kept going to 126 and moved the bound into a comment.
//
// `--cfg past_the_ladder` adds width 65 and must refuse at the definition site.

#![no_std]
#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Slot(i128);

pub trait Slots {
    const MIN: Slot;
    const MAX: Slot;
    const WIDTH: u32;
}

pub struct Signed<const BITS: u32>;
pub struct Unsigned<const BITS: u32>;

macro_rules! admit_widths {
    ($($w:literal),+ $(,)?) => {
        $(
            impl Slots for Signed<$w> {
                const MIN: Slot = Slot((i64::MIN >> (64 - $w)) as i128);
                const MAX: Slot = Slot((i64::MAX >> (64 - $w)) as i128);
                const WIDTH: u32 = $w;
            }
            impl Slots for Unsigned<$w> {
                const MIN: Slot = Slot(0);
                const MAX: Slot = Slot((u64::MAX >> (64 - $w)) as i128);
                const WIDTH: u32 = $w;
            }
            const _: () = {
                // Every width: the count is exactly 2^width, both signednesses, and
                // the signed range sits one slot further below zero than above it.
                let s_min = <Signed<$w> as Slots>::MIN.0;
                let s_max = <Signed<$w> as Slots>::MAX.0;
                let u_max = <Unsigned<$w> as Slots>::MAX.0;
                assert!(s_max - s_min + 1 == 1i128 << $w);
                assert!(u_max + 1 == 1i128 << $w);
                assert!(s_min == -s_max - 1);
                // Two members summed, which is what addition's exact step does, stay
                // far inside the carrier.
                assert!(2 * u_max < i128::MAX / (1i128 << 60));
            };
        )+
    };
}

admit_widths!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

// The two ends the platform-width point needs at a 64-bit target, stated against
// the host's own integers rather than against a restatement of the macro.
const _: () = assert!(<Unsigned<64> as Slots>::MAX.0 == u64::MAX as i128);
const _: () = assert!(<Signed<64> as Slots>::MIN.0 == i64::MIN as i128);
const _: () = assert!(<Signed<64> as Slots>::MAX.0 == i64::MAX as i128);

// And at the running target's own width, whatever it is.
const _: () = assert!(<Unsigned<{ usize::BITS }> as Slots>::MAX.0 == usize::MAX as i128);
const _: () = assert!(<Signed<{ usize::BITS }> as Slots>::MIN.0 == isize::MIN as i128);

#[cfg(past_the_ladder)]
admit_widths!(65);
