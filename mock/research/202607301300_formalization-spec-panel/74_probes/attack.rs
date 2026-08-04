//! probe 1b, crate `attack`, EXPECTED FAIL: a foreign crate cannot extend the
//! sealed vocabulary, so value-uniqueness survives the crate split.
#![no_std]
struct Rogue;
impl carrier::Pos for Rogue {
    const VALUE: u128 = 7;
}
