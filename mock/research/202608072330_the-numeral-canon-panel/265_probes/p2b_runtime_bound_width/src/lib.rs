//! p2b: the control for p2a. Same shape, the count read from a runtime function.
//!
//! If the line between a format and storage is const-availability, this must
//! refuse, and it must refuse at the one place the count is read.
#![no_std]

use arvo_format::width::Width;

static mut CONFIGURED: u32 = 32;

/// A count that only exists at runtime.
fn read_width() -> u32 {
    // SAFETY: a probe with no threads; the read is the point.
    unsafe { CONFIGURED }
}

/// The width, bound to a runtime datum.
pub const W: Width = Width::bits(read_width());
