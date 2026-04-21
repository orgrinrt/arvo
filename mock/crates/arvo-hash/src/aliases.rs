//! Domain aliases over `arvo_bits::Bits<N>`.

use arvo_bits::Bits;

/// 28-bit content-hash identity. Matches `hilavitkutin-str`'s
/// `Str::ID_MASK` low 28 bits.
pub type ContentHash = Bits<28>;
