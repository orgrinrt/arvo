//! P3b. The compile-fail half of P3's W2, as its own file because the wanted
//! outcome is a build failure.
//!
//! Two strategy markers resolve to the IDENTICAL triple: same value set, same
//! realisation, same completion. Their bits are identical and both are
//! repr(transparent) over u8. Carrying the marker in the type nonetheless
//! makes them incompatible.
//!
//! Expected: rustc refuses the assignment. If it compiles, the marker is not
//! actually part of the type's identity and W2 is not a witness.
//!
//! Build: rustc --edition 2021 -O p3b_marker_makes_identical_resolutions_incompatible.rs
#![allow(dead_code)]
use core::marker::PhantomData;

trait Realisation { const CONTAINER_BITS: u32; }
trait Completion { const KIND: u8; }
struct V8_0;
struct ByteRest;
impl Realisation for ByteRest { const CONTAINER_BITS: u32 = 8; }
struct Wrap;
impl Completion for Wrap { const KIND: u8 = 0; }

#[repr(transparent)]
#[derive(Copy, Clone)]
struct WithMarker<V, R, C, S>(u8, PhantomData<(V, R, C, S)>);

struct Speed;
struct Space;

type Sp = WithMarker<V8_0, ByteRest, Wrap, Speed>;
type Sc = WithMarker<V8_0, ByteRest, Wrap, Space>;

fn main() {
    let b: Sc = WithMarker(7, PhantomData);
    // Same value set, same realisation, same completion, same bits, same
    // layout. Different marker.
    let c: Sp = b;
    println!("if this line runs, the marker is not part of the identity: {}", c.0);
}
