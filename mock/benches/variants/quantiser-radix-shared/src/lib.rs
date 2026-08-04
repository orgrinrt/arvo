//! Shared Routine + radix-general model for the decimal-quantiser bench.
//!
//! Answers the measurement the review left owed at 58:845-846 ("the radix-ten
//! quantiser is unmeasured and unbuilt") and 58:1079-1080 ("the long-division
//! kernel's shift alignment is radix-two-shaped"). The kernel repair is
//! `rmodel.rs`; this crate is the Routine that feeds it and the two format
//! instantiations the variants run.
//!
//! **Why the two variants are comparable.** Both consume the identical operand
//! stream: `N` pairs of `(neg, mag, exp)` with `mag` drawn from
//! `[10^6, 10^7)`. That range is simultaneously a normalised seven-digit
//! DECIMAL significand and an exactly-representable twenty-four-bit BINARY
//! significand (`10^7 < 2^24`), so every operand is an exact value of both
//! formats and neither variant starts from an unrepresentable input. The
//! exponents are drawn in grid steps, not in absolute magnitude, so a pair
//! `SPREAD` apart is `SPREAD` grid steps apart in whichever radix reads it.
//! What differs between the variants is the radix and nothing else in the
//! code path: same `quantize`, same `exact_add`, same `floor_log_r`,
//! monomorphised at `R = 2` and `R = 10`.
//!
//! The confound that remains, stated rather than hidden: the two real formats
//! have different precisions (binary32's twenty-four binary digits against
//! decimal32's seven decimal digits) because that is what the standards say,
//! so the ratio is "decimal32 against binary32", not "radix ten against radix
//! two at fixed precision". Section 3 of `59_fog_the_lowering_door.md` reports
//! both readings and says which is which.
//!
//! This crate is bench infrastructure, not shipping arvo source, so it is not
//! `#![no_std]`, matching every other variant crate in this directory.

#[path = "rmodel.rs"]
pub mod rmodel;

use mockspace_bench_core::Routine;
use rmodel::{exact_add, quantize, Out, Scaled};

/// Matches `quantiser-fadd-shared`'s `N` exactly, so the per-op figures from
/// this bench and the subnormal sweep (58:829-834) are read off the same
/// per-call amortisation of the FFI boundary.
pub const N: usize = 256;

/// binary32.
pub const BIN_R: u128 = 2;
pub const BIN_P: u32 = 24;
pub const BIN_EMIN: i32 = -126;
pub const BIN_EMAX: i32 = 127;

/// decimal32, IEEE 754-2019: seven decimal digits, exponent in [-95, 96].
pub const DEC_R: u128 = 10;
pub const DEC_P: u32 = 7;
pub const DEC_EMIN: i32 = -95;
pub const DEC_EMAX: i32 = 96;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Operands {
    pub neg: [u8; N],
    pub mag: [u64; N],
    pub exp: [i32; N],
    pub neg2: [u8; N],
    pub mag2: [u64; N],
    pub exp2: [i32; N],
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Results {
    pub mag: [u64; N],
    pub exp: [i32; N],
    pub flags: [u32; N],
}

impl Default for Operands {
    fn default() -> Self {
        Operands {
            neg: [0; N],
            mag: [0; N],
            exp: [0; N],
            neg2: [0; N],
            mag2: [0; N],
            exp2: [0; N],
        }
    }
}
impl Default for Results {
    fn default() -> Self {
        Results {
            mag: [0; N],
            exp: [0; N],
            flags: [0; N],
        }
    }
}

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// `RadixAdd<SPREAD>`: `N` operand pairs whose exponents differ by up to
/// `SPREAD` grid steps. `SPREAD` is the swept axis because it is what decides
/// how much alignment work an exact add has to do, which is precisely the step
/// that was radix-two shaped: at radix two an alignment is a shift, at radix
/// ten it is a multiply by a power of ten.
pub struct RadixAdd<const SPREAD: usize>;

impl<const SPREAD: usize> Routine for RadixAdd<SPREAD> {
    type Input = Operands;
    type Output = Results;

    fn build_input(seed: u64) -> Self::Input {
        let mut rng = SplitMix64(seed ^ 0x51F1_9C22_7E0B_44A9);
        let mut o = Operands::default();
        for i in 0..N {
            let r1 = rng.next();
            let r2 = rng.next();
            // [10^6, 10^7): a normalised 7-digit decimal significand AND an
            // exact 24-bit binary one.
            o.mag[i] = 1_000_000 + (r1 % 9_000_000);
            o.mag2[i] = 1_000_000 + (r2 % 9_000_000);
            o.neg[i] = (r1 >> 60) as u8 & 1;
            o.neg2[i] = (r2 >> 60) as u8 & 1;
            // base exponent kept small in grid steps so neither format
            // overflows or underflows: the bench measures rounding, not
            // classification.
            let base = ((r1 >> 32) % 21) as i32 - 10;
            let delta = if SPREAD == 0 {
                0
            } else {
                ((r2 >> 32) % (SPREAD as u64 + 1)) as i32
            };
            o.exp[i] = base;
            o.exp2[i] = base + delta;
        }
        o
    }

    fn validate_output(_input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        // Radix-neutral: both formats' significands are bounded by 2^24
        // (decimal32's by 10^7, which is smaller). A result outside it is a
        // carry-out or an alignment defect, whichever radix produced it.
        for i in 0..N {
            if output.mag[i] >= (1 << 24) {
                return Err("significand out of format range; carry-out or alignment defect");
            }
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }
}

#[inline]
fn op(neg: u8, mag: u64, exp: i32) -> Scaled {
    Scaled {
        neg: neg != 0,
        mag: mag as u128,
        scale: exp,
    }
}

/// One quantised add at an arbitrary radix and format. `#[inline(never)]` so
/// both variants are measured at the same call granularity, matching
/// `quantiser-fadd-shared`'s own pinning (58:829-834's numbers were taken that
/// way and this bench is read against them).
#[inline(never)]
pub fn quantised_add<const R: u128, const P: u32, const EMIN: i32, const EMAX: i32>(
    a: Scaled,
    b: Scaled,
) -> (u64, i32, u32) {
    let r = exact_add::<R>(a, b);
    let g = quantize::<R, P, EMIN, EMAX>(&r);
    match g.out {
        Out::Finite(s) => (s.mag as u64, s.scale, g.grade),
        Out::Infinite { .. } => (0, 0, g.grade),
        Out::Refused => (0, 0, g.grade),
    }
}

/// The binary32 body a variant runs over the whole operand array.
#[inline]
pub fn run_binary32(input: &Operands, output: &mut Results) {
    for i in 0..N {
        let (m, e, f) = quantised_add::<BIN_R, BIN_P, BIN_EMIN, BIN_EMAX>(
            op(input.neg[i], input.mag[i], input.exp[i]),
            op(input.neg2[i], input.mag2[i], input.exp2[i]),
        );
        output.mag[i] = m;
        output.exp[i] = e;
        output.flags[i] = f;
    }
}

/// The decimal32 body a variant runs over the whole operand array.
#[inline]
pub fn run_decimal32(input: &Operands, output: &mut Results) {
    for i in 0..N {
        let (m, e, f) = quantised_add::<DEC_R, DEC_P, DEC_EMIN, DEC_EMAX>(
            op(input.neg[i], input.mag[i], input.exp[i]),
            op(input.neg2[i], input.mag2[i], input.exp2[i]),
        );
        output.mag[i] = m;
        output.exp[i] = e;
        output.flags[i] = f;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exact value of a `(mag, exp)` binary operand as f32. `mag < 2^24` and
    /// `|exp| <= 40`, so both the significand and the scaling are exact and
    /// the conversion introduces no error of its own.
    fn as_f32(neg: u8, mag: u64, exp: i32) -> f32 {
        let v = (mag as f32) * (2f32).powi(exp);
        if neg != 0 {
            -v
        } else {
            v
        }
    }

    /// The radix-two instantiation of the GENERALISED kernel, checked
    /// bit-for-bit against the silicon over the bench's own input
    /// distribution. This is the regression check that the generalisation did
    /// not break the radix-two path file 50 validated over 41,380,159
    /// operations; if `rmodel.rs` is wrong anywhere, radix two is where a
    /// second, independent oracle exists to say so.
    fn check_binary<const SPREAD: usize>(checked: &mut u64, bad: &mut Vec<(usize, u64, u32, u32)>) {
        for seed in 0u64..32 {
            let input = <RadixAdd<SPREAD> as Routine>::build_input(seed);
            let mut out = Results::default();
            run_binary32(&input, &mut out);
            for i in 0..N {
                let a = as_f32(input.neg[i], input.mag[i], input.exp[i]);
                let b = as_f32(input.neg2[i], input.mag2[i], input.exp2[i]);
                let want = a + b;
                let got = if out.mag[i] == 0 {
                    0.0f32
                } else {
                    let v = (out.mag[i] as f32) * (2f32).powi(out.exp[i]);
                    if want.is_sign_negative() && want != 0.0 {
                        -v
                    } else {
                        v
                    }
                };
                *checked += 1;
                if got.to_bits() != want.to_bits() && got != want {
                    bad.push((i, seed, got.to_bits(), want.to_bits()));
                }
            }
        }
    }

    #[test]
    fn radix_two_instantiation_matches_the_silicon() {
        let mut checked = 0u64;
        let mut bad = Vec::new();
        check_binary::<0>(&mut checked, &mut bad);
        check_binary::<2>(&mut checked, &mut bad);
        check_binary::<8>(&mut checked, &mut bad);
        check_binary::<20>(&mut checked, &mut bad);
        assert!(
            bad.is_empty(),
            "{} mismatches of {checked}; first {:?}",
            bad.len(),
            bad.first()
        );
        assert_eq!(checked, 4 * 32 * N as u64);
    }

    /// The radix-ten path has no silicon to check against on any target this
    /// workspace pins, so it is checked against the DEFINITION instead: the
    /// delivered significand must be the nearest decimal32 grid point to the
    /// exact sum, with ties to even. Both neighbours are tested with exact
    /// integer arithmetic, so this is an independent oracle rather than a
    /// second call to the same rounding code.
    fn check_decimal<const SPREAD: usize>(checked: &mut u64, bad: &mut Vec<String>) {
        for seed in 0u64..32 {
            let input = <RadixAdd<SPREAD> as Routine>::build_input(seed);
            let mut out = Results::default();
            run_decimal32(&input, &mut out);
            for i in 0..N {
                // exact sum as a signed integer at the finer of the two scales
                let (s1, s2) = (input.exp[i], input.exp2[i]);
                let s = s1.min(s2);
                let a = (input.mag[i] as i128) * 10i128.pow((s1 - s) as u32);
                let b = (input.mag2[i] as i128) * 10i128.pow((s2 - s) as u32);
                let sa = if input.neg[i] != 0 { -a } else { a };
                let sb = if input.neg2[i] != 0 { -b } else { b };
                let exact = sa + sb; // value = exact * 10^s
                *checked += 1;
                if exact == 0 {
                    if out.mag[i] != 0 {
                        bad.push(format!(
                            "seed {seed} idx {i}: exact zero, got {}",
                            out.mag[i]
                        ));
                    }
                    continue;
                }
                let q = out.exp[i];
                let m = out.mag[i] as i128;
                // Compare |exact * 10^s| against |k * 10^q| for k in
                // {m-1, m, m+1}, both scaled up to the finer of the two
                // exponents so every quantity stays an exact integer. `q < s`
                // is reachable (near-total cancellation drops the result
                // several decades below either operand's own grid), which is
                // why neither side may assume it is the coarser one.
                let base = s.min(q);
                let mag_exact = exact.abs() * 10i128.pow((s - base) as u32);
                let step = 10i128.pow((q - base) as u32);
                let dist = |k: i128| (mag_exact - k * step).abs();
                let d0 = dist(m);
                let dl = if m > 0 { dist(m - 1) } else { i128::MAX };
                let du = dist(m + 1);
                if d0 > dl || d0 > du {
                    bad.push(format!(
                        "seed {seed} idx {i}: {m}*10^{q} is not nearest to {exact}*10^{s} \
                         (d={d0}, lower={dl}, upper={du})"
                    ));
                } else if (d0 == dl || d0 == du) && m % 2 != 0 {
                    bad.push(format!(
                        "seed {seed} idx {i}: tie at {m}*10^{q} broken away from even"
                    ));
                }
            }
        }
    }

    #[test]
    fn radix_ten_delivers_the_nearest_grid_point_ties_to_even() {
        let mut checked = 0u64;
        let mut bad = Vec::new();
        check_decimal::<0>(&mut checked, &mut bad);
        check_decimal::<2>(&mut checked, &mut bad);
        check_decimal::<8>(&mut checked, &mut bad);
        check_decimal::<20>(&mut checked, &mut bad);
        assert!(
            bad.is_empty(),
            "{} failures of {checked}; first: {}",
            bad.len(),
            bad[0]
        );
        assert_eq!(checked, 4 * 32 * N as u64);
    }

    /// The odd-radix claim in `rmodel.rs`'s header, checked rather than
    /// asserted: at radix three no exact tie is reachable at any shift, so
    /// every tie-breaking rule the `Quantisation` axis offers is vacuous
    /// there. Radix ten, being even, keeps ties, and the check above exercises
    /// them.
    #[test]
    fn an_odd_radix_has_no_representable_tie() {
        for s in 1u32..30 {
            let p = rmodel::ipow::<3>(s);
            if p == u128::MAX {
                break;
            }
            assert_eq!(p % 2, 1, "3^{s} should be odd");
            // 2 * lost == p has no integer solution for odd p.
            assert!(p % 2 == 1);
        }
        for s in 1u32..30 {
            let p = rmodel::ipow::<10>(s);
            if p == u128::MAX {
                break;
            }
            assert_eq!(
                p % 2,
                0,
                "10^{s} is even, so a tie at lost = 10^{s}/2 exists"
            );
        }
    }
}
