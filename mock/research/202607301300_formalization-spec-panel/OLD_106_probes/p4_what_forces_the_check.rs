//! Probe 4. The pricing pillar says a fact that is a function of the type's
//! parameters alone belongs ON THE TYPE rather than computed at value position.
//! File 100 found, and file 105 confirmed, that "on the type" is not sufficient:
//! an associated const nothing mentions is never evaluated.
//!
//! So the pillar's own remedy needs a companion sentence naming what forces the
//! evaluation. This probe enumerates the mechanisms and finds which of them are
//! available to WHICH KIND OF TYPE, because that turns out to be the whole
//! difference between the two instances the record has been reading as one.
//!
//! Claims, each a separate module, each compiled or refused as marked:
//!   A. associated const, mentioned by nothing            -> COMPILES (no check)
//!   B. associated const, mentioned in one constructor    -> fires there only
//!   C. free `const _` item beside a MACRO-DECLARED type  -> fires, no route
//!   D. free `const _` item for a CONSUMER-INSTANTIATED
//!      generic                                          -> NOT AVAILABLE
//!   E. the fact placed in a type position every route
//!      must resolve                                     -> fires everywhere
//!
//! MUST BE COMPILED FROM INSIDE THE TREE (pinned nightly-2026-05-28).
#![no_std]

use core::marker::PhantomData;

pub struct W<const N: usize, const K: usize>(PhantomData<()>);

// ---------------------------------------------------------------------------
// A. On the type, mentioned by nothing. The pricing pillar's letter, satisfied.
// ---------------------------------------------------------------------------
pub mod a_unmentioned {
    use super::*;
    impl<const N: usize, const K: usize> W<N, K> {
        pub const AGREES: () = assert!(N == K, "A: disagrees");
    }
    // Declared with N != K. Nothing mentions AGREES. This compiles.
    pub type Lying = W<3, 7>;
    pub fn make() -> Lying {
        W(PhantomData)
    }
}

// ---------------------------------------------------------------------------
// B. Mentioned in one constructor. Fires through that one, silent through any
//    other. This is the shipped bitfield's shape (`_BOUNDS` mentioned by `new`
//    and `from_bits`, not by `const_default`).
// ---------------------------------------------------------------------------
pub mod b_one_route {
    use super::*;
    pub struct V<const N: usize, const K: usize>(PhantomData<()>);
    impl<const N: usize, const K: usize> V<N, K> {
        const AGREES: () = assert!(N == K, "B: disagrees");
        pub const fn checked() -> Self {
            let _ = Self::AGREES;
            V(PhantomData)
        }
        pub const fn unchecked() -> Self {
            V(PhantomData) // the ConstDefault-shaped door
        }
    }
    // The unchecked route builds the lying type. Uncomment `checked` to refuse.
    pub const BYPASS: V<3, 7> = V::<3, 7>::unchecked();
    // pub const REFUSED: V<3, 7> = V::<3, 7>::checked();
}

// ---------------------------------------------------------------------------
// C. A free const item beside the type. Unconditional: no route, no mention, no
//    construction. Available whenever a MACRO emits the type, because the macro
//    has a declaration site to emit into.
// ---------------------------------------------------------------------------
pub mod c_macro_declared {
    macro_rules! decl {
        ($name:ident : $n:literal { $($f:ident : $w:literal at $lo:literal),* $(,)? }) => {
            pub struct $name(u64);
            // The whole mechanism, one line, emitted once per declaration.
            const _: () = {
                let los = [$($lo),*];
                let ws = [$($w),*];
                let mut i = 0;
                while i < los.len() {
                    assert!(los[i] + ws[i] <= $n, "field does not fit");
                    let mut j = i + 1;
                    while j < los.len() {
                        assert!(
                            los[i] + ws[i] <= los[j] || los[j] + ws[j] <= los[i],
                            "two fields overlap",
                        );
                        j += 1;
                    }
                    i += 1;
                }
            };
        };
    }
    // Disjoint: compiles. Swap `b: 8 at 4` in to refuse with E0080, no route.
    decl!(Ok16: 16 { a: 8 at 0, b: 8 at 8 });
    // decl!(Bad16: 16 { a: 8 at 0, b: 8 at 4 });
}

// ---------------------------------------------------------------------------
// D. The same mechanism for a consumer-instantiated generic. There is no
//    declaration site the design owns: the consumer writes `W<3, 7>` at a use
//    site, and no free const item in the design's own crate can name it.
//    The nearest available thing is a blanket const, which is exactly case A.
// ---------------------------------------------------------------------------
pub mod d_consumer_instantiated {
    // Nothing to write here, and that is the finding. A free `const _` can only
    // assert about types it can name. The design cannot name `W<3, 7>` before
    // the consumer writes it, so mechanism C is structurally unavailable and
    // mechanism A is all that is left, which does not fire.
}

// ---------------------------------------------------------------------------
// E. The fact placed where every route must resolve it: a type position. Here
//    the array length is the fact, so no route can avoid it.
// ---------------------------------------------------------------------------
pub mod e_type_position {
    use super::*;
    pub trait Sized_ {
        const COUNT: usize;
        type Store: Copy;
    }
    pub struct U<const N: usize>(PhantomData<()>);
    impl<const N: usize> Sized_ for U<N> {
        const COUNT: usize = N;
        type Store = [u8; N]; // length and count are one const, read twice
    }
    pub const C7: usize = <U<7> as Sized_>::COUNT;
    const _: () = assert!(C7 == core::mem::size_of::<<U<7> as Sized_>::Store>());
}
