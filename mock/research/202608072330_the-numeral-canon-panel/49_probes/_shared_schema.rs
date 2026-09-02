// --- strategy axis -----------------------------------------------------

pub trait Strategy {}
pub struct Hot;
pub struct Warm;
pub struct Cold;
pub struct Precise;
impl Strategy for Hot {}
impl Strategy for Warm {}
impl Strategy for Cold {}
impl Strategy for Precise {}

// --- sign axis -----------------------------------------------------------

pub trait Signedness {}
pub struct Unsigned;
pub struct Signed;
impl Signedness for Unsigned {}
impl Signedness for Signed {}

// --- the fact schema -----------------------------------------------------
//
// One trait names every fact a lowering site needs. Every impl below fills
// the same schema with different values. The schema itself does not vary
// by strategy; only the values inside it do. This is the answer this probe
// exists to test: is the schema uniform while the values differ?

pub trait NumeralFacts<Sign: Signedness, const N: usize> {
    /// storage-at-rest representation: what a single value looks like when
    /// held in memory, alone or as one slot of a sequence
    type Storage: Copy;

    /// compute/operand representation: what a single value looks like while
    /// an operation is being performed on it. May equal Storage or diverge
    /// from it (I6: "[Cold] can use the same paths Hot uses").
    type Operand: Copy;

    /// byte alignment a sequence of these values is laid out at
    const ALIGN: usize;

    /// whether a sequence of these values is a genuinely bit-packed stream
    /// (numeral boundaries fall at arbitrary bit offsets) rather than an
    /// array of individually addressable `Storage` slots
    const PACKED: bool;

    /// the logical bit width, echoed back so downstream code can read it
    /// off the type without separately threading the const generic
    const WIDTH: usize = N;
}

// --- N = 13, unsigned: the width the panel keeps returning to, because it
// crosses no native boundary cleanly and forces every strategy to make a
// real choice rather than defaulting to "the obvious native type".

impl NumeralFacts<Unsigned, 13> for Hot {
    // hot wants the native register a normal add/cmp instruction can use.
    // 13 bits rounds up to the next native width; storage and compute
    // coincide because hot's whole intent is to avoid extra conversion
    // work on the hot path.
    type Storage = u16;
    type Operand = u16;
    const ALIGN: usize = 2;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 13> for Warm {
    // warm imitates what a rust programmer would reach for by instinct
    // (I3/I4): a `u16` field, nothing cleverer.
    type Storage = u16;
    type Operand = u16;
    const ALIGN: usize = 2;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 13> for Cold {
    // cold aggressively bitpacks (I6). storage is deliberately NOT the
    // native u16: it is a raw 2-byte array with no implied alignment, so a
    // sequence of these composes into a genuinely packed bitstream rather
    // than an array of u16 slots with 3 wasted bits each. per I6's second
    // quote, cold's operand representation is free to reuse hot's path
    // when nothing in cold's own intent fights it, so operand coincides
    // with hot/warm's u16 even though storage does not.
    type Storage = [u8; 2];
    type Operand = u16;
    const ALIGN: usize = 1;
    const PACKED: bool = true;
}

impl NumeralFacts<Unsigned, 13> for Precise {
    // precise sacrifices compute efficiency for accuracy across chains
    // (I7). storage matches warm (no reason to spend more bits at rest),
    // but the operand widens to reduce intermediate rounding across a
    // chain of operations, which is exactly what I7 asks for ("accurate
    // and precise, especially within chains").
    type Storage = u16;
    type Operand = u32;
    const ALIGN: usize = 2;
    const PACKED: bool = false;
}

// --- N = 8, unsigned: the trivial case, where a byte-exact width should
// collapse every strategy's storage/operand split to the same answer,
// because there is no padding to disagree about.

impl NumeralFacts<Unsigned, 8> for Hot {
    type Storage = u8;
    type Operand = u8;
    const ALIGN: usize = 1;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 8> for Warm {
    type Storage = u8;
    type Operand = u8;
    const ALIGN: usize = 1;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 8> for Cold {
    // no padding bits exist to strip at N=8, so cold's storage collapses
    // to the same shape as warm/hot: this is the case referenced by I6's
    // "nothing in its intent would fight it" line. packed stays false
    // because there is nothing left to pack; a sequence of exact bytes is
    // already minimal as a plain array.
    type Storage = u8;
    type Operand = u8;
    const ALIGN: usize = 1;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 8> for Precise {
    type Storage = u8;
    type Operand = u16;
    const ALIGN: usize = 1;
    const PACKED: bool = false;
}

// --- N = 17, unsigned: crosses a byte boundary AND a native width
// boundary (u16 is too narrow, u32 is the next native container), so this
// is the width most likely to expose a case the N=13 table does not.

impl NumeralFacts<Unsigned, 17> for Hot {
    type Storage = u32;
    type Operand = u32;
    const ALIGN: usize = 4;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 17> for Warm {
    type Storage = u32;
    type Operand = u32;
    const ALIGN: usize = 4;
    const PACKED: bool = false;
}

impl NumeralFacts<Unsigned, 17> for Cold {
    // 17 bits packs into 3 raw bytes rather than the 4-byte native
    // container hot/warm reach for; this is the case where cold's
    // storage saving is largest relative to the native alternative.
    type Storage = [u8; 3];
    type Operand = u32;
    const ALIGN: usize = 1;
    const PACKED: bool = true;
}

impl NumeralFacts<Unsigned, 17> for Precise {
    type Storage = u32;
    type Operand = u64;
    const ALIGN: usize = 4;
    const PACKED: bool = false;
}

// --- a lowering-site consumer -------------------------------------------
//
// This function stands in for a lowering site: it takes a generic
// strategy and asks the schema for exactly the facts it needs, computing
// everything else (here: a trivial byte-size-of-storage) rather than
// having it supplied as a separate fact.

pub const fn storage_byte_size<S, Sign, const N: usize>() -> usize
where
    S: NumeralFacts<Sign, N>,
    Sign: Signedness,
{
    core::mem::size_of::<S::Storage>()
}

pub const fn is_packed<S, Sign, const N: usize>() -> bool
where
    S: NumeralFacts<Sign, N>,
    Sign: Signedness,
{
    S::PACKED
}

// compile-time assertions: these are the "exhaustive count" this probe can
// make honestly, over the (strategy x width) cells it actually filled in.
// they are not runtime tests; a const-eval failure here is a compile
// error, which is the validation mechanism this derivation relies on.

const _: () = assert!(storage_byte_size::<Hot, Unsigned, 13>() == 2);
const _: () = assert!(storage_byte_size::<Cold, Unsigned, 13>() == 2);
const _: () =
    assert!(storage_byte_size::<Hot, Unsigned, 13>() == storage_byte_size::<Cold, Unsigned, 13>());
const _: () = assert!(!is_packed::<Hot, Unsigned, 13>());
const _: () = assert!(is_packed::<Cold, Unsigned, 13>());
const _: () = assert!(!is_packed::<Cold, Unsigned, 8>());
const _: () = assert!(storage_byte_size::<Cold, Unsigned, 17>() == 3);
const _: () = assert!(storage_byte_size::<Hot, Unsigned, 17>() == 4);

// operand widening for Precise, without a corresponding storage widening:
const _: () =
    assert!(core::mem::size_of::<<Precise as NumeralFacts<Unsigned, 13>>::Storage>() == 2);
const _: () =
    assert!(core::mem::size_of::<<Precise as NumeralFacts<Unsigned, 13>>::Operand>() == 4);

// cold's storage/operand divergence, and its coincidence with hot's operand:
const _: () = assert!(core::mem::size_of::<<Cold as NumeralFacts<Unsigned, 13>>::Storage>() == 2);
const _: () = assert!(core::mem::size_of::<<Cold as NumeralFacts<Unsigned, 13>>::Operand>() == 2);

pub fn touch() -> usize {
    storage_byte_size::<Cold, Unsigned, 13>() + storage_byte_size::<Precise, Unsigned, 17>()
}
