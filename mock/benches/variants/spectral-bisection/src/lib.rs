//! Bundle 2 (proper harness form): spectral-bisection Routine
//! + bench_variant cdylib for `fiedler_vector` over a dense
//! `Matrix<TF, C>` Laplacian.
//!
//! Same named-const dispatch pattern as Bundle 1: the Routine keeps
//! its const generic as `usize` for `bench_variant` compatibility,
//! and the variant body dispatches per N to functions that
//! instantiate `Matrix<TF, C>` with named `Cap` constants. This
//! sidesteps the rustc const-eval ICE that fires when arvo
//! algorithms receive Cap arguments computed from `const fn`
//! applications.
//!
//! TF (test float) is a local Copy newtype over f32 that satisfies
//! the arvo numeric trait surface (Sqrt, Recip, TotalOrd,
//! FromConstant, ...) required by `fiedler_vector` and friends.

#![no_std]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::cmp::Ordering;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::{Cap, USize};
use arvo_spectral::{
    Matrix, dense_laplacian_lambda_max_bound, fiedler_vector, laplacian, spectral_bisection,
};
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

const C16: Cap = Cap(USize(16));
const C32: Cap = Cap(USize(32));
const C64: Cap = Cap(USize(64));

/// Local Copy newtype over `f32` satisfying the arvo numeric trait
/// surface. Lives inside the bench so the orphan rule doesn't bite.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
#[repr(transparent)]
pub struct TF(pub f32);

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

/// FFI-safe input: row-major weight matrix as f32. Variant builds
/// the dense Laplacian per-N at call time.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct FiedlerInput<const N: usize> {
    pub weights: [[f32; N]; N],
}

impl<const N: usize> Default for FiedlerInput<N> {
    fn default() -> Self {
        Self { weights: [[0.0f32; N]; N] }
    }
}

/// Bisection output: partition id per node (0 or 1), packed as u8.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct FiedlerOutput<const N: usize> {
    pub ids: [u8; N],
}

impl<const N: usize> Default for FiedlerOutput<N> {
    fn default() -> Self {
        Self { ids: [0u8; N] }
    }
}

pub struct Fiedler<const N: usize>;

impl<const N: usize> Routine for Fiedler<N> {
    type Input = FiedlerInput<N>;
    type Output = FiedlerOutput<N>;

    fn build_input(seed: u64) -> Self::Input {
        // Two-cluster shape with one weak bridge. Seed-stable.
        let mut w = [[0.0f32; N]; N];
        let half = N / 2;
        for i in 0..half {
            for j in 0..half {
                if i != j {
                    w[i][j] = 5.0;
                }
            }
        }
        for i in half..N {
            for j in half..N {
                if i != j {
                    w[i][j] = 5.0;
                }
            }
        }
        if half >= 1 && half < N {
            w[half - 1][half] = 1.0;
            w[half][half - 1] = 1.0;
        }
        let _ = seed;
        FiedlerInput { weights: w }
    }

    fn validate_output(
        _input: &Self::Input,
        output: &Self::Output,
    ) -> Result<(), &'static str> {
        for id in output.ids.iter() {
            if *id > 1 {
                return Err("partition id out of range 0..2");
            }
        }
        Ok(())
    }
}

#[inline(never)]
fn run_at_c16(input: &FiedlerInput<16>, output: &mut FiedlerOutput<16>) {
    let mut w: Matrix<u32, C16> = Matrix::from_fn(|_, _| 0u32);
    for i in 0..16 {
        for j in 0..16 {
            w.set(USize(i), USize(j), input.weights[i][j] as u32);
        }
    }
    let lap: Matrix<TF, C16> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let fv: [TF; 16] = fiedler_vector(&lap, sigma, USize(50));
    let (_count, ids) = spectral_bisection::<C16, TF>(&fv);
    for i in 0..16 {
        output.ids[i] = ids[i].0 as u8;
    }
}

#[inline(never)]
fn run_at_c32(input: &FiedlerInput<32>, output: &mut FiedlerOutput<32>) {
    let mut w: Matrix<u32, C32> = Matrix::from_fn(|_, _| 0u32);
    for i in 0..32 {
        for j in 0..32 {
            w.set(USize(i), USize(j), input.weights[i][j] as u32);
        }
    }
    let lap: Matrix<TF, C32> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let fv: [TF; 32] = fiedler_vector(&lap, sigma, USize(50));
    let (_count, ids) = spectral_bisection::<C32, TF>(&fv);
    for i in 0..32 {
        output.ids[i] = ids[i].0 as u8;
    }
}

#[inline(never)]
fn run_at_c64(input: &FiedlerInput<64>, output: &mut FiedlerOutput<64>) {
    let mut w: Matrix<u32, C64> = Matrix::from_fn(|_, _| 0u32);
    for i in 0..64 {
        for j in 0..64 {
            w.set(USize(i), USize(j), input.weights[i][j] as u32);
        }
    }
    let lap: Matrix<TF, C64> = laplacian(&w);
    let sigma = dense_laplacian_lambda_max_bound(&lap);
    let fv: [TF; 64] = fiedler_vector(&lap, sigma, USize(50));
    let (_count, ids) = spectral_bisection::<C64, TF>(&fv);
    for i in 0..64 {
        output.ids[i] = ids[i].0 as u8;
    }
}

#[bench_variant(Fiedler, "fiedler-bisect-dense", sizes = [16, 32, 64])]
fn fiedler_variant<const N: usize>(
    input: &<Fiedler<N> as Routine>::Input,
    output: &mut <Fiedler<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            match N {
                16 => {
                    let i: &FiedlerInput<16> = unsafe { &*(input as *const _ as *const FiedlerInput<16>) };
                    let o: &mut FiedlerOutput<16> = unsafe { &mut *(output as *mut _ as *mut FiedlerOutput<16>) };
                    run_at_c16(i, o);
                }
                32 => {
                    let i: &FiedlerInput<32> = unsafe { &*(input as *const _ as *const FiedlerInput<32>) };
                    let o: &mut FiedlerOutput<32> = unsafe { &mut *(output as *mut _ as *mut FiedlerOutput<32>) };
                    run_at_c32(i, o);
                }
                64 => {
                    let i: &FiedlerInput<64> = unsafe { &*(input as *const _ as *const FiedlerInput<64>) };
                    let o: &mut FiedlerOutput<64> = unsafe { &mut *(output as *mut _ as *mut FiedlerOutput<64>) };
                    run_at_c64(i, o);
                }
                _ => unreachable!(),
            }
        }
    }
}
