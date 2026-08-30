//! Code-inspection artifact for file 142. Carries no timings.
//!
//! One exported symbol per case, so each arm's machine code can be read on its
//! own rather than hunted inside a 44-key dispatch. The transforms are
//! **imported** from the bench's own shared crate, so nothing here is a second
//! copy of the kernel and a change to the bench moves this artifact with it.
//!
//! The question it answers: the minimum container loses by up to 44x on a
//! clamping fold at arity, and the file claims the mechanism is one serial
//! dependence through an operator LLVM will not reassociate rather than
//! anything about the container. That claim is checkable by reading which of
//! these functions contains a vector register and which does not.

use bench_warm_clamp_shared::{fold_chunked, fold_chunked_lanes};

/// The element count is a **const** here, not a runtime argument, because that
/// is what the bench compiles. arvo carries a column's capacity as a `Cap`, so
/// the trip count is a compile-time fact; a probe passing it at run time would
/// emit a different program from the one the numbers came from, and reading it
/// would say nothing about them. Establishing that is itself one of the file's
/// measurements: the runtime-arity arm runs up to 20x slower.
const N: usize = 8192;

macro_rules! export {
    ($name:ident, $ty:ty, $body:expr) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name(p: &[$ty; N]) -> u64 {
            $body(&p[..])
        }
    };
}

// The losing arm: minimum container, eager clamp, arity 256, 16 bits.
export!(c_min_w16_a256, u16, fold_chunked::<u16, u16, 16, 256>);
// The same fold with the accumulator sized by interior safety.
export!(c_fit_w16_a256, u16, fold_chunked::<u16, u32, 16, 256>);
// The same fold with the reassociation supplied instead.
export!(c_lanes_w16_a256, u16, fold_chunked_lanes::<u16, 16, 256, 8>);
// The shipped doubled container.
export!(c_head_w16_a256, u32, fold_chunked::<u32, u32, 16, 256>);

// Arity 4, where the eager minimum form wins.
export!(c_min_w16_a4, u16, fold_chunked::<u16, u16, 16, 4>);
export!(c_fit_w16_a4, u16, fold_chunked::<u16, u32, 16, 4>);

// The exactly-filled width, where the eager clamp is one instruction.
export!(c_min_w64_a16, u64, fold_chunked::<u64, u64, 64, 16>);
export!(c_lanes_w64_a16, u64, fold_chunked_lanes::<u64, 64, 16, 8>);
