//! The identical zero-inter-value-padding buffer, read through the plan the
//! width itself determines: `P = 8 / gcd(W, 8)` fields per `W * P / 8` whole
//! bytes, every byte offset and bit shift a compile-time constant. See
//! `bench-bitpack-plan-shared`.

use bench_bitpack_plan_shared::{sum_windowed, Pack, PlanColumn};
use mockspace_bench_core::{timed, FfiBenchCall, Routine};
use mockspace_bench_macro::bench_variant;

#[bench_variant(PlanColumn, "bitpack-plan-windowed", sizes = [16384, 65536, 98304, 262144])]
fn run_plan_windowed<const N: usize>(
    input: &<PlanColumn<N> as Routine>::Input,
    output: &mut <PlanColumn<N> as Routine>::Output,
) -> FfiBenchCall {
    timed! {
        run {
            // SAFETY: `packed` carries 16 bytes of headroom past the last
            // group (`MAX_PACKED_BYTES`), and every N in this bench's size
            // list is a multiple of the width-13 period, 8.
            output.value = unsafe { sum_windowed::<Pack<13>>(&input.packed, N) };
        }
    }
}
