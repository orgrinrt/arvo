//! The shipped rule. `Warm` and `Precise` store a numeral of declared
//! width `W` in `rung(rung_bits(W) + 1)`, one rung above the minimum that
//! holds it (`arvo-strategy/src/container.rs:15-19`). The transform still
//! projects the result back to `W` after every operation, because the
//! semantics wrap (or saturate) at `W` and the container wraps at its own
//! width, so the projection is required whatever the container is.
//!
//! Table below is the shipped ladder read directly off that file:
//! `1..=8 -> u16`, `9..=16 -> u32`, `17..=32 -> u64`, `33..=64 -> u128`.
//!
//! See `bench-warm-container-shared` for the arms, the key encoding and the
//! single transform all four arms call.

use bench_warm_container_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-container-headroom",
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
fn run_headroom<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::headroom(KEY, input);
        }
    }
}
