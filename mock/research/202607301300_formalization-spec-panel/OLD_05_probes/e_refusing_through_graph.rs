#![feature(const_trait_impl, adt_const_params)]
// PROBE E: does a REFUSING policy reach arvo-graph if the refusal is
// delivered as an absorbing value rather than as a sum type?
//
// 04 section 3 established that `Precise` returning through Outcome fails
// arvo-graph's `W: Add<Output = W> + TotalOrd + Copy + FromConstant`
// (arvo-graph/src/rank.rs:39). This probe holds the POLICY fixed (refuse on
// out-of-range: no clamped answer is ever produced) and changes only the
// DELIVERY, to an absorbing bottom carried in the numeral's own spare
// patterns. The algorithm crate is not modified and does not know.
#![allow(dead_code)]

use arvo::strategy::{Hot, Unsigned};
use arvo::traits::{FromConstant, TotalOrd};
use arvo::ufixed::UFixed;
use arvo::{ibits, Bits, FBits, USize};
use arvo_bitmask::{BitMatrix, NodeId};
use arvo_graph::upward_rank;
use arvo_tensor::Dim;
use core::cmp::Ordering;
use core::ops::Add;
use notko::Outcome;

type W = UFixed<{ ibits(8) }, { FBits::ZERO }, Hot>;

#[derive(Clone, Copy, Debug)]
pub struct OutOfRange;

/// A refusing numeral whose refusal is a value. `NONE` means refused.
/// The logical range is [0, 200]; 201..=255 are spare patterns, and one of
/// them carries the refusal, so the type is the same size as W.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Refusing(W);

const LOGICAL_MAX: u8 = 200;
const BOTTOM: u8 = 255;

impl Refusing {
    pub fn value(v: u8) -> Self {
        Refusing(W::from_raw(v))
    }
    pub fn refused() -> Self {
        Refusing(W::from_raw(BOTTOM))
    }
    pub fn is_refused(self) -> bool {
        self.0.to_raw() == BOTTOM
    }
    /// Settle: the ONE place the refusal becomes control flow.
    pub fn settle(self) -> Outcome<W, OutOfRange> {
        if self.is_refused() {
            Outcome::Err(OutOfRange)
        } else {
            Outcome::Ok(self.0)
        }
    }
}

// Total. This is the whole point: the policy still refuses, the operator is
// still closed on the carrier, so the L2 bound is satisfied unchanged.
impl Add for Refusing {
    type Output = Refusing;
    #[inline]
    fn add(self, rhs: Refusing) -> Refusing {
        let a = self.0.to_raw();
        let b = rhs.0.to_raw();
        let s = a as u16 + b as u16;
        let bad = self.is_refused() | rhs.is_refused() | (s > LOGICAL_MAX as u16);
        if bad {
            Refusing::refused()
        } else {
            Refusing::value(s as u8)
        }
    }
}

const impl TotalOrd for Refusing {
    #[inline]
    fn total_cmp(self, other: Self) -> Ordering {
        // bottom sorts above every value, so a max-selection propagates it
        let a = self.0.to_raw();
        let b = other.0.to_raw();
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

const impl FromConstant for Refusing {
    #[inline]
    fn from_constant<const C: USize>() -> Self {
        Refusing(<W as FromConstant>::from_constant::<C>())
    }
}

fn nid(i: usize) -> NodeId {
    NodeId(USize(i))
}

fn main() {
    println!(
        "size_of Refusing = {}, size_of W = {}",
        core::mem::size_of::<Refusing>(),
        core::mem::size_of::<W>()
    );
    println!(
        "size_of Outcome<W, OutOfRange> = {}",
        core::mem::size_of::<Outcome<W, OutOfRange>>()
    );

    // 0 -> 1 -> 2 -> 3, weights 1: ranks 4,3,2,1. Nothing refuses.
    let mut dag: BitMatrix<Bits<64, Hot, Unsigned>, Dim<4>> =
        BitMatrix::<Bits<64, Hot, Unsigned>, _>::empty();
    dag.set_edge(nid(0), nid(1));
    dag.set_edge(nid(1), nid(2));
    dag.set_edge(nid(2), nid(3));
    let weights: [Refusing; 4] = [Refusing::value(1); 4];
    let r = upward_rank(&dag, &weights);
    println!("clean chain ranks: {:?}", r.map(|x| x.0.to_raw()));

    // same shape, weights that must overflow the logical range on the way up
    let heavy: [Refusing; 4] = [Refusing::value(80); 4];
    let r2 = upward_rank(&dag, &heavy);
    println!(
        "heavy chain refused per node: {:?}",
        r2.map(|x| x.is_refused())
    );
    println!("root settles to err: {}", r2[0].settle().is_err());
    println!("sink settles to ok:  {}", r2[3].settle().is_ok());
}
