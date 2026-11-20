//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! An outside consumer, written the way hilavitkutin uses a capacity, with no
//! feature line anywhere in it.
//!
//! Many dimensions bundled as the associated types of one parameter, a plan
//! whose fields are projections and a nested projection, filled and walked in
//! code generic over the bundle, and every count read in a constant and checked
//! against its array's length while the build evaluates it. Every array here is
//! real, sized by the capacity, never a phantom.

use arvo_capacity::{Capacity, Dim, count};
use arvo_format::Adapt;
use arvo_format::overflow::Wrap;
use arvo_format::rounding::TowardZero;
use arvo_numeral::{Abort, USize};

type A = Adapt<TowardZero, Wrap>;

/// A bundle of dimensions, one type parameter for every structure that is sized.
trait PlanDims {
    type Units: Capacity;
    type Cores: Capacity;
    type AccumsPerCore: Capacity;
}

/// The consumer's budget, declared once.
struct DefaultPlanDims;

impl PlanDims for DefaultPlanDims {
    type AccumsPerCore = Dim<16>;
    type Cores = Dim<8>;
    type Units = Dim<256>;
}

/// A second budget, so nothing in the generic code can be naming a length.
struct SmallPlanDims;

impl PlanDims for SmallPlanDims {
    type AccumsPerCore = Dim<3>;
    type Cores = Dim<2>;
    type Units = Dim<4>;
}

/// A plan sized by projections only, one-dimensional and nested.
struct Plan<D: PlanDims> {
    order:   <D::Units as Capacity>::Array<u32>,
    publish: <D::Cores as Capacity>::Array<<D::AccumsPerCore as Capacity>::Array<u32>>,
    live:    usize,
}

/// The count and the array's length agree, checked where the build evaluates it.
const fn agrees<C: Capacity>() {
    const {
        assert!(
            core::mem::size_of::<C::Array<u8>>() == count::<C, A, Abort>().host(),
            "the capacity's array and its count disagree",
        );
    }
}

/// The plan's byte total, read in a constant from the counts.
const fn order_bytes<D: PlanDims>() -> usize {
    count::<D::Units, A, Abort>().host() * core::mem::size_of::<u32>()
}

/// Builds a plan in generic code, fills it through the slice bridge, and
/// returns its order's sum, the nested cell written, and the outer count.
///
/// The `where` clause is the residue the nested projection carries: the outer
/// array is filled by copying an inner one.
fn build_and_walk<D: PlanDims>() -> (u32, u32, USize<A>)
where
    <D::AccumsPerCore as Capacity>::Array<u32>: Copy,
{
    agrees::<D::Units>();
    agrees::<D::Cores>();
    agrees::<D::AccumsPerCore>();
    let mut plan = Plan::<D> {
        order:   D::Units::filled(0),
        publish: D::Cores::filled(D::AccumsPerCore::filled(0)),
        live:    0,
    };
    for (i, slot) in plan.order.as_mut().iter_mut().enumerate() {
        *slot = i as u32;
        plan.live += 1;
    }
    assert_eq!(plan.live, plan.order.as_ref().len());
    let last_core = plan.publish.as_ref().len() - 1;
    let last_accum = plan.publish.as_ref()[last_core].as_ref().len() - 1;
    plan.publish.as_mut()[last_core].as_mut()[last_accum] = 7;
    let sum = plan.order.as_ref().iter().sum();
    let nested = plan.publish.as_ref()[last_core].as_ref()[last_accum];
    (sum, nested, count::<D::Cores, A, Abort>())
}

const DEFAULT_ORDER_BYTES: usize = order_bytes::<DefaultPlanDims>();

fn main() {
    let (sum, nested, cores) = build_and_walk::<DefaultPlanDims>();
    assert_eq!(sum, (0 .. 256).sum::<u32>());
    assert_eq!(nested, 7);
    assert_eq!(cores.host(), 8);

    let (sum, nested, cores) = build_and_walk::<SmallPlanDims>();
    assert_eq!(sum, 6);
    assert_eq!(nested, 7);
    assert_eq!(cores.host(), 2);

    assert_eq!(DEFAULT_ORDER_BYTES, 1024);
    assert_eq!(
        core::mem::size_of::<Plan<DefaultPlanDims>>(),
        256 * 4 + 8 * 16 * 4 + core::mem::size_of::<usize>(),
    );
    println!("default and small plans built and walked; counts agree with their arrays");
}
