//! The composition: minimum container, deferred projection, and a
//! lane-parallel accumulator.
//!
//! The two levers this bench separates are independent. Deferring the
//! projection removes the mask from the accumulator, which is what stops the
//! reduction being vectorisable. Reassociating the fold into lanes gives
//! instruction-level parallelism whether or not a mask is there. Neither
//! subsumes the other, and the width sweep shows each winning in a different
//! band: deferral wins by about 2x at 8, 13 and 16 bits, reassociation wins
//! by about 10% at 60 and 64, where the deferred serial form is still limited
//! by a single dependence chain and by this target having no 64-bit vector
//! multiply.
//!
//! This arm applies both, which is the shape a compile-time decision table
//! would pick if the answer is that they compose rather than that one of them
//! is the answer.
//!
//! Saturating rows are the same lane-parallel fold the `kernel` arm runs, so
//! on those two this arm and that one are a control pair.
//!
//! See `bench-warm-container-shared` for the arms, the key encoding and the
//! transforms every arm calls.

use bench_warm_container_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-container-lanes-deferred",
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
fn run_lanes_deferred<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::lanes_deferred(KEY, input);
        }
    }
}
