// No feature gates at all.
#[derive(Clone, Copy)]
pub struct Width(u32);
impl Width {
    pub const fn bits(n: u32) -> Self {
        Self(n)
    }
    pub const fn count(self) -> u32 {
        self.0
    }
}

pub trait Slots {
    const WIDTH: Width;
}
pub struct Eight;
impl Slots for Eight {
    const WIDTH: Width = Width::bits(8);
}
fn main() {
    assert_eq!(<Eight as Slots>::WIDTH.count(), 8);
    println!("ok, no feature gates needed for associated const of struct type");
}
