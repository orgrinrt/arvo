//! `Cap` → `usize` projection for nightly `generic_const_exprs`.

use arvo::{Cap, USize};

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
// generic_const_exprs (rustc #76560) is WATCH-tier in the stack soundness
// sweep (task #626). It is sound for this usage: the one known unsoundness
// (rustc #97156, const evaluation of type-identity reflection into types with
// higher-ranked-trait-bound subtyping) is unreachable, because the stack never
// reflects on type identity at runtime. This fn is the canonical GCE use-site
// for the array-length family; every `[(); cap_size(N)]:` bound resolves
// through it. Migration to generic_const_args is tracked: task #628.
#[inline(always)]
pub const fn cap_size(c: Cap) -> usize { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: nightly generic_const_exprs requires raw usize in const-generic array-length position (language grammar constraint); tracked: #121
    c.0.0
}

/// Build a `Cap` from a `usize`, the inverse of `cap_size`.
///
/// A literal `Cap(USize(n))` construction is rejected in const-generic
/// position ("struct/enum construction is not supported in generic
/// constants"); a named `const fn` is the accepted form, the same
/// constraint that forces `cap_size` to exist for the other direction.
/// `cap` is the bridge a consumer uses to form a `Cap`-parameterised
/// type (such as `Csr<{cap(N)}, ..>`) from a `usize` const-param.
#[inline(always)]
pub const fn cap(n: usize) -> Cap { // lint:allow(arvo-types-only) lint:allow(no-bare-numeric) reason: definition-site usize to Cap bridge boundary, inverse of cap_size; tracked: #121
    Cap(USize(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_roundtrips_with_cap_size() {
        for n in [0usize, 1, 7, 64, 1024] { // lint:allow(no-bare-numeric) reason: test fixture index values; tracked: #121
            assert_eq!(cap_size(cap(n)), n);
            assert_eq!(cap(n), Cap(USize(n)));
        }
    }
}
