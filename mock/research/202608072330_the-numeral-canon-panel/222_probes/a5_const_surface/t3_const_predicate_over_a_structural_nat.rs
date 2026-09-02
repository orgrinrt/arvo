// t3. Option 5's crossing back. The numeral is keyed on a structural natural and an arm's
// predicate has to be a const expression over the width, so the width has to leave the type
// level. `ruling::the_predicate_is_whatever_is_available_at_const_time` is what makes this
// the deciding question rather than a detail.
trait Nat {
    const USIZE: usize;
}
struct N13;
impl Nat for N13 {
    const USIZE: usize = 13;
}

struct Fixed<N: Nat>(core::marker::PhantomData<N>);

impl<N: Nat> Fixed<N> {
    const IS_NARROW: bool = N::USIZE <= 16;
    fn arm(&self) -> &'static str {
        if Self::IS_NARROW {
            "narrow"
        } else {
            "wide"
        }
    }
}

fn main() {
    let f: Fixed<N13> = Fixed(core::marker::PhantomData);
    println!("{}", f.arm());
}
