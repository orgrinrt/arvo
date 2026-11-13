//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Whether a debug build aborts when a result leaves the range.
//!
//! An axis of its own, beside the signature and not inside it. The overflow
//! policy says what a result that leaves the range becomes, and it is total; the
//! abort is how a debug build treats the leaving, and it is the consumer's to
//! choose. A release build never aborts under either marker.

use arvo_format::Bool;

/// Whether a debug build aborts when a result leaves the range.
///
/// Open: a check of another crate is one more marker. `Num` takes it as its
/// second parameter and defaults it to `Abort`.
pub trait DebugCheck {
    /// Whether a debug build aborts.
    const ABORTS: Bool;
}

/// A debug build aborts when a result leaves the range. The default.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Abort;

/// No build aborts: the completion the overflow policy names is what the
/// operation means.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Quiet;

impl DebugCheck for Abort {
    const ABORTS: Bool = Bool::TRUE;
}

impl DebugCheck for Quiet {
    const ABORTS: Bool = Bool::FALSE;
}
