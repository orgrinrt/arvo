//! The deletion `140_fog_warm_without_widening.md` proposed. The container
//! is the minimum native that holds the declared width `W`, identically for
//! every strategy, and the width discipline lives on the operation's result.
//! Same transform and same projection as the `headroom` arm; only the
//! carrier differs.
//!
//! Table below is `tag_hot_cold`'s ladder
//! (`arvo-strategy/src/container.rs:60-75`), which is what `Hot` and `Cold`
//! already get: `1..=8 -> u8`, `9..=16 -> u16`, `17..=32 -> u32`,
//! `33..=64 -> u64`.
//!
//! See `bench-warm-container-shared` for the arms, the key encoding and the
//! single transform all four arms call.

use bench_warm_container_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-container-minimum",
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
fn run_minimum<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::minimum(KEY, input);
        }
    }
}
