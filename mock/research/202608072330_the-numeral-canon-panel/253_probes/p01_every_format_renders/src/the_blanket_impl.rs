//! The arm. `impl<F: Format> Debug for F` over the open inventory.
//!
//! Compiled on its own, never as part of the crate, because a coherence refusal
//! aborts the crate it sits in. The stderr beside this file is the measurement.
//!
//! Reproduce, from the crate root, after `cargo test` has built the dependency:
//!
//! ```text
//! DEPS=target/debug/deps
//! A=$(ls $DEPS/libarvo_format-*.rlib | head -1)
//! rustc --edition 2024 --crate-type lib --extern arvo_format=$A \
//!       -L dependency=$DEPS --out-dir out src/the_blanket_impl.rs
//! ```
//!
//! Measured: exit 1, `E0210`. Running the same command against
//! `src/the_blanket_impl_control.rs`, which is identical but for a local trait
//! in `core::fmt::Debug`'s place, exits 0 with an empty stderr. So the refusal
//! is about implementing a foreign trait over a type parameter and not about
//! how rustc was called.

#![no_std]

use arvo_format::Format;
use core::fmt;

impl<F: Format> fmt::Debug for F {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a format")
    }
}
