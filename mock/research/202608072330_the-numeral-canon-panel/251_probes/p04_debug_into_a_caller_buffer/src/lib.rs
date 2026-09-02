//! Is the ordinary route to debug output actually unavailable under no-alloc?
//!
//! `obligation::debug_output_from_every_numeral_shape` gives as its reason that
//! "the no-alloc constraint means the ordinary route is unavailable". This tests
//! that sentence rather than the need above it.
//!
//! The ordinary route is `core::fmt::Debug` rendered through a `core::fmt::Write`
//! sink the caller owns. `core::fmt` is in `core`, so it needs neither `std` nor
//! `alloc`, and a fixed array on the caller's stack is a legal sink. If that
//! works, the reason is false and the row's need has to rest on something else.

#![no_std]


use core::fmt::{self, Write};

/// A caller-supplied fixed-size buffer. No alloc, no std, no growth.
///
/// Overflow is reported rather than silently truncated, because a debug routine
/// that lies about having printed everything is worse than one that refuses.
pub struct Buffer<const N: usize> {
    bytes: [u8; N],
    used: usize,
    overflowed: bool,
}

impl<const N: usize> Buffer<N> {
    #[must_use]
    pub const fn new() -> Self {
        Self { bytes: [0; N], used: 0, overflowed: false }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        // Every write came through `Write::write_str`, so the prefix is valid utf8.
        core::str::from_utf8(&self.bytes[..self.used]).unwrap_or("")
    }

    #[must_use]
    pub const fn overflowed(&self) -> bool {
        self.overflowed
    }
}

impl<const N: usize> Default for Buffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Write for Buffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let b = s.as_bytes();
        if self.used + b.len() > N {
            self.overflowed = true;
            return Err(fmt::Error);
        }
        self.bytes[self.used..self.used + b.len()].copy_from_slice(b);
        self.used += b.len();
        Ok(())
    }
}

/// Render any `Debug` into a caller-supplied buffer. This is the whole of the
/// "ordinary route", and it is eight lines.
///
/// # Errors
/// Returns `fmt::Error` when the value does not fit in `N` bytes.
pub fn render<const N: usize, T: fmt::Debug>(v: &T, out: &mut Buffer<N>) -> fmt::Result {
    write!(out, "{v:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three value-carrying coordinate types arvo exports today, each
    /// rendered into a fixed buffer with no alloc anywhere.
    #[test]
    fn the_ordinary_route_works_under_no_alloc() {
        let mut b = Buffer::<64>::new();
        render(&arvo_format::Width::bits(13), &mut b).expect("Width fits in 64 bytes");
        assert_eq!(b.as_str(), "Width(13)");
        assert!(!b.overflowed());

        let mut b = Buffer::<64>::new();
        render(&arvo_format::Bool::of(true), &mut b).expect("Bool fits in 64 bytes");
        assert_eq!(b.as_str(), "Bool(true)");

        let mut b = Buffer::<64>::new();
        render(&arvo_format::Slot::at(-7), &mut b).expect("Slot fits in 64 bytes");
        assert_eq!(b.as_str(), "Slot(-7)");
    }

    /// The control on the buffer. A buffer too small must refuse and say so,
    /// otherwise the test above would pass against a sink that dropped writes.
    #[test]
    fn the_control_a_buffer_too_small_refuses_rather_than_truncating() {
        let mut b = Buffer::<4>::new();
        let r = render(&arvo_format::Width::bits(13), &mut b);
        assert!(r.is_err(), "a 9-byte rendering must not fit in 4 bytes");
        assert!(b.overflowed());
    }

    /// The control on the renderer. It must distinguish two different values,
    /// or `the_ordinary_route_works` would pass against a stub printing a
    /// constant.
    #[test]
    fn the_control_two_different_values_render_differently() {
        let mut a = Buffer::<64>::new();
        let mut c = Buffer::<64>::new();
        render(&arvo_format::Width::bits(13), &mut a).unwrap();
        render(&arvo_format::Width::bits(14), &mut c).unwrap();
        assert_ne!(a.as_str(), c.as_str());
    }

    /// What the route does not reach, stated as a test so it is not a claim in
    /// prose. The four shipped points are zero sized and derive nothing, so
    /// there is no value to render and `render` cannot be called on one.
    ///
    /// Asserted here as the size fact; the absence of the impl is a compile
    /// failure and lives in `the_point_has_no_debug.rs` beside this file.
    #[test]
    fn what_the_route_does_not_reach_is_the_points() {
        assert_eq!(core::mem::size_of::<arvo_format::points::Integer<32>>(), 0);
    }
}
