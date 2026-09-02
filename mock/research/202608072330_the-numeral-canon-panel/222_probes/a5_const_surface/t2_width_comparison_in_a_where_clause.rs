// t2. `retirement::dl_const_generic_width_comparison_in_a_where_clause`: a comparison of two
// const parameters in a bound, at a generic definition site. No feature gates.
struct Fx<const W: usize>;
fn narrow<const A: usize, const B: usize>(_: Fx<A>) -> Fx<B>
where
    [(); (A >= B) as usize]:,
{
    Fx
}
fn main() {
    let _ = narrow::<13, 8>(Fx::<13>);
}
