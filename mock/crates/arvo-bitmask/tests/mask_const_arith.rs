//! Const-context smoke test for `MaskOps` across multiple widths.
//!
//! Closes audit Finding 26 (round 202605041051, task #324). The
//! existing `mask_ops_blanket_const.rs` shipped Round 5 covers
//! Mask64 only. This file extends to four representative widths
//! (8, 32, 64, 256), exercising the same `MaskOps` projection
//! through the chassis form `Mask<Bits<W, S, Unsigned>>` in const
//! context. The blanket `impl<W: ...> const MaskOps for Mask<W>`
//! that landed in round 202605040602 covers each instantiation.

#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use arvo::{Bits, Hot, USize, Unsigned, Warm};
use arvo_bitmask::Mask;
use arvo_mask_contracts::MaskOps;

type Word8 = Bits<8, Hot, Unsigned>;
type Mask8 = Mask<Word8>;

type Word32 = Bits<32, Hot, Unsigned>;
type Mask32 = Mask<Word32>;

type Word64 = Bits<64, Hot, Unsigned>;
type Mask64 = Mask<Word64>;

type Word256 = Bits<256, Warm, Unsigned>;
type Mask256 = Mask<Word256>;

const _MASK8_EMPTY_HAS_NO_BITS: () = {
    let m: Mask8 = <Mask8 as MaskOps>::empty();
    let n = <Mask8 as MaskOps>::count(m);
    assert!(n.0 == 0);
};

const _MASK8_FULL_HAS_ALL_BITS: () = {
    let m: Mask8 = <Mask8 as MaskOps>::full();
    let n = <Mask8 as MaskOps>::count(m);
    assert!(n.0 == 8);
};

const _MASK32_SET_AND_CLEAR: () = {
    let m: Mask32 = <Mask32 as MaskOps>::empty();
    let m = <Mask32 as MaskOps>::set(m, USize(5));
    let m = <Mask32 as MaskOps>::set(m, USize(10));
    assert!(<Mask32 as MaskOps>::count(m).0 == 2);
    let m = <Mask32 as MaskOps>::clear(m, USize(5));
    assert!(<Mask32 as MaskOps>::count(m).0 == 1);
};

const _MASK64_TEST_PROBES_BIT: () = {
    let m: Mask64 = <Mask64 as MaskOps>::empty();
    let m = <Mask64 as MaskOps>::set(m, USize(7));
    assert!(<Mask64 as MaskOps>::test(m, USize(7)).0);
    assert!(!<Mask64 as MaskOps>::test(m, USize(0)).0);
};

const _MASK64_UNION_INTERSECTION: () = {
    let a: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(0));
    let b: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(1));
    let u = <Mask64 as MaskOps>::union(a, b);
    assert!(<Mask64 as MaskOps>::count(u).0 == 2);
    let i = <Mask64 as MaskOps>::intersection(a, b);
    assert!(<Mask64 as MaskOps>::count(i).0 == 0);
};

const _MASK64_DIFFERENCE_AND_COMPLEMENT: () = {
    let full: Mask64 = <Mask64 as MaskOps>::full();
    let one: Mask64 = <Mask64 as MaskOps>::set(<Mask64 as MaskOps>::empty(), USize(0));
    let diff = <Mask64 as MaskOps>::difference(full, one);
    assert!(<Mask64 as MaskOps>::count(diff).0 == 63);
    let comp = <Mask64 as MaskOps>::complement(one);
    assert!(<Mask64 as MaskOps>::count(comp).0 == 63);
};

const _MASK256_EMPTY_HAS_NO_BITS: () = {
    let m: Mask256 = <Mask256 as MaskOps>::empty();
    let n = <Mask256 as MaskOps>::count(m);
    assert!(n.0 == 0);
};

const _MASK256_FULL_HAS_ALL_BITS: () = {
    let m: Mask256 = <Mask256 as MaskOps>::full();
    let n = <Mask256 as MaskOps>::count(m);
    assert!(n.0 == 256);
};

#[test]
fn maskops_runtime_dispatch_each_width() {
    let m: Mask8 = <Mask8 as MaskOps>::empty();
    let m = <Mask8 as MaskOps>::set(m, USize(0));
    assert_eq!(<Mask8 as MaskOps>::count(m), USize(1));

    let m: Mask32 = <Mask32 as MaskOps>::full();
    assert_eq!(<Mask32 as MaskOps>::count(m), USize(32));

    let m: Mask64 = <Mask64 as MaskOps>::empty();
    let m = <Mask64 as MaskOps>::set(m, USize(63));
    assert_eq!(<Mask64 as MaskOps>::count(m), USize(1));

    let m: Mask256 = <Mask256 as MaskOps>::full();
    assert_eq!(<Mask256 as MaskOps>::count(m), USize(256));
}
