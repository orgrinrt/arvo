//! The control on *where* the refusal becomes reachable.
//!
//! Same too-small buffer as `tests/ui/a_buffer_too_small_is_refused.rs`, forced
//! from a runtime call rather than bound in a `const` item.
//!
//! Reproduce, from the crate root, after `cargo test`:
//!
//! ```text
//! DEPS=target/debug/deps
//! A=$(ls $DEPS/libarvo_format-*.rlib | head -1)
//! P=$(ls $DEPS/libp03_the_fit_is_refused_at_compile_time-*.rlib | head -1)
//! X="--extern arvo_format=$A --extern p03_the_fit_is_refused_at_compile_time=$P"
//!
//! rustc --edition 2024 --emit=metadata --crate-type lib $X -L dependency=$DEPS \
//!       --out-dir /tmp/a tests/ui/a_buffer_too_small_is_refused.rs   # const-bound
//! rustc --edition 2024 --emit=metadata --crate-type lib $X -L dependency=$DEPS \
//!       --out-dir /tmp/b arms/from_a_runtime_call.rs                 # runtime call
//! rustc --edition 2024                 --crate-type lib $X -L dependency=$DEPS \
//!       --out-dir /tmp/c arms/from_a_runtime_call.rs                 # runtime call, codegen
//! ```
//!
//! Measured, in that order: exit 1, exit 0, exit 1.
//!
//! So a const gate inside a generic function is forced at monomorphisation, and
//! a compile-fail case written as a runtime call would be green under
//! `cargo check` and red only under `cargo build`. Binding it in a `const` item
//! is what makes the refusal reachable at check time. `arvo-format`'s own
//! `Format::PHASE` doctest records the same distinction from the other side, at
//! `crates/arvo-format/src/format.rs:180`.

use arvo_format::points::Integer;
use p03_the_fit_is_refused_at_compile_time::identity_of;

pub fn short() {
    let mut b = [0u8; 50];
    let _ = identity_of::<Integer<8>, 50>(&mut b);
}

// One reading of this disagreed with the others and it is worth the four lines.
//
// Run as three arms on one command line with stderr sent to /dev/null, arm two
// reported exit 1, which would have made the whole measurement say the binding
// changes nothing. Run one arm per invocation with stderr captured, it reports
// exit 0 and zero bytes of diagnostics, three times.
//
// The byte count is what settles it: an exit code alone cannot distinguish a
// refusal from a wrapper failing for its own reasons, and the empty
// `arm2_runtime_call_check.stderr` beside this file is the evidence that the
// compiler said nothing. Committed empty on purpose.
