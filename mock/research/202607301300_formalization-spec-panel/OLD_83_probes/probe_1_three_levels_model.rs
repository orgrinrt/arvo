//! Probe 1: the three width levels, modelled, with every derivation in const
//! position and the whole preset matrix asserted at compile time.
//!
//! Hypothesis: the design has three width levels (fields extent, stored width,
//! container width), of which exactly one is declared; the other two are
//! derived projections, one downward from the encoding and one upward through
//! the dispatch menu. The ratified preset tables instantiate all four presets
//! with no contradiction, `Layout` decides the granularity at which the
//! container level exists, and zero inter-value padding under `Bitpacked` is a
//! theorem of the group projection rather than an obligation.
//!
//! Zero feature gates. no_std. Every assertion is a `const` item, so the file
//! compiling IS the result; there is nothing to run.

#![no_std]

// ---------------------------------------------------------------------------
// level 1: fields extent. In the real design this is derived from the
// Numeral's parameters by the Encoding; the model declares instances directly.
// ---------------------------------------------------------------------------

pub trait FieldsModel {
    const EXTENT: usize;
}

/// a 13-bit fixed-point numeral's fields (sign + digits, extent 13)
pub struct F13;
impl FieldsModel for F13 {
    const EXTENT: usize = 13;
}

/// binary32's fields (1 + 8 + 23 = 32); the IEEE case where levels coincide
pub struct FBin32;
impl FieldsModel for FBin32 {
    const EXTENT: usize = 32;
}

// ---------------------------------------------------------------------------
// level 2: stored width. The one DECLARED level: the Lowering axis. The
// ratified instances are logical-relative: minimum = the fields' extent,
// doubled = twice it. Coverage (EXTENT <= STORED) is checked at declaration.
// ---------------------------------------------------------------------------

pub trait LoweringModel {
    type Fields: FieldsModel;
    const STORED: usize;
}

pub const fn coverage_holds<L: LoweringModel>() {
    assert!(
        <L::Fields as FieldsModel>::EXTENT <= L::STORED,
        "StoredWidth must cover the fields' extent"
    );
}

/// Hot fixed 13-bit: minimum
pub struct HotFixed13;
impl LoweringModel for HotFixed13 {
    type Fields = F13;
    const STORED: usize = F13::EXTENT; // minimum
}
const _: () = coverage_holds::<HotFixed13>();

/// Cold fixed 13-bit: minimum (identical stored width to Hot; only Layout differs)
pub struct ColdFixed13;
impl LoweringModel for ColdFixed13 {
    type Fields = F13;
    const STORED: usize = F13::EXTENT; // minimum
}
const _: () = coverage_holds::<ColdFixed13>();

/// Warm fixed 13-bit: doubled
pub struct WarmFixed13;
impl LoweringModel for WarmFixed13 {
    type Fields = F13;
    const STORED: usize = 2 * F13::EXTENT; // doubled
}
const _: () = coverage_holds::<WarmFixed13>();

/// Precise fixed 13-bit: doubled
pub struct PreciseFixed13;
impl LoweringModel for PreciseFixed13 {
    type Fields = F13;
    const STORED: usize = 2 * F13::EXTENT; // doubled
}
const _: () = coverage_holds::<PreciseFixed13>();

/// Warm float binary32: minimum (the ratified divergence from Warm's fixed row)
pub struct WarmBin32;
impl LoweringModel for WarmBin32 {
    type Fields = FBin32;
    const STORED: usize = FBin32::EXTENT; // minimum
}
const _: () = coverage_holds::<WarmBin32>();

// ---------------------------------------------------------------------------
// level 3: container width. NEVER declared: a projection of the stored width
// through the dispatch menu (Dense), or of the group arithmetic (Bitpacked).
// Written in const position per the const-position rule: these compute, so
// they are consumed only through `const` items below.
// ---------------------------------------------------------------------------

/// the Dense dispatch menu: smallest native width holding `stored` bits
pub const fn dense_container_width(stored: usize) -> usize {
    if stored <= 8 {
        8
    } else if stored <= 16 {
        16
    } else if stored <= 32 {
        32
    } else if stored <= 64 {
        64
    } else if stored <= 128 {
        128
    } else {
        // WideBits territory: whole bytes, align-1
        (stored + 7) / 8 * 8
    }
}

pub const fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// Bitpacked group: elements per group. A function of the STORED width.
pub const fn period(stored: usize) -> usize {
    8 / gcd(stored, 8)
}

/// Bitpacked group: container bytes per group. A function of the STORED width.
pub const fn group_bytes(stored: usize) -> usize {
    stored * period(stored) / 8
}

// ---------------------------------------------------------------------------
// the preset matrix, asserted. Each preset collapses a different pair of
// levels; only Warm/Precise fixed-point separates all three.
// ---------------------------------------------------------------------------

// Hot fixed 13: fields = stored = 13, container = 16. The [13, 16) region is
// the ungoverned one (probe 2 runs its misordering).
const _: () = assert!(F13::EXTENT == 13 && HotFixed13::STORED == 13);
const _: () = assert!(dense_container_width(HotFixed13::STORED) == 16);

// Warm fixed 13: 13 < 26 < 32. Three distinct numbers: the first preset at
// which all three levels separate, which is why no earlier model saw them.
const _: () = assert!(WarmFixed13::STORED == 26);
const _: () = assert!(dense_container_width(WarmFixed13::STORED) == 32);
const _: () = assert!(
    F13::EXTENT < WarmFixed13::STORED
        && WarmFixed13::STORED < dense_container_width(WarmFixed13::STORED)
);

// Precise fixed 13: same three numbers as Warm (they differ on Resolution,
// not on any width level).
const _: () = assert!(PreciseFixed13::STORED == 26);
const _: () = assert!(dense_container_width(PreciseFixed13::STORED) == 32);

// Cold fixed 13, Bitpacked: no per-value container. The group IS the
// container: 8 elements in exactly 13 bytes.
const _: () = assert!(ColdFixed13::STORED == 13);
const _: () = assert!(period(ColdFixed13::STORED) == 8);
const _: () = assert!(group_bytes(ColdFixed13::STORED) == 13);

// Warm float binary32: 32 = 32 = 32. All three levels coincide, which is why
// the float rows never showed the gap.
const _: () = assert!(FBin32::EXTENT == 32 && WarmBin32::STORED == 32);
const _: () = assert!(dense_container_width(WarmBin32::STORED) == 32);

// ---------------------------------------------------------------------------
// zero inter-value padding is a THEOREM of the group projection, not an
// obligation: for every stored width the group's byte count times eight is
// exactly the period times the stored width. Checked over the whole 1..=57
// range the decode plan serves, not a sample.
// ---------------------------------------------------------------------------

const _: () = {
    let mut w = 1;
    while w <= 57 {
        assert!(group_bytes(w) * 8 == w * period(w), "group has padding");
        // and the group is the smallest such unit: no smaller element count
        // lands on a byte boundary
        let mut k = 1;
        while k < period(w) {
            assert!((w * k) % 8 != 0, "period is not minimal");
            k += 1;
        }
        w += 1;
    }
};

// ---------------------------------------------------------------------------
// the two levels feed DIFFERENT consts the moment they diverge: the decode
// plan (period, stride) is a function of the stored width; the value mask is
// a function of the fields' extent. At Cold-minimum they coincide (13, 13),
// which is what let file 81 say "the logical width alone". A hypothetical
// bitpacked lowering with declared headroom separates them.
// ---------------------------------------------------------------------------

pub struct BitpackedWithHeadroom;
impl LoweringModel for BitpackedWithHeadroom {
    type Fields = F13;
    const STORED: usize = 16; // 13 fields + 3 declared padding bits
}
const _: () = coverage_holds::<BitpackedWithHeadroom>();

pub const fn value_mask(extent: usize) -> u64 {
    (1u64 << extent) - 1
}

// plan keys on STORED: period(16) = 1, group = 2 bytes. Mask keys on EXTENT.
const _: () = assert!(period(BitpackedWithHeadroom::STORED) == 1);
const _: () = assert!(group_bytes(BitpackedWithHeadroom::STORED) == 2);
const _: () = assert!(
    value_mask(F13::EXTENT) == 0x1FFF
        && value_mask(BitpackedWithHeadroom::STORED) != value_mask(F13::EXTENT)
);
// at Cold-minimum the two coincide, hiding the level split:
const _: () = assert!(ColdFixed13::STORED == F13::EXTENT);
