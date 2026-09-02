//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The suite, in four parts, each of which carries its own controls.
//!
//! **The case that must fail comes before the number.** A walk that reports
//! nothing looks exactly like a clean corpus, and a role classifier that returns
//! one bucket for everything looks exactly like a stack that writes one kind of
//! number. So every part here plants an input the instrument has to answer
//! differently, and asserts the difference rather than the answer.

mod controls;
mod design;
mod positions;
mod roles;
mod spec;

/// A fixture exercising one of every grammatical position, used by more than one
/// module.
pub const EVERY_POSITION: &str = r#"
pub fn free_param(count: u32, flag: bool) -> usize { 0 }

pub struct Held {
    pub width: u8,
    private: u16,
    pub nested: Option<[u64; 4]>,
}

pub struct Tupled(pub i8, u128);

pub enum Shape {
    Flat { extent: u32 },
    Ordered(f32),
}

pub trait Contract {
    const BASE: i32;
    const SIGNED: bool;
    type Carrier;
    fn ask(&self, idx: usize) -> f64;
}

pub struct Impl;

impl Contract for Impl {
    const BASE: i32 = 1;
    const SIGNED: bool = false;
    type Carrier = u16;
    fn ask(&self, idx: usize) -> f64 { 0.0 }
}

impl Impl {
    pub const INHERENT: u32 = 7;
    fn hidden(&self, n: u32) {}
}

pub const TOP: u64 = 9;
pub static ALSO: i16 = 3;
pub type Alias = u32;

pub fn generic<const N: usize, const W: u8>() -> [u8; N] { [0; N] }

pub fn interior() {
    let local: u32 = 1;
    let cast = 2i64 as u8;
}

mod private_module {
    pub fn unreachable(x: u32) {}
}

#[cfg(test)]
mod tests {
    pub fn not_shipped(y: u64) {}
}
"#;
