//! Shared Routine + reference-model surface for the quantiser-vs-hardware-fadd bench.
//!
//! Answers the runtime question file 50 left owed (`50_fog_the_float_model.md` section 7):
//! what the design's own round-first software quantiser costs against a native hardware
//! `fadd`, with the subnormal fraction of the input swept as the parameter that separates
//! the two paths the most (`50:63-68`, "the exact result lies on a finer subgroup than the
//! result's own", which is exactly where gradual underflow does the most rounding work).
//!
//! `model.rs` is copied unmodified from `50_probes/model.rs` per the review's own
//! compose-rather-than-reinvent discipline (file 50 copied file 46's tower the same way).
//! It is a validated reference implementation of the design's ratified round-first
//! quantiser (`49:170-186`), checked bit-exact against binary32 silicon over 41,380,159
//! operations, 0 mismatches (`50_fog_the_float_model.md` section 2). This crate is bench
//! infrastructure, not shipping arvo source: it has no `#![no_std]` (matching every existing
//! bench variant in this directory, none of which is `no_std` either), because the arvo
//! `no_std`/no-`alloc` mandate governs `arvo*` library crates, not the bench harness that
//! measures them.

#[path = "model.rs"]
mod model;

use mockspace_bench_core::Routine;

/// 256 operand pairs is large enough to amortise the FFI call boundary against the
/// per-add cost (a single add is a handful of ns; the harness's counter-quantisation
/// floor is 2048 ticks, `mockspace_bench_core::CALIBRATION_FLOOR_TICKS`) and small
/// enough that the array stays well inside L1 for both variants, so what the bench
/// measures is the add path itself, not a memory-bandwidth artefact.
pub const N: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct AddPairs {
    pub a: [f32; N],
    pub b: [f32; N],
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Sums {
    pub s: [f32; N],
}

impl Default for AddPairs {
    fn default() -> Self {
        AddPairs {
            a: [0.0; N],
            b: [0.0; N],
        }
    }
}
impl Default for Sums {
    fn default() -> Self {
        Sums { s: [0.0; N] }
    }
}

/// splitmix64. Deterministic per seed, no external RNG dependency (the bench harness's
/// `no_std`-by-default posture keeps this crate free of a `rand`-family dependency; the
/// harness only feeds one `u64` seed per `build_input` call, which is exactly what
/// splitmix64 wants).
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

/// Builds one operand, uniformly signed, with magnitude drawn from the subnormal range
/// `[2^-149, 2^-126)` (a 1-in-256-of-the-exponent-field-wide band; f32's actual subnormal
/// share of exponent codes) when `subnormal` is true, and from a normal-range band
/// `[2^-8, 2^8)` otherwise. Both operands of a pair are drawn the same way, so `a + b`
/// stays inside the subnormal grid when `subnormal` is true (an add mixing a subnormal
/// and a far-larger normal operand would just return the normal operand unrounded, which
/// tests nothing about the quantiser's subnormal path).
fn one_operand(rng: &mut SplitMix64, subnormal: bool) -> f32 {
    let bits = rng.next();
    let neg = (bits & 1) != 0;
    if subnormal {
        // exponent field 0, mantissa nonzero: the whole subnormal range.
        let mantissa = ((bits >> 1) as u32) & 0x7f_ffff;
        let mantissa = if mantissa == 0 { 1 } else { mantissa };
        f32::from_bits(((neg as u32) << 31) | mantissa)
    } else {
        // exponent field in [119, 134] (2^-8 .. 2^8), mantissa arbitrary: a normal band
        // wide enough that additions land on varied grid positions without ever
        // overflowing to infinity or underflowing to the subnormal range themselves.
        let exp = 119 + ((bits >> 1) % 16) as u32;
        let mantissa = ((bits >> 5) as u32) & 0x7f_ffff;
        f32::from_bits(((neg as u32) << 31) | (exp << 23) | mantissa)
    }
}

/// `AddSweep<PCT>`: `Routine` whose input is `N` operand pairs, `PCT` percent of which
/// (by pair, both operands drawn subnormal together) are subnormal. `PCT` is a
/// compile-time dispatch parameter (matches `sizes` in `bench.toml`) rather than a
/// runtime knob, per the harness's own per-N monomorphisation shape (`Fiedler<N>`,
/// `Rcm<N>` in the sibling variant crates use the same pattern for their own size axis).
pub struct AddSweep<const PCT: usize>;

impl<const PCT: usize> Routine for AddSweep<PCT> {
    type Input = AddPairs;
    type Output = Sums;

    fn build_input(seed: u64) -> Self::Input {
        let mut rng = SplitMix64(seed ^ 0xD1B5_4A32_D192_ED03);
        let mut a = [0.0f32; N];
        let mut b = [0.0f32; N];
        for i in 0..N {
            // deterministic per-index threshold rather than a fresh random draw per
            // pair, so PCT% of the N pairs are subnormal exactly, not approximately.
            let subnormal = i < (N * PCT) / 100;
            a[i] = one_operand(&mut rng, subnormal);
            b[i] = one_operand(&mut rng, subnormal);
        }
        AddPairs { a, b }
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        for i in 0..N {
            if output.s[i].is_nan() {
                return Err("unexpected NaN in a finite-input sum");
            }
            let expect_finite_bound = input.a[i].abs() + input.b[i].abs() + 1.0;
            if output.s[i].is_finite() && output.s[i].abs() > expect_finite_bound {
                return Err("sum exceeds a generous finite bound; likely a decode/encode defect");
            }
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }
}

/// The software quantiser add: decode both operands, form the exact rational sum, run
/// the design's own round-first quantiser (`model::quantize`, `Dir::Nearest`), encode
/// back. Faithful to file 50's validated model; NaN/Inf inputs fall back to native `+`
/// because this bench's subject is the quantiser's rounding path, not its special-value
/// path (file 50 section 4.4 already covers specials, compiled separately), and
/// `build_input` above never generates NaN or Inf operands.
#[inline(never)]
pub fn software_add(a: f32, b: f32) -> f32 {
    use model::{decode_f32, encode_f32, exact_add, quantize, Dir, F32Val, BINARY32};
    let (da, db) = match (decode_f32(a), decode_f32(b)) {
        (F32Val::Fin(da), F32Val::Fin(db)) => (da, db),
        _ => return a + b,
    };
    let r = exact_add(da, db);
    let g = quantize(&BINARY32, &r, Dir::Nearest);
    f32::from_bits(encode_f32(g.out))
}

/// The hardware add: the native `fadd` instruction, no software quantisation. Split into
/// its own function (rather than inlined `a + b` at the call site) so both variants can
/// be `#[inline(never)]`-pinned the same way and the comparison is apples to apples in
/// the asm, matching file 50's own measured shape (`f_add: fadd s0, s0, s1`, `50:442`).
#[inline(never)]
pub fn hardware_add(a: f32, b: f32) -> f32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cross-checks `software_add` against native `+` bit-for-bit, over the
    /// exact input distribution `AddSweep<PCT>::build_input` generates for
    /// every PCT this bench sweeps and 64 distinct seeds each, matching file
    /// 50's own zero-mismatch methodology (`50_fog_the_float_model.md`
    /// section 2) applied to this bench's own reference model rather than
    /// assumed to transfer from it. Reuses `AddSweep::build_input` directly
    /// (one transform, one generation path) instead of re-deriving the
    /// subnormal-fraction distribution a second time in the test.
    fn check_pct<const PCT: usize>(
        checked: &mut u64,
        mismatches: &mut std::vec::Vec<(f32, f32, u32, u32)>,
    ) {
        for seed in 0u64..64 {
            let input = <AddSweep<PCT> as Routine>::build_input(seed);
            for i in 0..N {
                let (a, b) = (input.a[i], input.b[i]);
                let sw = software_add(a, b);
                let hw = hardware_add(a, b);
                *checked += 1;
                if sw.to_bits() != hw.to_bits() && !(sw.is_nan() && hw.is_nan()) {
                    mismatches.push((a, b, sw.to_bits(), hw.to_bits()));
                }
            }
        }
    }

    #[test]
    fn software_add_matches_hardware_bit_exact() {
        let mut checked = 0u64;
        let mut mismatches: std::vec::Vec<(f32, f32, u32, u32)> = std::vec::Vec::new();
        check_pct::<0>(&mut checked, &mut mismatches);
        check_pct::<10>(&mut checked, &mut mismatches);
        check_pct::<25>(&mut checked, &mut mismatches);
        check_pct::<50>(&mut checked, &mut mismatches);
        check_pct::<75>(&mut checked, &mut mismatches);
        check_pct::<100>(&mut checked, &mut mismatches);
        assert!(
            mismatches.is_empty(),
            "{} mismatches out of {checked} checked; first: {:?}",
            mismatches.len(),
            mismatches.first()
        );
        assert_eq!(checked, 6 * 64 * N as u64);
    }
}
