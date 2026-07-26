//! Const-generic scale helpers for the fixed-point multiply.
//!
//! `IFixed` / `UFixed` `*` is a fixed-point multiply: the per-strategy `X_mul_fixed::<{ FRAC }>` shifts the
//! widened product right by the fractional bit count. The shift `FRAC` is the type's `F`, statically known,
//! so it is forwarded as a const-generic turbofish argument. This module holds the helpers that make that
//! forwarding type-check under `generic_const_exprs`.

use arvo_storage::FBits;

/// The fractional bit count of a fixed-point type, as the shift amount for its fixed-point multiply
/// (`X_mul_fixed::<{ frac(F) }>`). A free const fn (like `ifixed_bits`) so the value reads as a constrained
/// const expression in a const-generic turbofish: the inherent `FBits::raw()` method is flagged
/// "unconstrained generic constant" in that position, whereas a free-fn call is accepted, the same way
/// `ifixed_bits(I, F)` is accepted as a const-generic argument throughout.
pub(crate) const fn frac(f: FBits) -> u16 {
    // lint:allow(no-bare-numeric) reason: const-generic shift-amount carrier mirroring ifixed_bits; tracked: #256
    f.raw()
}

/// Constrains a generic-dependent const shift so it can be passed as a method turbofish argument
/// (`X_mul_fixed::<{ frac(F) }>`) under `generic_const_exprs`. A const expression in a *type* position
/// (`Bits<{ ifixed_bits(I, F) }, ..>`) is accepted, but the same kind of expression in a *method* turbofish
/// is flagged "unconstrained generic constant"; surfacing it through this trait bound (`(): FracShift<{
/// frac(F) }>`) marks it constrained. The `[(); { frac(F) } as usize]:` array predicate is the usual
/// alternative but overflows well-formedness checking at every `*` use site, so the trait-bound form is
/// used. Trivially satisfied by the blanket impl.
pub(crate) trait FracShift<const FRAC: u16> {} // lint:allow(no-bare-numeric) reason: const-shift constraint marker; tracked: #256
impl<const FRAC: u16> FracShift<FRAC> for () {} // lint:allow(no-bare-numeric) reason: const-shift constraint marker; tracked: #256
