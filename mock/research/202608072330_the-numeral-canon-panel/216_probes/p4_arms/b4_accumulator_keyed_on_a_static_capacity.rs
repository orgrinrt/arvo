//! B4, must compile, gate-free. The same widening fold with the count at stage zero.
//!
//! The control for B1. If B1 fails and B4 compiles, the boundary is located at the count's
//! binding time and nowhere else, which is what the row says. Without B4, B1's failure
//! could be about the widening rather than about the staging.
//!
//! The capacity is a TYPE rather than a const expression in type position, so no
//! `generic_const_exprs` gate appears. That is this workspace's prescribed shape and it is
//! also what makes the arm honest: a control that needed a forbidden feature would be
//! demonstrating something the design cannot use.
#![allow(dead_code)]

/// A capacity, carried as a type. Its array shape and its width are both projections.
trait Capacity {
    type Store: Default;
    const LEN: usize;
    const ACC_BITS: usize;
    fn store() -> Self::Store;
    fn first(s: &mut Self::Store) -> &mut i64;
}

/// One capacity: three elements, accumulating into ten bits.
struct Cap3;
impl Capacity for Cap3 {
    type Store = [i64; 3];
    const LEN: usize = 3;
    const ACC_BITS: usize = 10;
    fn store() -> Self::Store {
        [0i64; 3]
    }
    fn first(s: &mut Self::Store) -> &mut i64 {
        &mut s[0]
    }
}

/// Another, so the fold is genuinely generic over the capacity rather than fixed to one.
struct Cap8;
impl Capacity for Cap8 {
    type Store = [i64; 8];
    const LEN: usize = 8;
    const ACC_BITS: usize = 11;
    fn store() -> Self::Store {
        [0i64; 8]
    }
    fn first(s: &mut Self::Store) -> &mut i64 {
        &mut s[0]
    }
}

/// The widening fold. The accumulator's shape comes from the capacity, at stage zero.
fn fold<C: Capacity>(xs: &[i8]) -> C::Store {
    assert!(xs.len() <= C::LEN);
    let mut store = C::store();
    let mut s = 0i64;
    for &x in xs {
        s += x as i64;
    }
    *C::first(&mut store) = s;
    store
}

fn main() {
    let a = fold::<Cap3>(&[1i8, 2, 3]);
    let b = fold::<Cap8>(&[1i8, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(a[0], 6);
    assert_eq!(b[0], 36);
    assert_eq!(Cap3::ACC_BITS, 10);
}
