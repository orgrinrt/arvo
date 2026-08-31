//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The laws the design names, over the whole matrix of the axis that moves them.
//!
//! The axis is occupancy, so every law below is asserted on both sides of it
//! rather than on the side that was convenient.
//!
//! One law here is catalogued red on purpose and says why at its own site. The
//! design states an independence in both directions and this crate's single
//! packing rule exhibits one of them, so the honest state is the assertion
//! standing and failing rather than an assertion narrowed until it passes.

use crate::{
    declared_width, derive_shared, derive_sole, narrowest_carrier, Carrier, Carrier16, Carrier32,
    Carrier64, Carrier8, Objective, Occupancy, Placement, LADDER,
};
use arvo_format::overflow::Wrap;
use arvo_format::points::{Integer, UFixed};
use arvo_format::rounding::Floor;
use arvo_format::{Adapt, Signature, Width};

type Sig<F> = Signature<F, Adapt<Floor, Wrap>>;

/// Every declared width from one to sixty-four, as widths rather than counts.
fn every_width() -> impl Iterator<Item = Width> {
    (1u32..=64).map(Width::bits)
}

// --- the control -------------------------------------------------------------

#[test]
fn the_control_the_ladder_rungs_are_distinct_and_ordered() {
    for i in 1..LADDER.len() {
        assert!(
            LADDER[i].covers(LADDER[i - 1]).get() && !LADDER[i].equals(LADDER[i - 1]).get(),
            "the ladder has to be ordered or `narrowest_carrier` is picking arbitrarily"
        );
    }
    assert_eq!(<Carrier8 as Carrier>::BITS, Width::bits(8));
    assert_eq!(<Carrier16 as Carrier>::BITS, Width::bits(16));
    assert_eq!(<Carrier32 as Carrier>::BITS, Width::bits(32));
    assert_eq!(<Carrier64 as Carrier>::BITS, Width::bits(64));
}

// --- the output count moves with occupancy, and that is the axis -------------

#[test]
fn the_output_count_is_one_at_sole_and_three_at_shared() {
    let sole = derive_sole::<Sig<Integer<13>>, { Objective::Footprint }>();
    let shared = derive_shared::<Sig<Integer<13>>, { Objective::Footprint }>();

    assert_eq!(sole.output_count(), Width::bits(1));
    assert_eq!(shared.output_count(), Width::bits(3));
    assert_ne!(
        sole.output_count(),
        shared.output_count(),
        "if the count did not move, occupancy would not be an axis and the two \
         arms would be one"
    );
}

#[test]
fn the_footprint_is_observable_at_sole_occupancy_and_not_at_shared() {
    let sole = derive_sole::<Sig<Integer<13>>, { Objective::Footprint }>();
    let shared = derive_shared::<Sig<Integer<13>>, { Objective::Footprint }>();

    assert!(sole.footprint_is_observable().get());
    assert!(!shared.footprint_is_observable().get());
}

#[test]
fn at_sole_occupancy_the_three_numbers_collapse_to_the_carrier() {
    for declared in every_width() {
        let c = narrowest_carrier(declared);
        if c.is_none().get() {
            continue;
        }
        let p = Placement {
            carrier: c,
            access: c,
            stride: c,
            occupancy: Occupancy::Sole,
        };
        assert_eq!(p.carrier, p.access);
        assert_eq!(p.carrier, p.stride);
    }
}

#[test]
fn at_shared_occupancy_the_stride_is_the_declared_width_and_not_the_carrier() {
    let p = derive_shared::<Sig<Integer<13>>, { Objective::Footprint }>();
    assert_eq!(p.stride, declared_width::<Sig<Integer<13>>>());
    assert_ne!(
        p.stride, p.carrier,
        "at a declared width the ladder does not land on, the stride and the \
         carrier have to differ or nothing is packed"
    );
}

// --- the independence, one direction at a time -------------------------------
//
// The design states that the access width is not a function of the carrier and
// that the carrier is not a function of the access width. Those are two claims
// and this crate's derivation exhibits one of them, so they are two tests.
//
// The full relation over every width the ladder reaches:
//
//   carrier 8  -> accesses {8, 16}
//   carrier 16 -> accesses {32}
//   carrier 32 -> accesses {64}
//
//   access 8  -> carriers {8}
//   access 16 -> carriers {8}
//   access 32 -> carriers {16}
//   access 64 -> carriers {32}

/// Every declared width the carrier ladder reaches, with its two derived numbers.
fn carrier_and_access_over_the_ladder() -> impl Iterator<Item = (Width, Width, Width)> {
    every_width().filter_map(|w| {
        let c = narrowest_carrier(w);
        let a = narrowest_carrier(w.add(w).less_one());
        if c.is_none().get() || a.is_none().get() {
            None
        } else {
            Some((w, c, a))
        }
    })
}

#[test]
fn the_access_width_is_not_a_function_of_the_carrier() {
    // Two declared widths sharing a carrier and differing in access. Found by
    // sweeping the ladder rather than by picking a pair and hoping, which is what
    // the first cut of this test did: it chose widths 9 and 13, which share a
    // carrier and also share an access, so it failed for the right reason and
    // said nothing about the law.
    let mut witness = None;
    for (w1, c1, a1) in carrier_and_access_over_the_ladder() {
        for (_w2, c2, a2) in carrier_and_access_over_the_ladder() {
            if c1 == c2 && a1 != a2 {
                witness = Some((w1, c1, a1, a2));
            }
        }
    }
    let (w1, carrier, a1, a2) = witness.expect(
        "no two declared widths share a carrier and differ in access, so the access \
         width is a function of the carrier and the design's claim is refuted here",
    );
    assert_eq!(carrier, narrowest_carrier(w1));
    assert_ne!(a1, a2);
}

#[test]
#[ignore = "catalogue: the converse independence is not exhibited by this crate's single packing \
            rule. The design states the carrier is not a function of the access width, measured \
            under both packing rules; only one rule ships here, and under it every access width \
            reaches exactly one carrier. Red until the second packing rule lands."]
fn the_carrier_is_not_a_function_of_the_access_width() {
    // The assertion is the intended behaviour and is not weakened to pass. Under a
    // derivation carrying both packing rules there are two declared widths sharing
    // an access width and differing in carrier; under the one rule here there are
    // none, and this test is the record of that gap rather than a claim it does
    // not matter.
    let mut witness = None;
    for (w1, c1, a1) in carrier_and_access_over_the_ladder() {
        for (_w2, c2, a2) in carrier_and_access_over_the_ladder() {
            if a1 == a2 && c1 != c2 {
                witness = Some((w1, a1, c1, c2));
            }
        }
    }
    let (w, access, c1, c2) = witness.expect(
        "no two declared widths share an access width and differ in carrier, so the \
         carrier is a function of the access width under the packing rule this \
         crate ships",
    );
    assert_eq!(access, narrowest_carrier(w.add(w).less_one()));
    assert_ne!(c1, c2);
}

#[test]
fn the_two_derived_numbers_are_not_the_same_number_wearing_two_names() {
    // Weaker than either independence claim and it holds, so it is the thing this
    // crate can honestly assert about the pair: somewhere on the ladder the access
    // and the carrier differ, which is what makes them two outputs at all.
    let differing = carrier_and_access_over_the_ladder()
        .filter(|(_, c, a)| c != a)
        .count();
    assert!(
        differing > 0,
        "the access width equals the carrier at every width, which would make the \
         shared arm's three outputs two"
    );
}

// --- the derivation reads the declared signature and nothing else ------------

#[test]
fn the_declared_width_comes_from_the_slot_range_and_not_from_a_container() {
    assert_eq!(declared_width::<Sig<Integer<8>>>(), Width::bits(8));
    assert_eq!(declared_width::<Sig<Integer<13>>>(), Width::bits(13));
    assert_eq!(declared_width::<Sig<Integer<32>>>(), Width::bits(32));
    assert_eq!(declared_width::<Sig<UFixed<13, -4>>>(), Width::bits(13));

    assert_eq!(
        declared_width::<Sig<UFixed<13, -4>>>(),
        declared_width::<Sig<UFixed<13, 0>>>(),
        "the quantum's exponent is not part of the declared width and a placement \
         that read it would be deriving from the wrong thing"
    );
}

#[test]
fn two_adaptations_over_one_format_derive_one_placement() {
    use arvo_format::overflow::Saturate;
    use arvo_format::rounding::HalfEven;

    type A = Signature<Integer<13>, Adapt<Floor, Wrap>>;
    type B = Signature<Integer<13>, Adapt<HalfEven, Saturate>>;

    assert_eq!(
        derive_sole::<A, { Objective::Footprint }>(),
        derive_sole::<B, { Objective::Footprint }>()
    );
    assert_eq!(
        derive_shared::<A, { Objective::Footprint }>(),
        derive_shared::<B, { Objective::Footprint }>()
    );
}

// --- the ladder, and where it runs out ---------------------------------------

#[test]
fn the_narrowest_carrier_is_the_narrowest_rung_that_holds_the_width() {
    for declared in every_width() {
        let picked = narrowest_carrier(declared);
        assert!(
            !picked.is_none().get(),
            "width {declared:?} should be on the ladder"
        );
        assert!(
            picked.covers(declared).get(),
            "width {declared:?} was given a carrier of {picked:?}, which cannot hold it"
        );
        for rung in LADDER {
            if rung.covers(declared).get() {
                assert!(
                    rung.covers(picked).get(),
                    "width {declared:?} took {picked:?} when {rung:?} also holds it"
                );
            }
        }
    }
}

#[test]
fn the_ladder_runs_out_and_says_so_rather_than_widening_silently() {
    assert!(narrowest_carrier(Width::bits(65)).is_none().get());
    assert!(narrowest_carrier(Width::bits(123)).is_none().get());
    assert!(narrowest_carrier(Width::bits(1024)).is_none().get());
    assert_eq!(narrowest_carrier(Width::bits(64)), Width::bits(64));
}

#[test]
fn the_access_ladder_runs_out_before_the_carrier_ladder_does() {
    let mut carrier_last = Width::NONE;
    let mut access_last = Width::NONE;
    for declared in every_width() {
        if !narrowest_carrier(declared).is_none().get() {
            carrier_last = declared;
        }
        if !narrowest_carrier(declared.add(declared).less_one())
            .is_none()
            .get()
        {
            access_last = declared;
        }
    }
    assert!(
        carrier_last.covers(access_last).get() && !carrier_last.equals(access_last).get(),
        "the two ladders ran out together, which would make the access width a \
         function of the carrier after all"
    );
}

// --- the objective is what the ladder is keyed on ----------------------------

/// The objective is a const parameter, so a width sweep over it is a macro
/// rather than a loop: both coordinates are compile-time and neither can be a
/// runtime variable.
macro_rules! objective_sweep {
    ($($w:literal),+ $(,)?) => {
        #[test]
        fn the_two_objectives_agree_at_sole_occupancy_and_that_is_a_result() {
            // The value owns its allocation, so there is nothing to trade: the
            // narrowest carrier that holds the width is at once the smallest
            // footprint and the widest native access. Asserted as an equality so
            // it reads as the finding it is rather than as the collapse it was.
            $(
                assert_eq!(
                    derive_sole::<Sig<Integer<$w>>, { Objective::Footprint }>(),
                    derive_sole::<Sig<Integer<$w>>, { Objective::Access }>(),
                    "the objectives differ at sole occupancy at width {}", $w
                );
            )+
        }

        #[test]
        fn the_objective_selects_at_every_width_the_ladder_reaches() {
            // One witness is not a selection. Counted over the whole sweep so a
            // return of the dead-parameter defect fails here rather than passing
            // on the one width somebody happened to pick.
            let mut differing = 0usize;
            $(
                {
                    let f = derive_shared::<Sig<Integer<$w>>, { Objective::Footprint }>();
                    let a = derive_shared::<Sig<Integer<$w>>, { Objective::Access }>();
                    if f != a { differing += 1; }
                }
            )+
            assert!(
                differing > 0,
                "the objective changed nothing at any width, which is the \
                 dead-parameter defect returning"
            );
        }
    };
}

objective_sweep!(2, 3, 5, 8, 9, 13, 16, 17, 27, 31, 32, 33, 47, 62);

#[test]
fn the_two_objectives_derive_two_placements_at_shared_occupancy() {
    // This is the comparison the old version of this test was named for and did
    // not make: it asserted both occupancies were `Sole` and that two enum
    // variants differ, and never compared the two derivations it took.
    //
    // Footprint packs, so the stride is the declared width and the access reaches
    // wider. Access does not pack, so stride and access are both the carrier.
    type S13 = Sig<Integer<13>>;
    let f = derive_shared::<S13, { Objective::Footprint }>();
    let a = derive_shared::<S13, { Objective::Access }>();

    assert_ne!(
        f, a,
        "the objective selected the same placement at shared occupancy, which is \
         the defect this test exists for"
    );
    assert_eq!(f.stride, declared_width::<S13>(), "footprint should pack");
    assert_eq!(a.stride, a.carrier, "access should not pack");
    assert!(
        !f.access.equals(f.carrier).get(),
        "a packed element that can straddle needs a read wider than its carrier"
    );
    assert_eq!(
        a.access, a.carrier,
        "an unpacked element needs one native read"
    );

    // The carrier is the one output they agree on, which is why the difference
    // lives in the other two.
    assert_eq!(f.carrier, a.carrier);
}

// --- what is deliberately absent ---------------------------------------------

#[test]
fn a_placement_carries_no_adaptation_and_no_operation() {
    // Structural. `Placement` has four fields and they are the three widths and
    // the axis. A rounding mode or an operation arriving in this struct would
    // break this destructuring, which is why it is written rather than left to a
    // comment.
    let p = derive_sole::<Sig<Integer<13>>, { Objective::Footprint }>();
    let Placement {
        carrier,
        access,
        stride,
        occupancy,
    } = p;
    assert!(!carrier.is_none().get());
    assert!(!access.is_none().get());
    assert!(!stride.is_none().get());
    assert_eq!(occupancy, Occupancy::Sole);
}
