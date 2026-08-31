//! A3, must compile. The same associated-const carrier, consumed only as a const value.
//!
//! The control that says A2's failure is about the SORT the consuming site wants, not
//! about associated consts being unusable. Without this arm A2 proves only that the
//! program does not compile, which is a much weaker statement.
#![allow(dead_code)]

trait Carries {
    const W: usize;
}

struct SevenBit;
impl Carries for SevenBit {
    const W: usize = 7;
}

fn doubled<C: Carries>() -> usize {
    C::W * 2
}

fn main() {
    assert_eq!(doubled::<SevenBit>(), 14);
}
