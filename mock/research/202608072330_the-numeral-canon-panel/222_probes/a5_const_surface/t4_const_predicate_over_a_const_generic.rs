// t4. Option 6's predicate. The width is a const generic and the arm reads it directly, with
// no crossing at all.
struct Fx<const W: usize>;

impl<const W: usize> Fx<W> {
    const IS_NARROW: bool = W <= 16;
    fn arm(&self) -> &'static str {
        if Self::IS_NARROW {
            "narrow"
        } else {
            "wide"
        }
    }
}

fn main() {
    println!("{}", Fx::<13>.arm());
}
