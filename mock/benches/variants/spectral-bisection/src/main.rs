//! Bundle 2 bench: spectral-bisection.
//!
//! Times power_iteration and fiedler_vector + spectral_bisection on
//! dense Matrix<TF, N> Laplacians at several N and graph shapes.
//! Output is a Markdown table consumed by
//! findings_graph_spectral_202605111719.md.
//!
//! Limitations vs the full sweep named in the topic file:
//! - SparseLaplacian operator-vs-dense comparison is out of scope
//!   here. SparseLaplacian's borrowed CSR shape needs a fixture
//!   builder that exceeds this round's lock window. The dense
//!   path measured here establishes the baseline.
//! - Weight distribution axis collapses to uniform {0, 1}. Exponential
//!   and skewed distributions are follow-up work.
//! - N capped at 64 for parity with Bundle 1.

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::cmp::Ordering;
use core::ops::{Add, Mul, Sub};

use std::time::Instant;

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::{Cap, USize};
use arvo_bitmask::cap_size;
use arvo_spectral::{
    Matrix, dense_laplacian_lambda_max_bound, fiedler_vector, laplacian, power_iteration,
    spectral_bisection,
};

const fn cap(n: usize) -> Cap {
    Cap(USize(n))
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
struct TF(pub f32);

impl Add for TF {
    type Output = TF;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        TF(self.0 + rhs.0)
    }
}

impl Sub for TF {
    type Output = TF;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        TF(self.0 - rhs.0)
    }
}

impl Mul for TF {
    type Output = TF;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        TF(self.0 * rhs.0)
    }
}

impl Sqrt for TF {
    type Output = Self;
    #[inline(always)]
    fn sqrt(self) -> Self {
        if self.0 < 0.0 || self.0.is_nan() {
            return TF(f32::NAN);
        }
        if self.0 == 0.0 {
            return self;
        }
        let bits = self.0.to_bits();
        let guess_bits = (bits >> 1) + (0x3f80_0000u32 >> 1);
        let mut g = f32::from_bits(guess_bits);
        let x = self.0;
        for _ in 0..5 {
            g = 0.5 * (g + x / g);
        }
        TF(g)
    }
}

impl Recip for TF {
    type Output = Self;
    #[inline(always)]
    fn recip(self) -> Self {
        TF(1.0_f32 / self.0)
    }
}

impl TotalOrd for TF {
    #[inline(always)]
    fn total_cmp(self, other: Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl FromConstant for TF {
    #[inline(always)]
    fn from_constant<const C: USize>() -> Self {
        TF(C.0 as f32)
    }
}

impl From<u32> for TF {
    fn from(v: u32) -> TF {
        TF(v as f32)
    }
}

fn linear_weights<const N: Cap>() -> Matrix<u32, N>
where
    [(); cap_size(N)]:,
{
    let mut m: Matrix<u32, N> = Matrix::from_fn(|_, _| 0u32);
    let n = N.0.0;
    for i in 0..(n - 1) {
        m.set(USize(i), USize(i + 1), 1);
        m.set(USize(i + 1), USize(i), 1);
    }
    m
}

fn two_cluster_weights<const N: Cap>() -> Matrix<u32, N>
where
    [(); cap_size(N)]:,
{
    let mut m: Matrix<u32, N> = Matrix::from_fn(|_, _| 0u32);
    let n = N.0.0;
    let half = n / 2;
    for i in 0..half {
        for j in 0..half {
            if i != j {
                m.set(USize(i), USize(j), 5);
            }
        }
    }
    for i in half..n {
        for j in half..n {
            if i != j {
                m.set(USize(i), USize(j), 5);
            }
        }
    }
    m.set(USize(half - 1), USize(half), 1);
    m.set(USize(half), USize(half - 1), 1);
    m
}

fn time_micros<F: FnMut()>(mut f: F, iters: u32) -> f64 {
    f();
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    let elapsed = start.elapsed();
    elapsed.as_nanos() as f64 / iters as f64 / 1000.0
}

fn run_n<const N: Cap>(label: &str)
where
    [(); cap_size(N)]:,
{
    let n = N.0.0;
    let iters = 100;
    let pi_iters = USize(50);
    let fv_iters = USize(50);

    let lin = linear_weights::<N>();
    let lin_lap: Matrix<TF, N> = laplacian(&lin);
    let lin_sigma = dense_laplacian_lambda_max_bound(&lin_lap);

    let two = two_cluster_weights::<N>();
    let two_lap: Matrix<TF, N> = laplacian(&two);
    let two_sigma = dense_laplacian_lambda_max_bound(&two_lap);

    let pi_lin = time_micros(|| {
        let v: [TF; cap_size(N)] = power_iteration(&lin_lap, pi_iters);
        std::hint::black_box(&v);
    }, iters);
    let pi_two = time_micros(|| {
        let v: [TF; cap_size(N)] = power_iteration(&two_lap, pi_iters);
        std::hint::black_box(&v);
    }, iters);

    let fv_lin = time_micros(|| {
        let v: [TF; cap_size(N)] = fiedler_vector(&lin_lap, lin_sigma, fv_iters);
        std::hint::black_box(&v);
    }, iters);
    let fv_two = time_micros(|| {
        let v: [TF; cap_size(N)] = fiedler_vector(&two_lap, two_sigma, fv_iters);
        std::hint::black_box(&v);
    }, iters);

    let lin_fv: [TF; cap_size(N)] = fiedler_vector(&lin_lap, lin_sigma, fv_iters);
    let two_fv: [TF; cap_size(N)] = fiedler_vector(&two_lap, two_sigma, fv_iters);
    let bi_lin = time_micros(|| {
        let r = spectral_bisection::<N, TF>(&lin_fv);
        std::hint::black_box(&r);
    }, iters * 10);
    let bi_two = time_micros(|| {
        let r = spectral_bisection::<N, TF>(&two_fv);
        std::hint::black_box(&r);
    }, iters * 10);

    println!(
        "| {label} (N={n}) | {pi_lin:>9.3} | {pi_two:>9.3} | {fv_lin:>9.3} | {fv_two:>9.3} | {bi_lin:>8.3} | {bi_two:>8.3} |"
    );
}

fn main() {
    println!("# Bundle 2: spectral-bisection\n");
    println!("Microseconds per call. PI/FV: mean of 100 iterations, 50 algorithm");
    println!("iterations each. Bisection: mean of 1000 calls. Dense Matrix<TF, N>");
    println!("Laplacian; shapes: linear chain vs two-cluster + bridge.\n");
    println!("| Variant | PI (lin) | PI (2cl) | FV (lin) | FV (2cl) | Bi (lin) | Bi (2cl) |");
    println!("|---|---:|---:|---:|---:|---:|---:|");

    run_n::<{ cap(16) }>("spectral-bisection");
    run_n::<{ cap(32) }>("spectral-bisection");
    run_n::<{ cap(64) }>("spectral-bisection");
}
