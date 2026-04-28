//! `Cap` → `usize` projection for nightly `generic_const_exprs`.

use arvo::Cap;

/// Unwrap `Cap` to `usize` for array sizing in
/// `generic_const_exprs` contexts.
///
/// Rust arrays `[T; N]` require `N: usize` at the language grammar
/// level. `Cap` is a newtype over `USize` which is a newtype over
/// `usize`; nightly rejects the inline double-unwrap `N.0.0` in
/// const-generic position but accepts a named `const fn` that returns
/// the same value. Canonical home for the projection; arvo-bitmask,
/// arvo-spectral, arvo-comb, and future algorithm crates all call
/// through this one function.
#[inline(always)]
pub const fn cap_size(c: Cap) -> usize { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: nightly generic_const_exprs requires raw usize in const-generic array-length position (language grammar constraint); tracked: #121
    c.0.0
}
