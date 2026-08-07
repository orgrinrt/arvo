// P6c. Where the defaulted-parameter route breaks: a METHOD CALL, where the
// default does not apply because nothing named the trait.
pub struct Erased(pub u32);
pub trait Sum<Out = Self> {
    fn sum(self, o: Self) -> Out;
}
impl Sum<u32> for u32 {
    fn sum(self, o: u32) -> u32 {
        self + o
    }
}
impl Sum<Erased> for u32 {
    fn sum(self, o: u32) -> Erased {
        Erased(self + o)
    }
}
fn main() {
    let y = 3u32.sum(4);
    let _ = y;
}
