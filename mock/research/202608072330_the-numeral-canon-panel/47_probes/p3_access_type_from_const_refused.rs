// p3: attacking 16's verdict that the packed access width is "not a third output".
//
// 16 section 4 (16:173-198) and section 11.3 (16:664-673) dismiss the access width, the load
// a compiler must emit to cover one element of a packed run at an arbitrary bit phase, on the
// grounds that it is "recoverable in closed form from the declared width", floor((W+6)/8)+1
// bytes, verified against an exhaustive phase scan with zero mismatches.
//
// That verdict is about INFORMATION. The quantity a lowering site needs is a TYPE: the thing it
// loads into. This file checks whether the closed form transfers across that boundary when the
// declared width is carried as a CONST, which is the kind arvo's own consumer surface writes
// (`UFixed<13, 3, S>`: two const generic arguments).
//
// EXPECTED TO FAIL TO COMPILE. The committed .err is the result. Three positions again:
//
//   arm A1: type alias
//   arm A2: function return position
//   arm A3: a where-clause, which is where a bound would have to live
//
//   rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p3_access_type_from_const_refused.rs
//
// The complementary positive arms, where the same recovery DOES work and what each costs, are
// in p3b. Read them together; this file alone establishes only that one route is closed.
//
// No #![feature] gate.

#![no_std]

// ---- the byte-count-keyed access ladder a recovery would have to land on ----
pub struct Bytes<const N: u32>;
pub trait AccessFor {
    type T: Copy;
}
impl AccessFor for Bytes<1> {
    type T = u8;
}
impl AccessFor for Bytes<2> {
    type T = u16;
}
impl AccessFor for Bytes<3> {
    type T = u32;
}
impl AccessFor for Bytes<4> {
    type T = u32;
}
impl AccessFor for Bytes<5> {
    type T = u64;
}
impl AccessFor for Bytes<6> {
    type T = u64;
}
impl AccessFor for Bytes<7> {
    type T = u64;
}
impl AccessFor for Bytes<8> {
    type T = u64;
}

/// 16's closed form, verbatim in arithmetic: the maximum byte span of a W-bit field at unknown
/// phase. As a const fn over a plain value it is fine, and it is exhaustively correct per
/// 16_probes/p4. Everything below is about whether it can reach a type.
pub const fn access_bytes(w: u32) -> u32 {
    (w + 6) / 8 + 1
}

const _: () = assert!(access_bytes(13) == 3);
const _: () = assert!(access_bytes(5) == 2);
const _: () = assert!(access_bytes(31) == 5);

// arm A1: the access type as a type alias, from the width carried as a const.
pub type AccessOf<const W: u32> = <Bytes<{ access_bytes(W) }> as AccessFor>::T;

// arm A2: the same in a function's return position. This is the real site: a packed read
// generic over the declared width, returning the loaded window.
pub fn load_window<const W: u32, const STRIDE: u32>(
    _base: *const u8,
    _k: usize,
) -> <Bytes<{ access_bytes(W) }> as AccessFor>::T {
    unimplemented!()
}

// arm A3: as a where-clause, which is where the bound would sit if the type were named
// elsewhere.
pub fn load_window_bounded<C: Copy, const W: u32, const STRIDE: u32>(_c: C)
where
    Bytes<{ access_bytes(W) }>: AccessFor,
{
}
