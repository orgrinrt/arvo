//! Integration tests for the `Capacity` trait and `Dim` marker.
//!
//! These assert the GCE-free fixed-capacity foundation: the typed-capacity
//! surface, `filled` correctness at exact backing length, the fully-generic
//! build-and-walk shape that ICE'd under the `Cap`-const-generic form, and
//! 2-D composition via nested `Capacity::Array`. There are deliberately no
//! `#![feature(...)]` gates: the foundation escapes the `generic_const_exprs`
//! surface entirely (contrast `tests/array.rs`, which needs the gate because
//! `Array<T, const N: Cap>` puts `cap_size(N)` in type position).

use arvo_tensor::{cap, cap_size, Capacity, Dim};

#[test]
fn dim_cap_is_typed_and_exact() {
    assert_eq!(<Dim<3> as Capacity>::CAP, cap(3));
    assert_eq!(<Dim<1> as Capacity>::CAP, cap(1));
    assert_eq!(<Dim<47> as Capacity>::CAP, cap(47));
}

#[test]
fn filled_populates_every_slot_at_exact_length() {
    let a = <Dim<7> as Capacity>::filled(9u8);
    let slots: &[u8] = a.as_ref();
    assert_eq!(slots.len(), cap_size(<Dim<7> as Capacity>::CAP));
    assert_eq!(slots.len(), 7);
    assert!(slots.iter().all(|&x| x == 9));
}

// The load-bearing test. Fully generic over `C: Capacity`: build via `filled`,
// mutate through `as_mut`, read through `as_ref`, walk the storage. This is the
// exact shape that overflowed `generic_const_exprs` as `cap_size(cap(N))` when
// the capacity was a `Cap` const generic threaded through a generic fn.
fn reverse_fill_then_assert<C: Capacity>(live: usize) {
    let mut arr = C::filled(0usize);
    let backing = {
        let slots: &mut [usize] = arr.as_mut();
        let backing = slots.len();
        let mut i = 0;
        while i < live && i < backing {
            slots[i] = live - 1 - i;
            i += 1;
        }
        backing
    };
    assert_eq!(backing, cap_size(<C as Capacity>::CAP));
    let slots: &[usize] = arr.as_ref();
    let mut i = 0;
    while i < live && i < backing {
        assert_eq!(slots[i], live - 1 - i, "slot {i} did not reverse-fill");
        i += 1;
    }
    // slack beyond `live` keeps the fill value, untouched by the walk.
    while i < backing {
        assert_eq!(slots[i], 0, "slack slot {i} was disturbed");
        i += 1;
    }
}

#[test]
fn generic_build_and_walk_is_gce_free() {
    reverse_fill_then_assert::<Dim<4>>(4); // exact fill
    reverse_fill_then_assert::<Dim<8>>(6); // partial fill, slack preserved
    reverse_fill_then_assert::<Dim<13>>(13); // non-power-of-two width
}

// 2-D storage is the composition of two capacities, no separate type. Guards
// the `AsRef`/`AsMut` bound the container migration (task #651) leans on.
fn diagonal_sum<R: Capacity, C: Capacity>(rows: usize, cols: usize) -> u32
where
    <C as Capacity>::Array<u32>: Copy,
{
    let mut m: <R as Capacity>::Array<<C as Capacity>::Array<u32>> =
        R::filled(C::filled(0u32));
    {
        let outer: &mut [<C as Capacity>::Array<u32>] = m.as_mut();
        let mut r = 0;
        while r < rows && r < outer.len() {
            let inner: &mut [u32] = outer[r].as_mut();
            let mut c = 0;
            while c < cols && c < inner.len() {
                inner[c] = (r * 10 + c) as u32;
                c += 1;
            }
            r += 1;
        }
    }
    let outer: &[<C as Capacity>::Array<u32>] = m.as_ref();
    let mut sum = 0u32;
    let mut d = 0;
    while d < rows && d < cols && d < outer.len() {
        let inner: &[u32] = outer[d].as_ref();
        if d < inner.len() {
            sum += inner[d];
        }
        d += 1;
    }
    sum
}

#[test]
fn two_dim_composition_builds_and_indexes() {
    // 3x3 diagonal: m[0][0]=0, m[1][1]=11, m[2][2]=22 -> 33.
    assert_eq!(diagonal_sum::<Dim<4>, Dim<8>>(3, 3), 33);
}
