// Same trait as probe2_works_validate_erase.rs, but this time the bad
// instantiation (I=0, no representable one) is actually reached, to
// confirm the assert refuses it at compile time rather than silently
// emitting a wrong runtime value (the failure mode the memory note
// records: UFixed<0, 8, Hot>::ONE held raw 0, and x * ONE == 0 for every
// purely fractional value, discovered only when the identity law was
// finally tested over the FULL matrix of shapes rather than a sample).
trait Shape {
    const I: u32;
    const F: u32;
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
    S::ONE_RAW
}

fn main() {
    // I=0: the exact shape from the arvo bug. this line should be
    // refused at compile time, not silently return 0.
    let bad = one_of::<Fixed<0, 8>>();
    println!("{bad}");
}
