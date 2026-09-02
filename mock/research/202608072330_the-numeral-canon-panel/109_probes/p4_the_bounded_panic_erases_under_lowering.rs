//! P4. Does I18's bounded panic survive I15's "never any runtime checks"?
//!
//! I15: "Never any runtime checks, ever. We catch invalids on compile time,
//! and unused paths we clear out when lowered." I18: a native-primitive-style
//! overflow panic is permitted, bounded by build (dev and debug only) and by
//! concern (where imitating the native primitive is the point, not where cost
//! is).
//!
//! Read as claims about a shipped artifact, those two are compatible exactly
//! when the panic PATH IS GONE from a release build. If a release binary
//! still contains the compare and the call to the panic handler, I18 is a
//! hole in I15 rather than a bounded exception inside it, and saying
//! otherwise is a claim about intent that the object file refutes.
//!
//! Two gates, and the probe checks both because they are different
//! mechanisms:
//!
//!   G1. The BUILD gate: `cfg!(debug_assertions)`, a const the compiler knows.
//!   G2. The CONCERN gate: a const on the strategy, so one concern carries the
//!       check and another does not, in the same binary, at the same
//!       optimisation level.
//!
//! G2 is the one worth checking. G1 is well-trodden. G2 asks whether a
//! per-type const erases as cleanly as a build flag, which is what makes I18's
//! second bound implementable at all.
//!
//! Build: rustc --edition 2021 -O --crate-type=lib --emit asm
//!        p4_the_bounded_panic_erases_under_lowering.rs

#![allow(dead_code)]

/// The concern a strategy stands for, as far as this probe needs it. I18's
/// marker names are not load-bearing and op says so; these name concerns.
trait Concern {
    /// True where imitating the native primitive is the point.
    const IMITATES_NATIVE: bool;
}

/// The imitate-the-native-primitive concern.
struct Imitating;
impl Concern for Imitating {
    const IMITATES_NATIVE: bool = true;
}

/// The speed-first concern. A path chosen for cost does not carry a check
/// that exists for familiarity.
struct SpeedFirst;
impl Concern for SpeedFirst {
    const IMITATES_NATIVE: bool = false;
}

/// The addition. One body, two gates, no duplication of the arithmetic.
#[inline(always)]
fn add<C: Concern>(a: u8, b: u8) -> u8 {
    let (v, overflowed) = a.overflowing_add(b);
    if cfg!(debug_assertions) && C::IMITATES_NATIVE && overflowed {
        panic!("attempt to add with overflow");
    }
    v
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn add_imitating(a: u8, b: u8) -> u8 {
    add::<Imitating>(a, b)
}

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn add_speed_first(a: u8, b: u8) -> u8 {
    add::<SpeedFirst>(a, b)
}

/// The reference: what a bare machine add lowers to with no gate at all.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn add_bare(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

/// A control, which is the arm that makes this a comparison rather than a
/// demonstration: the same check with NO const gate on it, so it cannot be
/// erased and must appear in every build. If this one also came out clean,
/// the probe would be measuring the optimiser deleting dead arithmetic rather
/// than the gate doing its job.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn add_ungated_check(a: u8, b: u8) -> u8 {
    let (v, overflowed) = a.overflowing_add(b);
    if overflowed {
        panic!("attempt to add with overflow");
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three gated paths agree with the bare one wherever no overflow
    /// occurs, which is the region a correct program stays in. Exhaustive.
    #[test]
    fn the_gated_paths_agree_where_no_overflow_occurs() {
        let mut checked = 0u32;
        for a in 0u16..=255 {
            for b in 0u16..=255 {
                if a + b > 255 {
                    continue;
                }
                let (a, b) = (a as u8, b as u8);
                assert_eq!(add_imitating(a, b), add_bare(a, b));
                assert_eq!(add_speed_first(a, b), add_bare(a, b));
                assert_eq!(add_ungated_check(a, b), add_bare(a, b));
                checked += 1;
            }
        }
        // 256*257/2 pairs sum to at most 255.
        assert_eq!(checked, 32_896);
    }

    /// The speed-first path does not panic on overflow in ANY build, which is
    /// I18's concern bound. The imitating path's behaviour is
    /// build-dependent, so it is not asserted here; the assembly is where
    /// that is read.
    #[test]
    fn the_speed_first_concern_never_panics() {
        assert_eq!(add_speed_first(200, 200), 144); // 400 mod 256
        assert_eq!(add_speed_first(255, 255), 254);
        assert_eq!(add_bare(200, 200), 144);
    }

    /// The concern consts are actually different, so the gate has something
    /// to gate on. Without this the two paths could be identical for a reason
    /// that has nothing to do with the mechanism.
    #[test]
    fn the_two_concerns_differ() {
        assert!(<Imitating as Concern>::IMITATES_NATIVE);
        assert!(!<SpeedFirst as Concern>::IMITATES_NATIVE);
    }
}
