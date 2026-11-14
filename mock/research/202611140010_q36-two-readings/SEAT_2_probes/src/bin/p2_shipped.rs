//! Spike p2. What the shipped value crate does at a lossy crossing, read through
//! its public doors only, and whether the three options of the question are
//! each writable from the one conversion that takes a target signature.
//!
//! The expected values are computed here in plain integers, never by a second
//! call into the crate.

use arvo_format::overflow::{Saturate, Wrap};
use arvo_format::points::{Fixed, Integer};
use arvo_format::rounding::{Floor, HalfEven, TowardZero};
use arvo_format::{Adapt, Exact, Signature, Slot, WithOverflow, WithRounding};
use arvo_numeral::{Num, Quiet};

type Sig<P, R, O> = Signature<P, Adapt<R, O>>;

type WideWrap = Sig<Integer<8>, TowardZero, Wrap>;
type WideSat = Sig<Integer<8>, TowardZero, Saturate>;
type NarrowSat = Sig<Integer<4>, TowardZero, Saturate>;
type NarrowWrap = Sig<Integer<4>, TowardZero, Wrap>;

fn wrap4(v: i128) -> i128 {
    (v + 8).rem_euclid(16) - 8
}
fn sat4(v: i128) -> i128 {
    v.clamp(-8, 7)
}

fn main() {
    // 1. The source's selection takes no part: a wrapping source and a saturating
    //    source give one answer onto each target.
    let (mut src_matters, mut is_target_sat, mut is_target_wrap) = (0, 0, 0);
    for v in -128i128 ..= 127 {
        let w: Num<WideWrap, Quiet> = Num::adapted(Exact::on_grid(Slot::at(v)));
        let s: Num<WideSat, Quiet> = Num::adapted(Exact::on_grid(Slot::at(v)));
        let (ws, ss) = (
            w.convert::<NarrowSat>().slot().index(),
            s.convert::<NarrowSat>().slot().index(),
        );
        let (ww, sw) = (
            w.convert::<NarrowWrap>().slot().index(),
            s.convert::<NarrowWrap>().slot().index(),
        );
        if ws != ss || ww != sw {
            src_matters += 1;
        }
        is_target_sat += (ws == sat4(v) && ss == sat4(v)) as u32;
        is_target_wrap += (ww == wrap4(v) && sw == wrap4(v)) as u32;
    }
    println!("source selection changes the answer on {src_matters}/256 members");
    println!("onto a saturating target the answer is the clamp on {is_target_sat}/256");
    println!("onto a wrapping target the answer is the residue on {is_target_wrap}/256");
    assert_eq!((src_matters, is_target_sat, is_target_wrap), (0, 256, 256));

    // 2. The three options, each written at the crossing site from the one door,
    //    each arriving as a value of the same target signature.
    let mut one_two_differ = 0;
    for v in -128i128 ..= 127 {
        let a: Num<WideWrap, Quiet> = Num::adapted(Exact::on_grid(Slot::at(v)));
        // Option 2: the target's.
        let two: Num<NarrowSat, Quiet> = a.convert::<NarrowSat>();
        // Option 1: the source's kind, named at the target's sibling, then the
        // sibling crossing that computes nothing.
        let one: Num<NarrowSat, Quiet> = a
            .convert::<WithOverflow<NarrowSat, Wrap>>()
            .under::<NarrowSat>();
        assert_eq!(two.slot().index(), sat4(v));
        assert_eq!(one.slot().index(), wrap4(v));
        if one.slot().index() != two.slot().index() {
            one_two_differ += 1;
        }
    }
    println!(
        "option 1 and option 2, both written from the one door, differ on {one_two_differ}/256"
    );
    assert_eq!(one_two_differ, 226);

    // Option 3: a reduction neither endpoint selected. Eighths under floor onto
    // integers under floor, with half-even named at the site.
    type Eighths = Sig<Fixed<8, -3>, Floor, Saturate>;
    type Whole = Sig<Integer<8>, Floor, Saturate>;
    let mut third_differs = 0;
    for v in -128i128 ..= 127 {
        let a: Num<Eighths, Quiet> = Num::adapted(Exact::on_grid(Slot::at(v)));
        let endpoint: Num<Whole, Quiet> = a.convert::<Whole>();
        let third: Num<Whole, Quiet> = a
            .convert::<WithRounding<Whole, HalfEven>>()
            .under::<Whole>();
        let floor = v.div_euclid(8);
        let r = v.rem_euclid(8);
        let half_even =
            if r > 4 || (r == 4 && floor.rem_euclid(2) != 0) { floor + 1 } else { floor };
        assert_eq!(endpoint.slot().index(), floor);
        assert_eq!(third.slot().index(), half_even);
        if floor != half_even {
            third_differs += 1;
        }
    }
    println!(
        "option 3, half-even named at the site between two floor endpoints, differs from both on {third_differs}/256"
    );
    assert!(third_differs > 0);

    // 3. The value that arrives is the target's from then on, whichever option
    //    carried it there.
    let top: Num<WideWrap, Quiet> = Num::adapted(Exact::on_grid(Slot::at(7)));
    let arrived: Num<NarrowSat, Quiet> = top
        .convert::<WithOverflow<NarrowSat, Wrap>>()
        .under::<NarrowSat>();
    let one: Num<NarrowSat, Quiet> = Num::adapted(Exact::on_grid(Slot::at(1)));
    println!(
        "7 arrived under the source's kind, then 7 + 1 in the target is {}",
        arrived.add(one).slot().index()
    );
    assert_eq!(arrived.add(one).slot().index(), 7);
    println!("every assertion held");
}
