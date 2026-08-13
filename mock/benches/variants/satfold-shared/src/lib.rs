//! Shared data model, oracle and transforms for the saturating-fold
//! reassociation bench.
//!
//! ## What this bench exists to price
//!
//! Panel file `80` section 5.3 and `82` section 9 report inner-loop
//! **instructions per element**, read by hand off emitted assembly, for a
//! reduction of saturating additions reassociated under the associativity law:
//! 6.000 for the fold as written, 8.500 for the first law-licensed attempt
//! (worse than doing nothing, because the bounds were not provable so the
//! backend abandoned vectorisation), 0.250 once `chunks_exact(16)` supplied the
//! bounds, 0.141 with four accumulators and a tree combine, against a wrapping
//! control at 0.125 that the backend vectorises unaided. Both files say plainly
//! that instructions per element is not time and that nothing there was timed.
//!
//! So the magnitudes are unpriced. This bench prices them, on the harness, with
//! the arms someone might genuinely choose rather than a strawman.
//!
//! ## The arms, and why each is a real competitor
//!
//! `seq` is the fold as a consumer writes it: one loop-carried accumulator,
//! strictly left-associated. It is `80`'s `sat_sum_seq`.
//!
//! `iterfold` is the same value written the way most Rust is written,
//! `xs.iter().fold(0, |a, &x| a.saturating_add(x))`. `80` never included it, and
//! it is the first thing a competent engineer types. Whether it is the same code
//! as `seq` is a question the harness answers by disassembling both.
//!
//! `lanes4_idx` is `80`'s `sat_sum_lanes`: four accumulators, indexed slice
//! access, so the bounds are not provable. It is the arm that lost, and it is
//! kept precisely because it lost.
//!
//! `lanes16` is `80`'s `sat_sum_lanes16`: sixteen accumulators over
//! `chunks_exact(16)`, which is the bounds proof the backend needed.
//!
//! `lanes64` is `80`'s `sat_sum_lanes64`: four vector accumulators over
//! `chunks_exact(64)` plus a tree combine.
//!
//! `nolaw` is `82`'s attribution control, never run for the unsigned case: the
//! same `chunks_exact(16)` bounds proof with the accumulation chain left
//! strictly serial. If this arm wins as much as `lanes16`, the bounds proof was
//! the whole story and the law bought nothing.
//!
//! `neon` is what someone writes when they know the law and do not want to
//! negotiate with the autovectoriser: `vqaddq_u8` by hand, four accumulators,
//! vector tree combine. `80` and `82` never had this arm, so nothing established
//! whether the compiler-vectorised licensed form is near the machine's ceiling
//! or merely better than the scalar one.
//!
//! `neon8` is the same hand-written kernel with eight accumulators rather than
//! four, added after the first run to attack the mechanism behind `neon`'s own
//! plateau rather than report it.
//!
//! `lanes16_constl` is `lanes16` with the reduction length lifted from a runtime
//! value to a const generic, and it exists to price exactly one static lever:
//! what knowing the fold length at compile time is worth with everything else
//! held.
//!
//! ## The shape, and why it is a chunked column rather than one long reduction
//!
//! A saturating `u8` accumulator over a long slice pins at 255 after a handful
//! of elements and then stops depending on the input. `141` shipped six void
//! rows to exactly that, and `bench-warm-clamp-shared` documents it. A bench
//! whose answer is 255 whatever the input has a cross-validation that cannot
//! fail, which is the instrument defect this panel has now found four times.
//!
//! So the workload folds a column of `n` elements as `n / L` independent
//! saturating reductions of length `L`, combining the per-reduction results with
//! a rotate and an exclusive or so every element is load-bearing and the order
//! of reductions matters. `L` is the swept axis and it is the reduction length
//! the law is applied over, which is exactly where the crossover lives: a
//! vectorised arm pays a prologue and a horizontal combine per reduction, and
//! below some `L` that fixed cost is the whole cost.
//!
//! The element distribution is chosen per `L` so that reductions alternate
//! between an expected sum of 160, comfortably under the limit, and 500,
//! comfortably over it. Half the reductions clamp and half do not, by
//! construction. That is the only regime where the clamp is neither dead nor
//! absorbing, and it is the regime an accumulator sized for a declared width
//! actually runs in. `validate_output` **checks** it rather than assuming it: it
//! rejects a run where every reduction returned the same byte, or where the
//! saturated fraction falls outside 20% to 80%. Those checks can fail, which is
//! the point of having them.
//!
//! ## Why `wrapping` is an op rather than a separate control arm
//!
//! `80` used a wrapping fold as its density comparator. Wrapping addition is
//! associative unconditionally, so the backend may reassociate it with no help
//! from any typestate, and the comparison "law-licensed saturating arm against
//! the wrapping form the backend vectorises for free" is the honest ceiling.
//!
//! Making it a **second op over the identical arm set** rather than one extra
//! variant buys two things. The harness's cross-validation stays meaningful,
//! because every arm at a given key computes the same value. And the question
//! becomes richer: what does the saturating operator cost relative to wrapping
//! *at each arm shape*, which is a per-arm answer rather than one number.
//!
//! ## Key encoding
//!
//! `KEY = LI * 1000 + NC * 100 + AL * 10 + OP`
//!
//! `LI` indexes `L_TABLE`, the reduction length. `NC` selects the column size
//! (0 is 32768 elements, resident in this host's L1 data cache; 1 is 16777216,
//! past every level of it). `AL` is 0 for a 64-byte-aligned column start and 1
//! for a start offset by one byte. `OP` is 0 for saturating addition and 1 for
//! wrapping.
//!
//! `LI` is one-based, so `5000` is `L = 32`, the small column, aligned,
//! saturating. No row carries `n = 0`, which would read as an absent size.
//!
//! Bench infrastructure, not shipping arvo source: `std` and bare primitives are
//! used freely, matching every sibling variant crate in this directory.

use mockspace_bench_core::Routine;

// ---------------------------------------------------------------------------
// Key decoding.
// ---------------------------------------------------------------------------

/// Reduction lengths. The lane counts the arms use are 4, 16 and 64, so the
/// table brackets each of them with a value one below and one above: an arm
/// whose advantage depends on the length being an exact multiple of its lane
/// count loses it at 15, 17, 63 and 65, and the ragged tail cost is visible
/// rather than inferred.
pub const L_TABLE: [usize; 12] = [8, 15, 16, 17, 32, 63, 64, 65, 128, 256, 1024, 4096];

/// `LI` is stored one-based so no bench row carries `n = 0`, which reads as an
/// absent size rather than a declared one.
pub const fn key_li(key: usize) -> usize {
    key / 1_000 - 1
}
pub const fn key_nc(key: usize) -> usize {
    (key / 100) % 10
}
pub const fn key_al(key: usize) -> usize {
    (key / 10) % 10
}
pub const fn key_op(key: usize) -> usize {
    key % 10
}
pub const fn key_l(key: usize) -> usize {
    L_TABLE[key_li(key)]
}

/// Small column: 32 KiB of `u8`, inside this host's 64 KiB reported L1 data
/// cache (`sysctl hw.l1dcachesize` on the Apple M1 this ran on), so the
/// measurement is compute density rather than memory traffic.
pub const N_SMALL: usize = 32_768;

/// Large column: 16 MiB. `sysctl hw.l2cachesize` reports 4 MiB on this host and
/// the physical shared L2 on an M1 is 12 MiB, so 16 MiB is past both and the
/// measurement is what the advantage is worth once the load stream is the
/// binding constraint.
pub const N_LARGE: usize = 16_777_216;

pub const fn key_n(key: usize) -> usize {
    if key_nc(key) == 0 {
        N_SMALL
    } else {
        N_LARGE
    }
}

/// Byte offset of the column start inside the buffer. 0 is 64-byte aligned
/// because the buffer is; 1 is deliberately not.
pub const fn key_off(key: usize) -> usize {
    key_al(key)
}

// ---------------------------------------------------------------------------
// The operator axis.
// ---------------------------------------------------------------------------

/// The addition a reduction is built from.
///
/// `IS_SAT` is a const so a monomorphised arm folds the selection away; nothing
/// here dispatches at runtime.
pub trait AddOp: Copy + 'static {
    const IS_SAT: bool;
    fn add(a: u8, b: u8) -> u8;
}

/// Saturating addition. Associative and commutative on unsigned integers, which
/// is the law every reassociated arm below rests on, re-established
/// exhaustively at 8 bits by `80_probes/p4_what_the_law_unlocks.rs` and by
/// `oracle_law_holds_exhaustively` here.
#[derive(Clone, Copy)]
pub struct Sat;

/// Wrapping addition. Associative unconditionally, so the backend may
/// reassociate it without being told anything.
#[derive(Clone, Copy)]
pub struct Wrap;

impl AddOp for Sat {
    const IS_SAT: bool = true;
    #[inline(always)]
    fn add(a: u8, b: u8) -> u8 {
        a.saturating_add(b)
    }
}

impl AddOp for Wrap {
    const IS_SAT: bool = false;
    #[inline(always)]
    fn add(a: u8, b: u8) -> u8 {
        a.wrapping_add(b)
    }
}

// ---------------------------------------------------------------------------
// The kernels. One definition each; every arm names one of them.
// ---------------------------------------------------------------------------

/// One reduction of a slice to a byte. Every arm is one implementation of this.
pub trait Kernel: Copy + 'static {
    fn fold<O: AddOp>(xs: &[u8]) -> u8;
}

/// The fold as a consumer writes it: one loop-carried accumulator, strictly
/// left-associated, no law invoked. `80`'s `sat_sum_seq`.
#[derive(Clone, Copy)]
pub struct Seq;

impl Kernel for Seq {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        let mut acc: u8 = 0;
        for &x in xs {
            acc = O::add(acc, x);
        }
        acc
    }
}

/// The same value written idiomatically. Whether this is the same machine code
/// as `Seq` is measured, not assumed: the harness disassembles `bench_entry` in
/// both dylibs and reports identical pairs.
#[derive(Clone, Copy)]
pub struct IterFold;

impl Kernel for IterFold {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        xs.iter().fold(0u8, |a, &x| O::add(a, x))
    }
}

/// `80`'s `sat_sum_lanes`: four independent accumulators, licensed by
/// associativity, combined at the end. Indexed slice access, so the bounds are
/// not provable and the backend has to emit checks.
///
/// This arm is kept because it lost. `80` measured it at 8.500 instructions per
/// element against the unlicensed 6.000: the law was true, the arm was legal,
/// and the arm was worse than not having had the law.
#[derive(Clone, Copy)]
pub struct Lanes4Indexed;

impl Kernel for Lanes4Indexed {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        let mut a: [u8; 4] = [0; 4];
        let mut i = 0usize;
        while i + 4 <= xs.len() {
            a[0] = O::add(a[0], xs[i]);
            a[1] = O::add(a[1], xs[i + 1]);
            a[2] = O::add(a[2], xs[i + 2]);
            a[3] = O::add(a[3], xs[i + 3]);
            i += 4;
        }
        let mut acc = O::add(O::add(O::add(a[0], a[1]), a[2]), a[3]);
        while i < xs.len() {
            acc = O::add(acc, xs[i]);
            i += 1;
        }
        acc
    }
}

/// `80`'s `sat_sum_lanes16`: the repair that supplies the missing bounds proof
/// rather than a missing intrinsic. `chunks_exact(16)` yields an element of
/// known length, so no bound has to be proved, and the reassociation into
/// sixteen lanes is licensed by the same law.
#[derive(Clone, Copy)]
pub struct Lanes16;

impl Kernel for Lanes16 {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        let mut acc = [0u8; 16];
        let mut it = xs.chunks_exact(16);
        for c in &mut it {
            let c: &[u8; 16] = c.try_into().unwrap();
            for l in 0..16 {
                acc[l] = O::add(acc[l], c[l]);
            }
        }
        let mut total: u8 = 0;
        for l in 0..16 {
            total = O::add(total, acc[l]);
        }
        for &x in it.remainder() {
            total = O::add(total, x);
        }
        total
    }
}

/// `80`'s `sat_sum_lanes64`: four vector accumulators, 64 elements per
/// iteration, with the horizontal combine folded as a tree. Both the unroll and
/// the tree are licensed by the same associativity.
#[derive(Clone, Copy)]
pub struct Lanes64;

impl Kernel for Lanes64 {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        let mut a0 = [0u8; 16];
        let mut a1 = [0u8; 16];
        let mut a2 = [0u8; 16];
        let mut a3 = [0u8; 16];
        let mut it = xs.chunks_exact(64);
        for c in &mut it {
            for l in 0..16 {
                a0[l] = O::add(a0[l], c[l]);
                a1[l] = O::add(a1[l], c[16 + l]);
                a2[l] = O::add(a2[l], c[32 + l]);
                a3[l] = O::add(a3[l], c[48 + l]);
            }
        }
        let mut b = [0u8; 16];
        for l in 0..16 {
            b[l] = O::add(O::add(a0[l], a1[l]), O::add(a2[l], a3[l]));
        }
        let mut w = 16usize;
        while w > 1 {
            let h = w / 2;
            for l in 0..h {
                b[l] = O::add(b[l], b[l + h]);
            }
            w = h;
        }
        let mut total = b[0];
        for &x in it.remainder() {
            total = O::add(total, x);
        }
        total
    }
}

/// `82`'s attribution control, run here for the unsigned case for the first
/// time: the identical `chunks_exact(16)` bounds proof, with the accumulation
/// chain left strictly serial so no law is invoked.
///
/// This is the arm that decides what the win is attributable to. If it lands
/// with `Lanes16`, the bounds proof was the whole story. If it lands with `Seq`,
/// the law is load-bearing.
#[derive(Clone, Copy)]
pub struct NoLawChunked;

impl Kernel for NoLawChunked {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        let mut acc: u8 = 0;
        let mut it = xs.chunks_exact(16);
        for c in &mut it {
            let c: &[u8; 16] = c.try_into().unwrap();
            for l in 0..16 {
                acc = O::add(acc, c[l]);
            }
        }
        for &x in it.remainder() {
            acc = O::add(acc, x);
        }
        acc
    }
}

/// Hand-written NEON, which is what someone writes when they know the law and
/// do not want to negotiate with the autovectoriser. Four `uint8x16_t`
/// accumulators, 64 elements per iteration, horizontal combine as a four-step
/// vector tree via `vextq_u8`.
///
/// The tree combine is correct for exactly the reason the reassociated arms
/// are: unsigned saturating addition is associative and commutative, so each
/// `vext` step pairs lanes that have not already been paired and the sixteen
/// lanes are summed once each.
///
/// Neither `80` nor `82` had this arm, so nothing established whether the
/// compiler-vectorised licensed form is near the machine's ceiling or merely
/// better than the scalar one.
#[derive(Clone, Copy)]
pub struct Neon;

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn neon_fold<O: AddOp, const A: usize>(xs: &[u8]) -> u8 {
    use core::arch::aarch64::*;
    unsafe {
        #[inline(always)]
        unsafe fn vadd<O: AddOp>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
            use core::arch::aarch64::*;
            if O::IS_SAT {
                unsafe { vqaddq_u8(a, b) }
            } else {
                unsafe { vaddq_u8(a, b) }
            }
        }

        let mut acc = [vdupq_n_u8(0); A];

        let mut it = xs.chunks_exact(16 * A);
        for c in &mut it {
            let p = c.as_ptr();
            let mut i = 0usize;
            while i < A {
                acc[i] = vadd::<O>(acc[i], vld1q_u8(p.add(16 * i)));
                i += 1;
            }
        }
        let rem = it.remainder();

        // Whole 16-byte groups left after the wide unroll.
        let mut it16 = rem.chunks_exact(16);
        for c in &mut it16 {
            acc[0] = vadd::<O>(acc[0], vld1q_u8(c.as_ptr()));
        }
        let tail = it16.remainder();

        // Tree over the accumulators, then over the sixteen lanes. Both are
        // licensed by the same associativity the arm rests on.
        let mut w = A;
        while w > 1 {
            let h = w / 2;
            let mut i = 0usize;
            while i < h {
                acc[i] = vadd::<O>(acc[i], acc[i + h]);
                i += 1;
            }
            w = h;
        }
        let mut v = acc[0];
        v = vadd::<O>(v, vextq_u8::<8>(v, v));
        v = vadd::<O>(v, vextq_u8::<4>(v, v));
        v = vadd::<O>(v, vextq_u8::<2>(v, v));
        v = vadd::<O>(v, vextq_u8::<1>(v, v));
        let mut total = vgetq_lane_u8::<0>(v);

        for &x in tail {
            total = O::add(total, x);
        }
        total
    }
}

#[cfg(target_arch = "aarch64")]
impl Kernel for Neon {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        neon_fold::<O, 4>(xs)
    }
}

/// The same hand-written kernel with eight accumulators instead of four, 128
/// elements per iteration.
///
/// This arm exists to attack a mechanism rather than to report it. On the small
/// column `Neon` reaches roughly 114 GB/s at `L = 1024` and falls back to 89
/// GB/s at `L = 4096`, which is the signature of a loop-carried dependency
/// rather than of a bandwidth limit: at the longer reduction there are fewer
/// independent reductions to overlap, so the four accumulator chains are the
/// whole of the available parallelism. If the limit is `uqadd` latency, doubling
/// the accumulators recovers it. If the limit is `uqadd` throughput or the load
/// ports, doubling them buys nothing and the arm is a measured negative, which
/// is a result about the mechanism either way.
#[derive(Clone, Copy)]
pub struct Neon8;

#[cfg(target_arch = "aarch64")]
impl Kernel for Neon8 {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        neon_fold::<O, 8>(xs)
    }
}

#[cfg(not(target_arch = "aarch64"))]
impl Kernel for Neon8 {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        Lanes64::fold::<O>(xs)
    }
}

/// Portable stand-in so the crate still builds off aarch64. No bench row in
/// this repository runs on such a host, and a row that did would be measuring
/// `Lanes64` under the `neon` name, which is why the meta's `cpu` field is part
/// of every finding's predicate.
#[cfg(not(target_arch = "aarch64"))]
impl Kernel for Neon {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        Lanes64::fold::<O>(xs)
    }
}

// ---------------------------------------------------------------------------
// The const gate, and what it costs.
// ---------------------------------------------------------------------------

/// A verdict computed the way `89`'s gate computes one: an exhaustive sweep
/// inside a `const fn`, at a width small enough that rustc's const evaluator
/// will finish it, deciding whether the law the arm rests on holds.
///
/// Six bits is 262,144 triples, comfortably inside the evaluator's default
/// budget. `80_probes/p2` established that a positive verdict at nine bits is
/// what const evaluation refuses under `long_running_const_eval`, so this is the
/// same shape a shipping gate would have and not a token constant.
pub const fn saturating_add_is_associative_at(bits: u32) -> bool {
    let hi: u32 = (1u32 << bits) - 1;
    let mut a = 0u32;
    while a <= hi {
        let mut b = 0u32;
        while b <= hi {
            let mut c = 0u32;
            while c <= hi {
                let ab = if a + b > hi { hi } else { a + b };
                let l = if ab + c > hi { hi } else { ab + c };
                let bc = if b + c > hi { hi } else { b + c };
                let r = if a + bc > hi { hi } else { a + bc };
                if l != r {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

/// The same shape over saturating **subtraction**, which is not associative, so
/// the verdict is false and the gate must select the fallback. It is not a
/// hardcoded `false`: it is the identical sweep over an operation that fails it,
/// which is what makes the negative arm a control rather than a stub.
pub const fn saturating_sub_is_associative_at(bits: u32) -> bool {
    let hi: u32 = (1u32 << bits) - 1;
    let mut a = 0u32;
    while a <= hi {
        let mut b = 0u32;
        while b <= hi {
            let mut c = 0u32;
            while c <= hi {
                let ab = a.saturating_sub(b);
                let l = ab.saturating_sub(c);
                let bc = b.saturating_sub(c);
                let r = a.saturating_sub(bc);
                if l != r {
                    return false;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    true
}

/// The licensed arm reached **through** a const gate whose verdict is computed
/// by the sweep above.
///
/// `82` F11 reports three licensed declarations assembling to one symbol and
/// concludes the declaration is fully erased; `80` section 5.1 reports a
/// const-gated arm carrying no trace of its predicate. Both are compile-time
/// observations. This arm turns the claim into one a bench can refute: if the
/// gate erases, this and `Lanes16` are the same machine code and the same time.
#[derive(Clone, Copy)]
pub struct GateTrueLanes16;

impl Kernel for GateTrueLanes16 {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        if const { saturating_add_is_associative_at(6) } {
            Lanes16::fold::<O>(xs)
        } else {
            Seq::fold::<O>(xs)
        }
    }
}

/// The same gate over a law that is false, so the fallback is selected. The
/// control that makes the pair above mean something: if this timed like
/// `Lanes16` rather than like `Seq`, the gate would not be selecting at all and
/// the agreement of the other two would prove nothing.
#[derive(Clone, Copy)]
pub struct GateFalseLanes16;

impl Kernel for GateFalseLanes16 {
    #[inline(always)]
    fn fold<O: AddOp>(xs: &[u8]) -> u8 {
        if const { saturating_sub_is_associative_at(6) } {
            Lanes16::fold::<O>(xs)
        } else {
            Seq::fold::<O>(xs)
        }
    }
}

// ---------------------------------------------------------------------------
// The column driver. Every arm runs the identical outer loop; only the kernel
// inside it differs.
// ---------------------------------------------------------------------------

/// Fold `data` as `data.len() / l` independent reductions of length `l`,
/// combining the per-reduction bytes with a rotate and an exclusive or.
///
/// The combine is one instruction per reduction, identical across arms, and it
/// exists so the result depends on every element and on the order of the
/// reductions. At `l = 8` it is roughly an eighth of an instruction per element,
/// which compresses every ratio at the short end; that is stated in the findings
/// rather than hidden.
#[inline(always)]
pub fn run_column<K: Kernel, O: AddOp>(data: &[u8], l: usize) -> u64 {
    let mut out: u64 = 0;
    let mut base = 0usize;
    while base + l <= data.len() {
        let r = K::fold::<O>(&data[base..base + l]);
        out = out.rotate_left(7) ^ (r as u64);
        base += l;
    }
    out
}

/// The same driver with the reduction length as a const generic, so the kernel
/// sees a known length and a known-empty remainder.
#[inline(always)]
pub fn run_column_ct<K: Kernel, O: AddOp, const L: usize>(data: &[u8]) -> u64 {
    let mut out: u64 = 0;
    let mut it = data.chunks_exact(L);
    for c in &mut it {
        let r = K::fold::<O>(c);
        out = out.rotate_left(7) ^ (r as u64);
    }
    out
}

/// Recovers the swept reduction length as a compile-time constant. `KEY` is a
/// const generic and `generic_const_exprs` is forbidden, so the length is
/// matched as an ordinary value and each arm folds after inlining, which is the
/// same shape `bench-warm-clamp-shared` uses for its arity axis.
#[inline(always)]
pub fn run_column_constl<K: Kernel, O: AddOp>(data: &[u8], l: usize) -> u64 {
    match l {
        8 => run_column_ct::<K, O, 8>(data),
        15 => run_column_ct::<K, O, 15>(data),
        16 => run_column_ct::<K, O, 16>(data),
        17 => run_column_ct::<K, O, 17>(data),
        32 => run_column_ct::<K, O, 32>(data),
        63 => run_column_ct::<K, O, 63>(data),
        64 => run_column_ct::<K, O, 64>(data),
        65 => run_column_ct::<K, O, 65>(data),
        128 => run_column_ct::<K, O, 128>(data),
        256 => run_column_ct::<K, O, 256>(data),
        1024 => run_column_ct::<K, O, 1024>(data),
        4096 => run_column_ct::<K, O, 4096>(data),
        other => panic!("reduction length {other} is not declared by this bench"),
    }
}

/// One entry point per arm. `KEY` carries the op and the column geometry.
#[inline(always)]
pub fn run_arm<K: Kernel>(key: usize, buf: &Buf) -> u64 {
    let n = key_n(key);
    let off = key_off(key);
    let l = key_l(key);
    let data = &buf.bytes[off..off + n];
    if key_op(key) == 0 {
        run_column::<K, Sat>(data, l)
    } else {
        run_column::<K, Wrap>(data, l)
    }
}

/// `run_arm` with the reduction length lifted to a constant.
#[inline(always)]
pub fn run_arm_constl<K: Kernel>(key: usize, buf: &Buf) -> u64 {
    let n = key_n(key);
    let off = key_off(key);
    let l = key_l(key);
    let data = &buf.bytes[off..off + n];
    if key_op(key) == 0 {
        run_column_constl::<K, Sat>(data, l)
    } else {
        run_column_constl::<K, Wrap>(data, l)
    }
}

// ---------------------------------------------------------------------------
// Input.
// ---------------------------------------------------------------------------

/// One byte past the largest column, so an offset start still has room.
pub const BUF_BYTES: usize = N_LARGE + 64;

#[repr(C, align(64))]
#[derive(Clone, Copy)]
pub struct Buf {
    pub bytes: [u8; BUF_BYTES],
}

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

/// Draws one element of reduction `j` at reduction length `l`.
///
/// Reductions alternate between a low target sum of 160 and a high target of
/// 500, so **exactly half the reductions clamp and half do not, by
/// construction** rather than by luck. Per element that is a mean of
/// `target / l`, drawn uniformly from `[0, 2*target/l]` where that range holds
/// at least one value and as a Bernoulli draw with probability `target / l`
/// where it does not.
///
/// The alternation is deliberate and it replaces an earlier design that drew
/// every reduction from one distribution centred on the limit. That design was
/// correct in expectation and flaky in practice: at `l = 4096` the small column
/// holds eight reductions, so the chance of all eight landing on one side of the
/// limit was around one in 125 per seed, and the harness runs a hundred seeds
/// per row. A degeneracy check that fires on half the runs is not a check, it is
/// a coin toss wearing one.
///
/// A uniform draw from the whole byte range, which is the obvious thing to
/// write, pins every reduction at 255 at every length this bench sweeps. A bench
/// whose answer is 255 whatever the input has a cross-validation that cannot
/// fail. `validate_output` checks the distribution came out as intended rather
/// than trusting this comment.
#[inline]
fn draw(rng: &mut SplitMix64, l: usize, j: usize) -> u8 {
    const LOW_TARGET: usize = 160;
    const HIGH_TARGET: usize = 500;
    let target = if j.is_multiple_of(2) { LOW_TARGET } else { HIGH_TARGET };
    let r = rng.next();
    let hi = (2 * target) / l;
    if hi >= 1 {
        let hi = hi.min(255);
        (r % (hi as u64 + 1)) as u8
    } else if (r % l as u64) < target as u64 {
        1
    } else {
        0
    }
}

pub fn build_bytes(key: usize, seed: u64) -> std::vec::Vec<u8> {
    let n = key_n(key);
    let off = key_off(key);
    let l = key_l(key);
    // `vec![0; ..]` is calloc, so the untouched tail costs address space and no
    // pages. Only the `n` bytes an arm reads are written, which keeps the small
    // rows cheap to validate even though the buffer is sized for the large ones.
    let mut buf = std::vec![0u8; BUF_BYTES];
    let mut rng = SplitMix64(seed ^ 0x5A17_F01D_0BAD_F00D);
    for i in 0..n {
        buf[off + i] = draw(&mut rng, l, i / l);
    }
    buf
}

// ---------------------------------------------------------------------------
// The oracle, and the checks that can fail.
// ---------------------------------------------------------------------------

/// What one column reduction produced, in a form that lets `validate_output`
/// reject a degenerate workload as well as a wrong answer.
pub struct OracleReport {
    pub value: u64,
    pub reductions: usize,
    pub saturated: usize,
    pub min_result: u8,
    pub max_result: u8,
}

/// The independent reference. Accumulates in `u64` with an explicit comparison
/// against the limit, sharing no code and no carrier type with any arm, and
/// left-associated in the order the fold as written would take.
pub fn oracle(data: &[u8], l: usize, op: usize) -> OracleReport {
    let mut out: u64 = 0;
    let mut base = 0usize;
    let mut reductions = 0usize;
    let mut saturated = 0usize;
    let mut min_result = u8::MAX;
    let mut max_result = 0u8;
    while base + l <= data.len() {
        let mut acc: u64 = 0;
        let mut clamped = false;
        for &x in &data[base..base + l] {
            acc += x as u64;
            if op == 0 {
                if acc > 255 {
                    acc = 255;
                    clamped = true;
                }
            } else {
                acc &= 0xFF;
            }
        }
        let r = acc as u8;
        if clamped {
            saturated += 1;
        }
        if r < min_result {
            min_result = r;
        }
        if r > max_result {
            max_result = r;
        }
        out = out.rotate_left(7) ^ (r as u64);
        reductions += 1;
        base += l;
    }
    OracleReport {
        value: out,
        reductions,
        saturated,
        min_result,
        max_result,
    }
}

/// Associativity of unsigned saturating addition at eight bits, over the whole
/// domain rather than asserted. Returns `(triples, failures)`.
///
/// This is the law every reassociated arm rests on. `80_probes` established it
/// with a different instrument; it is re-established here so this crate does not
/// import its premise from a probe transcript.
pub fn oracle_law_holds_exhaustively() -> (u64, u64) {
    let mut total: u64 = 0;
    let mut bad: u64 = 0;
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            for c in 0..=255u8 {
                total += 1;
                if a.saturating_add(b).saturating_add(c) != a.saturating_add(b.saturating_add(c)) {
                    bad += 1;
                }
            }
        }
    }
    (total, bad)
}

impl<const KEY: usize> Routine for Case<KEY> {
    type Input = Buf;
    type Output = Sum;

    fn build_input(_seed: u64) -> Self::Input {
        unreachable!("Case::build_input is never called by the bench path; Self::Input is 16 MiB")
    }

    fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
        build_bytes(KEY, seed)
    }

    /// `outputs_may_differ` is true, so the harness runs this per variant and
    /// skips its cross-variant byte comparison. That is the stronger of the two
    /// checks and not a relaxation: every arm is compared against an independent
    /// reference computed from the same input, so agreement between arms follows,
    /// and an error that moved every arm the same way is caught here where a
    /// cross-variant comparison would pass it.
    ///
    /// Every sibling bench in this directory writes a `validate_output` and
    /// leaves `outputs_may_differ` at its default, which means the harness never
    /// calls it. Their oracles are dead code.
    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let n = key_n(KEY);
        let off = key_off(KEY);
        let l = key_l(KEY);
        let op = key_op(KEY);
        let rep = oracle(&input.bytes[off..off + n], l, op);

        if rep.reductions == 0 {
            return Err("the column holds no complete reduction, so nothing was measured");
        }
        if rep.min_result == rep.max_result {
            return Err(
                "every reduction in the column returned the same byte, so the output does not \
                 depend on the input and no cross-check between arms can fail",
            );
        }
        if op == 0 {
            let pct = (rep.saturated * 100) / rep.reductions;
            if !(20..=80).contains(&pct) {
                return Err(
                    "the saturated fraction is outside 5% to 95%, so the clamp is either dead or \
                     absorbing and the workload is not the one this bench claims to measure",
                );
            }
        }
        if rep.value != output.value {
            return Err(
                "output disagrees with the independent u64 reference, so the timed arm does not \
                 compute the declared reduction",
            );
        }
        Ok(())
    }

    fn outputs_may_differ() -> bool {
        true
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        key_n(KEY) as u64
    }
}

// ---------------------------------------------------------------------------
// The arm declaration each variant crate uses.
// ---------------------------------------------------------------------------

/// Names one arm: a kernel and whether the reduction length reaches it as a
/// constant.
#[macro_export]
macro_rules! declare_satfold_arm {
    ($name:ident, $k:ty, runtime_len) => {
        /// `inline(always)` because the variant cdylibs are built without fat
        /// LTO, matching every sibling bench family in this directory. Without
        /// it the timed region would contain a cross-crate call the real
        /// deployment does not have, and the arms would differ by call overhead
        /// rather than by kernel. The disassembly dumps committed beside this
        /// bench are the check that it took.
        #[inline(always)]
        pub fn $name(key: usize, buf: &$crate::Buf) -> u64 {
            $crate::run_arm::<$k>(key, buf)
        }
    };
    ($name:ident, $k:ty, const_len) => {
        #[inline(always)]
        pub fn $name(key: usize, buf: &$crate::Buf) -> u64 {
            $crate::run_arm_constl::<$k>(key, buf)
        }
    };
}

pub mod arms {
    use crate::{
        GateFalseLanes16, GateTrueLanes16, IterFold, Lanes16, Lanes4Indexed, Lanes64, Neon, Neon8,
        NoLawChunked, Seq,
    };

    crate::declare_satfold_arm!(seq, Seq, runtime_len);
    crate::declare_satfold_arm!(iterfold, IterFold, runtime_len);
    crate::declare_satfold_arm!(lanes4_idx, Lanes4Indexed, runtime_len);
    crate::declare_satfold_arm!(lanes16, Lanes16, runtime_len);
    crate::declare_satfold_arm!(lanes64, Lanes64, runtime_len);
    crate::declare_satfold_arm!(nolaw, NoLawChunked, runtime_len);
    crate::declare_satfold_arm!(neon, Neon, runtime_len);
    crate::declare_satfold_arm!(neon8, Neon8, runtime_len);
    crate::declare_satfold_arm!(lanes16_constl, Lanes16, const_len);
    crate::declare_satfold_arm!(gate_true, GateTrueLanes16, runtime_len);
    crate::declare_satfold_arm!(gate_false, GateFalseLanes16, runtime_len);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every kernel computes the same value as the oracle, at every declared
    /// reduction length, for both operators. This is the check that makes a
    /// timing comparison mean anything, and it runs without the harness.
    #[test]
    fn every_kernel_agrees_with_the_oracle_at_every_length() {
        for (li, &l) in L_TABLE.iter().enumerate() {
            for op in 0..2usize {
                let key = (li + 1) * 1000 + op;
                let bytes = build_bytes(key, 0xDEAD_BEEF ^ li as u64);
                let n = key_n(key);
                let off = key_off(key);
                let buf: &Buf = unsafe { &*(bytes.as_ptr() as *const Buf) };
                let want = oracle(&bytes[off..off + n], l, op).value;
                assert_eq!(arms::seq(key, buf), want, "seq l={l} op={op}");
                assert_eq!(arms::iterfold(key, buf), want, "iterfold l={l} op={op}");
                assert_eq!(arms::lanes4_idx(key, buf), want, "lanes4_idx l={l} op={op}");
                assert_eq!(arms::lanes16(key, buf), want, "lanes16 l={l} op={op}");
                assert_eq!(arms::lanes64(key, buf), want, "lanes64 l={l} op={op}");
                assert_eq!(arms::nolaw(key, buf), want, "nolaw l={l} op={op}");
                assert_eq!(arms::neon(key, buf), want, "neon l={l} op={op}");
                assert_eq!(arms::neon8(key, buf), want, "neon8 l={l} op={op}");
                assert_eq!(
                    arms::lanes16_constl(key, buf),
                    want,
                    "lanes16_constl l={l} op={op}"
                );
                assert_eq!(arms::gate_true(key, buf), want, "gate_true l={l} op={op}");
                assert_eq!(arms::gate_false(key, buf), want, "gate_false l={l} op={op}");
            }
        }
    }

    /// The agreement above is worthless if the arms could not have disagreed, so
    /// three deliberately wrong kernels are run against the same oracle. Each is
    /// the shape a real defect in one of these arms would take.
    ///
    /// A fourth is kept as a test of the instrument's own sensitivity boundary
    /// rather than of an arm, because it is caught at some lengths and not at
    /// others and the boundary is worth pinning.
    #[derive(Clone, Copy)]
    struct WrongOp;
    impl Kernel for WrongOp {
        #[inline(always)]
        fn fold<O: AddOp>(xs: &[u8]) -> u8 {
            // The other operator: what a mis-monomorphised arm would compute.
            let mut acc: u8 = 0;
            for &x in xs {
                acc = if O::IS_SAT {
                    acc.wrapping_add(x)
                } else {
                    acc.saturating_add(x)
                };
            }
            acc
        }
    }

    #[derive(Clone, Copy)]
    struct DropsALane;
    impl Kernel for DropsALane {
        #[inline(always)]
        fn fold<O: AddOp>(xs: &[u8]) -> u8 {
            // One lane of sixteen never accumulated: what an off-by-one in a
            // lane loop produces.
            let mut acc: u8 = 0;
            for (i, &x) in xs.iter().enumerate() {
                if i % 16 != 15 {
                    acc = O::add(acc, x);
                }
            }
            acc
        }
    }

    #[derive(Clone, Copy)]
    struct DropsTheRemainder;
    impl Kernel for DropsTheRemainder {
        #[inline(always)]
        fn fold<O: AddOp>(xs: &[u8]) -> u8 {
            // The ragged tail never folded: the defect a `chunks_exact` arm is
            // most likely to have, and the reason the length table brackets each
            // lane count with a value one below and one above it.
            let mut acc: u8 = 0;
            let mut it = xs.chunks_exact(16);
            for c in &mut it {
                for &x in c {
                    acc = O::add(acc, x);
                }
            }
            acc
        }
    }

    #[derive(Clone, Copy)]
    struct DropsOneElement;
    impl Kernel for DropsOneElement {
        #[inline(always)]
        fn fold<O: AddOp>(xs: &[u8]) -> u8 {
            let mut acc: u8 = 0;
            for &x in xs.iter().take(xs.len().saturating_sub(1)) {
                acc = O::add(acc, x);
            }
            acc
        }
    }

    /// Where this bench's cross-check stops being sensitive to a one-element
    /// defect, pinned rather than assumed.
    ///
    /// A kernel dropping the last single element of each reduction is caught at
    /// every reduction length up to 1024 and **not caught at 4096**: at 4096 the
    /// element draw is Bernoulli with probability 255/4096, and there are only
    /// eight reductions in the small column, so with probability around 0.6 no
    /// reduction's last element is nonzero and dropping it is invisible. That is
    /// a real bound on the instrument and it is stated here rather than removed
    /// by deleting the case.
    ///
    /// An earlier version of this comment said the bound was 256, from reading
    /// the first version of the test, which asserted the defect was caught at
    /// every length and panicked at the first length where it was not. That
    /// panic named 4096 and said nothing about 1024, and 1024 was written into
    /// the comment anyway. Running the scoped assertion is what corrected it.
    ///
    /// It does not weaken the arms' cross-check, because the arms differ in
    /// association order and in remainder handling rather than in which elements
    /// they touch, and the three defect shapes above cover both at every length
    /// where they are expressible.
    ///
    /// A change that flips either half of this assertion is a change in the
    /// element distribution and should be read as one.
    #[test]
    fn the_one_element_defect_is_caught_up_to_1024_and_not_above_it() {
        for (li, &l) in L_TABLE.iter().enumerate() {
            let key = (li + 1) * 1000;
            let bytes = build_bytes(key, 0x1234 ^ li as u64);
            let n = key_n(key);
            let off = key_off(key);
            let want = oracle(&bytes[off..off + n], l, 0).value;
            let got = run_column::<DropsOneElement, Sat>(&bytes[off..off + n], l);
            if l <= 1024 {
                assert_ne!(got, want, "a one-element defect was not caught at l={l}");
            } else {
                assert_eq!(
                    got, want,
                    "a one-element defect became visible at l={l}; the element \
                     distribution moved and this bound has to be re-derived"
                );
            }
        }
    }

    #[test]
    fn a_wrong_operator_is_caught_at_every_length_and_both_ops() {
        for (li, &l) in L_TABLE.iter().enumerate() {
            for op in 0..2usize {
                let key = (li + 1) * 1000 + op;
                let bytes = build_bytes(key, 0x1234 ^ li as u64);
                let n = key_n(key);
                let off = key_off(key);
                let want = oracle(&bytes[off..off + n], l, op).value;
                let got = if op == 0 {
                    run_column::<WrongOp, Sat>(&bytes[off..off + n], l)
                } else {
                    run_column::<WrongOp, Wrap>(&bytes[off..off + n], l)
                };
                assert_ne!(
                    got, want,
                    "the wrong operator was not caught at l={l} op={op}"
                );
            }
        }
    }

    /// Asserted where a sixteenth lane exists to drop. At `l < 16` a reduction
    /// has no index congruent to 15, so this defect is the identity there and
    /// asserting it would be asserting something false.
    #[test]
    fn a_dropped_lane_is_caught_wherever_a_sixteenth_lane_exists() {
        for (li, &l) in L_TABLE.iter().enumerate() {
            if l < 16 {
                continue;
            }
            let key = (li + 1) * 1000;
            let bytes = build_bytes(key, 0x1234 ^ li as u64);
            let n = key_n(key);
            let off = key_off(key);
            let want = oracle(&bytes[off..off + n], l, 0).value;
            let got = run_column::<DropsALane, Sat>(&bytes[off..off + n], l);
            assert_ne!(got, want, "a dropped lane was not caught at l={l}");
        }
    }

    /// Asserted exactly where a remainder exists. At `l % 16 == 0` there is no
    /// ragged tail for an arm to mishandle, so this defect is not expressible
    /// there and asserting it would be asserting something false.
    #[test]
    fn a_dropped_remainder_is_caught_wherever_one_exists() {
        for (li, &l) in L_TABLE.iter().enumerate() {
            if l % 16 == 0 {
                continue;
            }
            let key = (li + 1) * 1000;
            let bytes = build_bytes(key, 0x1234 ^ li as u64);
            let n = key_n(key);
            let off = key_off(key);
            let want = oracle(&bytes[off..off + n], l, 0).value;
            let got = run_column::<DropsTheRemainder, Sat>(&bytes[off..off + n], l);
            assert_ne!(got, want, "a dropped remainder was not caught at l={l}");
        }
    }

    /// The distribution is the one the module documentation claims, at every
    /// declared length: the reductions are not all equal and the saturated
    /// fraction is neither zero nor everything. This is the check whose absence
    /// voided six rows of an earlier bench.
    #[test]
    fn the_workload_is_not_degenerate_at_any_length() {
        for (li, &l) in L_TABLE.iter().enumerate() {
            let key = (li + 1) * 1000;
            let bytes = build_bytes(key, 0xABCD ^ li as u64);
            let n = key_n(key);
            let off = key_off(key);
            let rep = oracle(&bytes[off..off + n], l, 0);
            assert!(rep.reductions > 0, "l={l}: no complete reduction");
            assert_ne!(
                rep.min_result, rep.max_result,
                "l={l}: every reduction returned the same byte"
            );
            let pct = (rep.saturated * 100) / rep.reductions;
            assert!(
                (20..=80).contains(&pct),
                "l={l}: saturated fraction {pct}% is outside 20..=80"
            );
        }
    }

    /// The offset column is a different byte sequence from the aligned one, so
    /// the alignment axis is measuring alignment rather than re-measuring the
    /// same buffer twice.
    #[test]
    fn the_offset_column_is_actually_offset() {
        let aligned = build_bytes(7000, 7);
        let offset = build_bytes(7010, 7);
        assert_eq!(
            aligned[0], offset[1],
            "the offset row did not shift the fill"
        );
        let ptr_off = offset.as_ptr() as usize;
        assert_eq!(
            ptr_off % 64,
            0,
            "the buffer allocation is not 64-byte aligned"
        );
    }

    /// The law the reassociated arms rest on, over its whole domain.
    #[test]
    fn saturating_addition_is_associative_at_eight_bits() {
        let (total, bad) = oracle_law_holds_exhaustively();
        assert_eq!(total, 1 << 24);
        assert_eq!(bad, 0);
    }

    /// Wrapping addition is associative too, which is why the backend may
    /// reassociate the `Wrap` op unaided and why it is the honest ceiling rather
    /// than a strawman.
    #[test]
    fn wrapping_addition_is_associative_at_eight_bits() {
        let mut bad = 0u64;
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                for c in 0..=255u8 {
                    if a.wrapping_add(b).wrapping_add(c) != a.wrapping_add(b.wrapping_add(c)) {
                        bad += 1;
                    }
                }
            }
        }
        assert_eq!(bad, 0);
    }

    /// The two const verdicts are what the gate arms claim they are, and they
    /// are computed rather than asserted: one law holds and the other does not,
    /// so the true gate selects the reassociated arm and the false gate selects
    /// the fallback. Without this the pair is two names for one thing.
    #[test]
    fn the_two_const_verdicts_differ_and_are_computed() {
        const TRUE_VERDICT: bool = saturating_add_is_associative_at(6);
        const FALSE_VERDICT: bool = saturating_sub_is_associative_at(6);
        assert!(TRUE_VERDICT, "saturating addition should be associative");
        assert!(
            !FALSE_VERDICT,
            "saturating subtraction should not be associative, so the false gate is not a control"
        );
    }

    /// The key encoding round-trips, so a bench row's `n` names the geometry the
    /// findings say it names.
    #[test]
    fn the_key_encoding_round_trips() {
        for li in 0..L_TABLE.len() {
            for nc in 0..2usize {
                for al in 0..2usize {
                    for op in 0..2usize {
                        let key = (li + 1) * 1000 + nc * 100 + al * 10 + op;
                        assert_eq!(key_li(key), li);
                        assert_eq!(key_nc(key), nc);
                        assert_eq!(key_al(key), al);
                        assert_eq!(key_op(key), op);
                        assert_eq!(key_l(key), L_TABLE[li]);
                    }
                }
            }
        }
    }
}
