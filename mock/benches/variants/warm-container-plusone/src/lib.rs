//! `rung(W + 1)`, composing before rounding rather than after. The
//! dispatching brief named it as a candidate, so it is in the arm set.
//!
//! It is not a third container anywhere, and the table below is the proof
//! read off directly: at 8, 16, 32 and 64 the width fills its rung, so
//! `rung(W+1)` is the next rung and this arm is byte-identical to
//! `headroom`; at 13 and 60 the width is below its rung, so `rung(W+1)` is
//! that same rung and this arm is byte-identical to `minimum`.
//! `bench-warm-container-shared`'s `plusone_is_never_a_third_container`
//! asserts that over all 64 widths rather than over this sample.
//!
//! So the arm's value is as a **control**. Wherever it aliases another arm
//! it is running identical machine code on identical data, so the gap
//! between the two is this harness's noise floor for this workload, and any
//! difference between the real arms narrower than that gap is not signal.
//!
//! See `bench-warm-container-shared` for the arms, the key encoding and the
//! single transform all four arms call.

use bench_warm_container_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-container-plusone",
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
fn run_plusone<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::plusone(KEY, input);
        }
    }
}
