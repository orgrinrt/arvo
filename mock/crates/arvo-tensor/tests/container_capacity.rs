//! Generic-threading tests for the `Capacity`-migrated containers.
//!
//! These assert the contract task #651 ships: `Array<T, C>` and `Matrix<W, C>`
//! thread the capacity as a TYPE, so a function generic over `C: Capacity` can
//! build and walk them. There are deliberately no `#![feature(...)]` gates: the
//! migrated containers escape the `generic_const_exprs` surface entirely
//! (contrast the pre-migration `Array<T, const N: Cap>`, whose `cap_size(N)` in
//! type position required the gate and ICE'd when threaded through generic
//! code). The gate's absence is the proof.

use arvo::USize;
use arvo_tensor::{Array, Capacity, Dim, Matrix};

// Build an `Array<u32, C>` generic over the capacity, write through typed `set`,
// walk it through `IntoIterator`. This is the shape that needed the GCE gate
// (and ICE'd when nested) under the `Cap`-const-generic form.
fn fill_and_sum<C: Capacity>(live: usize) -> u32 {
    let mut a: Array<u32, C> = Array::filled(0);
    let backing = Array::<u32, C>::len().0;
    let mut i = 0;
    while i < live && i < backing {
        a.set(USize(i), (i as u32) + 1);
        i += 1;
    }
    let mut sum = 0u32;
    for t in &a {
        sum += *t;
    }
    sum
}

#[test]
fn array_threads_generically_over_capacity() {
    // Dim<4> fully filled: 1+2+3+4 = 10.
    assert_eq!(fill_and_sum::<Dim<4>>(4), 10);
    // Dim<8> partial (live=3): 1+2+3 = 6; slack slots keep 0.
    assert_eq!(fill_and_sum::<Dim<8>>(3), 6);
    // Dim<13>, non-power-of-two width, fully filled: sum 1..=13 = 91.
    assert_eq!(fill_and_sum::<Dim<13>>(13), 91);
}

// Build a `Matrix<u32, C>` generic over the capacity via `from_fn`, read the
// diagonal back as an `Array<u32, C>`. The 2-D backing is the composition
// `C::Array<C::Array<u32>>`, with no `cap_size` in type position.
fn diagonal_sum<C: Capacity>(span: usize) -> u32 {
    let m: Matrix<u32, C> = Matrix::from_fn(|i, j| (i.0 * 10 + j.0) as u32);
    let d = m.diagonal();
    let backing = Array::<u32, C>::len().0;
    let mut sum = 0u32;
    let mut i = 0;
    while i < span && i < backing {
        sum += *d.get(USize(i));
        i += 1;
    }
    sum
}

#[test]
fn matrix_threads_generically_over_capacity() {
    // Dim<4> diagonal over first 3: m[0][0]=0, m[1][1]=11, m[2][2]=22 -> 33.
    assert_eq!(diagonal_sum::<Dim<4>>(3), 33);
    // Dim<7> diagonal over first 4: 0 + 11 + 22 + 33 = 66.
    assert_eq!(diagonal_sum::<Dim<7>>(4), 66);
}
