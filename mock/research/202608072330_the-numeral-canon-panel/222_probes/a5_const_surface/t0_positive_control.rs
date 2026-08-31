// t0. The positive control. If this does not compile, the driver is broken and every
// refusal below is a fact about the driver rather than about the language.
struct Fx<const W: usize>;
fn main() { let _ = Fx::<13>; }
