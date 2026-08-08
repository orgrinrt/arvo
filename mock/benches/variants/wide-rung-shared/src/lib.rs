//! Shared data model and transform for the wide-rung payload-shape bench.
//!
//! ## What this prices
//!
//! `seed/SETTLED_container.md:335-350` records as RATIFIED that above the
//! native container rungs a numeral's payload is **ragged** (sized to the
//! exact bit count) for `Cold` and `Precise` and **word-rounded** (whole
//! 64-bit limbs) for `Hot` and `Warm`, quoting `137b:47-53`:
//!
//! > Measured at one numeral: ragged is fourteen instructions and twenty-five
//! > bytes, word-rounded is eleven and thirty-two. Three instructions per
//! > operation against seven bytes per value is exactly the trade the strategy
//! > axis exists to carry.
//!
//! That is an instruction count at a single numeral. By this workspace's own
//! standard it is an ad-hoc quick spike, not a bench, and no committed harness
//! run reaches above a declared width of 64 bits: the widths present across
//! every key-encoded section of `bench.toml` are `[8, 13, 16, 32, 60, 64]`.
//! `20` section 3.4 names this as the largest hole in the directory and
//! section 4 names it as the cheapest to fill. This crate fills it.
//!
//! The instruction figure is not what this measures and cannot be. Three
//! instructions per operation is a statement about emitted code; whether those
//! three instructions cost anything depends on whether the loop is issue-bound
//! or bandwidth-bound, and seven bytes per value is a statement about a
//! working set whose cost depends on where it lands in the cache hierarchy.
//! Both halves are throughput questions and the harness is the instrument for
//! throughput.
//!
//! ## The arms
//!
//! Five, all producing the identical answer for the identical input, so the
//! harness's cross-variant byte comparison is live on every run.
//!
//! `ragged` and `ragged-overread` are the byte-exact payload, read two ways.
//! `wordround` is the shipped rule for `Hot` and `Warm`. `wordround-alias` is
//! the noise floor. `align16` is the vector-aligned baseline `15` section 5
//! raised and left unpriced. `load.rs` states why each is a competitor.
//!
//! ## Key encoding
//!
//! The harness dispatches a variant by one `usize` per size row, so the size
//! field is a key: `KEY = W * 1000 + NC * 100 + D`. `W` is the declared width,
//! `NC` selects the element count (0 is 2048, 1 is 458752), `D` is the
//! operations applied per element before the accumulation. So `200003` is 200
//! bits, 2048 elements, three operations.
//!
//! ## Why the answer cannot be deleted
//!
//! `20` section 2.1 found six committed cells reporting throughput 48 to 66
//! times above this host's arithmetic roofline, because a saturating fold
//! reaches an absorbing value after three elements and LLVM deletes the
//! remaining thousand iterations while the answer stays correct. The
//! cross-variant check could not see it: the answer did not depend on the
//! input, so an arm that read no data at all would have passed.
//!
//! Every operation in this bench's cycle is a **bijection** on the `W`-bit
//! residues, so no value is absorbed and the answer depends on every element.
//! `wide.rs` argues it and `every_operation_in_the_cycle_is_injective` checks
//! it. `the_answer_moves_when_any_single_element_moves` checks the consequence
//! directly, at every key the manifest declares, at the true element count
//! rather than a convenient one, and without `black_box` anywhere: `20`
//! section 2.1 found the previous family's diagnostic passing because
//! `black_box` on the operand hid the constant that made the fixpoint
//! provable, so the test proved the loop ran in a configuration the bench did
//! not use. This one runs the bench's own configuration.
//!
//! Bench infrastructure, not shipping arvo source: `std` is used freely,
//! matching every sibling variant crate in this directory.

pub mod column;
pub mod load;
pub mod oracle;
pub mod shape;
pub mod wide;

use mockspace_bench_core::Routine;

use column::{decode, key_d, key_n, key_w, Column};
use load::Loader;
use shape::{a16_bytes, rag_bytes, wr_bytes};
use wide::{operand_for, steps, wide_add, xor_operand_for};

/// The bench routine, one monomorphisation per key.
pub struct Case<const KEY: usize>;

/// The observed answer: the whole wide accumulator.
///
/// All four limbs are written, so no arm can skip the arithmetic on a limb it
/// would otherwise be free to drop, and the cross-variant byte comparison
/// covers the entire value rather than a fold of it.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WideSum {
    pub limbs: [u64; 4],
}

/// The one transform. Every arm calls this with a different loader.
#[inline(always)]
pub fn run<L: Loader, const W: u32, const D: usize>(base: *const u8, n: usize) -> [u64; 4] {
    let k = operand_for(W);
    let x2 = xor_operand_for(W);
    let mut acc = [0u64; 4];
    let mut i = 0usize;
    while i < n {
        let v = unsafe { L::load::<W>(base, i) };
        acc = wide_add::<W>(acc, steps::<W, D>(v, k, x2));
        i += 1;
    }
    acc
}

/// Region index a loader reads from. Ragged shapes share one region because
/// they share a stride; the word-rounded shapes share theirs for the same
/// reason, which is what makes the control a control.
pub const REGION_RAGGED: usize = 0;
pub const REGION_WORDROUND: usize = 1;
pub const REGION_ALIGN16: usize = 2;

impl<const KEY: usize> Routine for Case<KEY> {
    type Input = Column;
    type Output = WideSum;

    /// Unreachable at every key. `Self::Input` is 44 MB for every
    /// monomorphisation regardless of which element count the key selects, so
    /// there is no small case at which a by-value construction is safe.
    /// `routine_bridge!` takes only `build_input_bytes` as a function pointer,
    /// so the real bench path never reaches this.
    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "Case::build_input is never called by the bench path and is not safe to \
             call at any key: Self::Input is 44 MB for every monomorphisation. Use \
             build_input_bytes."
        )
    }

    fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
        column::build_bytes(KEY, seed)
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let w = key_w(KEY);
        let n = key_n(KEY);
        let d = key_d(KEY);

        let bytes = unsafe {
            core::slice::from_raw_parts(
                input as *const Column as *const u8,
                column::TOTAL_INPUT_BYTES,
            )
        };
        let from_rag = decode(bytes, REGION_RAGGED, n, rag_bytes(w), w);
        let from_wr = decode(bytes, REGION_WORDROUND, n, wr_bytes(w), w);
        let from_a16 = decode(bytes, REGION_ALIGN16, n, a16_bytes(w), w);
        if from_rag != from_wr || from_rag != from_a16 {
            return Err(
                "the three payload regions hold different logical columns, so the arms \
                 were not fed the same input and no comparison between them means anything",
            );
        }
        if oracle::reference(&from_rag, w, d) != output.limbs {
            return Err(
                "output disagrees with the independent 128-bit-radix reference, so the \
                 timed transform does not compute the declared semantics",
            );
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        (key_n(KEY) * (key_d(KEY) + 1)) as u64
    }
}

/// Dispatches one arm for a key.
///
/// `KEY` is a const generic and `generic_const_exprs` is forbidden, so the
/// width and the operation count cannot be recovered as const generic
/// arguments by arithmetic on `KEY`. They are recovered as ordinary values and
/// matched; after inlining they are constants and the match folds, so nothing
/// in the timed path branches on them. The table is generated rather than
/// written out per arm, so a width present in one arm cannot be absent from
/// another.
#[macro_export]
macro_rules! __for_each_d {
    ($l:ty, $w:literal, $d:expr, $base:expr, $n:expr) => {
        match $d {
            0 => $crate::run::<$l, $w, 0>($base, $n),
            1 => $crate::run::<$l, $w, 1>($base, $n),
            2 => $crate::run::<$l, $w, 2>($base, $n),
            3 => $crate::run::<$l, $w, 3>($base, $n),
            4 => $crate::run::<$l, $w, 4>($base, $n),
            8 => $crate::run::<$l, $w, 8>($base, $n),
            other => panic!("unsupported operation count D={}", other),
        }
    };
}

/// Declares one arm: a table from declared width to the shared transform at
/// that width, with the loader fixed.
#[macro_export]
macro_rules! declare_arm {
    ($name:ident, $loader:ty, $region:expr) => {
        /// Runs this arm for a key. Panics loudly at a width the table does
        /// not declare, rather than silently running a neighbour.
        pub fn $name(key: usize, input: &$crate::column::Column) -> [u64; 4] {
            $crate::column::assert_aligned(input);
            let n = $crate::column::key_n(key);
            let d = $crate::column::key_d(key);
            let base = unsafe {
                (input as *const $crate::column::Column as *const u8)
                    .add($region * $crate::column::REGION_BYTES)
            };
            match $crate::column::key_w(key) {
                129 => $crate::__for_each_d!($loader, 129, d, base, n),
                160 => $crate::__for_each_d!($loader, 160, d, base, n),
                192 => $crate::__for_each_d!($loader, 192, d, base, n),
                200 => $crate::__for_each_d!($loader, 200, d, base, n),
                232 => $crate::__for_each_d!($loader, 232, d, base, n),
                256 => $crate::__for_each_d!($loader, 256, d, base, n),
                other => panic!("unsupported declared width W={}", other),
            }
        }
    };
}

/// The five arms, declared side by side so the whole competitor set is one
/// screen and an asymmetry between them is visible rather than buried.
pub mod arms {
    use crate::load::{Align16, Ragged, RaggedOverread, WordRound, WordRoundAlias};

    crate::declare_arm!(ragged, Ragged, crate::REGION_RAGGED);
    crate::declare_arm!(ragged_overread, RaggedOverread, crate::REGION_RAGGED);
    crate::declare_arm!(wordround, WordRound, crate::REGION_WORDROUND);
    crate::declare_arm!(wordround_alias, WordRoundAlias, crate::REGION_WORDROUND);
    crate::declare_arm!(align16, Align16, crate::REGION_ALIGN16);
}

/// Every key this bench declares, in one list, so a test sweeps the whole
/// manifest rather than a chosen subset.
pub const ALL_KEYS: &[usize] = &[
    // width sweep, cache-resident, three operations
    129003, 160003, 192003, 200003, 232003, 256003,
    // width sweep, past L2, three operations
    129103, 160103, 192103, 200103, 232103, 256103,
    // operation-count sweep at the ratified numeral, cache-resident
    200001, 200002, 200004, 200008,
    // the bare column walk past L2: one wide operation per element, which is
    // the highest byte-to-work ratio the transform can reach and therefore the
    // only shape in which a footprint difference can show as throughput. The
    // D=3 sweep runs at 8 to 12 GB/s against this host's roughly 60, so it is
    // issue-bound on the limb arithmetic and every arm runs at the same
    // limb-op rate regardless of how many bytes it touched.
    129100, 160100, 192100, 200100, 232100, 256100,
    // the same bare walk cache-resident, as the discriminator between a
    // memory-system effect and a core one: if the width pattern in the l2
    // walk survives at 2048 elements it is not about memory at all.
    129000, 160000, 192000, 200000, 232000, 256000,
];

#[cfg(test)]
mod tests;
