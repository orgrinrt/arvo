//! Radix-ten variant of the decimal-quantiser bench: the SAME generalised
//! kernel as the radix-two variant, monomorphised at `R = 10` with decimal32's
//! format parameters. See `bench-quantiser-radix-shared` for the Routine, the
//! kernel, and why the two variants are comparable.

use bench_quantiser_radix_shared::{RadixAdd, run_decimal32};
use mockspace_bench_core::{FfiBenchCall, Routine, timed};
use mockspace_bench_macro::bench_variant;

#[bench_variant(RadixAdd, "quantiser-radix10", sizes = [0, 2, 8, 20])]
fn run_radix10<const SPREAD: usize>(
    input: &<RadixAdd<SPREAD> as Routine>::Input,
    output: &mut <RadixAdd<SPREAD> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            run_decimal32(input, output);
        }
    }
}
