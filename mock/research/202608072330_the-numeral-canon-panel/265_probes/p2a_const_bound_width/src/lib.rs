//! p2a: the pointer width is a const-available fact of the compilation.
//!
//! Hypothesis: whatever the platform width depends on is resolved before
//! monomorphisation, so it is on the const side of the line the canon draws
//! between a format (representable set a constant of the type) and storage
//! (a value set depending on other data). Three spellings of the same fact are
//! asserted equal at compile time, and an arm gated on it is written as a
//! `const fn` so the lowering can be read for the absence of a branch.
#![no_std]

use arvo_format::width::Width;

/// The width, bound to the target through the host's own constant.
pub const W: Width = Width::bits(usize::BITS);

/// The same fact through conditional compilation, which is the other
/// const-available spelling the language offers.
#[cfg(target_pointer_width = "16")]
pub const W_CFG: Width = Width::bits(16);
#[cfg(target_pointer_width = "32")]
pub const W_CFG: Width = Width::bits(32);
#[cfg(target_pointer_width = "64")]
pub const W_CFG: Width = Width::bits(64);

/// And through the layout observation, which is the one the design does not own.
pub const W_LAYOUT: Width = Width::bits((core::mem::size_of::<usize>() * 8) as u32);

const _: () = assert!(
    W.equals(W_CFG).get(),
    "usize::BITS and cfg(target_pointer_width) agree"
);
const _: () = assert!(
    W.equals(W_LAYOUT).get(),
    "usize::BITS and size_of::<usize>() agree"
);

/// An arm selected by the platform width. A const predicate in the sense of the
/// ratified arms ruling: one region per width, nothing else.
#[must_use]
pub const fn arm() -> u32 {
    match usize::BITS {
        16 => 1,
        32 => 2,
        64 => 3,
        _ => 0,
    }
}

/// The runtime entry, so the emitted code for `arm` can be read.
#[unsafe(no_mangle)]
pub extern "C" fn observe_arm() -> u32 {
    arm()
}
