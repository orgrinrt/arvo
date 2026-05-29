//! Const-context smoke tests for the `MaskOps` blanket impl.
//!
//! Round 5 (#315) reshaped `MaskOps` to drop the const-Width
//! parameter and added a single blanket `impl<W: ...> const
//! MaskOps for Mask<W>` in arvo-bitmask. These tests exercise the
//! const-trait surface uniformly across the chassis form
//! `Mask<Bits<W, Hot, Unsigned>>`. Trait-call syntax (`<Mask<W> as
//! MaskOps>::method(m, ...)`) is used throughout because the
//! chassis carries inherent methods of the same names that are not
//! `const fn`; only the trait projection is const-callable.

#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::Mask;
use arvo_mask_contracts::MaskOps;

type Word64 = Bits<64, Hot, Unsigned>;
type Mask64 = Mask<Word64>;

const _MASK64_EMPTY_HAS_NO_BITS: () = {
    let m: Mask64 = <Mask64 as MaskOps>::empty();
    let n = <Mask64 as MaskOps>::count(m);
    assert!(n.0 == 0);
};

const _MASK64_FULL_HAS_ALL_BITS: () = {
    let m: Mask64 = <Mask64 as MaskOps>::full();
    let n = <Mask64 as MaskOps>::count(m);
    assert!(n.0 == 64);
};

const _MASK64_SET_BIT_INCREMENTS_COUNT: () = {
    let m: Mask64 = <Mask64 as MaskOps>::empty();
    let m = <Mask64 as MaskOps>::set(m, USize(3));
    let n = <Mask64 as MaskOps>::count(m);
    assert!(n.0 == 1);
};

const _MASK64_CLEAR_REMOVES_BIT: () = {
    let m: Mask64 = <Mask64 as MaskOps>::full();
    let m = <Mask64 as MaskOps>::clear(m, USize(0));
    let n = <Mask64 as MaskOps>::count(m);
    assert!(n.0 == 63);
};

const _MASK64_TEST_PROBES_BIT: () = {
    let m: Mask64 = <Mask64 as MaskOps>::empty();
    let m = <Mask64 as MaskOps>::set(m, USize(7));
    assert!(<Mask64 as MaskOps>::test(m, USize(7)).0);
    assert!(!<Mask64 as MaskOps>::test(m, USize(0)).0);
};

const _MASK64_UNION_COMBINES: () = {
    let a: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(0));
    let b: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(1));
    let u = <Mask64 as MaskOps>::union(a, b);
    let n = <Mask64 as MaskOps>::count(u);
    assert!(n.0 == 2);
};

const _MASK64_INTERSECTION_NARROWS: () = {
    let a: Mask64 = <Mask64 as MaskOps>::full();
    let b: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(0));
    let i = <Mask64 as MaskOps>::intersection(a, b);
    let n = <Mask64 as MaskOps>::count(i);
    assert!(n.0 == 1);
};

const _MASK64_COMPLEMENT_FLIPS: () = {
    let m: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(0));
    let c = <Mask64 as MaskOps>::complement(m);
    let n = <Mask64 as MaskOps>::count(c);
    assert!(n.0 == 63);
};

#[test]
fn blanket_dispatch_runtime_64() {
    let m: Mask64 = <Mask64 as MaskOps>::empty();
    let m = <Mask64 as MaskOps>::set(m, USize(0));
    let m = <Mask64 as MaskOps>::set(m, USize(1));
    let m = <Mask64 as MaskOps>::set(m, USize(2));
    assert_eq!(<Mask64 as MaskOps>::count(m), USize(3));
}
