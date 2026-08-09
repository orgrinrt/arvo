// Negative control for probe 1 part D: adding a third variant to Shrunk without updating
// the match must fail to compile. If this file compiles clean, the exhaustiveness claim
// in probe 1 is false and needs correcting before it enters the record.
#![no_std]
enum Shrunk {
    A,
    B,
    C,
}
const fn decode_shrunk(s: &Shrunk) -> u8 {
    match s {
        Shrunk::A => 0,
        Shrunk::B => 1,
    }
}
fn main() {}
