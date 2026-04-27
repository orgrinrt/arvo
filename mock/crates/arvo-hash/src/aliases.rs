//! Hash domain aliases.

use arvo::{Bits, Hot};

/// 28-bit content-hash identity. Matches `hilavitkutin-str`'s
/// `Str::ID_MASK` low 28 bits.
pub type ContentHash = Bits<28, Hot>;
