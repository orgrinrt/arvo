// Two signed numerals of stored width 32 and 33.
// True product width:  1 + (31 + 32) = 64  -> fits u64.
// 129's law demands:       32 + 33   = 65  -> next rung, u128.
const TRUE_WIDTH: u32 = 1 + (31 + 32);
const LAW_WIDTH: u32 = 32 + 33;
const fn rung(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else if w <= 64 {
        64
    } else {
        128
    }
}
const _: () = assert!(TRUE_WIDTH == 64);
const _: () = assert!(LAW_WIDTH == 65);
const _: () = assert!(rung(TRUE_WIDTH) == 64);
const _: () = assert!(rung(LAW_WIDTH) == 128);
fn main() {}
