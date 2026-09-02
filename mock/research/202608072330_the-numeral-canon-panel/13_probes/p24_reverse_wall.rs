// P24. The reverse direction: a structural nat carries its value as an
// associated const for free. Putting that const back into a const-argument slot
// is the SAME wall as the forward direction.
#![no_std]
#![allow(dead_code)]

pub struct Z;
pub struct O<N>(N);
pub struct E<N>(N);

pub trait Val {
    const V: u32;
}
impl Val for Z {
    const V: u32 = 0;
}
impl<N: Val> Val for O<N> {
    const V: u32 = 2 * N::V + 1;
} // free: ordinary assoc const
impl<N: Val> Val for E<N> {
    const V: u32 = 2 * N::V;
}

pub struct L<const K: u32>;

// the attempt: name the width back as an L<K> without a table
pub trait Named {
    type Out;
}
impl<N: Val> Named for N {
    type Out = L<{ <N as Val>::V }>;
}
