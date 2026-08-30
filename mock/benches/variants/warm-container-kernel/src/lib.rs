//! The fifth arm: minimum container, deferred projection, and for a
//! saturating fold a lane-parallel accumulator instead of a serial one.
//!
//! It exists because the other four arms all agree on something that turns
//! out to be a limitation of the shape rather than of the semantics: a fold
//! written with one accumulator is a loop-carried dependence, so it runs
//! serially in every container, and the hardware's lane-parallel saturating
//! add is unreachable from any of them. `140:222-235` argued that instruction
//! is what `Precise` at an exactly-filled width should compile to, and no arm
//! in this bench before this one could reach it, so the claim stayed
//! unmeasured while looking measured.
//!
//! Unsigned saturating addition is associative and commutative, so the
//! reassociation is a property of the semantics rather than a liberty; the
//! harness's cross-variant comparison and the shared crate's
//! `all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`
//! are what hold it to that.
//!
//! On the wrapping and elementwise rows this arm is identical to `native`, so
//! wherever it appears on those it is a second noise-floor control.
//!
//! See `bench-warm-container-shared` for the arms, the key encoding and the
//! transforms every arm calls.

use bench_warm_container_shared::{arms, Case};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(
    Case,
    "warm-container-kernel",
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
fn run_kernel<const KEY: usize>(
    input: &<Case<KEY> as Routine>::Input,
    output: &mut <Case<KEY> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            output.value = arms::kernel(KEY, input);
        }
    }
}
