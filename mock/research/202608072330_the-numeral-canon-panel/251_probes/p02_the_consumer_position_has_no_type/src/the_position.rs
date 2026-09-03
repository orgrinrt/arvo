//! The arm that does not compile, kept out of the build on purpose.
//!
//! Build it with `rustc --edition 2024 --crate-type lib src/the_position.rs
//! --extern arvo_format=<path>` or paste it into `lib.rs`; the stderr beside
//! this directory is what it produces. It is the consumer's field, verbatim in
//! shape, asking arvo for the type its own design names.

use arvo_format::USize;

pub enum LinkError {
    PathNotFound,
    LoadFailed {
        platform_code: USize,
    },
}
