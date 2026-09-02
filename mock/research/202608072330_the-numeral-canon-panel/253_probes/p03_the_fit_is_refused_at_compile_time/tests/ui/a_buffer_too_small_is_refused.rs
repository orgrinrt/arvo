//! A buffer one byte short of the rendering must not build.
//!
//! Bound in a `const` item rather than called from a runtime body, because a
//! const assertion inside a generic function is forced at codegen, so a runtime
//! call reaches it under `cargo build` and not under `cargo check`.

use arvo_format::points::Integer;
use p03_the_fit_is_refused_at_compile_time::identity_of;

// `Integer<8>` renders in 51 bytes. 50 must be refused.
const _SHORT: () = {
    let mut b = [0u8; 50];
    identity_of::<Integer<8>, 50>(&mut b);
};

fn main() {}
