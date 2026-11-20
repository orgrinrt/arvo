//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! An outside consumer, written the way hilavitkutin uses a capacity today.
//!
//! No `#![feature(...)]` line anywhere in this crate. Every item below sizes a
//! real array, never a phantom, from code generic over the capacity, because
//! `retirement::dl_feasibility_probe_compiled_the_load_bearing_path` records
//! that the earlier probe stopped short of exactly that.
//!
//! What it mirrors, from hilavitkutin `dev` at `56a4ba4f`:
//! `PlanDims` bundling many dimensions as associated types of one parameter
//! (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1124-1141`), projections such as
//! `<D::Units as Capacity>::Array<UnitMeta>` (`:1196`), the nested
//! `<Cores as Capacity>::Array<<AccumsPerCore as Capacity>::Array<_>>`
//! (`:1156-1158`), a live count bounded by the dimension's count (`:1200-1202`),
//! and the arvo-value count read in a constant (`hilavitkutin-api/src/footprint.rs:45`).

#![no_std]

use capacity::Dim;
use capacity::sealed::Capacity;

/// A bundle of dimensions, one type parameter for every structure that is sized.
pub trait PlanDims {
    type Units: Capacity;
    type Cores: Capacity;
    type AccumsPerCore: Capacity;
}

/// The consumer's budget, declared once.
pub struct DefaultPlanDims;

impl PlanDims for DefaultPlanDims {
    type AccumsPerCore = Dim<16>;
    type Cores = Dim<8>;
    type Units = Dim<256>;
}

/// A second budget, to show nothing in the generic code names a length.
pub struct SmallPlanDims;

impl PlanDims for SmallPlanDims {
    type AccumsPerCore = Dim<3>;
    type Cores = Dim<2>;
    type Units = Dim<4>;
}

/// A plan sized by projections only, one-dimensional and nested.
pub struct Plan<D: PlanDims> {
    pub order:   <D::Units as Capacity>::Array<u32>,
    pub publish: <D::Cores as Capacity>::Array<<D::AccumsPerCore as Capacity>::Array<u32>>,
    pub live:    usize,
}

/// The agreement between a capacity's array and its count, checked where it
/// is read, at build, for every capacity a generic caller instantiates.
pub const fn agrees<C: Capacity>() {
    const {
        assert!(
            core::mem::size_of::<C::Array<u8>>() == C::COUNT.host(),
            "the capacity's array and its count disagree",
        );
    }
}

/// Builds a plan in generic code, fills it through the slice bridge, and
/// returns its order's sum and the count of the nested array's outer axis.
///
/// The `where` clause is the residue the nested projection carries: the outer
/// array is filled by copying an inner one, and the trait cannot say that an
/// array of `Copy` elements is `Copy` for every capacity. hilavitkutin states
/// the same bound at `mock/crates/hilavitkutin/src/thread/assignment.rs:53-54`.
pub fn build_and_walk<D: PlanDims>() -> (u32, usize)
where
    <D::AccumsPerCore as Capacity>::Array<u32>: Copy,
{
    agrees::<D::Units>();
    agrees::<D::Cores>();
    agrees::<D::AccumsPerCore>();
    let mut plan = Plan::<D> {
        order:   <D::Units as Capacity>::filled(0),
        publish: <D::Cores as Capacity>::filled(<D::AccumsPerCore as Capacity>::filled(0)),
        live:    0,
    };
    let units = plan.order.as_mut();
    let mut i = 0;
    while i < units.len() {
        units[i] = i as u32;
        i += 1;
    }
    plan.live = units.len();
    plan.publish.as_mut()[1].as_mut()[2] = 7;
    let mut sum = 0;
    for v in plan.order.as_ref() {
        sum += *v;
    }
    let nested = plan.publish.as_ref()[1].as_ref()[2];
    (sum + nested, D::Cores::COUNT.host())
}

/// Arm B at its best case: a container whose `Cap` parameter the provider
/// declares, named here with a constant. The consumer declares no `Cap`
/// parameter of its own.
pub type Window = value_cap::Seq<u32, { value_cap::Cap::new(4) }>;

/// The count of `Window`, read in a constant.
pub const WINDOW_COUNT: usize = Window::COUNT;
