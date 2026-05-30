//! Generic-threading proof for the `Capacity`-migrated arvo-spectral algorithms.
//!
//! The migrated functions run when the capacity is threaded through a caller's
//! OWN generic `C: Capacity` (not a concrete `Dim<N>` fixed at the call site).
//! That is the shape #652 (hilavitkutin threading `PlanDims` capacities) needs,
//! and the exact shape that overflowed `generic_const_exprs` under the
//! `const N: Cap` form. No `#![feature(...)]` gate: its absence is the proof
//! the algorithms escaped the GCE surface.
//!
//! `adt_const_params` is still gated here, but only because the `common::TF`
//! `FromConstant` impl declares `from_constant<const C: USize>`. The
//! GCE-escape proof is the absence of `#![feature(generic_const_exprs)]`,
//! which the migration removed.

#![feature(adt_const_params)]

use arvo::USize;
use arvo_spectral::{
    Matrix, dense_laplacian_lambda_max_bound, fiedler_vector, k_way_partition, laplacian,
    power_iteration, spectral_bisection,
};
use arvo_tensor::{Capacity, Dim};

mod common;
use common::TF;

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

// Power iteration threaded through a caller's own `C: Capacity` over a dense
// identity-like Matrix operator. The identity matrix preserves the seed, so the
// L2-normalised all-ones vector has every entry 1/sqrt(N). Generic in `C`.
fn power_identity_entry<C: Capacity>(n: usize) -> f32
where
    C::Array<TF>: Copy,
    C::Array<C::Array<TF>>: Copy,
{
    let m: Matrix<TF, C> =
        Matrix::from_fn(|i, j| if i.0 == j.0 && i.0 < n { TF(1.0) } else { TF(0.0) });
    let v: C::Array<TF> = power_iteration::<Matrix<TF, C>, C, TF>(&m, USize(8));
    v.as_ref()[0].0
}

#[test]
fn power_iteration_threads_generically() {
    // 1/sqrt(4) = 0.5; 1/sqrt(3) ~= 0.5773.
    assert!((power_identity_entry::<Dim<4>>(4) - 0.5).abs() < 1e-4);
    assert!((power_identity_entry::<Dim<3>>(3) - (1.0f32 / 3.0f32.sqrt())).abs() < 1e-4);
}

// Dense Laplacian -> Fiedler -> bisection threaded through a caller's own
// `C: Capacity`. Two clusters {0,1} and {2,3} bridged weakly; the bisection
// must place each cluster's nodes in the same class. Generic in `C`.
fn dense_bisection_clusters_agree<C: Capacity>() -> bool
where
    C::Array<TF>: Copy,
    C::Array<USize>: Copy,
    C::Array<C::Array<TF>>: Copy,
    C::Array<u32>: Copy,
    C::Array<C::Array<u32>>: Copy,
{
    let mut w: Matrix<u32, C> = Matrix::from_fn(|_, _| 0u32);
    w.set(USize(0), USize(1), 10);
    w.set(USize(1), USize(0), 10);
    w.set(USize(2), USize(3), 10);
    w.set(USize(3), USize(2), 10);
    w.set(USize(1), USize(2), 1);
    w.set(USize(2), USize(1), 1);
    let lap: Matrix<TF, C> = laplacian::<C, u32, TF>(&w);
    let sigma = dense_laplacian_lambda_max_bound::<C, TF>(&lap);
    let fiedler: C::Array<TF> = fiedler_vector::<Matrix<TF, C>, C, TF>(&lap, sigma, USize(100));
    let (_count, class) = spectral_bisection::<C, TF>(&fiedler, USize(4));
    let cs = class.as_ref();
    // Cluster A nodes agree, cluster B nodes agree, clusters differ.
    cs[0] == cs[1] && cs[2] == cs[3] && cs[0] != cs[2]
}

#[test]
fn fiedler_bisection_threads_generically() {
    assert!(dense_bisection_clusters_agree::<Dim<4>>());
}

// k_way_partition threaded through a caller's own operator capacity `C` and a
// distinct partition-budget capacity `K`. Exercises the second `Capacity`
// parameter (the work-stack `K::Array<USize>`) generically.
fn kway_count<C: Capacity, K: Capacity>() -> usize
where
    C::Array<TF>: Copy,
    C::Array<USize>: Copy,
    C::Array<C::Array<TF>>: Copy,
    C::Array<u32>: Copy,
    C::Array<C::Array<u32>>: Copy,
    K::Array<USize>: Copy,
{
    let mut w: Matrix<u32, C> = Matrix::from_fn(|_, _| 0u32);
    w.set(USize(0), USize(1), 10);
    w.set(USize(1), USize(0), 10);
    w.set(USize(2), USize(3), 10);
    w.set(USize(3), USize(2), 10);
    w.set(USize(1), USize(2), 1);
    w.set(USize(2), USize(1), 1);
    let lap: Matrix<TF, C> = laplacian::<C, u32, TF>(&w);
    let sigma = dense_laplacian_lambda_max_bound::<C, TF>(&lap);
    let (count, _ids) = k_way_partition::<Matrix<TF, C>, C, K, TF>(&lap, sigma, USize(100));
    count.0
}

#[test]
fn k_way_threads_generically() {
    // 2-cluster graph in a Dim<4> operator with a Dim<2> partition budget.
    assert_eq!(kway_count::<Dim<4>, Dim<2>>(), 2);
}
