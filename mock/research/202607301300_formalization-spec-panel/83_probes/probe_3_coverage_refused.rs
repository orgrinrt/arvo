//! Probe 3: EXPECTED FAIL. The level ordering (fields extent <= stored width)
//! is a declaration-site refusal, not a runtime check.
//!
//! A lowering declaring a stored width narrower than its fields' extent must
//! refuse with E0080 at the declaration, in the ByteCap/ShortCap coverage
//! shape (spine-rule firings ten and eleven), before any use site exists.

#![no_std]

pub trait FieldsModel {
    const EXTENT: usize;
}

pub struct F13;
impl FieldsModel for F13 {
    const EXTENT: usize = 13;
}

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

/// WRONG: 8 stored bits cannot hold a 13-bit field extent
pub struct Undersized;
impl LoweringModel for Undersized {
    type Fields = F13;
    const STORED: usize = 8;
}

// this line must fail to compile with E0080
const _: () = coverage_holds::<Undersized>();
