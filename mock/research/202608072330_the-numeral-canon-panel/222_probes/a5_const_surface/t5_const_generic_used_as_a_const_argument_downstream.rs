// t5. Whether a const generic can be handed on to another generic unchanged, which is what an
// arm selecting a lowering has to do. Passing it through is not arithmetic on it.
struct Fx<const W: usize>;
struct Acc<const W: usize>;
fn accumulate<const W: usize>(_: Fx<W>) -> Acc<W> {
    Acc
}
fn main() {
    let _ = accumulate(Fx::<13>);
}
