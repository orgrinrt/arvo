//! Bundle 2 (proper harness form): spectral-bisection Routine
//! + bench_variant cdylib for `fiedler_vector` over a dense
//! `Matrix<TF, C>` Laplacian.
//!
//! The capacity-as-type migration replaced the spectral algorithms'
//! `const N: Cap` parameter with `C: Capacity`. The variant body
//! dispatches per N to functions that instantiate `Matrix<TF, Dim<N>>`
//! directly; the prior named-`Cap`-constant ICE workaround is gone
//! because `Dim<N>` is plain min-const-generics with no `cap_size`
//! const-eval. The Routine keeps its const generic as `usize` for
//! `bench_variant` compatibility.
//!
//! TF (test float) is a local Copy newtype over f32 that satisfies
//! the arvo numeric trait surface (Sqrt, Recip, TotalOrd,
//! FromConstant, ...) required by `fiedler_vector` and friends.

#![no_std]
// `adt_const_params` is required by the local `TF` `FromConstant` impl
// (`from_constant<const C: USize>`), not by capacity arithmetic. The
// migration dropped `generic_const_exprs`; this gate is independent of it.
#![feature(adt_const_params)]

use core::cmp::Ordering;
use core::ops::{Add, Mul, Sub};

use arvo::traits::{FromConstant, Recip, Sqrt, TotalOrd};
use arvo::USize;
use arvo_spectral::{
    Matrix, dense_laplacian_lambda_max_bound, fiedler_vector, laplacian, spectral_bisection,
};
use arvo_tensor::Dim;
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

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
    // bench-local: low-precision Newton-Raphson sqrt; NOT a reference impl.
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

/// Per-N safe dispatch trait. Each supported size implements this
/// with a body that instantiates the arvo algorithm at the
/// corresponding named `Cap` constant. The variant fn dispatches
/// via `<Fiedler<N> as FiedlerDispatch>::run(input, output)`,
/// propagating the `Fiedler<N>: FiedlerDispatch` bound through the
/// `bench_variant` macro expansion. The compiler resolves the impl
/// per literal `N` the macro emits, so no runtime match and no
/// unsafe pointer casts.
pub trait FiedlerDispatch: Routine {
    fn run(input: &Self::Input, output: &mut Self::Output);
}

impl FiedlerDispatch for Fiedler<16> {
    #[inline(never)]
    fn run(input: &FiedlerInput<16>, output: &mut FiedlerOutput<16>) {
        let mut w: Matrix<u32, Dim<16>> = Matrix::from_fn(|_, _| 0u32);
        for i in 0..16 {
            for j in 0..16 {
                w.set(USize(i), USize(j), input.weights[i][j] as u32);
            }
        }
        let lap: Matrix<TF, Dim<16>> = laplacian(&w);
        let sigma = dense_laplacian_lambda_max_bound(&lap);
        let fv: [TF; 16] = fiedler_vector(&lap, sigma, USize(50));
        let (_count, ids) = spectral_bisection::<Dim<16>, TF>(&fv, USize(16));
        for i in 0..16 {
            output.ids[i] = ids[i].0 as u8;
        }
    }
}

impl FiedlerDispatch for Fiedler<32> {
    #[inline(never)]
    fn run(input: &FiedlerInput<32>, output: &mut FiedlerOutput<32>) {
        let mut w: Matrix<u32, Dim<32>> = Matrix::from_fn(|_, _| 0u32);
        for i in 0..32 {
            for j in 0..32 {
                w.set(USize(i), USize(j), input.weights[i][j] as u32);
            }
        }
        let lap: Matrix<TF, Dim<32>> = laplacian(&w);
        let sigma = dense_laplacian_lambda_max_bound(&lap);
        let fv: [TF; 32] = fiedler_vector(&lap, sigma, USize(50));
        let (_count, ids) = spectral_bisection::<Dim<32>, TF>(&fv, USize(32));
        for i in 0..32 {
            output.ids[i] = ids[i].0 as u8;
        }
    }
}

impl FiedlerDispatch for Fiedler<64> {
    #[inline(never)]
    fn run(input: &FiedlerInput<64>, output: &mut FiedlerOutput<64>) {
        let mut w: Matrix<u32, Dim<64>> = Matrix::from_fn(|_, _| 0u32);
        for i in 0..64 {
            for j in 0..64 {
                w.set(USize(i), USize(j), input.weights[i][j] as u32);
            }
        }
        let lap: Matrix<TF, Dim<64>> = laplacian(&w);
        let sigma = dense_laplacian_lambda_max_bound(&lap);
        let fv: [TF; 64] = fiedler_vector(&lap, sigma, USize(50));
        let (_count, ids) = spectral_bisection::<Dim<64>, TF>(&fv, USize(64));
        for i in 0..64 {
            output.ids[i] = ids[i].0 as u8;
        }
    }
}

#[bench_variant(Fiedler, "fiedler-bisect-dense", sizes = [16, 32, 64])]
fn fiedler_variant<const N: usize>(
    input: &<Fiedler<N> as Routine>::Input,
    output: &mut <Fiedler<N> as Routine>::Output,
) -> FfiBenchCall
where
    Fiedler<N>: FiedlerDispatch,
{
    timed! {
        run {
            <Fiedler<N> as FiedlerDispatch>::run(input, output);
        }
    }
}
