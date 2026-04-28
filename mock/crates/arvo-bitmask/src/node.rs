//! Node index newtype.
//!
//! `NodeId` wraps a `USize` to keep node indices distinct from bit
//! positions, column indices, or other counts at the type level. Used
//! by `BitMatrix64` / `BitMatrix256` and any consumer that speaks in
//! nodes rather than raw integers.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

use arvo::USize;

/// Node index newtype.
///
/// `#[repr(transparent)]` over `USize`; zero cost after compilation.
/// No `Deref<Target = usize>` — node ids should not silently decay
/// into plain integers. `Ord` and `Hash` are implemented manually
/// because `arvo::newtype::USize` does not derive them.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NodeId(pub USize);

impl NodeId {
    /// Construct a `NodeId` from a `USize`.
    #[inline(always)]
    pub const fn new(u: USize) -> Self {
        Self(u)
    }
}

impl Ord for NodeId {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0).0.cmp(&(other.0).0)
    }
}

impl PartialOrd for NodeId {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { // lint:allow(no-bare-option) reason: core::cmp::PartialOrd::partial_cmp trait-method signature returns Option<Ordering>; tracked: #115
        Some(self.cmp(other))
    }
}

impl Hash for NodeId {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.0).0.hash(state);
    }
}
