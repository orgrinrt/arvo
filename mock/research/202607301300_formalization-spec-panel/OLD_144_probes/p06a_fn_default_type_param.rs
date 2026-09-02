// P6a. Op's output generic, read literally: "it will have a generic to describe
// the output, and they can override its default". Spelled as a default on a
// function's own type parameter.
pub struct Erased;
pub fn algo<T, Out = T>(x: T) -> Out
where
    T: Into<Out>,
{
    x.into()
}
fn main() {
    let _y: u32 = algo(1u8);
}
