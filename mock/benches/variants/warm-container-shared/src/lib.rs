//! Shared data model and transform for the container fork bench.
//!
//! ## The fork
//!
//! `arvo-strategy/src/container.rs:15-19` sizes the `Warm` and `Precise`
//! container at one rung above the declared width `W`: `1..=8 -> u16`,
//! `9..=16 -> u32`, `17..=32 -> u64`, `33..=64 -> u128`. Written as a
//! function that is `rung(rung_bits(W) + 1)`, a rounding applied to a
//! rounding, so every width at or below 64 bits is stored in a container
//! twice the size of the minimum that holds it. `Hot` and `Cold` take the
//! minimum (`tag_hot_cold`, same file `:60-75`).
//!
//! Two panel dispatches argued about replacing that rule and neither priced
//! it (`139_ovadia_the_derivations_that_stop_short.md`,
//! `140_fog_warm_without_widening.md`, and op's disposition at
//! `140b_op_checkpoint_thirtythree.md:69-71`). This crate is the pricing.
//!
//! ## The arms, and why each one is a competitor rather than a strawman
//!
//! Four arms, all producing the identical output value for the identical
//! input, so the harness's cross-variant byte comparison is live on every
//! run and an arm that computes something else is refused rather than
//! reported as fast.
//!
//! `headroom` is the shipped rule. The value lives in `rung(rung_bits(W)+1)`
//! and the transform projects the result back to `W` after every operation,
//! because `Warm` wraps at `W` and a container of `C > W` wraps at `C`, so
//! the projection is required whatever the container is.
//!
//! `minimum` is the deletion `140` proposed. Same transform, same
//! projection, container is the minimum native that holds `W`.
//!
//! `plusone` is `rung(W+1)`, composing before rounding rather than after.
//! It is in the arm set because the dispatching brief named it, and running
//! it establishes what reading the rule already tells you: `rung(W+1)`
//! equals `rung(rung_bits(W)+1)` at every exactly-filled width and equals
//! `rung(W)` at every other, so it is never a third container. It is kept
//! as a **control**: at each width it compiles to byte-identical code to
//! one of the other two arms, so the spread between `plusone` and the arm
//! it aliases is this harness's own noise floor on this workload, and any
//! difference between the real arms smaller than that gap is not signal.
//!
//! `native` is the bar op's own definition of `Warm` sets
//! (`140b:16-21`, "It should behave like native primitives in regular old
//! rust would"). Minimum container, and the projection written **once**,
//! before the value is observed, which is what a Rust programmer who needed
//! `W`-bit wrapping would write by hand. Against `minimum` it isolates
//! exactly one thing: the cost of writing the projection after every
//! operation rather than once. `140:176-206` claims the compiler sinks the
//! redundant ones and offers symbol identity as evidence; `native` against
//! `minimum` is that claim measured as throughput instead of read off an
//! assembler alias.
//!
//! ## Key encoding
//!
//! The harness dispatches a variant by a single `usize` per size row, and
//! this bench varies four things, so the size field is a key:
//!
//! `KEY = W * 10000 + NC * 1000 + OP * 100 + D`
//!
//! `W` is the declared width in bits. `NC` selects the element count (0 is
//! 8192, small enough that both containers stay inside this host's 128 KB
//! L1 at every swept width; 1 is 1048576, where the minimum container fits
//! this host's 12 MiB L2 at `W = 64` and the headroom container does not).
//! `OP` selects the semantics (0 wrapping, for `Warm`; 1 saturating, for
//! `Precise`). `D` is the arithmetic operations applied per element before
//! the accumulation, which is the axis `140:339-343` names as the one no
//! instruction count can settle.
//!
//! So `130003` is 13 bits, 8192 elements, wrapping, three operations.
//!
//! ## Why the step cycle contains an exclusive or
//!
//! The first cut of this bench cycled add, multiply-by-three, subtract. Every
//! one of those is affine in the value with constant coefficients, so a chain
//! of any length composes to a single multiply-add, and LLVM performed that
//! composition: at `D = 8` the lazily-projected form emitted **one** `mla`
//! per vector rather than eight, which the committed inspection artifact at
//! `141_probes/` shows directly. The eager form could not collapse, because
//! the mask between operations is not affine. So the density sweep as first
//! built compared a collapsed chain against an uncollapsed one and measured
//! the optimiser rather than the density, and its numbers were discarded.
//!
//! The exclusive or breaks the composition while preserving the property the
//! `native` arm depends on: exclusive or is bitwise, so it commutes with
//! reduction modulo `2^W` for any operand below `2^W`, and the eager and lazy
//! forms still agree. `eager_and_lazy_wrapping_agree_everywhere` asserts that
//! rather than assuming it.
//!
//! ## One transform, four containers
//!
//! `run_wrap` and `run_sat` below are the only transforms. Every arm calls
//! one of them with a different carrier type and a different `EAGER` flag;
//! no arm carries its own copy of the arithmetic. A bench whose arms each
//! re-derive the kernel measures four possibly-drifted programs rather than
//! one program in four containers.
//!
//! Bench infrastructure, not shipping arvo source: `std` is used freely,
//! matching every sibling variant crate in this directory.

use mockspace_bench_core::Routine;

// ---------------------------------------------------------------------------
// Key decoding.
// ---------------------------------------------------------------------------

/// Declared width in bits.
pub const fn key_w(key: usize) -> u32 {
    (key / 10_000) as u32
}
/// Element-count class: 0 is `N_SMALL`, 1 is `N_LARGE`.
pub const fn key_nc(key: usize) -> usize {
    (key / 1_000) % 10
}
/// Semantics: 0 is a wrapping reduction (`Warm`), 1 is a saturating reduction
/// (`Precise`), 2 is a wrapping elementwise transform (`Warm`, no loop-carried
/// arvo value), 3 is a saturating elementwise transform (`Precise`, likewise),
/// 4 is a wrapping reduction whose steps are all affine, 5 is a saturating
/// accumulation of a `W`-bit column into a 64-bit accumulator.
pub const fn key_op(key: usize) -> usize {
    (key / 100) % 10
}
/// Arithmetic operations applied per element before the accumulation.
pub const fn key_d(key: usize) -> usize {
    key % 100
}

/// Small element count. At `W = 64` the minimum container occupies 64 KB
/// and the headroom container 128 KB, so both are cache-resident on this
/// host and the reading is compute-bound.
pub const N_SMALL: usize = 8_192;
/// Large element count. At `W = 64` the minimum container occupies 8 MiB
/// (inside this host's 12 MiB L2) and the headroom container 16 MiB
/// (outside it), which is the crossover the footprint half of the fork
/// lives at.
pub const N_LARGE: usize = 1_048_576;

pub const fn key_n(key: usize) -> usize {
    if key_nc(key) == 0 {
        N_SMALL
    } else {
        N_LARGE
    }
}

/// The rung a width lands on: the width of the smallest native integer
/// that holds it.
pub const fn rung_bits(w: u32) -> u32 {
    if w <= 8 {
        8
    } else if w <= 16 {
        16
    } else if w <= 32 {
        32
    } else if w <= 64 {
        64
    } else {
        128
    }
}

/// Bytes in the minimum native container for `w` bits.
pub const fn min_bytes_for(w: u32) -> usize {
    (rung_bits(w) / 8) as usize
}
/// Bytes in the shipped `Warm` / `Precise` container for `w` bits:
/// `rung(rung_bits(w) + 1)`.
pub const fn head_bytes_for(w: u32) -> usize {
    (rung_bits(rung_bits(w) + 1) / 8) as usize
}
/// Bytes in `rung(w + 1)`, the compose-before-rounding candidate.
pub const fn plus_bytes_for(w: u32) -> usize {
    (rung_bits(w + 1) / 8) as usize
}

// ---------------------------------------------------------------------------
// Carrier.
// ---------------------------------------------------------------------------

/// A machine integer a numeral of some declared width can live in.
///
/// Every method is `#[inline(always)]` so that a caller passing a constant
/// `w` folds the width-dependent branches away. The branches exist because
/// `1 << BITS` is not representable: at an exactly-filled width the mask is
/// the whole container and there is nothing to emit, which is the case the
/// whole fork turns on.
pub trait Carrier: Copy + 'static {
    const BITS: u32;
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;

    fn from_u128(v: u128) -> Self;
    fn to_u64(self) -> u64;

    fn wadd(self, o: Self) -> Self;
    fn wsub(self, o: Self) -> Self;
    fn wmul(self, o: Self) -> Self;
    fn bxor(self, o: Self) -> Self;

    fn sat_add(self, o: Self) -> Self;
    fn sat_sub(self, o: Self) -> Self;
    fn min_with(self, o: Self) -> Self;

    /// `2^w - 1` in this carrier, or `Self::MAX` when `w` fills it.
    fn limit(w: u32) -> Self;
    /// `self mod 2^w`, a no-op when `w` fills the carrier.
    fn mask_to(self, w: u32) -> Self;
}

macro_rules! impl_carrier {
    ($t:ty) => {
        impl Carrier for $t {
            const BITS: u32 = <$t>::BITS;
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MAX: Self = <$t>::MAX;

            #[inline(always)]
            fn from_u128(v: u128) -> Self {
                v as $t
            }
            #[inline(always)]
            fn to_u64(self) -> u64 {
                self as u64
            }
            #[inline(always)]
            fn wadd(self, o: Self) -> Self {
                self.wrapping_add(o)
            }
            #[inline(always)]
            fn wsub(self, o: Self) -> Self {
                self.wrapping_sub(o)
            }
            #[inline(always)]
            fn wmul(self, o: Self) -> Self {
                self.wrapping_mul(o)
            }
            #[inline(always)]
            fn bxor(self, o: Self) -> Self {
                self ^ o
            }
            #[inline(always)]
            fn sat_add(self, o: Self) -> Self {
                self.saturating_add(o)
            }
            #[inline(always)]
            fn sat_sub(self, o: Self) -> Self {
                self.saturating_sub(o)
            }
            #[inline(always)]
            fn min_with(self, o: Self) -> Self {
                if self < o {
                    self
                } else {
                    o
                }
            }
            #[inline(always)]
            fn limit(w: u32) -> Self {
                if w >= <$t>::BITS {
                    <$t>::MAX
                } else {
                    (1 as $t).wrapping_shl(w).wrapping_sub(1)
                }
            }
            #[inline(always)]
            fn mask_to(self, w: u32) -> Self {
                if w >= <$t>::BITS {
                    self
                } else {
                    self & (1 as $t).wrapping_shl(w).wrapping_sub(1)
                }
            }
        }
    };
}

impl_carrier!(u8);
impl_carrier!(u16);
impl_carrier!(u32);
impl_carrier!(u64);
impl_carrier!(u128);

// ---------------------------------------------------------------------------
// The two transforms. One definition each; every arm calls one of them.
// ---------------------------------------------------------------------------

/// The per-element operand. Derived from `W` alone so every arm at a given
/// width sees the same numeric stream, and odd so the multiply step cannot
/// be folded into a shift by a lucky constant.
pub const fn operand_for(w: u32) -> u128 {
    let lim: u128 = if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    };
    (lim / 3) | 1
}

/// The xor operand. Strictly below `2^W`, which is what makes the exclusive
/// or commute with the projection: `(a ^ k2) mod 2^W == (a mod 2^W) ^ k2`
/// holds bitwise as long as `k2` has no bits at or above `W`.
pub const fn xor_operand_for(w: u32) -> u128 {
    let lim: u128 = if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    };
    (lim / 7) ^ 0b1011
}

/// Wrapping-at-`W` transform. `D` operations per element, then a wrapping
/// accumulation, then one projection before the value is observed.
///
/// With `EAGER` the projection is written after every operation, which is
/// what arvo would emit. Without it the projection is written once at the
/// end, which is what a Rust programmer writes by hand. The two agree for
/// `+`, `-` and `*` because reduction modulo `2^W` factors through
/// reduction modulo `2^C` whenever `W <= C`, which is the property the
/// `native` arm exists to test at throughput rather than by symbol
/// identity.
#[inline(always)]
pub fn run_wrap<C: Carrier, const W: u32, const D: usize, const EAGER: bool>(
    data: &[C],
    k: C,
) -> u64 {
    let x2 = C::from_u128(xor_operand_for(W));
    let mut acc = C::ZERO;
    for &x in data {
        acc = acc.wadd(wrap_steps::<C, W, D, EAGER>(x, k, x2));
        if EAGER {
            acc = acc.mask_to(W);
        }
    }
    acc.mask_to(W).to_u64()
}

/// Saturating-at-`W` transform, for `Precise`.
///
/// Saturation is not a ring homomorphism, so there is no lazy form and no
/// `EAGER` parameter: every arm clamps after every operation. Operations
/// alternate saturating add and saturating subtract; multiplication is left
/// out because a saturating multiply's cost is dominated by its overflow
/// detection in every container alike and is not what the fork is about.
///
/// At an exactly-filled width `limit(W)` is `C::MAX`, so the `min_with`
/// folds away and the body is the machine's own saturating add. Below the
/// rung the carrier's spare bits make `saturating_add` unable to overflow,
/// so it folds to a plain add and the clamp is the `min_with`. Both shapes
/// fall out of the same source.
#[inline(always)]
pub fn run_sat<C: Carrier, const W: u32, const D: usize>(data: &[C], k: C) -> u64 {
    let lim = C::limit(W);
    let mut acc = C::ZERO;
    for &x in data {
        acc = acc.sat_add(sat_steps::<C, D>(x, k, lim)).min_with(lim);
    }
    acc.to_u64()
}

/// The per-element wrapping step sequence, factored so the serial fold and
/// the lane-parallel fold share one definition of the arithmetic.
#[inline(always)]
fn wrap_steps<C: Carrier, const W: u32, const D: usize, const EAGER: bool>(
    mut v: C,
    k: C,
    x2: C,
) -> C {
    let mut j = 0usize;
    while j < D {
        v = match j % 4 {
            0 => v.wadd(k),
            1 => v.wmul(C::from_u128(3)),
            2 => v.wsub(k),
            _ => v.bxor(x2),
        };
        if EAGER {
            v = v.mask_to(W);
        }
        j += 1;
    }
    v
}

/// The kernel answer for a wrapping fold that keeps its projection eager:
/// eight independent lane accumulators, each projected after every
/// accumulation, combined at the end.
///
/// This is the arm that tests what arvo knows and LLVM does not.
/// `(a + b) mod 2^W` is associative and commutative, so the fold may be
/// reassociated into lanes. LLVM does not do it: the emitted code for the
/// single-accumulator form is a six-instruction serial scalar loop at every
/// sub-rung width, because `and` interposed in the accumulation is not a
/// reduction operator it recognises. arvo's typestate knows the operator's
/// algebra from the declared semantics, statically, with nothing to infer,
/// so it can emit this shape where the compiler will not.
///
/// If this arm reaches the deferred-projection arm's throughput, the
/// sub-rung reduction has two independent fixes rather than one, and the
/// projection does not have to be deferred to get the vector form.
#[inline(always)]
pub fn run_wrap_lanes<C: Carrier, const W: u32, const D: usize, const EAGER: bool>(
    data: &[C],
    k: C,
) -> u64 {
    const L: usize = 8;
    let x2 = C::from_u128(xor_operand_for(W));
    let mut lanes = [C::ZERO; L];
    let chunks = data.chunks_exact(L);
    let rem = chunks.remainder();
    for ch in chunks {
        let mut l = 0usize;
        while l < L {
            lanes[l] = lanes[l].wadd(wrap_steps::<C, W, D, EAGER>(ch[l], k, x2));
            if EAGER {
                lanes[l] = lanes[l].mask_to(W);
            }
            l += 1;
        }
    }
    let mut acc = C::ZERO;
    let mut l = 0usize;
    while l < L {
        acc = acc.wadd(lanes[l]);
        if EAGER {
            acc = acc.mask_to(W);
        }
        l += 1;
    }
    for &x in rem {
        acc = acc.wadd(wrap_steps::<C, W, D, EAGER>(x, k, x2));
        if EAGER {
            acc = acc.mask_to(W);
        }
    }
    acc.mask_to(W).to_u64()
}

/// The kernel's saturating step sequence, using a range fact the machine
/// code cannot carry.
///
/// arvo knows statically that every value of a `W`-bit numeral is below
/// `2^W`, and it knows the container width. When `W < C` it follows that
/// `v + k < 2^(W+1) <= 2^C`, so the addition cannot overflow the container
/// and the entire saturation is the clamp. `saturating_add` still emits its
/// overflow detection, because LLVM sees a load from memory and must assume
/// the full range of the machine type; the fact that the column only ever
/// holds `W`-bit values is a property of arvo's typestate that does not
/// survive into the loaded value.
///
/// So this is the shape of the shortcut a heavier typestate buys: not a
/// better algorithm, but permission to omit a check the compiler is obliged
/// to keep. At `W == C` the fact is not available, the addition genuinely can
/// overflow, and the kernel falls back to the machine's own saturating add,
/// which at that width is the whole operation in one instruction. Which of
/// the two applies is decided by a const comparison, so both arms of the
/// choice are compiled and exactly one survives.
///
/// The subtraction gets no such help: `v - k` saturating at zero needs the
/// comparison whatever the range, since zero is inside it.
#[inline(always)]
fn sat_steps_witness<C: Carrier, const W: u32, const D: usize>(mut v: C, k: C, lim: C) -> C {
    let mut j = 0usize;
    while j < D {
        v = if j.is_multiple_of(2) {
            if W < C::BITS {
                v.wadd(k).min_with(lim)
            } else {
                v.sat_add(k)
            }
        } else {
            v.sat_sub(k)
        };
        j += 1;
    }
    v
}

/// Saturating elementwise transform using the range witness above.
#[inline(always)]
pub fn run_sat_elementwise_witness<C: Carrier, const W: u32, const D: usize>(
    data: &[C],
    k: C,
) -> u64 {
    let lim = C::limit(W);
    let mut chk = C::ZERO;
    for &x in data {
        chk = chk.bxor(sat_steps_witness::<C, W, D>(x, k, lim));
    }
    chk.to_u64()
}

/// The per-element saturating step sequence, factored so the serial fold and
/// the lane-parallel fold share one definition of the arithmetic and differ
/// only in how the results are combined.
#[inline(always)]
fn sat_steps<C: Carrier, const D: usize>(mut v: C, k: C, lim: C) -> C {
    let mut j = 0usize;
    while j < D {
        v = if j.is_multiple_of(2) {
            v.sat_add(k).min_with(lim)
        } else {
            v.sat_sub(k)
        };
        j += 1;
    }
    v
}

/// The kernel answer for a saturating fold: eight independent lane
/// accumulators combined at the end, instead of one serial accumulator.
///
/// Legal because unsigned saturation is a clamp of the true sum, so the fold
/// is associative and commutative: `min(min(a + b, M) + c, M)` is
/// `min(a + b + c, M)` in both the clamped and unclamped case, and the
/// combining order therefore cannot change the answer. That is what lets the
/// operation be reassociated at all, and it is a property of the semantics
/// rather than a liberty taken by the bench, which is why the harness's
/// cross-variant comparison and
/// `all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key`
/// are what check it rather than a tolerance.
///
/// The point is what the reassociation reaches. `run_sat`'s single
/// accumulator is a loop-carried dependence, so the loop is serial in every
/// container. Eight independent ones are not, and at a width that exactly
/// fills its container `sat_add` is the machine's own saturating add, which
/// this target has as a lane-parallel instruction. Nothing built over a
/// wider container can be that instruction, so this is the cell where the
/// container choice and the kernel choice are the same decision.
#[inline(always)]
pub fn run_sat_lanes<C: Carrier, const W: u32, const D: usize>(data: &[C], k: C) -> u64 {
    const L: usize = 8;
    let lim = C::limit(W);
    let mut lanes = [C::ZERO; L];
    let chunks = data.chunks_exact(L);
    let rem = chunks.remainder();
    for ch in chunks {
        let mut l = 0usize;
        while l < L {
            lanes[l] = lanes[l]
                .sat_add(sat_steps::<C, D>(ch[l], k, lim))
                .min_with(lim);
            l += 1;
        }
    }
    let mut acc = C::ZERO;
    let mut l = 0usize;
    while l < L {
        acc = acc.sat_add(lanes[l]).min_with(lim);
        l += 1;
    }
    for &x in rem {
        acc = acc.sat_add(sat_steps::<C, D>(x, k, lim)).min_with(lim);
    }
    acc.to_u64()
}

/// Wrapping transform whose every step is affine in the value with constant
/// coefficients: add, multiply by three, subtract, repeat. No exclusive or.
///
/// This is the workload the main sweep deliberately avoids, and it is here
/// because avoiding it hid the largest single effect in this investigation.
/// A chain of affine steps composes to one multiply-add, by ordinary algebra.
/// LLVM performs that composition, but only when it can see the chain: with
/// a projection between the steps it cannot, because a mask is not affine and
/// the compiler has no license to move it. arvo does have that license, since
/// reduction modulo `2^W` is a ring homomorphism, so removing the interior
/// projections is semantics-preserving and it is what hands the chain to the
/// optimiser in a form it can collapse.
///
/// So this pair does not measure the cost of a mask. It measures what the
/// mask was preventing, which is an entire algebraic simplification, and the
/// simplification is worth more than the instruction removed. This is the
/// clearest instance in this file of a law arvo holds statically buying a
/// rewrite the compiler cannot reach on its own.
///
/// A fixed-point filter, a colour transform and a scale-and-bias pass are all
/// this shape, so it is not a contrived case.
#[inline(always)]
pub fn run_wrap_affine<C: Carrier, const W: u32, const D: usize, const EAGER: bool>(
    data: &[C],
    k: C,
) -> u64 {
    let mut acc = C::ZERO;
    for &x in data {
        let mut v = x;
        let mut j = 0usize;
        while j < D {
            v = match j % 3 {
                0 => v.wadd(k),
                1 => v.wmul(C::from_u128(3)),
                _ => v.wsub(k),
            };
            if EAGER {
                v = v.mask_to(W);
            }
            j += 1;
        }
        acc = acc.wadd(v);
        if EAGER {
            acc = acc.mask_to(W);
        }
    }
    acc.mask_to(W).to_u64()
}

/// Saturating accumulation of a `W`-bit column into a 64-bit saturating
/// accumulator, with and without the theorem that says the saturation cannot
/// happen.
///
/// The theorem: every element of the column is below `2^W`, and there are `N`
/// of them, so the exact sum is below `N * 2^W <= 2^(W + ceil(log2 N))`. When
/// that bound is at or below the accumulator's own width, the accumulation
/// cannot reach the saturation point, every clamp in the fold is dead, and
/// the operation is a plain wrapping sum with identical results.
///
/// Both facts the theorem needs are things arvo holds statically and the
/// machine code cannot carry. The element bound is the declared width, which
/// survives into the type and not into the loaded value: LLVM sees a load
/// from a `u16` and must assume the full range. The count bound is the column
/// capacity, which arvo carries as a `Cap` and which LLVM sees as a runtime
/// slice length.
///
/// What it buys is not one instruction. A saturating fold is a loop-carried
/// dependence through an operation LLVM will not reassociate, so it is serial
/// at every width and in every container. Deleting the saturation turns it
/// into a plain wrapping reduction, which vectorises. The theorem does not
/// make the loop cheaper, it changes which loop is being compiled.
///
/// `THEOREM` selects the two forms. The predicate that decides whether it may
/// be used is `theorem_applies` below, and at the widths where it does not
/// apply the honest form is the only correct one, so this is a cell of a
/// compile-time table rather than a blanket rewrite.
#[inline(always)]
pub fn run_sat_widening<C: Carrier, const W: u32, const THEOREM: bool>(data: &[C]) -> u64 {
    let mut acc: u64 = 0;
    if THEOREM {
        for &x in data {
            acc = acc.wrapping_add(x.to_u64());
        }
    } else {
        for &x in data {
            acc = acc.saturating_add(x.to_u64());
        }
    }
    acc
}

/// Whether the no-saturation theorem holds for a `W`-bit column of `n`
/// elements accumulated at 64 bits: `W + ceil(log2 n) <= 64`.
///
/// A `const fn` of exactly the two quantities the typestate already carries,
/// which is what makes this a static decision rather than a runtime check.
pub const fn theorem_applies(w: u32, n: usize) -> bool {
    let mut bits = 0u32;
    let mut m = n - 1;
    while m > 0 {
        bits += 1;
        m >>= 1;
    }
    w + bits <= 64
}

/// Saturating elementwise transform, no loop-carried arvo value.
///
/// This is the shape the saturating question is actually about. A saturating
/// **fold** turned out to be untestable as first written: with an operand
/// near a third of the container's range the accumulator pins at the limit
/// after a handful of elements and the answer stops depending on the input,
/// which LLVM noticed, proving the whole loop constant and deleting it. The
/// evidence is in the emitted code, where the `D = 1` and `D = 3` cases jump
/// straight to the epilogue with `x0 = -1` and never touch the data.
///
/// Elementwise, each result depends on its own element and the exclusive-or
/// checksum depends on all of them, so nothing here is constant. It is also
/// the shape `140:222-235` is about: whether a width that exactly fills its
/// container reaches the machine's lane-parallel saturating add, which is a
/// per-element question rather than a reduction one.
#[inline(always)]
pub fn run_sat_elementwise<C: Carrier, const W: u32, const D: usize>(data: &[C], k: C) -> u64 {
    let lim = C::limit(W);
    let mut chk = C::ZERO;
    for &x in data {
        chk = chk.bxor(sat_steps::<C, D>(x, k, lim));
    }
    chk.to_u64()
}

/// Elementwise transform, no loop-carried arvo value.
///
/// The same per-element step sequence as `run_wrap`, with the accumulation
/// removed. Each element is projected to `W` before it leaves, and the
/// results are folded into an exclusive-or checksum at the container width,
/// which stands in for a store: it is not an arvo value, so it carries no
/// projection of its own and puts nothing on the critical path. Every
/// element is below `2^W` by the time it is folded, so the checksum is the
/// same number in every carrier and the arms stay byte-comparable.
///
/// This is the regime the reduction benches do not cover, and it is a
/// different one: with nothing carried between iterations the eager
/// projection does not block vectorisation in any container, so what is left
/// to measure is the per-operation cost of the projection and the lane count
/// the container leaves.
#[inline(always)]
pub fn run_elementwise<C: Carrier, const W: u32, const D: usize, const EAGER: bool>(
    data: &[C],
    k: C,
) -> u64 {
    let x2 = C::from_u128(xor_operand_for(W));
    let mut chk = C::ZERO;
    for &x in data {
        let mut v = x;
        let mut j = 0usize;
        while j < D {
            v = match j % 4 {
                0 => v.wadd(k),
                1 => v.wmul(C::from_u128(3)),
                2 => v.wsub(k),
                _ => v.bxor(x2),
            };
            if EAGER {
                v = v.mask_to(W);
            }
            j += 1;
        }
        chk = chk.bxor(v.mask_to(W));
    }
    chk.to_u64()
}

// ---------------------------------------------------------------------------
// Input layout.
// ---------------------------------------------------------------------------

/// Bytes reserved for the headroom region: `N_LARGE` elements at the widest
/// headroom carrier this bench reaches (`u128`, at `W` in 33..=64).
pub const HEAD_BYTES: usize = N_LARGE * 16;
/// Bytes reserved for the minimum region: `N_LARGE` elements at the widest
/// minimum carrier this bench reaches (`u64`, at `W` in 33..=64).
pub const MIN_BYTES: usize = N_LARGE * 8;
pub const TOTAL_INPUT_BYTES: usize = HEAD_BYTES + MIN_BYTES;

/// The same logical column stored twice, once at the headroom container's
/// stride and once at the minimum container's, so each arm reads a
/// contiguous array of its own carrier and the footprint difference between
/// the two rules is a real difference in bytes touched rather than a
/// conversion inserted at the load.
///
/// Both regions are declared at `N_LARGE` and at the widest carrier
/// regardless of which key a monomorphisation represents, for the reason
/// `bench-bitpack-footprint-shared` documents at its own `FootprintColumn`:
/// a struct field length that is an expression of the struct's own const
/// generic needs `generic_const_exprs`, which is forbidden. Only the
/// `N`-proportional prefix of each region is ever written or read.
///
/// `align(16)` so a `u128` view of the headroom region is aligned. The
/// minimum region starts at `HEAD_BYTES`, a multiple of 16, so it is
/// aligned too. Whether the pointer the harness hands over is itself
/// 16-aligned is checked at run time by `assert_aligned` below rather than
/// assumed.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Cols {
    pub head: [u8; HEAD_BYTES],
    pub min: [u8; MIN_BYTES],
}

impl Cols {
    /// Typed view of the headroom region. `C` must match
    /// `head_bytes_for(w)`; the tests below assert that for every swept
    /// width and the arm dispatchers pick `C` from the same table.
    ///
    /// # Safety
    /// Caller guarantees `n * size_of::<C>() <= HEAD_BYTES` and that the
    /// region holds initialised values at `C`'s stride.
    #[inline(always)]
    pub fn head_slice<C: Carrier>(&self, n: usize) -> &[C] {
        assert!(n * core::mem::size_of::<C>() <= HEAD_BYTES);
        unsafe { core::slice::from_raw_parts(self.head.as_ptr() as *const C, n) }
    }

    /// Typed view of the minimum region. See `head_slice`.
    #[inline(always)]
    pub fn min_slice<C: Carrier>(&self, n: usize) -> &[C] {
        assert!(n * core::mem::size_of::<C>() <= MIN_BYTES);
        unsafe { core::slice::from_raw_parts(self.min.as_ptr() as *const C, n) }
    }
}

/// Fails loudly if the harness handed over a buffer that is not aligned for
/// `Cols`. Called once per timed call, outside the element loop, so it does
/// not enter the per-element cost. A silently misaligned `u128` view would
/// be undefined behaviour that this target happens to tolerate, which is
/// exactly the kind of thing that reads as a valid measurement.
#[inline(always)]
pub fn assert_aligned(cols: &Cols) {
    let addr = cols as *const Cols as usize;
    assert!(
        addr.is_multiple_of(16),
        "harness handed an input buffer at {addr:#x}, which is not 16-aligned; \
         the u128 view of the headroom region would be unsound and every number \
         from this run is void"
    );
}

/// The bench routine, one monomorphisation per key.
pub struct Case<const KEY: usize>;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sum {
    pub value: u64,
}

struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Writes `v` little-endian into `buf[off..off + bytes]`.
fn store_le(buf: &mut [u8], off: usize, bytes: usize, v: u128) {
    let le = v.to_le_bytes();
    buf[off..off + bytes].copy_from_slice(&le[..bytes]);
}

/// Reads a little-endian value of `bytes` width from `buf[off..]`.
fn load_le(buf: &[u8], off: usize, bytes: usize) -> u128 {
    let mut le = [0u8; 16];
    le[..bytes].copy_from_slice(&buf[off..off + bytes]);
    u128::from_le_bytes(le)
}

/// The independent oracle. Computes the same result in `u128` with explicit
/// modular or clamping arithmetic, sharing no code with `run_wrap` or
/// `run_sat` and no carrier type with any arm, so a bug inside the `Carrier`
/// impls or the width-dependent branches is visible to validation rather
/// than agreed with by it.
pub fn reference(values: &[u128], w: u32, op: usize, d: usize) -> u64 {
    let m: u128 = if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    };
    let k = operand_for(w);
    let x2 = xor_operand_for(w);
    let mut acc: u128 = 0;
    for &x in values {
        let mut v = x;
        if op == 0 {
            for j in 0..d {
                v = match j % 4 {
                    0 => (v + k) & m,
                    1 => (v.wrapping_mul(3)) & m,
                    2 => (v + m + 1 - k) & m,
                    _ => (v ^ x2) & m,
                };
            }
            acc = (acc + v) & m;
        } else if op == 1 {
            for j in 0..d {
                v = if j % 2 == 0 {
                    let s = v + k;
                    if s > m {
                        m
                    } else {
                        s
                    }
                } else { v.saturating_sub(k) };
            }
            let s = acc + v;
            acc = if s > m { m } else { s };
        } else if op == 2 {
            for j in 0..d {
                v = match j % 4 {
                    0 => (v + k) & m,
                    1 => (v.wrapping_mul(3)) & m,
                    2 => (v + m + 1 - k) & m,
                    _ => (v ^ x2) & m,
                };
            }
            acc ^= v;
        } else if op == 3 {
            for j in 0..d {
                v = if j % 2 == 0 {
                    let s = v + k;
                    if s > m {
                        m
                    } else {
                        s
                    }
                } else { v.saturating_sub(k) };
            }
            acc ^= v;
        } else if op == 4 {
            for j in 0..d {
                v = match j % 3 {
                    0 => (v + k) & m,
                    1 => (v.wrapping_mul(3)) & m,
                    _ => (v + m + 1 - k) & m,
                };
            }
            acc = (acc + v) & m;
        } else {
            // op 5: saturating accumulation at 64 bits, elements untransformed.
            let s = acc + v;
            acc = if s > (u64::MAX as u128) {
                u64::MAX as u128
            } else {
                s
            };
        }
    }
    acc as u64
}

/// Decodes the logical column back out of the minimum region.
pub fn decode_min(buf: &[u8], n: usize, w: u32) -> std::vec::Vec<u128> {
    let b = min_bytes_for(w);
    (0..n)
        .map(|i| load_le(buf, HEAD_BYTES + i * b, b))
        .collect()
}

/// Decodes the logical column back out of the headroom region.
pub fn decode_head(buf: &[u8], n: usize, w: u32) -> std::vec::Vec<u128> {
    let b = head_bytes_for(w);
    (0..n).map(|i| load_le(buf, i * b, b)).collect()
}

/// Builds one input buffer for a key. Free rather than a method on
/// `Case<KEY>` so a test can sweep every key in `ALL_KEYS` at run time;
/// `Case::build_input_bytes` is a one-line forward to it, so the bench path
/// and the tests construct input through the same code.
///
/// The buffer is over-aligned to 16 by construction: it is built as a
/// `Vec<u128>` and reinterpreted, because `Cols` needs 16-byte alignment for
/// its `u128` view and a plain `Vec<u8>` promises only 1.
pub fn build_bytes(key: usize, seed: u64) -> std::vec::Vec<u8> {
    let w = key_w(key);
    let n = key_n(key);
    let mb = min_bytes_for(w);
    let hb = head_bytes_for(w);
    let m: u128 = (1u128 << w) - 1;

    let mut rng = SplitMix64(seed ^ 0x00C0_FFEE_0BAD_F00D);
    let mut buf = std::vec![0u8; TOTAL_INPUT_BYTES];
    for i in 0..n {
        let v = (rng.next() as u128) & m;
        store_le(&mut buf, i * hb, hb, v);
        store_le(&mut buf, HEAD_BYTES + i * mb, mb, v);
    }
    buf
}

impl<const KEY: usize> Routine for Case<KEY> {
    type Input = Cols;
    type Output = Sum;

    /// Unreachable at every key. `Self::Input` is 24 MiB for every
    /// monomorphisation regardless of which element count the key selects,
    /// so there is no small case at which a by-value construction is safe;
    /// `bench-bitpack-footprint-shared` records the same fact about its own
    /// input and the same remedy. `routine_bridge!` takes only
    /// `build_input_bytes` as a function pointer, so the real bench path
    /// never reaches this.
    fn build_input(_seed: u64) -> Self::Input {
        unreachable!(
            "Case::build_input is never called by the bench path and is not safe \
             to call at any key: Self::Input is 24 MiB for every monomorphisation. \
             Use build_input_bytes."
        )
    }

    fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
        build_bytes(KEY, seed)
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let w = key_w(KEY);
        let n = key_n(KEY);
        let d = key_d(KEY);
        let op = key_op(KEY);
        let mb = min_bytes_for(w);
        let hb = head_bytes_for(w);

        let from_min: std::vec::Vec<u128> =
            (0..n).map(|i| load_le(&input.min, i * mb, mb)).collect();
        let from_head: std::vec::Vec<u128> =
            (0..n).map(|i| load_le(&input.head, i * hb, hb)).collect();
        if from_min != from_head {
            return Err(
                "the two carrier regions hold different logical columns, so the arms \
                 were not fed the same input and no comparison between them means anything",
            );
        }
        if reference(&from_min, w, op, d) != output.value {
            return Err(
                "output disagrees with the independent u128 reference, so the timed \
                 transform does not compute the declared semantics",
            );
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        (key_n(KEY) * (key_d(KEY) + 1)) as u64
    }
}

// ---------------------------------------------------------------------------
// Arm dispatch.
//
// `KEY` is a const generic and arvo forbids `generic_const_exprs`, so the
// width and density cannot be recovered as const generic arguments by
// arithmetic on `KEY`. They are recovered as ordinary values and matched;
// after inlining they are constants and the match folds, so nothing in the
// timed path branches on them. The nested match is generated rather than
// written out, so all four arms share one dispatch shape and a width
// present in one arm cannot be absent from another.
// ---------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d {
    ($run:ident, $c:ty, $w:literal, $eager:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::$run::<$c, $w, 1, $eager>($slice, $k),
            2 => $crate::$run::<$c, $w, 2, $eager>($slice, $k),
            3 => $crate::$run::<$c, $w, 3, $eager>($slice, $k),
            4 => $crate::$run::<$c, $w, 4, $eager>($slice, $k),
            8 => $crate::$run::<$c, $w, 8, $eager>($slice, $k),
            16 => $crate::$run::<$c, $w, 16, $eager>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_sat {
    ($c:ty, $w:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_sat::<$c, $w, 1>($slice, $k),
            2 => $crate::run_sat::<$c, $w, 2>($slice, $k),
            3 => $crate::run_sat::<$c, $w, 3>($slice, $k),
            4 => $crate::run_sat::<$c, $w, 4>($slice, $k),
            8 => $crate::run_sat::<$c, $w, 8>($slice, $k),
            16 => $crate::run_sat::<$c, $w, 16>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_ew {
    ($c:ty, $w:literal, $eager:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_elementwise::<$c, $w, 1, $eager>($slice, $k),
            2 => $crate::run_elementwise::<$c, $w, 2, $eager>($slice, $k),
            3 => $crate::run_elementwise::<$c, $w, 3, $eager>($slice, $k),
            4 => $crate::run_elementwise::<$c, $w, 4, $eager>($slice, $k),
            8 => $crate::run_elementwise::<$c, $w, 8, $eager>($slice, $k),
            16 => $crate::run_elementwise::<$c, $w, 16, $eager>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_sat_ew {
    ($c:ty, $w:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_sat_elementwise::<$c, $w, 1>($slice, $k),
            2 => $crate::run_sat_elementwise::<$c, $w, 2>($slice, $k),
            3 => $crate::run_sat_elementwise::<$c, $w, 3>($slice, $k),
            4 => $crate::run_sat_elementwise::<$c, $w, 4>($slice, $k),
            8 => $crate::run_sat_elementwise::<$c, $w, 8>($slice, $k),
            16 => $crate::run_sat_elementwise::<$c, $w, 16>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_affine {
    ($c:ty, $w:literal, $eager:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_wrap_affine::<$c, $w, 1, $eager>($slice, $k),
            2 => $crate::run_wrap_affine::<$c, $w, 2, $eager>($slice, $k),
            3 => $crate::run_wrap_affine::<$c, $w, 3, $eager>($slice, $k),
            4 => $crate::run_wrap_affine::<$c, $w, 4, $eager>($slice, $k),
            8 => $crate::run_wrap_affine::<$c, $w, 8, $eager>($slice, $k),
            16 => $crate::run_wrap_affine::<$c, $w, 16, $eager>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __sat_widening {
    ($c:ty, $w:literal, $theorem:expr, $slice:expr) => {
        $crate::run_sat_widening::<$c, $w, { $theorem }>($slice)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_sat_ew_witness {
    ($c:ty, $w:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_sat_elementwise_witness::<$c, $w, 1>($slice, $k),
            2 => $crate::run_sat_elementwise_witness::<$c, $w, 2>($slice, $k),
            3 => $crate::run_sat_elementwise_witness::<$c, $w, 3>($slice, $k),
            4 => $crate::run_sat_elementwise_witness::<$c, $w, 4>($slice, $k),
            8 => $crate::run_sat_elementwise_witness::<$c, $w, 8>($slice, $k),
            16 => $crate::run_sat_elementwise_witness::<$c, $w, 16>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_sat_lanes {
    ($c:ty, $w:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_sat_lanes::<$c, $w, 1>($slice, $k),
            2 => $crate::run_sat_lanes::<$c, $w, 2>($slice, $k),
            3 => $crate::run_sat_lanes::<$c, $w, 3>($slice, $k),
            4 => $crate::run_sat_lanes::<$c, $w, 4>($slice, $k),
            8 => $crate::run_sat_lanes::<$c, $w, 8>($slice, $k),
            16 => $crate::run_sat_lanes::<$c, $w, 16>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_d_wrap_lanes {
    ($c:ty, $w:literal, $eager:literal, $d:expr, $slice:expr, $k:expr) => {
        match $d {
            1 => $crate::run_wrap_lanes::<$c, $w, 1, $eager>($slice, $k),
            2 => $crate::run_wrap_lanes::<$c, $w, 2, $eager>($slice, $k),
            3 => $crate::run_wrap_lanes::<$c, $w, 3, $eager>($slice, $k),
            4 => $crate::run_wrap_lanes::<$c, $w, 4, $eager>($slice, $k),
            8 => $crate::run_wrap_lanes::<$c, $w, 8, $eager>($slice, $k),
            16 => $crate::run_wrap_lanes::<$c, $w, 16, $eager>($slice, $k),
            other => panic!("unsupported operation density D={}", other),
        }
    };
}

/// Declares one arm the same way as `declare_arm`, except that a saturating
/// fold runs the lane-parallel kernel rather than the serial accumulator.
/// The wrapping and elementwise cells are identical to `declare_arm` with
/// `EAGER = false`, so on those rows this arm is a second control.
#[macro_export]
macro_rules! declare_kernel_arm {
    ($name:ident, $eager:literal, $( $w:literal => ($c:ty, $reg:ident) ),+ $(,)?) => {
        pub fn $name(key: usize, cols: &$crate::Cols) -> u64 {
            $crate::assert_aligned(cols);
            let w = $crate::key_w(key);
            let d = $crate::key_d(key);
            let n = $crate::key_n(key);
            let op = $crate::key_op(key);
            match w {
                $(
                    $w => {
                        let s = cols.$reg::<$c>(n);
                        let k = <$c as $crate::Carrier>::from_u128($crate::operand_for($w));
                        if op == 0 {
                            $crate::__for_each_d_wrap_lanes!($c, $w, $eager, d, s, k)
                        } else if op == 1 {
                            $crate::__for_each_d_sat_lanes!($c, $w, d, s, k)
                        } else if op == 2 {
                            $crate::__for_each_d_ew!($c, $w, false, d, s, k)
                        } else if op == 3 {
                            $crate::__for_each_d_sat_ew_witness!($c, $w, d, s, k)
                        } else if op == 4 {
                            $crate::__for_each_d_affine!($c, $w, false, d, s, k)
                        } else {
                            $crate::__sat_widening!($c, $w, $crate::theorem_applies($w, 8192), s)
                        }
                    }
                )+
                other => panic!("unsupported declared width W={}", other),
            }
        }
    };
}

/// Declares one arm: a table from declared width to (carrier, region), plus
/// whether the wrapping form writes the projection after every operation.
#[macro_export]
macro_rules! declare_arm {
    ($name:ident, $eager:literal, $( $w:literal => ($c:ty, $reg:ident) ),+ $(,)?) => {
        /// Runs this arm for one key. Panics on a width or density the arm
        /// does not declare, rather than silently running a neighbour.
        pub fn $name(key: usize, cols: &$crate::Cols) -> u64 {
            $crate::assert_aligned(cols);
            let w = $crate::key_w(key);
            let d = $crate::key_d(key);
            let n = $crate::key_n(key);
            let op = $crate::key_op(key);
            match w {
                $(
                    $w => {
                        let s = cols.$reg::<$c>(n);
                        let k = <$c as $crate::Carrier>::from_u128($crate::operand_for($w));
                        if op == 0 {
                            $crate::__for_each_d!(run_wrap, $c, $w, $eager, d, s, k)
                        } else if op == 1 {
                            $crate::__for_each_d_sat!($c, $w, d, s, k)
                        } else if op == 2 {
                            $crate::__for_each_d_ew!($c, $w, $eager, d, s, k)
                        } else if op == 3 {
                            $crate::__for_each_d_sat_ew!($c, $w, d, s, k)
                        } else if op == 4 {
                            $crate::__for_each_d_affine!($c, $w, $eager, d, s, k)
                        } else {
                            $crate::__sat_widening!($c, $w, false, s)
                        }
                    }
                )+
                other => panic!("unsupported declared width W={}", other),
            }
        }
    };
}

// ---------------------------------------------------------------------------
// The four arms, declared side by side so the whole competitor set is one
// screen and a width present in one arm cannot be silently absent from
// another. Each variant cdylib calls exactly one of these; the others are
// dead code in that dylib and are stripped.
//
// The carrier tables ARE the arms. Read them against
// `arvo-strategy/src/container.rs:15-19` (headroom) and `:60-75` (minimum).
// ---------------------------------------------------------------------------

pub mod arms {
    #[allow(unused_imports)]
    use super::*;

    declare_arm!(
        headroom,
        true,
        8 => (u16, head_slice),
        13 => (u32, head_slice),
        16 => (u32, head_slice),
        32 => (u64, head_slice),
        60 => (u128, head_slice),
        64 => (u128, head_slice),
    );

    declare_arm!(
        minimum,
        true,
        8 => (u8, min_slice),
        13 => (u16, min_slice),
        16 => (u16, min_slice),
        32 => (u32, min_slice),
        60 => (u64, min_slice),
        64 => (u64, min_slice),
    );

    declare_arm!(
        plusone,
        true,
        8 => (u16, head_slice),
        13 => (u16, min_slice),
        16 => (u32, head_slice),
        32 => (u64, head_slice),
        60 => (u64, min_slice),
        64 => (u128, head_slice),
    );

    declare_kernel_arm!(
        kernel,
        true,
        8 => (u8, min_slice),
        13 => (u16, min_slice),
        16 => (u16, min_slice),
        32 => (u32, min_slice),
        60 => (u64, min_slice),
        64 => (u64, min_slice),
    );

    declare_kernel_arm!(
        lanes_deferred,
        false,
        8 => (u8, min_slice),
        13 => (u16, min_slice),
        16 => (u16, min_slice),
        32 => (u32, min_slice),
        60 => (u64, min_slice),
        64 => (u64, min_slice),
    );

    declare_arm!(
        native,
        false,
        8 => (u8, min_slice),
        13 => (u16, min_slice),
        16 => (u16, min_slice),
        32 => (u32, min_slice),
        60 => (u64, min_slice),
        64 => (u64, min_slice),
    );
}

/// Every key this bench declares, in one list, so a test can sweep the whole
/// matrix rather than a chosen subset of it.
pub const ALL_KEYS: [usize; 57] = [
    80003, 130003, 160003, 320003, 600003, 640003, 81003, 131003, 161003, 321003, 601003, 641003,
    130001, 130002, 130004, 130008, 130016, 640001, 640002, 640004, 640008, 640016, 80103, 130103,
    160103, 320103, 600103, 640103, 80204, 130204, 160204, 320204, 600204, 640204, 80304, 130304,
    160304, 320304, 600304, 640304, 80403, 130403, 160403, 320403, 600403, 640403, 80501, 130501,
    160501, 320501, 600501, 640501, 130401, 130402, 130404, 130408, 130416,
];

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    /// The table every arm's dispatch is generated from. Kept here so the
    /// tests below check the same mapping the arms use rather than a second
    /// copy of it.
    const SWEPT: [u32; 6] = [8, 13, 16, 32, 60, 64];

    /// The load-bearing test for the whole bench: all four arms must produce
    /// the identical value, on the identical input, for **every** key the
    /// manifest declares, not for a chosen subset. Three of the arms differ
    /// only in the carrier they read through and the fourth also in where
    /// the projection sits; if any of them computes something else the
    /// timing comparison is meaningless and the fast one is fast because it
    /// is doing less.
    ///
    /// The value is also checked against the independent `u128` reference,
    /// so four agreeing arms sharing one wrong transform is caught rather
    /// than confirmed.
    #[test]
    fn all_four_arms_agree_with_each_other_and_with_the_oracle_on_every_key() {
        for &key in ALL_KEYS.iter() {
            let n = key_n(key);
            let w = key_w(key);
            let d = key_d(key);
            let op = key_op(key);
            let buf = build_bytes(key, 3);
            assert_eq!(
                buf.as_ptr() as usize % 16,
                0,
                "key {key}: the input buffer is not 16-aligned, so the headroom \
                 arm's u128 view of it is unsound and no number from this bench \
                 means anything"
            );
            let cols: &Cols = unsafe { &*(buf.as_ptr() as *const Cols) };

            let h = arms::headroom(key, cols);
            let m = arms::minimum(key, cols);
            let p = arms::plusone(key, cols);
            let nat = arms::native(key, cols);
            let ker = arms::kernel(key, cols);
            let ld = arms::lanes_deferred(key, cols);
            assert_eq!(h, m, "key {key}: headroom and minimum disagree");
            assert_eq!(h, p, "key {key}: headroom and plusone disagree");
            assert_eq!(h, nat, "key {key}: headroom and native disagree");
            assert_eq!(
                h, ker,
                "key {key}: the lane-parallel kernel disagrees, so the reassociation \
                 it performs is not legal for these semantics"
            );
            assert_eq!(
                h, ld,
                "key {key}: the deferred lane-parallel arm disagrees, so composing \
                 the two levers is not semantics-preserving"
            );

            let vals = decode_min(&buf, n, w);
            assert_eq!(
                h,
                reference(&vals, w, op, d),
                "key {key}: the arms agree with each other and disagree with the oracle"
            );
        }
    }

    /// The two aliasing cells named in the arm doc comments must really
    /// alias, since the whole noise-floor argument rests on it. Checked as a
    /// statement about the carrier tables rather than about timing.
    #[test]
    fn the_control_arm_aliases_a_real_arm_at_every_swept_width() {
        for &w in &SWEPT {
            let p = plus_bytes_for(w);
            assert!(
                p == min_bytes_for(w) || p == head_bytes_for(w),
                "W={w}: plusone is not an alias of either real arm, so it is not a control"
            );
        }
    }

    /// Diagnostic, not a bench: an ad-hoc quick spike with no substance, kept
    /// only long enough to answer whether the lane-parallel saturating fold
    /// executes at all at the sizes the harness reports single-digit
    /// nanoseconds for.
    ///
    /// The first version of this check reported zero nanoseconds and was
    /// wrong: it exclusive-ored the result into a sink an even number of
    /// times, so the sink was provably zero and the whole loop was dead.
    /// `black_box` on both ends is what makes the call observable.
    #[test]
    fn diag_sat_lanes_actually_runs() {
        use std::hint::black_box;
        let n = 8192usize;
        let vals: std::vec::Vec<u64> = (0..n as u64).map(|i| i.wrapping_mul(2654435761)).collect();
        let k = <u64 as Carrier>::from_u128(operand_for(64));
        let reps = 1001usize;

        let t0 = std::time::Instant::now();
        let mut a = 0u64;
        for _ in 0..reps {
            a = a.wrapping_add(run_sat_lanes::<u64, 64, 3>(black_box(&vals), black_box(k)));
        }
        black_box(a);
        let lanes = t0.elapsed().as_nanos() as f64 / reps as f64;

        let t1 = std::time::Instant::now();
        let mut b = 0u64;
        for _ in 0..reps {
            b = b.wrapping_add(run_sat::<u64, 64, 3>(black_box(&vals), black_box(k)));
        }
        black_box(b);
        let serial = t1.elapsed().as_nanos() as f64 / reps as f64;

        std::eprintln!("lanes {lanes:.0} ns   serial {serial:.0} ns   a={a} b={b}");
        assert_eq!(a, b, "the two folds must agree");
        assert!(
            lanes > 100.0,
            "the lane-parallel fold returned in {lanes:.0} ns for {n} elements, \
             which is not enough time to touch them; the loop is not running"
        );
    }

    #[test]
    fn key_encoding_round_trips_over_the_whole_declared_matrix() {
        for &w in &SWEPT {
            for nc in 0..2usize {
                for op in 0..2usize {
                    for &d in &[1usize, 2, 3, 4, 8, 16] {
                        let key = w as usize * 10_000 + nc * 1_000 + op * 100 + d;
                        assert_eq!(key_w(key), w, "key {key}");
                        assert_eq!(key_nc(key), nc, "key {key}");
                        assert_eq!(key_op(key), op, "key {key}");
                        assert_eq!(key_d(key), d, "key {key}");
                    }
                }
            }
        }
    }

    /// The claim the `plusone` arm exists to make visible: composing before
    /// rounding never produces a third container. Asserted over every width
    /// from 1 to 64, not over the swept sample, because the claim is about
    /// the rule rather than about the widths this bench happens to run.
    #[test]
    fn plusone_is_never_a_third_container() {
        for w in 1u32..=64 {
            let p = plus_bytes_for(w);
            let mn = min_bytes_for(w);
            let hd = head_bytes_for(w);
            assert!(
                p == mn || p == hd,
                "W={w}: rung(W+1) is {p} bytes, which is neither the minimum {mn} \
                 nor the headroom {hd}"
            );
            if w == rung_bits(w) {
                assert_eq!(
                    p, hd,
                    "W={w} fills its rung, so rung(W+1) must be the headroom"
                );
            } else {
                assert_eq!(
                    p, mn,
                    "W={w} is below its rung, so rung(W+1) must be the minimum"
                );
            }
        }
    }

    /// The shipped rule widens every width at or below 64 bits. Asserted
    /// over all 64, since a sample would not establish "every".
    #[test]
    fn the_shipped_rule_widens_every_width_to_64() {
        for w in 1u32..=64 {
            assert_eq!(
                head_bytes_for(w),
                2 * min_bytes_for(w),
                "W={w}: the shipped Warm container is not twice the minimum"
            );
        }
    }

    /// Both regions must decode to the same logical column, for every swept
    /// width and both element-count classes. A bench whose arms are fed
    /// different data reports a difference that is not the difference it
    /// claims to measure.
    #[test]
    fn both_regions_hold_the_same_column() {
        fn check<const KEY: usize>() {
            let n = key_n(KEY);
            let w = key_w(KEY);
            let buf = <Case<KEY> as Routine>::build_input_bytes(7);
            assert_eq!(buf.len(), TOTAL_INPUT_BYTES);
            let a = decode_min(&buf, n, w);
            let b = decode_head(&buf, n, w);
            assert_eq!(a, b, "KEY={KEY}");
            let lim: u128 = (1u128 << w) - 1;
            assert!(
                a.iter().all(|&v| v <= lim),
                "KEY={KEY}: a value exceeds 2^W - 1"
            );
        }
        check::<80003>();
        check::<130003>();
        check::<160003>();
        check::<320003>();
        check::<600003>();
        check::<640003>();
        check::<81003>();
        check::<131003>();
        check::<161003>();
        check::<321003>();
        check::<601003>();
        check::<641003>();
    }

    /// The carrier a given arm picks must be exactly as wide as the region
    /// stride `build_input_bytes` wrote at. Checked for every swept width in
    /// all three tables, because a mismatch here reads every other element
    /// and produces a wrong-but-plausible number.
    #[test]
    fn carrier_widths_match_the_region_strides() {
        for &w in &SWEPT {
            assert_eq!(min_bytes_for(w), (rung_bits(w) / 8) as usize, "W={w}");
            assert_eq!(head_bytes_for(w), 2 * min_bytes_for(w), "W={w}");
        }
        assert_eq!(min_bytes_for(8), 1);
        assert_eq!(head_bytes_for(8), 2);
        assert_eq!(min_bytes_for(13), 2);
        assert_eq!(head_bytes_for(13), 4);
        assert_eq!(min_bytes_for(16), 2);
        assert_eq!(head_bytes_for(16), 4);
        assert_eq!(min_bytes_for(32), 4);
        assert_eq!(head_bytes_for(32), 8);
        assert_eq!(min_bytes_for(60), 8);
        assert_eq!(head_bytes_for(60), 16);
        assert_eq!(min_bytes_for(64), 8);
        assert_eq!(head_bytes_for(64), 16);
    }

    /// `mask_to` at an exactly-filled width must be the identity, and below
    /// it must be a real reduction. This is the branch the whole fork turns
    /// on and it is the one place a shift-by-BITS would be wrong.
    #[test]
    fn mask_to_is_identity_at_the_rung_and_reduces_below_it() {
        assert_eq!(u8::MAX.mask_to(8), u8::MAX);
        assert_eq!(u16::MAX.mask_to(16), u16::MAX);
        assert_eq!(u32::MAX.mask_to(32), u32::MAX);
        assert_eq!(u64::MAX.mask_to(64), u64::MAX);
        assert_eq!(u128::MAX.mask_to(128), u128::MAX);
        assert_eq!(u16::MAX.mask_to(13), 0x1FFF);
        assert_eq!(u64::MAX.mask_to(60), (1u64 << 60) - 1);
        assert_eq!(u32::MAX.mask_to(1), 1);
        assert_eq!(u8::limit(8), u8::MAX);
        assert_eq!(u16::limit(13), 0x1FFF);
        assert_eq!(u128::limit(64), u64::MAX as u128);
    }

    /// The eager and lazy wrapping forms must agree, at every swept width
    /// and every declared density, in both containers. This is `140`'s
    /// homomorphism claim asserted as a value equality rather than read off
    /// a symbol alias; if it fails the `native` arm is not a valid
    /// competitor and the whole comparison is void.
    #[test]
    fn eager_and_lazy_wrapping_agree_everywhere() {
        macro_rules! pair {
            ($c:ty, $w:literal, $d:literal) => {{
                let k = <$c as Carrier>::from_u128(operand_for($w));
                let vals: std::vec::Vec<$c> = (0..257u32)
                    .map(|i| {
                        <$c as Carrier>::from_u128((i as u128).wrapping_mul(2_654_435_761))
                            .mask_to($w)
                    })
                    .collect();
                let eager = run_wrap::<$c, $w, $d, true>(&vals, k);
                let lazy = run_wrap::<$c, $w, $d, false>(&vals, k);
                assert_eq!(
                    eager,
                    lazy,
                    "W={} D={} carrier={}",
                    $w,
                    $d,
                    core::any::type_name::<$c>()
                );
                let refv = reference(
                    &vals
                        .iter()
                        .map(|v| v.to_u64() as u128)
                        .collect::<std::vec::Vec<_>>(),
                    $w,
                    0,
                    $d,
                );
                assert_eq!(eager, refv, "W={} D={} disagrees with the oracle", $w, $d);
            }};
        }
        pair!(u8, 8, 1);
        pair!(u16, 8, 3);
        pair!(u16, 13, 1);
        pair!(u32, 13, 3);
        pair!(u16, 13, 16);
        pair!(u16, 16, 4);
        pair!(u32, 16, 8);
        pair!(u32, 32, 3);
        pair!(u64, 32, 8);
        pair!(u64, 60, 3);
        pair!(u128, 60, 4);
        pair!(u64, 64, 3);
        pair!(u128, 64, 16);
    }

    /// The saturating transform must agree with the oracle in both
    /// containers. Saturation is not a homomorphism, so this is the check
    /// that the wider container does not quietly compute something else.
    #[test]
    fn saturating_agrees_with_the_oracle_in_both_containers() {
        macro_rules! sat {
            ($c:ty, $w:literal, $d:literal) => {{
                let k = <$c as Carrier>::from_u128(operand_for($w));
                let vals: std::vec::Vec<$c> = (0..257u32)
                    .map(|i| {
                        <$c as Carrier>::from_u128((i as u128).wrapping_mul(2_654_435_761))
                            .mask_to($w)
                    })
                    .collect();
                let got = run_sat::<$c, $w, $d>(&vals, k);
                let refv = reference(
                    &vals
                        .iter()
                        .map(|v| v.to_u64() as u128)
                        .collect::<std::vec::Vec<_>>(),
                    $w,
                    1,
                    $d,
                );
                assert_eq!(
                    got,
                    refv,
                    "W={} D={} carrier={}",
                    $w,
                    $d,
                    core::any::type_name::<$c>()
                );
            }};
        }
        sat!(u8, 8, 3);
        sat!(u16, 8, 3);
        sat!(u16, 13, 3);
        sat!(u32, 13, 3);
        sat!(u16, 16, 3);
        sat!(u32, 16, 3);
        sat!(u32, 32, 3);
        sat!(u64, 32, 3);
        sat!(u64, 60, 3);
        sat!(u128, 60, 3);
        sat!(u64, 64, 3);
        sat!(u128, 64, 3);
        sat!(u64, 64, 16);
        sat!(u128, 64, 16);
    }

    /// The oracle must be able to disagree. A validation whose reference
    /// cannot produce a different answer from the thing it validates is not
    /// a validation.
    #[test]
    fn the_oracle_is_sensitive_to_a_perturbed_column() {
        let vals: std::vec::Vec<u128> = (0..64u128).map(|i| i * 37 % 8192).collect();
        let a = reference(&vals, 13, 0, 3);
        let mut vals2 = vals.clone();
        vals2[10] ^= 1;
        assert_ne!(a, reference(&vals2, 13, 0, 3));
        assert_ne!(
            a,
            reference(&vals, 13, 0, 4),
            "density must move the result"
        );
        assert_ne!(a, reference(&vals, 16, 0, 3), "width must move the result");
        assert_ne!(
            a,
            reference(&vals, 13, 1, 3),
            "semantics must move the result"
        );
    }

    /// `validate_output` must refuse a wrong value. Exercised through the
    /// byte-level path the harness actually calls.
    #[test]
    fn validate_output_refuses_a_wrong_sum() {
        const KEY: usize = 130003;
        let buf = <Case<KEY> as Routine>::build_input_bytes(11);
        let n = key_n(KEY);
        let vals = decode_min(&buf, n, key_w(KEY));
        let good = Sum {
            value: reference(&vals, key_w(KEY), 0, key_d(KEY)),
        };
        let good_bytes = unsafe {
            core::slice::from_raw_parts(&good as *const _ as *const u8, core::mem::size_of::<Sum>())
        };
        <Case<KEY> as Routine>::validate_output_bytes(&buf, good_bytes)
            .expect("the reference value must validate");

        let bad = Sum {
            value: good.value ^ 1,
        };
        let bad_bytes = unsafe {
            core::slice::from_raw_parts(&bad as *const _ as *const u8, core::mem::size_of::<Sum>())
        };
        assert!(
            <Case<KEY> as Routine>::validate_output_bytes(&buf, bad_bytes).is_err(),
            "a wrong sum must fail validation"
        );
    }

    #[test]
    fn build_input_bytes_is_deterministic_and_seed_sensitive() {
        assert_eq!(
            <Case<130003> as Routine>::build_input_bytes(5),
            <Case<130003> as Routine>::build_input_bytes(5)
        );
        assert_ne!(
            <Case<130003> as Routine>::build_input_bytes(5),
            <Case<130003> as Routine>::build_input_bytes(6)
        );
    }

    /// `Cols` must be exactly `TOTAL_INPUT_BYTES` and 16-aligned, since the
    /// harness's own byte path casts a `Vec<u8>` of that length straight to
    /// it.
    #[test]
    fn cols_layout_matches_the_byte_buffer() {
        assert_eq!(core::mem::size_of::<Cols>(), TOTAL_INPUT_BYTES);
        assert_eq!(core::mem::align_of::<Cols>(), 16);
        assert_eq!(HEAD_BYTES % 16, 0);
    }
}
