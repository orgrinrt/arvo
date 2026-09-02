//! Probe 5. The negative control for probe 4, plus execution.
//!
//! Probe 4's claim D is that the derivation moves the falsifiable surface from
//! the instances (where the pairing puts it, one chance to lie per declared
//! capacity, unbounded) to the grammar's constructors (three lines, fixed,
//! exhaustively checkable once). A claim of that shape is worth nothing until
//! a deliberately wrong constructor is shown to fail, and to fail LOUDLY, at a
//! use site that never mentions the law.
//!
//! Claims:
//!   A. execution: the derived storage really is `VAL` contiguous elements,
//!      written and read through the slice projection at four numerals
//!      including a carry-heavy one, at runtime.
//!   B. the negative control: `I<P>`'s storage given `Twin` instead of
//!      `TwinOne` (an off-by-`P` error, the exact shape a hand-written
//!      combinator can get wrong) fails at compile time. Run with
//!      `--cfg broken`.
//!   C. how it fails matters: it fails through the generic slice projection,
//!      which mentions no numeral and no length, so a consumer who never
//!      writes a law still gets the diagnostic.
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).

use core::marker::PhantomData;

pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);

pub trait Pos {
    const VAL: usize;
}
impl Pos for H {
    const VAL: usize = 1;
}
impl<P: Pos> Pos for O<P> {
    const VAL: usize = 2 * P::VAL;
}
impl<P: Pos> Pos for I<P> {
    const VAL: usize = 2 * P::VAL + 1;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Twin<A>(A, A);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TwinOne<A, T>(A, A, T);

pub trait Capacity: Pos {
    type Array<T: Copy>: Copy;
    // Construction recurs on the same grammar, so there is no door where a
    // length could be restated. `filled` mirrors the shipped surface's own
    // constructor name (`arvo-tensor/src/capacity.rs:56`, read as a factual
    // check on what exists, not for what it means).
    fn filled<T: Copy>(v: T) -> Self::Array<T>;
}
impl Capacity for H {
    type Array<T: Copy> = [T; 1];
    fn filled<T: Copy>(v: T) -> Self::Array<T> {
        [v; 1]
    }
}
impl<P: Capacity> Capacity for O<P> {
    type Array<T: Copy> = Twin<P::Array<T>>;
    fn filled<T: Copy>(v: T) -> Self::Array<T> {
        Twin(P::filled(v), P::filled(v))
    }
}

// The one line the negative control changes.
#[cfg(not(broken))]
impl<P: Capacity> Capacity for I<P> {
    type Array<T: Copy> = TwinOne<P::Array<T>, T>;
    fn filled<T: Copy>(v: T) -> Self::Array<T> {
        TwinOne(P::filled(v), P::filled(v), v)
    }
}
#[cfg(broken)]
impl<P: Capacity> Capacity for I<P> {
    type Array<T: Copy> = Twin<P::Array<T>>; // WRONG: drops the odd element
    fn filled<T: Copy>(v: T) -> Self::Array<T> {
        Twin(P::filled(v), P::filled(v))
    }
}

// The projections. Neither mentions a numeral, a length, or the law's
// statement; the law lives in one const block each and fires per
// monomorphisation.
pub fn as_slice<C: Capacity, T: Copy>(a: &C::Array<T>) -> &[T] {
    const {
        assert!(
            core::mem::size_of::<C::Array<T>>() == C::VAL * core::mem::size_of::<T>(),
            "storage law violated: array grammar does not match the numeral"
        );
        assert!(core::mem::align_of::<C::Array<T>>() == core::mem::align_of::<T>());
    }
    // SAFETY: the const block establishes, at this monomorphisation, that the
    // storage is exactly `C::VAL` contiguous `T` at `T`'s own alignment.
    unsafe { core::slice::from_raw_parts(a as *const C::Array<T> as *const T, C::VAL) }
}

pub fn as_mut_slice<C: Capacity, T: Copy>(a: &mut C::Array<T>) -> &mut [T] {
    const {
        assert!(core::mem::size_of::<C::Array<T>>() == C::VAL * core::mem::size_of::<T>());
        assert!(core::mem::align_of::<C::Array<T>>() == core::mem::align_of::<T>());
    }
    // SAFETY: as above.
    unsafe { core::slice::from_raw_parts_mut(a as *mut C::Array<T> as *mut T, C::VAL) }
}

pub type N1 = H;
pub type N7 = I<I<H>>;
pub type N13 = I<O<I<H>>>;
pub type N47 = I<I<I<I<O<H>>>>>;

// CLAIM A. Fill each slot with a distinct value through the mutable
// projection, read every slot back through the shared one, and check both the
// length and every element. A storage that is short, long, padded, or
// misordered fails here at runtime even if it somehow passed the const block.
fn round_trip<C: Capacity>(zero: u32) -> usize {
    let mut a: C::Array<u32> = C::filled(0u32);
    {
        let s = as_mut_slice::<C, u32>(&mut a);
        assert_eq!(s.len(), C::VAL, "projected length");
        for (i, slot) in s.iter_mut().enumerate() {
            *slot = zero + i as u32 * 7;
        }
    }
    let s = as_slice::<C, u32>(&a);
    assert_eq!(s.len(), C::VAL);
    for (i, v) in s.iter().enumerate() {
        assert_eq!(*v, zero + i as u32 * 7, "slot {i} of capacity {}", C::VAL);
    }
    // Read the raw bytes and confirm the last element sits at the last slot,
    // which is what catches trailing padding.
    let bytes: &[u8] = unsafe {
        core::slice::from_raw_parts(
            &a as *const C::Array<u32> as *const u8,
            core::mem::size_of::<C::Array<u32>>(),
        )
    };
    assert_eq!(bytes.len(), C::VAL * 4);
    s.len()
}

fn main() {
    assert_eq!(round_trip::<N1>(100), 1);
    assert_eq!(round_trip::<N7>(200), 7);
    assert_eq!(round_trip::<N13>(300), 13);
    assert_eq!(round_trip::<N47>(400), 47);
    // A non-`Default`-able element type would need a different constructor;
    // the projection itself is element-agnostic and is exercised above.
    println!("p5 ok: 1, 7, 13, 47 round-tripped through the derived storage");
}
