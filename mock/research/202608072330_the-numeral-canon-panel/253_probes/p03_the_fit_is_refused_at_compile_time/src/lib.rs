//! Can a buffer too small to hold a rendering be refused at compile time?
//!
//! `p02` renders into a `core::fmt::Write` sink, which reports a shortfall by
//! returning `Err` at runtime. The operating constraints say invalids are
//! refused at compile time and runtime validation does not exist, so the sink
//! arm concedes something. This asks whether the concession is necessary.
//!
//! Everything a declaration renders is a constant of the type, per the ratified
//! identity clause, so the rendered length is a constant too. If it can be
//! computed in const context and compared against a caller's buffer size, the
//! shortfall is a build failure and there is no runtime path at all.
//!
//! The wall being tested for: a public `const MAX_LEN: usize` would put a bare
//! `usize` at a public API position, which I14 forbids and for which no
//! byte-count type exists. So the length is kept private and the caller's size
//! arrives as a const generic parameter, which is the one excepted position.

#![no_std]

use arvo_format::slots::declared_slot_width;
use arvo_format::{radix, slot_count, smallest_step_exponent, Format};

/// Decimal digits an `i64` needs, sign included. Const, and private.
const fn digits_i64(mut v: i64) -> usize {
    let mut n = if v < 0 { 1 } else { 0 };
    if v == 0 {
        return 1;
    }
    while v != 0 {
        n += 1;
        v /= 10;
    }
    n
}

const L0: &[u8] = b"radix ";
const L1: &[u8] = b", ";
const L2: &[u8] = b" slots over ";
const L3: &[u8] = b" bits, step 2^";
const L4: &[u8] = b", phase ";
const L5: &[u8] = b"/";

/// The exact rendered length for `F`, in bytes. Private on purpose: it is a
/// count of bytes, and no type in the stack means that.
///
/// The literals are summed from the same constants the writer emits, never from
/// a second copy of the string. A hand-typed measuring copy was the first version
/// and it was one space short, which the const gate caught at build time.
const fn rendered_len<F: Format>() -> usize {
    L0.len() + L1.len() + L2.len() + L3.len() + L4.len() + L5.len()
        + digits_i64(radix::<F>().base() as i64)
        + digits_i64(slot_count::<F::Slots>().count())
        + digits_i64(declared_slot_width::<F::Slots>().count() as i64)
        + digits_i64(smallest_step_exponent::<F>().power() as i64)
        + digits_i64(F::PHASE.numerator())
        + digits_i64(F::PHASE.denominator())
}

/// Write `F`'s identity into a caller-sized buffer, returning what was written.
///
/// The caller's size is a const generic, the one position the bare form is
/// excepted at. A buffer too small is a build failure and never a value, so
/// there is no error path and nothing to check at runtime.
pub const fn identity_of<F: Format, const N: usize>(buf: &mut [u8; N]) -> &mut [u8] {
    const { assert!(N >= rendered_len::<F>(), "the buffer is too small for this format's identity") };
    let mut at = 0;
    at = put(buf, at, L0);
    at = put_i64(buf, at, radix::<F>().base() as i64);
    at = put(buf, at, L1);
    at = put_i64(buf, at, slot_count::<F::Slots>().count());
    at = put(buf, at, L2);
    at = put_i64(buf, at, declared_slot_width::<F::Slots>().count() as i64);
    at = put(buf, at, L3);
    at = put_i64(buf, at, smallest_step_exponent::<F>().power() as i64);
    at = put(buf, at, L4);
    at = put_i64(buf, at, F::PHASE.numerator());
    at = put(buf, at, L5);
    at = put_i64(buf, at, F::PHASE.denominator());
    buf.split_at_mut(at).0
}

const fn put<const N: usize>(buf: &mut [u8; N], mut at: usize, s: &[u8]) -> usize {
    let mut i = 0;
    while i < s.len() {
        buf[at] = s[i];
        at += 1;
        i += 1;
    }
    at
}

const fn put_i64<const N: usize>(buf: &mut [u8; N], mut at: usize, v: i64) -> usize {
    if v < 0 {
        buf[at] = b'-';
        at += 1;
    }
    let mut mag = if v < 0 { -(v as i128) } else { v as i128 };
    let width = if v < 0 { digits_i64(v) - 1 } else { digits_i64(v) };
    let mut i = width;
    while i > 0 {
        i -= 1;
        buf[at + i] = b'0' + (mag % 10) as u8;
        mag /= 10;
    }
    at + width
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo_format::ambient::BinaryRationals;
    use arvo_format::format::Phase;
    use arvo_format::points::{Integer, UFixed};
    use arvo_format::quantum::Constant;
    use arvo_format::slots::Signed;

    fn seen<F: Format, const N: usize>(buf: &mut [u8; N]) -> &str {
        core::str::from_utf8(identity_of::<F, N>(buf)).expect("ascii only")
    }

    /// The whole rendering is a constant of the type, so it is available in a
    /// `const` item and costs nothing at runtime.
    #[test]
    fn the_rendering_is_available_at_const_time() {
        const LEN: usize = rendered_len::<Integer<8>>();
        const RENDERED: [u8; LEN] = {
            let mut b = [0u8; LEN];
            identity_of::<Integer<8>, LEN>(&mut b);
            b
        };
        assert_eq!(
            core::str::from_utf8(&RENDERED).unwrap(),
            "radix 2, 256 slots over 8 bits, step 2^0, phase 0/1"
        );
    }

    /// The length the const gate compares against is the exact length written,
    /// not an upper bound. If it were an over-estimate the gate would refuse
    /// buffers that in fact fit, and if an under-estimate it would admit ones
    /// that do not.
    #[test]
    fn the_computed_length_is_exactly_what_gets_written() {
        let mut b = [0u8; 128];
        let n = seen::<Integer<8>, 128>(&mut b).len();
        assert_eq!(n, rendered_len::<Integer<8>>());

        let mut b = [0u8; 128];
        let n = seen::<UFixed<8, -4>, 128>(&mut b).len();
        assert_eq!(n, rendered_len::<UFixed<8, -4>>());
    }

    /// A negative in the rendering, because `put_i64`'s sign handling is the
    /// place an off-by-one would hide and every shipped point has a zero phase.
    #[test]
    fn a_negative_exponent_and_a_nonzero_phase_render_and_measure() {
        struct Skewed;
        impl Format for Skewed {
            type Ambient = BinaryRationals;
            type Quantum = Constant<-3>;
            type Slots = Signed<11>;
            const PHASE: Phase = Phase::of(-7, 2);
        }
        let mut b = [0u8; 128];
        let s = seen::<Skewed, 128>(&mut b);
        assert_eq!(s, "radix 2, 2048 slots over 11 bits, step 2^-3, phase -7/2");
        assert_eq!(s.len(), rendered_len::<Skewed>());
    }

    /// The control on discrimination, as in `p02`: the gate and the writer must
    /// not collapse two different formats onto one rendering.
    #[test]
    fn the_control_two_formats_render_differently() {
        let mut a = [0u8; 128];
        let mut c = [0u8; 128];
        let x = seen::<Integer<8>, 128>(&mut a).len();
        let y = seen::<Integer<32>, 128>(&mut c).len();
        let mut a = [0u8; 128];
        let mut c = [0u8; 128];
        assert_ne!(seen::<Integer<8>, 128>(&mut a), seen::<Integer<32>, 128>(&mut c));
        assert_ne!(x, y, "the lengths differ too, so the gate discriminates");
    }

    /// The control on the gate: a buffer of exactly the needed size is accepted,
    /// so the refusal next door is about the shortfall and not about the gate
    /// refusing everything.
    #[test]
    fn the_control_a_buffer_of_exactly_the_needed_size_is_accepted() {
        const LEN: usize = rendered_len::<Integer<8>>();
        let mut b = [0u8; LEN];
        assert_eq!(seen::<Integer<8>, LEN>(&mut b).len(), LEN);
    }

    /// The refusal itself is a build failure and cannot be a runtime assertion,
    /// so it lives in `tests/ui/` and is asserted by `trybuild`.
    #[test]
    fn the_refusal_of_a_short_buffer_is_a_build_failure() {
        trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
    }
}
