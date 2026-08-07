//! The bar op's own definition of `Warm` sets: "It should behave like
//! native primitives in regular old rust would"
//! (`140b_op_checkpoint_thirtythree.md:16-21`).
//!
//! Minimum container, and the projection written **once**, before the value
//! is observed, which is what a Rust programmer who needed `W`-bit wrapping
//! writes by hand: keep the column in the natural primitive, do the
//! arithmetic, mask at the end. At an exactly-filled width there is no mask
//! at all and this arm is plain Rust integer arithmetic with nothing added.
//!
//! Against `minimum` it isolates one thing and only one: the cost of
//! writing the projection after every operation rather than once.
//! `140:176-206` claims the compiler removes the redundant ones and offers
//! an assembler symbol alias as the evidence. This arm is that claim
//! measured as throughput. If the two arms separate, the claim does not
//! survive contact with a real loop; if they do not, the projection is free
//! and per-operation cost is the wrong axis for the whole fork.
//!
//! Under saturating semantics (`OP = 1`) this arm is byte-identical to
//! `minimum`, because saturation has no lazy form: there is nothing to
//! defer and the machine's saturating instruction at an exactly-filled
//! width is what both arms emit. Those cells are a second noise-floor
//! control, alongside `plusone`.
//!
//! See `bench-warm-container-shared` for the arms, the key encoding and the
//! single transform all four arms call.

use bench_warm_container_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-container-native",
    sizes = [
        80003, 130003, 160003, 320003, 600003, 640003,
        81003, 131003, 161003, 321003, 601003, 641003,
        130001, 130002, 130004, 130008, 130016,
        640001, 640002, 640004, 640008, 640016,
        80103, 130103, 160103, 320103, 600103, 640103,
        80204, 130204, 160204, 320204, 600204, 640204,
        80304, 130304, 160304, 320304, 600304, 640304,
        80403, 130403, 160403, 320403, 600403, 640403,
        80501, 130501, 160501, 320501, 600501, 640501,
        130401, 130402, 130404, 130408, 130416
    ]
)]
fn run_native<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::native(KEY, input);
        }
    }
}
