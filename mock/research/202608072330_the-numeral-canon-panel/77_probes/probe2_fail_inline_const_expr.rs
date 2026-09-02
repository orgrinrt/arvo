// Attempt the naive inline-const-expression bound for "this shape has a
// representable multiplicative identity", the pattern that needs
// generic_const_exprs. Expected: refused without the feature.
struct Shape<const I: u32, const F: u32>;

fn make_one<const I: u32, const F: u32>() -> u128
where
    [(); (I >= 1) as usize]:,
{
    1u128 << F
}

fn main() {
    let _ = make_one::<3, 5>();
}
