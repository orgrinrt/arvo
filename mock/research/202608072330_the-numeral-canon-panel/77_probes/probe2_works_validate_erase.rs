// The trait-decomposition alternative. "has a representable multiplicative
// identity" becomes an associated const computed by a const fn body, which
// is free to use assert! against the type's own const generic parameters
// (no generic_const_exprs needed: the parameters are used as themselves,
// not folded into a new const-generic array-length expression). The
// assert fires as a hard compile error at monomorphization for a shape
// where the law does not hold, and compiles to nothing when it does: this
// is derive (HAS_ONE / ONE_RAW as an associated const, computed from I
// and F), validate (the assert! inside the const fn, evaluated at
// compile time), and erase (a passing assert lowers to no runtime code;
// the raw encoding is baked into the emitted constant) enacted as one
// compiler mechanism.
trait Shape {
    const I: u32;
    const F: u32;
    const HAS_ONE: bool = Self::I >= 1;
    // validate: this associated const's initializer is evaluated at
    // compile time; a failing assert is a hard error, not a runtime
    // panic. no generic_const_exprs, no specialization, no TypeId.
    const ONE_RAW: u128 = {
        assert!(Self::I >= 1, "shape has no representable multiplicative identity: I=0 means the container has no integer bits to hold a 1");
        1u128 << Self::F
    };
}

struct Fixed<const I: u32, const F: u32>;

impl<const I: u32, const F: u32> Shape for Fixed<I, F> {
    const I: u32 = I;
    const F: u32 = F;
}

fn one_of<S: Shape>() -> u128 {
    // referencing ONE_RAW is what triggers validation for THIS S; a
    // generic fn that never instantiates S = Fixed<0, F> never pays for
    // (or trips) the assert, exactly like a where-clause that only bites
    // the instantiations that actually reach it.
    S::ONE_RAW
}

fn main() {
    // the good case: I=3 has a representable one, compiles and runs.
    let good = one_of::<Fixed<3, 5>>();
    println!("Fixed<3,5>::ONE_RAW = {good}");
    assert_eq!(good, 1u128 << 5);

    // uncomment to observe the bad case refused at compile time:
    // let _bad = one_of::<Fixed<0, 8>>();
}
