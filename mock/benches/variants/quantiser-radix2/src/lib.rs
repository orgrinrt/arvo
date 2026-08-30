//! Radix-two variant of the decimal-quantiser bench: the SAME generalised
//! kernel as the radix-ten variant, monomorphised at `R = 2` with binary32's
//! format parameters. See `bench-quantiser-radix-shared` for the Routine, the
//! kernel, and why the two variants are comparable.

use bench_quantiser_radix_shared::{run_binary32, RadixAdd};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(RadixAdd, "quantiser-radix2", sizes = [0, 2, 8, 20])]
fn run_radix2<const SPREAD: usize>(
    input: &<RadixAdd<SPREAD> as Routine>::Input,
    output: &mut <RadixAdd<SPREAD> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            run_binary32(input, output);
        }
    }
}
