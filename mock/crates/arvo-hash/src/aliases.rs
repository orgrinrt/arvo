//! Hash domain aliases.

use arvo::{Bits, Hot};

/// 64-bit content-hash identity.
///
/// Carries the canonical content-addressed hash for downstream consumers
/// (interner identity, persistence keys, pass-fingerprint identity,
/// loimu-style entity content addressing). 64 bits matches the natural
/// output width of the substrate's default hash family (XxHash3) and
/// keeps collision probability negligible across million-entity workloads.
pub type ContentHash = Bits<64, Hot>;
