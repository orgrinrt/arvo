//! Dulmage-Mendelsohn structural decomposition.
//!
//! Classifies each node of a `BitMatrix<Bits<64, Hot, Unsigned>, N>` adjacency into one of
//! three disjoint masks based on the presence of incoming and
//! outgoing edges:
//!
//! - `horizontal`: sinks — node has incoming edges but no outgoing
//!   edges. Row reads but never writes (dead-end output).
//! - `vertical`: sources and isolates — node has no incoming edges.
//!   Covers pure producers *and* nodes with no edges at all (treated
//!   as read-only constants).
//! - `square`: core — node has both incoming and outgoing edges. The
//!   matched row-column pairs.
//!
//! The three masks partition the `N` node indices exactly: every
//! node `i` in `0..N` appears in exactly one mask.

use arvo::{Cap, USize};
use arvo::{Bits, Hot, Unsigned};
use arvo_bitmask::{BitMatrix, Mask, cap_size};

use arvo_bitmask::NodeId;

/// Dulmage-Mendelsohn decomposition result.
///
/// `horizontal`, `vertical`, `square` are pairwise disjoint and
/// together cover the node indices `0..N`.
#[derive(Copy, Clone, Default)]
pub struct DulmageMendelsohn<const N: Cap>
where
    [(); cap_size(N)]:,
{
    /// Nodes with incoming edges but no outgoing edges.
    pub horizontal: Mask<Bits<64, Hot, Unsigned>>,
    /// Nodes with no incoming edges (sources and isolates).
    pub vertical: Mask<Bits<64, Hot, Unsigned>>,
    /// Nodes with both incoming and outgoing edges.
    pub square: Mask<Bits<64, Hot, Unsigned>>,
}

/// Classify each node in the adjacency into one of the three masks.
#[inline]
pub fn dulmage_mendelsohn<const N: Cap>(
    adjacency: &BitMatrix<Bits<64, Hot, Unsigned>, N>,
) -> DulmageMendelsohn<N>
where
    [(); cap_size(N)]:,
{
    let mut out: DulmageMendelsohn<N> = DulmageMendelsohn {
        horizontal: Mask::<Bits<64, Hot, Unsigned>>::empty(),
        vertical: Mask::<Bits<64, Hot, Unsigned>>::empty(),
        square: Mask::<Bits<64, Hot, Unsigned>>::empty(),
    };

    let mut i = 0usize;
    while i < cap_size(N) {
        let node = NodeId::new(USize(i));
        let has_succ = !*adjacency.successors(node).is_empty();
        let has_pred = !*adjacency.predecessors(node).is_empty();

        if !has_pred {
            // No incoming edges: source or isolate.
            out.vertical.insert(USize(i));
        } else if !has_succ {
            // Has incoming but no outgoing: dead-end sink.
            out.horizontal.insert(USize(i));
        } else {
            // Both directions: core.
            out.square.insert(USize(i));
        }
        i += 1;
    }

    out
}
