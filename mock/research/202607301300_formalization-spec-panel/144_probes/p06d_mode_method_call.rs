// P6d. Does the mode-marker route survive a METHOD CALL, where p06c showed the
// output-as-parameter route does not?
pub struct Erased(pub u32);
pub struct Native;
pub struct Erase;
pub trait Mode {}
impl Mode for Native {}
impl Mode for Erase {}
pub trait Algo<M: Mode = Native> {
    type Out;
    fn run(self, o: Self) -> Self::Out;
}
impl Algo<Native> for u32 {
    type Out = u32;
    fn run(self, o: u32) -> u32 {
        self + o
    }
}
impl Algo<Erase> for u32 {
    type Out = Erased;
    fn run(self, o: u32) -> Erased {
        Erased(self + o)
    }
}
fn main() {
    let y = 3u32.run(4); // elides M: does the default apply?
    let z = Algo::<Erase>::run(3u32, 4); // names M
    let _ = (y, z.0);
}
