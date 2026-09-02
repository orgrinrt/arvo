//! The candidate, compiled: can a rendering reach the open inventory without
//! amending the ratified `Format` spine, and without a bare primitive at any
//! public position?
//!
//! `p01` established that `impl<F: Format> Debug for F` is refused with `E0210`,
//! so `core::fmt::Debug` cannot be blanket-supplied over the inventory. Its
//! control established that the same blanket over a *local* trait is accepted.
//! This builds that: a local trait, blanket-implemented, rendering the identity
//! `ruling::the_format_spine_is_canon` names, which is the ambient domain and the
//! representable set, both constants of the type.
//!
//! What it is testing, one thing at a time:
//!
//! - that the identity is reachable from `F: Format` alone, so nothing keys on
//!   which point of the inventory it holds;
//! - that the sink is `core::fmt::Write`, so no length, count or capacity ever
//!   crosses a public position and `question::what_a_platform_width_type_is`
//!   is not touched;
//! - that two declarations differing only in a const parameter render
//!   differently, which is the discrimination `#[derive(Debug)]` cannot supply
//!   on a const-generic unit struct;
//! - that a format this crate does not know about renders through the same
//!   blanket, which is what `the_concept_is_closed_and_the_inventory_is_open`
//!   requires of anything claiming to reach every numeral.
//!
//! This is a spike. It checks the four things above and nothing else is designed.

#![no_std]

use arvo_format::slots::declared_slot_width;
use arvo_format::{radix, slot_count, smallest_step_exponent, Format};
use core::fmt::{Result, Write};

/// A rendering of what identifies a format.
///
/// Generic over the sink rather than taking `&mut dyn Write`, because
/// monomorphisation is the dispatch and `dyn` is out. Generic over the sink also
/// means the caller owns the buffer and its size, so no count of bytes appears
/// anywhere in this signature.
pub trait RenderIdentity {
    /// Write the identity into a caller's sink.
    ///
    /// # Errors
    /// Whatever the sink returns. The sink owns its capacity and its refusal.
    fn render_identity<W: Write>(w: &mut W) -> Result;
}

impl<F: Format> RenderIdentity for F {
    fn render_identity<W: Write>(w: &mut W) -> Result {
        // Every one of these is a constant of the type, per the ratified
        // identity clause. None of them reads a value, because there is no value.
        let base = radix::<F>().base();
        let slots = slot_count::<F::Slots>().count();
        let width = declared_slot_width::<F::Slots>().count();
        let step = smallest_step_exponent::<F>().power();
        let phase_num = F::PHASE.numerator();
        let phase_den = F::PHASE.denominator();

        write!(
            w,
            "radix {base}, {slots} slots over {width} bits, step 2^{step}, phase {phase_num}/{phase_den}"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arvo_format::ambient::BinaryRationals;
    use arvo_format::format::Phase;
    use arvo_format::points::{Integer, UFixed};
    use arvo_format::quantum::Constant;
    use arvo_format::slots::Signed;

    /// A sink with no allocation anywhere, standing in for whatever a caller
    /// brings. It is the caller's, which is the point: arvo supplies none.
    struct Sink<const N: usize> {
        bytes: [u8; N],
        used: usize,
    }

    impl<const N: usize> Sink<N> {
        const fn new() -> Self {
            Self { bytes: [0; N], used: 0 }
        }
        fn seen(&self) -> &str {
            core::str::from_utf8(&self.bytes[..self.used]).expect("ascii only")
        }
    }

    impl<const N: usize> Write for Sink<N> {
        fn write_str(&mut self, s: &str) -> Result {
            let b = s.as_bytes();
            if self.used + b.len() > N {
                return Err(core::fmt::Error);
            }
            self.bytes[self.used..self.used + b.len()].copy_from_slice(b);
            self.used += b.len();
            Ok(())
        }
    }

    fn rendered<F: Format, const N: usize>(buf: &mut Sink<N>) -> &str {
        F::render_identity(buf).expect("fits");
        buf.seen()
    }

    /// The blanket reaches a point of the shipped inventory at all.
    #[test]
    fn a_shipped_point_renders_its_identity() {
        let mut b = Sink::<128>::new();
        assert_eq!(
            rendered::<Integer<8>, 128>(&mut b),
            "radix 2, 256 slots over 8 bits, step 2^0, phase 0/1"
        );
    }

    /// The discrimination `#[derive(Debug)]` cannot supply: two declarations
    /// differing only in a const parameter are different formats, because their
    /// representable sets differ, and the identity clause says the set is what
    /// identifies them.
    #[test]
    fn two_widths_of_one_family_render_differently() {
        let mut a = Sink::<128>::new();
        let mut c = Sink::<128>::new();
        let x = rendered::<Integer<8>, 128>(&mut a);
        let y = rendered::<Integer<32>, 128>(&mut c);
        assert_ne!(x, y, "Integer<8> and Integer<32> are different formats");
    }

    /// Two families at one width are different formats too, so the rendering
    /// must not collapse to the width.
    #[test]
    fn two_families_at_one_width_render_differently() {
        let mut a = Sink::<128>::new();
        let mut c = Sink::<128>::new();
        let x = rendered::<Integer<8>, 128>(&mut a);
        let y = rendered::<UFixed<8, -4>, 128>(&mut c);
        assert_ne!(x, y, "Integer<8> and UFixed<8,-4> are different formats");
    }

    /// The open inventory. A format declared here, which `arvo-format` does not
    /// know about, reaches the same blanket with no edit anywhere.
    struct AFormatArvoDoesNotKnowAbout;

    impl Format for AFormatArvoDoesNotKnowAbout {
        type Ambient = BinaryRationals;
        type Quantum = Constant<-3>;
        type Slots = Signed<11>;
        const PHASE: Phase = Phase::halves(1);
    }

    #[test]
    fn a_format_this_crate_does_not_know_about_renders() {
        let mut b = Sink::<128>::new();
        let s = rendered::<AFormatArvoDoesNotKnowAbout, 128>(&mut b);
        assert!(s.starts_with("radix 2, 2048 slots over 11 bits"), "got {s}");
    }

    /// The control on the sink. Without it every assertion above would pass
    /// against a sink that dropped writes and reported an empty string.
    #[test]
    fn the_control_a_sink_too_small_refuses() {
        let mut b = Sink::<4>::new();
        assert!(Integer::<8>::render_identity(&mut b).is_err());
    }

    /// The control on the renderer. Without it the two `assert_ne!` arms would
    /// pass against a renderer emitting a nonce.
    #[test]
    fn the_control_one_declaration_renders_the_same_twice() {
        let mut a = Sink::<128>::new();
        let mut c = Sink::<128>::new();
        assert_eq!(
            rendered::<Integer<8>, 128>(&mut a),
            rendered::<Integer<8>, 128>(&mut c)
        );
    }
}
