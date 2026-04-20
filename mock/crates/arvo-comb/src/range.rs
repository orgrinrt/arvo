//! Half-open index interval.
//!
//! `Range { start, end }` denotes the half-open interval `[start, end)`
//! over item indices. Used as the output element of `greedy_group`;
//! downstream algorithms that consume sequential groupings accept a
//! slice of `Range` plus a count.

use arvo::newtype::USize;

/// Half-open interval `[start, end)` over `USize` indices.
///
/// Stored as two independent fields; not `repr(transparent)`. The
/// default is the empty range `[0, 0)`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Range {
    /// Inclusive start index.
    pub start: USize,
    /// Exclusive end index.
    pub end: USize,
}

impl Default for Range {
    #[inline]
    fn default() -> Self {
        Range {
            start: USize(0),
            end: USize(0),
        }
    }
}
