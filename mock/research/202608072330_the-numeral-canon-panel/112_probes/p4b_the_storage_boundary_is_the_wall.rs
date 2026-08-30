// p4b. EXPECTED COMPILE FAILURE.
//
// The half of a missed merge that parametric abstraction does not repair.
// `p4` shows one generic function accepting both spellings.  This file asks
// for one homogeneous CONTAINER over both, which is the shape a column store
// has, and which I17 names as the path not to be deprioritised.
//
// Predicted before compiling: E0308, at the array literal, naming the two
// const arguments.  No signature, bound, blanket impl or const predicate
// repairs it, because a homogeneous container is one type by construction.

use core::marker::PhantomData;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Spur<const RADIX: u32>(u8);

trait Bound {
    const HI: u8;
}
struct Lit<const N: u8>;
impl<const N: u8> Bound for Lit<N> {
    const HI: u8 = N;
}

#[repr(transparent)]
struct Ref<B: Bound>(u8, PhantomData<B>);
impl<B: Bound> Clone for Ref<B> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<B: Bound> Copy for Ref<B> {}

fn main() {
    // (a) the spurious axis: two spellings of ONE primitive in one array
    let _column: [Spur<2>; 2] = [Spur::<2>(1), Spur::<10>(2)];

    // (b) the refinement: two extents of ONE primitive in one array.  Note
    // this one is arguably WANTED to fail, since widening exists and the
    // consumer can call it; the point is that the container itself offers no
    // implicit coercion, so a heterogeneous column is refused either way.
    let _mixed: [Ref<Lit<100>>; 2] = [Ref(1, PhantomData), Ref::<Lit<200>>(2, PhantomData)];
}
