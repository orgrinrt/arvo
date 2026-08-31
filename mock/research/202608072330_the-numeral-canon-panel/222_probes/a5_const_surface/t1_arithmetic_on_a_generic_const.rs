// t1. `retirement::dl_width_arithmetic_as_a_const_generic`: arithmetic over a still-generic
// const parameter on its own right-hand side. No feature gates, which is the condition
// `obligation::the_unstable_machinery_does_not_reach_a_consumer` imposes.
struct Fx<const W: usize>;
fn widen<const W: usize>(_: Fx<W>) -> Fx<{ W + 1 }> { Fx }
fn main() { let _ = widen(Fx::<13>); }
