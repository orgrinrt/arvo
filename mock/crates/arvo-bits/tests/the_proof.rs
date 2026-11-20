//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The proof's auto traits do not follow the container's: `Within` is `Send`
//! and `Sync` for a container that is neither.

use core::marker::PhantomData;

use arvo_bits::{BitWidth, Bits, Within};
use arvo_format::Width;

/// A container that is neither `Send` nor `Sync`, through a raw pointer it
/// carries.
#[allow(dead_code)]
struct Pinned(Bits<8>, PhantomData<*const ()>);

impl BitWidth for Pinned {
    const WIDTH: Width = Width::bits(8);
}

const fn send_and_sync<T: Send + Sync>() {}

/// Built only where the bound holds, so a field that carried the container's
/// auto traits, `PhantomData<B>` among them, fails this file at `cargo build`.
const _: () = send_and_sync::<Within<Pinned, 0>>();
const _: () = send_and_sync::<Within<Pinned, 7>>();

#[test]
fn the_proof_is_send_and_sync_for_a_container_that_is_neither() {
    // The constants above are the assertion; this names the binary's one
    // concern in the test list, and builds the proof the bound was asked of.
    let _ = Within::<Pinned, 7>::NEW;
}
