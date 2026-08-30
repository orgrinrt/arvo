//! Shared data model and transforms for the clamping-semantics bench.
//!
//! ## Why this exists beside `bench-warm-container-shared`
//!
//! File `141` benched the container fork with six arms, and every one of them
//! implements `Warm` as **wrapping** at the declared width `W`. The standing
//! base's ratified fixed-point preset table
//! (`124_consolidation_twelve.md:2604-2612`, ratified in full at `70b`) gives
//! `Warm` and `Cold` the resolution **clamp**, and gives `ReduceModulo`
//! (wrapping) to `Hot` alone. So `141`'s `Warm` half measures `Hot`'s
//! resolution, and the question of what the doubled container is worth under
//! the semantics the design actually ratified is unmeasured.
//!
//! It matters because the two resolutions have opposite algebra. Reduction
//! modulo `2^W` is a ring homomorphism, so the eagerly-projected and the
//! deferred forms compute the same value and the container can only ever be
//! overhead; that is `141` sections 6 and 11 and it is correct. Clamping is
//! **not** a homomorphism, and the ratified ground for the doubling is exactly
//! that difference: "doubled storage lets a chain of operations retain more
//! than one operation's exactness before a narrow forces a decision"
//! (`124:2616-2617`).
//!
//! ## The workload, and why it is a chunked fold rather than a whole column
//!
//! `141` section 3 records that its saturating fold over 8192 terms was
//! constant-folded by LLVM, because a saturating accumulator over that many
//! terms pins at the limit after a handful of elements and stops depending on
//! the input. That is not a defect of the arm, it is a fact about the shape: a
//! clamping reduction over a whole column at any width this bench sweeps
//! always saturates.
//!
//! The shape the design's own fold rule is about is a fold of **arity `n`**
//! (`124:1470-1482`, section 1.8). A filter tap count, a MAC length, a
//! per-vertex neighbour sum. So the workload here folds the column in chunks
//! of `n`, clamps each chunk's sum at `W`, and combines the chunk results with
//! an exclusive or so nothing is dead. The clamp fires on a data-dependent
//! fraction of chunks, so the answer depends on every input element and the
//! loop cannot be folded to a constant. `chunked_answer_depends_on_every_element`
//! asserts that rather than assuming it, which is the check `141`'s void run
//! needed and did not have.
//!
//! ## The interior-safety predicate, which is what selects an arm
//!
//! From `124:1474-1476`: a fold of arity `n` over destination numeral `N` with
//! accumulator numeral `M` is interior-safe when `(n-1) * [min V(N), max V(N)]`
//! is contained in `[min V(M), max V(M)]`. For an unsigned `W`-bit numeral that
//! is `W + ceil(log2 n) <= width(M)`. Where it holds, no clamp fires in the
//! interior and the fold is one clamp at the root over a plain wrapping sum.
//! Where it fails, every step must clamp.
//!
//! **Every arm computes the same value.** Clamping is a retraction on
//! non-negative addition: `min(min(a + b, L) + c, L) = min(a + b + c, L)`,
//! because once the running sum reaches `L` no further non-negative term brings
//! it back below, and while it has not the inner `min` is the identity. So the
//! eagerly-clamped and the once-clamped forms agree exactly, and an arm that is
//! interior-safe is not computing something easier, it is computing the
//! identical thing with the clamps proved dead.
//!
//! ## The arms, and why each is a competitor
//!
//! `head` is the shipped rule: storage is `rung(rung_bits(W)+1)`, and the
//! accumulator is the storage type. Interior-safe when
//! `W + ceil(log2 n) <= 2 * rung(W)`.
//!
//! `min` is `140`'s deletion and `141`'s verdict: storage and accumulator are
//! both the minimum native that holds `W`. Interior-safe only when
//! `W + ceil(log2 n) <= rung(W)`, which at an exactly-filled width is never.
//!
//! `acc64` is `141`'s own theorem arm generalised to arity: minimum storage,
//! accumulator pinned at 64 bits. Interior-safe when `W + ceil(log2 n) <= 64`.
//!
//! `accfit` is the design's own rule: minimum storage, accumulator the
//! **narrowest** rung satisfying the predicate. Always interior-safe, and
//! narrower than `acc64` at every width and arity where the predicate leaves
//! room, which is where the lane count doubles.
//!
//! `accfit_dyn` is `accfit` with the arity passed as a runtime value rather
//! than a const generic. It exists to price one static lever on its own: what
//! knowing the fold arity at compile time is worth, with everything else held.
//!
//! ## Key encoding
//!
//! `KEY = W * 10000 + NC * 1000 + LOG2A * 10 + OP`
//!
//! `W` is the declared width. `NC` selects the element count (0 is 8192, 1 is
//! 1048576). `LOG2A` is the base-two logarithm of the fold arity. `OP` selects
//! the transform: 0 is the chunked clamping fold, 1 is an elementwise clamping
//! chain of four steps whose value is combined by exclusive or.
//!
//! So `80040` is 8 bits, 8192 elements, arity 16, chunked fold.
//!
//! Bench infrastructure, not shipping arvo source: `std` is used freely,
//! matching every sibling variant crate in this directory.

use mockspace_bench_core::Routine;

// ---------------------------------------------------------------------------
// Key decoding.
// ---------------------------------------------------------------------------

pub const fn key_w(key: usize) -> u32 {
    (key / 10_000) as u32
}
pub const fn key_nc(key: usize) -> usize {
    (key / 1_000) % 10
}
pub const fn key_log2a(key: usize) -> u32 {
    ((key / 10) % 100) as u32
}
pub const fn key_op(key: usize) -> usize {
    key % 10
}
pub const fn key_arity(key: usize) -> usize {
    1usize << key_log2a(key)
}

pub const N_SMALL: usize = 8_192;
pub const N_LARGE: usize = 1_048_576;

pub const fn key_n(key: usize) -> usize {
    if key_nc(key) == 0 {
        N_SMALL
    } else {
        N_LARGE
    }
}

/// The rung a width lands on.
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
pub const fn min_bytes_for(w: u32) -> usize {
    (rung_bits(w) / 8) as usize
}
pub const fn head_bytes_for(w: u32) -> usize {
    (rung_bits(rung_bits(w) + 1) / 8) as usize
}

/// `ceil(log2 n)` for a power of two `n`, which is what the arity always is
/// here.
pub const fn ceil_log2(n: usize) -> u32 {
    (usize::BITS - 1) - (n.leading_zeros())
}

/// The interior-safety threshold from `124:1474-1476`, specialised to an
/// unsigned `W`-bit numeral and a fold of arity `n`: the accumulator must hold
/// `n * (2^W - 1)`, so it needs `W + ceil(log2 n)` bits.
pub const fn accumulator_bits_needed(w: u32, arity: usize) -> u32 {
    w + ceil_log2(arity)
}

/// The narrowest native rung that satisfies the predicate, which is what
/// `accfit` picks. Above 128 there is no rung and the caller must clamp
/// eagerly; no key this bench declares reaches that.
pub const fn accfit_bits(w: u32, arity: usize) -> u32 {
    rung_bits(accumulator_bits_needed(w, arity))
}

// ---------------------------------------------------------------------------
// Carrier.
// ---------------------------------------------------------------------------

/// A machine integer a numeral of some declared width can live in.
///
/// Deliberately a second, independent declaration rather than an import from
/// `bench-warm-container-shared`. Sharing the trait would couple two bench
/// sections whose arms differ, and a change made for one would silently move
/// the other's numbers.
pub trait Carrier: Copy + 'static {
    const BITS: u32;
    const ZERO: Self;
    const MAX: Self;

    fn from_u128(v: u128) -> Self;
    fn to_u128(self) -> u128;

    fn wadd(self, o: Self) -> Self;
    fn wmul(self, o: Self) -> Self;
    fn bxor(self, o: Self) -> Self;
    fn shr1(self) -> Self;
    fn sat_add(self, o: Self) -> Self;
    fn sat_sub(self, o: Self) -> Self;
    fn min_with(self, o: Self) -> Self;

    /// `2^w - 1` in this carrier, or `Self::MAX` when `w` fills it.
    fn limit(w: u32) -> Self;
}

macro_rules! impl_carrier {
    ($t:ty) => {
        impl Carrier for $t {
            const BITS: u32 = <$t>::BITS;
            const ZERO: Self = 0;
            const MAX: Self = <$t>::MAX;

            #[inline(always)]
            fn from_u128(v: u128) -> Self {
                v as $t
            }
            #[inline(always)]
            fn to_u128(self) -> u128 {
                self as u128
            }
            #[inline(always)]
            fn wadd(self, o: Self) -> Self {
                self.wrapping_add(o)
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
            fn shr1(self) -> Self {
                self >> 1
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
        }
    };
}

impl_carrier!(u8);
impl_carrier!(u16);
impl_carrier!(u32);
impl_carrier!(u64);
impl_carrier!(u128);

// ---------------------------------------------------------------------------
// The transforms. One definition each; every arm calls one of them.
// ---------------------------------------------------------------------------

/// The additive operand for the elementwise chain, derived from `W` alone so
/// every arm at a width sees the same numeric stream.
pub const fn operand_for(w: u32) -> u128 {
    let lim: u128 = if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    };
    (lim / 3) | 1
}

/// Chunked clamping fold, arity known at compile time.
///
/// `S` is the storage carrier the column is read through. `A` is the
/// accumulator carrier. When `A` is wide enough for `W + ceil(log2 ARITY)`
/// bits the interior clamps are provably dead and the body below is a plain
/// wrapping sum with one clamp per chunk; when it is not, the eager path
/// clamps every step. Both compute the same value, per the retraction lemma in
/// the module documentation, and `every_arm_agrees_with_the_oracle_on_every_key`
/// checks it rather than trusting it.
#[inline(always)]
pub fn fold_chunked<S: Carrier, A: Carrier, const W: u32, const ARITY: usize>(data: &[S]) -> u64 {
    let limit = A::limit(W);
    let safe = accumulator_bits_needed(W, ARITY) <= A::BITS;
    let mut out = A::ZERO;
    let mut base = 0usize;
    while base + ARITY <= data.len() {
        let chunk = &data[base..base + ARITY];
        let mut acc = A::ZERO;
        if safe {
            for &x in chunk {
                acc = acc.wadd(A::from_u128(x.to_u128()));
            }
            acc = acc.min_with(limit);
        } else {
            for &x in chunk {
                acc = acc.sat_add(A::from_u128(x.to_u128())).min_with(limit);
            }
        }
        out = out.bxor(acc);
        base += ARITY;
    }
    out.to_u128() as u64
}

/// Chunked clamping fold, eager, with the fold reassociated into `LANES`
/// independent saturating accumulators combined at the end.
///
/// Legal for exactly the reason the retraction lemma gives: unsigned
/// saturating addition is associative and commutative, so the lanes may be
/// filled in any order and combined by the same operation. `141:620-621` names
/// this reassociation and reports it unpriced, because the workload it had for
/// it saturated and folded to a constant. The chunked shape here does not.
///
/// This arm exists to attack the mechanism rather than report it. A minimum
/// container at an arity above a handful loses by more than an order of
/// magnitude, and the mechanism is one serial dependence through an operator
/// LLVM will not reassociate. If supplying the reassociation recovers the
/// loss, the container was never the thing that cost anything.
#[inline(always)]
pub fn fold_chunked_lanes<S: Carrier, const W: u32, const ARITY: usize, const LANES: usize>(
    data: &[S],
) -> u64 {
    let limit = S::limit(W);
    let mut out = S::ZERO;
    let mut base = 0usize;
    while base + ARITY <= data.len() {
        let chunk = &data[base..base + ARITY];
        let mut lane = [S::ZERO; LANES];
        let full = ARITY - (ARITY % LANES);
        let mut i = 0usize;
        while i < full {
            let mut l = 0usize;
            while l < LANES {
                lane[l] = lane[l].sat_add(chunk[i + l]).min_with(limit);
                l += 1;
            }
            i += LANES;
        }
        let mut acc = S::ZERO;
        let mut l = 0usize;
        while l < LANES {
            acc = acc.sat_add(lane[l]).min_with(limit);
            l += 1;
        }
        while i < ARITY {
            acc = acc.sat_add(chunk[i]).min_with(limit);
            i += 1;
        }
        out = out.bxor(acc);
        base += ARITY;
    }
    out.to_u128() as u64
}

/// Chunked clamping fold with the arity as a runtime value.
///
/// Identical body to `fold_chunked` with `ARITY` moved from a const generic to
/// a parameter, so the difference between the two arms is exactly one static
/// fact and nothing else.
#[inline(always)]
pub fn fold_chunked_dyn<S: Carrier, A: Carrier, const W: u32>(data: &[S], arity: usize) -> u64 {
    let limit = A::limit(W);
    let safe = accumulator_bits_needed(W, arity) <= A::BITS;
    let mut out = A::ZERO;
    let mut base = 0usize;
    while base + arity <= data.len() {
        let chunk = &data[base..base + arity];
        let mut acc = A::ZERO;
        if safe {
            for &x in chunk {
                acc = acc.wadd(A::from_u128(x.to_u128()));
            }
            acc = acc.min_with(limit);
        } else {
            for &x in chunk {
                acc = acc.sat_add(A::from_u128(x.to_u128())).min_with(limit);
            }
        }
        out = out.bxor(acc);
        base += arity;
    }
    out.to_u128() as u64
}

/// Elementwise clamping chain: four steps of `v = clamp(v*2 + k)` then
/// `v = v >> 1`, combined by exclusive or.
///
/// The doubling is what makes the clamp fire on a data-dependent fraction of
/// elements, and the halving is what stops the chain from pinning at the limit
/// after one step. `S` is the storage carrier; `A` is the register the chain
/// runs in, which is the axis this transform exists to measure: the shipped
/// rule pays for a wide `A` in memory, and the alternative is to load narrow
/// and widen in register.
#[inline(always)]
pub fn chain_elementwise<S: Carrier, A: Carrier, const W: u32, const D: usize>(data: &[S]) -> u64 {
    let limit = A::limit(W);
    let k = A::from_u128(operand_for(W));
    let mut out = A::ZERO;
    for &x in data {
        let mut v = A::from_u128(x.to_u128());
        for _ in 0..D {
            v = v.sat_add(v).min_with(limit);
            v = v.sat_add(k).min_with(limit);
            v = v.shr1();
        }
        out = out.bxor(v);
    }
    out.to_u128() as u64
}

// ---------------------------------------------------------------------------
// Input layout. Two regions so the storage width an arm reads through is a
// real memory-traffic difference rather than a cast.
// ---------------------------------------------------------------------------

pub const HEAD_BYTES: usize = N_LARGE * 16;
pub const MIN_BYTES: usize = N_LARGE * 8;
pub const TOTAL_INPUT_BYTES: usize = HEAD_BYTES + MIN_BYTES;

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Cols {
    pub head: [u8; HEAD_BYTES],
    pub min: [u8; MIN_BYTES],
}

impl Cols {
    #[inline(always)]
    pub fn head_slice<C: Carrier>(&self, n: usize) -> &[C] {
        assert!(n * core::mem::size_of::<C>() <= HEAD_BYTES);
        unsafe { core::slice::from_raw_parts(self.head.as_ptr() as *const C, n) }
    }
    #[inline(always)]
    pub fn min_slice<C: Carrier>(&self, n: usize) -> &[C] {
        assert!(n * core::mem::size_of::<C>() <= MIN_BYTES);
        unsafe { core::slice::from_raw_parts(self.min.as_ptr() as *const C, n) }
    }
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

fn store_le(buf: &mut [u8], off: usize, bytes: usize, v: u128) {
    let le = v.to_le_bytes();
    buf[off..off + bytes].copy_from_slice(&le[..bytes]);
}
fn load_le(buf: &[u8], off: usize, bytes: usize) -> u128 {
    let mut le = [0u8; 16];
    le[..bytes].copy_from_slice(&buf[off..off + bytes]);
    u128::from_le_bytes(le)
}

/// The independent oracle. Computes the declared semantics in `u128` with
/// explicit clamping, sharing no code with the transforms above and no carrier
/// type with any arm.
pub fn reference(values: &[u128], w: u32, op: usize, arity: usize) -> u64 {
    let l: u128 = if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    };
    if op == 0 {
        let mut out: u128 = 0;
        let mut base = 0usize;
        while base + arity <= values.len() {
            let mut acc: u128 = 0;
            for &x in &values[base..base + arity] {
                acc += x;
                if acc > l {
                    acc = l;
                }
            }
            out ^= acc;
            base += arity;
        }
        out as u64
    } else {
        let k = operand_for(w);
        let mut out: u128 = 0;
        for &x in values {
            let mut v = x;
            for _ in 0..4 {
                v = v + v;
                if v > l {
                    v = l;
                }
                v += k;
                if v > l {
                    v = l;
                }
                v >>= 1;
            }
            out ^= v;
        }
        out as u64
    }
}

/// The magnitude an element is drawn from, which is not the numeral's full
/// range and the reason is the defect `141` section 3 records.
///
/// A clamping fold of arity `n` over elements drawn uniformly from the whole
/// `W`-bit range saturates in the first few terms at every arity above one,
/// so the chunk result stops depending on the data and LLVM is free to delete
/// the loop. `141`'s saturating fold hit exactly this and six committed rows
/// are void because of it.
///
/// The fix is not a smaller width, it is the distribution a clamping fold
/// actually runs on. A MAC of `n` taps over a normalised signal has terms of
/// order `L / n`; that is what "the destination numeral holds the result" means
/// for a fold that is not meant to clip on every input. Drawing from
/// `[0, 2L/n]` puts the expected chunk sum at `L`, so roughly half the chunks
/// clamp and the other half do not, which is the only distribution where the
/// clamp is neither dead nor absorbing.
///
/// **arvo's accumulator sizing does not get to know this.** The interior-safety
/// predicate is a function of the *declared* width, so every arm still sizes
/// its accumulator for `n * (2^W - 1)`. The data being friendlier than the
/// declaration is realistic and it is not something the typestate can exploit.
pub const fn element_cap(w: u32, arity: usize) -> u128 {
    let l: u128 = if w >= 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    };
    if arity <= 1 {
        return l;
    }
    let a = arity as u128;
    let c = (2 * l).div_ceil(a);
    if c == 0 {
        1
    } else {
        c
    }
}

pub fn build_bytes(key: usize, seed: u64) -> std::vec::Vec<u8> {
    let w = key_w(key);
    let n = key_n(key);
    let mb = min_bytes_for(w);
    let hb = head_bytes_for(w);
    let cap = element_cap(w, key_arity(key));

    let mut rng = SplitMix64(seed ^ 0x00C0_FFEE_0BAD_F00D);
    let mut buf = std::vec![0u8; TOTAL_INPUT_BYTES];
    for i in 0..n {
        let v = (rng.next() as u128) % (cap + 1);
        store_le(&mut buf, i * hb, hb, v);
        store_le(&mut buf, HEAD_BYTES + i * mb, mb, v);
    }
    buf
}

impl<const KEY: usize> Routine for Case<KEY> {
    type Input = Cols;
    type Output = Sum;

    fn build_input(_seed: u64) -> Self::Input {
        unreachable!("Case::build_input is never called by the bench path; Self::Input is 24 MiB")
    }

    fn build_input_bytes(seed: u64) -> std::vec::Vec<u8> {
        build_bytes(KEY, seed)
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let w = key_w(KEY);
        let n = key_n(KEY);
        let arity = key_arity(KEY);
        let op = key_op(KEY);
        let mb = min_bytes_for(w);
        let hb = head_bytes_for(w);

        let from_min: std::vec::Vec<u128> =
            (0..n).map(|i| load_le(&input.min, i * mb, mb)).collect();
        let from_head: std::vec::Vec<u128> =
            (0..n).map(|i| load_le(&input.head, i * hb, hb)).collect();
        if from_min != from_head {
            return Err(
                "the two carrier regions hold different logical columns, so the arms were \
                 not fed the same input and no comparison between them means anything",
            );
        }
        if reference(&from_min, w, op, arity) != output.value {
            return Err(
                "output disagrees with the independent u128 reference, so the timed \
                 transform does not compute the declared clamping semantics",
            );
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        key_n(KEY) as u64
    }
}

// ---------------------------------------------------------------------------
// Arm dispatch. `KEY` is a const generic and `generic_const_exprs` is
// forbidden, so `W` and the arity are recovered as ordinary values and
// matched; after inlining they are constants and the match folds.
// ---------------------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! __clamp_arity_arm {
    ($w:literal, $s:ty, $accsel:ident, $a:expr, $cols:expr, $n:expr) => {{
        match $a {
            2 => $crate::__clamp_pick!($w, $s, $accsel, 2, $cols, $n),
            4 => $crate::__clamp_pick!($w, $s, $accsel, 4, $cols, $n),
            8 => $crate::__clamp_pick!($w, $s, $accsel, 8, $cols, $n),
            16 => $crate::__clamp_pick!($w, $s, $accsel, 16, $cols, $n),
            64 => $crate::__clamp_pick!($w, $s, $accsel, 64, $cols, $n),
            256 => $crate::__clamp_pick!($w, $s, $accsel, 256, $cols, $n),
            1024 => $crate::__clamp_pick!($w, $s, $accsel, 1024, $cols, $n),
            other => panic!("arity {} is not declared by this bench", other),
        }
    }};
}

/// Chooses the accumulator carrier for one `(W, arity)` cell and one arm
/// policy, then calls the single transform.
///
/// `storage` uses the storage carrier as the accumulator. `wide64` pins the
/// accumulator at 64 bits. `fit` picks the narrowest rung that satisfies the
/// interior-safety predicate, which is the design's own rule at
/// `124:1474-1476`.
#[doc(hidden)]
#[macro_export]
macro_rules! __clamp_pick {
    ($w:literal, $s:ty, storage, $a:literal, $cols:expr, $n:expr) => {
        $crate::fold_chunked::<$s, $s, $w, $a>($cols.min_slice::<$s>($n))
    };
    ($w:literal, $s:ty, storage_head, $a:literal, $cols:expr, $n:expr) => {
        $crate::fold_chunked::<$s, $s, $w, $a>($cols.head_slice::<$s>($n))
    };
    ($w:literal, $s:ty, wide64, $a:literal, $cols:expr, $n:expr) => {
        $crate::fold_chunked::<$s, u64, $w, $a>($cols.min_slice::<$s>($n))
    };
    ($w:literal, $s:ty, fit, $a:literal, $cols:expr, $n:expr) => {{
        const BITS: u32 = $crate::accfit_bits($w, $a);
        match BITS {
            8 => $crate::fold_chunked::<$s, u8, $w, $a>($cols.min_slice::<$s>($n)),
            16 => $crate::fold_chunked::<$s, u16, $w, $a>($cols.min_slice::<$s>($n)),
            32 => $crate::fold_chunked::<$s, u32, $w, $a>($cols.min_slice::<$s>($n)),
            64 => $crate::fold_chunked::<$s, u64, $w, $a>($cols.min_slice::<$s>($n)),
            _ => $crate::fold_chunked::<$s, u128, $w, $a>($cols.min_slice::<$s>($n)),
        }
    }};
}

/// Declares one arm: a storage carrier per width, an accumulator policy, and a
/// register carrier for the elementwise chain.
#[doc(hidden)]
#[macro_export]
macro_rules! declare_clamp_arm {
    ($name:ident, $accsel:ident, $region:ident, $($w:literal => ($s:ty, $chain:ty)),+ $(,)?) => {
        pub fn $name(key: usize, cols: &$crate::Cols) -> u64 {
            let n = $crate::key_n(key);
            let a = $crate::key_arity(key);
            let op = $crate::key_op(key);
            match ($crate::key_w(key), op) {
                $(
                    ($w, 0) => $crate::__clamp_arity_arm!($w, $s, $accsel, a, cols, n),
                    ($w, 1) => $crate::chain_elementwise::<$s, $chain, $w, 4>(
                        cols.$region::<$s>(n)
                    ),
                )+
                (w, op) => panic!("width {} op {} is not declared by this bench", w, op),
            }
        }
    };
}

pub mod arms {
    #[allow(unused_imports)]
    use super::*;

    // The shipped rule: storage is one rung above the minimum, and both the
    // fold accumulator and the chain register are that storage type.
    declare_clamp_arm!(
        head,
        storage_head,
        head_slice,
        8 => (u16, u16),
        13 => (u32, u32),
        16 => (u32, u32),
        32 => (u64, u64),
        60 => (u128, u128),
        64 => (u128, u128),
    );

    // `140`'s deletion and `141`'s verdict: minimum storage, and nothing wider
    // anywhere. At an exactly-filled width the fold's interior clamps are live
    // and the chain runs in the storage type.
    declare_clamp_arm!(
        min,
        storage,
        min_slice,
        8 => (u8, u8),
        13 => (u16, u16),
        16 => (u16, u16),
        32 => (u32, u32),
        60 => (u64, u64),
        64 => (u64, u64),
    );

    // `141`'s theorem arm generalised to arity: minimum storage, accumulator
    // pinned at 64 bits. For the elementwise chain the register is the next
    // rung up, which is the "load narrow, widen in register" shape.
    declare_clamp_arm!(
        acc64,
        wide64,
        min_slice,
        8 => (u8, u16),
        13 => (u16, u32),
        16 => (u16, u32),
        32 => (u32, u64),
        60 => (u64, u128),
        64 => (u64, u128),
    );

    // The design's own rule at `124:1474-1476`: minimum storage, accumulator
    // the narrowest rung satisfying `W + ceil(log2 n) <= width(M)`.
    declare_clamp_arm!(
        accfit,
        fit,
        min_slice,
        8 => (u8, u16),
        13 => (u16, u32),
        16 => (u16, u32),
        32 => (u32, u64),
        60 => (u64, u128),
        64 => (u64, u128),
    );
}

/// The minimum container with the eager clamping fold reassociated into eight
/// lanes. Separate from the macro because it takes a third const parameter.
pub mod arms_lanes {
    #[allow(unused_imports)]
    use super::*;

    macro_rules! lanes {
        ($w:literal, $s:ty, $cols:expr, $n:expr, $a:expr) => {
            match $a {
                4 => fold_chunked_lanes::<$s, $w, 4, 4>($cols.min_slice::<$s>($n)),
                8 => fold_chunked_lanes::<$s, $w, 8, 8>($cols.min_slice::<$s>($n)),
                2 => fold_chunked_lanes::<$s, $w, 2, 2>($cols.min_slice::<$s>($n)),
                16 => fold_chunked_lanes::<$s, $w, 16, 8>($cols.min_slice::<$s>($n)),
                64 => fold_chunked_lanes::<$s, $w, 64, 8>($cols.min_slice::<$s>($n)),
                256 => fold_chunked_lanes::<$s, $w, 256, 8>($cols.min_slice::<$s>($n)),
                other => panic!("arity {} is not declared by this bench", other),
            }
        };
    }

    pub fn min_lanes(key: usize, cols: &Cols) -> u64 {
        let n = key_n(key);
        let a = key_arity(key);
        let op = key_op(key);
        match (key_w(key), op) {
            (8, 0) => lanes!(8, u8, cols, n, a),
            (13, 0) => lanes!(13, u16, cols, n, a),
            (16, 0) => lanes!(16, u16, cols, n, a),
            (32, 0) => lanes!(32, u32, cols, n, a),
            (60, 0) => lanes!(60, u64, cols, n, a),
            (64, 0) => lanes!(64, u64, cols, n, a),
            (8, 1) => chain_elementwise::<u8, u8, 8, 4>(cols.min_slice::<u8>(n)),
            (13, 1) => chain_elementwise::<u16, u16, 13, 4>(cols.min_slice::<u16>(n)),
            (16, 1) => chain_elementwise::<u16, u16, 16, 4>(cols.min_slice::<u16>(n)),
            (32, 1) => chain_elementwise::<u32, u32, 32, 4>(cols.min_slice::<u32>(n)),
            (60, 1) => chain_elementwise::<u64, u64, 60, 4>(cols.min_slice::<u64>(n)),
            (64, 1) => chain_elementwise::<u64, u64, 64, 4>(cols.min_slice::<u64>(n)),
            (w, op) => panic!("width {} op {} is not declared by this bench", w, op),
        }
    }
}

/// `accfit` with the arity as a runtime value. Separate from the macro because
/// it deliberately does not take the const-generic path.
pub mod arms_dyn {
    #[allow(unused_imports)]
    use super::*;

    macro_rules! fit_dyn {
        ($w:literal, $s:ty, $cols:expr, $n:expr, $a:expr) => {{
            let bits = accfit_bits($w, $a);
            match bits {
                8 => fold_chunked_dyn::<$s, u8, $w>($cols.min_slice::<$s>($n), $a),
                16 => fold_chunked_dyn::<$s, u16, $w>($cols.min_slice::<$s>($n), $a),
                32 => fold_chunked_dyn::<$s, u32, $w>($cols.min_slice::<$s>($n), $a),
                64 => fold_chunked_dyn::<$s, u64, $w>($cols.min_slice::<$s>($n), $a),
                _ => fold_chunked_dyn::<$s, u128, $w>($cols.min_slice::<$s>($n), $a),
            }
        }};
    }

    pub fn accfit_dyn(key: usize, cols: &Cols) -> u64 {
        let n = key_n(key);
        let a = key_arity(key);
        let op = key_op(key);
        match (key_w(key), op) {
            (8, 0) => fit_dyn!(8, u8, cols, n, a),
            (13, 0) => fit_dyn!(13, u16, cols, n, a),
            (16, 0) => fit_dyn!(16, u16, cols, n, a),
            (32, 0) => fit_dyn!(32, u32, cols, n, a),
            (60, 0) => fit_dyn!(60, u64, cols, n, a),
            (64, 0) => fit_dyn!(64, u64, cols, n, a),
            (8, 1) => chain_elementwise::<u8, u16, 8, 4>(cols.min_slice::<u8>(n)),
            (13, 1) => chain_elementwise::<u16, u32, 13, 4>(cols.min_slice::<u16>(n)),
            (16, 1) => chain_elementwise::<u16, u32, 16, 4>(cols.min_slice::<u16>(n)),
            (32, 1) => chain_elementwise::<u32, u64, 32, 4>(cols.min_slice::<u32>(n)),
            (60, 1) => chain_elementwise::<u64, u128, 60, 4>(cols.min_slice::<u64>(n)),
            (64, 1) => chain_elementwise::<u64, u128, 64, 4>(cols.min_slice::<u64>(n)),
            (w, op) => panic!("width {} op {} is not declared by this bench", w, op),
        }
    }
}

/// Every key this bench declares, in one list, so a test sweeps the whole
/// matrix rather than a chosen subset of it.
/// Every key this bench declares.
///
/// The arity sweep is **ragged by width and that is a finding rather than a
/// convenience.** A clamping fold of arity `n` into a `W`-bit destination has
/// a distribution where the clamp is neither dead nor absorbing only while the
/// mean term `L / n` is at least a few quanta. At `W = 8` and `n = 256` the
/// mean term is under one, so every distribution of non-negative integers with
/// that mean is mostly zeros, and the fold is degenerate rather than
/// measurable. Those cells are absent, named here, and
/// `the_clamp_fires_on_a_real_fraction_of_chunks_at_every_chunked_key` is what
/// would catch them being added back.
pub const ALL_KEYS: [usize; 46] = [
    // op 0, chunked clamping fold, 8192 elements.
    // arity 2, 4, 8 and 16 at every width. Two and eight exist because the
    // eager and interior-safe forms cross somewhere in that range and a sweep
    // that skips it would report the crossover as a step.
    80010, 130010, 160010, 320010, 600010, 640010, 80020, 130020, 160020, 320020, 600020, 640020,
    80030, 130030, 160030, 320030, 600030, 640030, 80040, 130040, 160040, 320040, 600040, 640040,
    // arity 64 and 256 above 8 bits only.
    130060, 160060, 320060, 600060, 640060, 130080, 160080, 320080, 600080, 640080,
    // op 1, elementwise clamping chain.
    80001, 130001, 160001, 320001, 600001, 640001, // op 0, arity 16, 1048576 elements.
    81040, 131040, 161040, 321040, 601040, 641040,
];

#[cfg(test)]
mod tests {
    use super::*;

    const SWEPT: [u32; 6] = [8, 13, 16, 32, 60, 64];

    fn cols_for(key: usize) -> (std::vec::Vec<u8>, usize) {
        (build_bytes(key, 3), key)
    }

    /// The load-bearing test. Every arm must produce the identical value on
    /// the identical input at **every** key the manifest declares, and that
    /// value must equal the independent `u128` reference. Five arms agreeing
    /// on a shared wrong transform is caught by the oracle rather than
    /// confirmed by the agreement.
    #[test]
    fn every_arm_agrees_with_the_oracle_on_every_key() {
        for &key in ALL_KEYS.iter() {
            let (buf, _) = cols_for(key);
            assert_eq!(
                buf.as_ptr() as usize % 16,
                0,
                "key {key}: input buffer is not 16-aligned, so the head arm's u128 view \
                 is unsound and no number from this bench means anything"
            );
            let cols: &Cols = unsafe { &*(buf.as_ptr() as *const Cols) };

            let h = arms::head(key, cols);
            let m = arms::min(key, cols);
            let a64 = arms::acc64(key, cols);
            let af = arms::accfit(key, cols);
            let ad = arms_dyn::accfit_dyn(key, cols);
            let al = arms_lanes::min_lanes(key, cols);

            let n = key_n(key);
            let w = key_w(key);
            let mb = min_bytes_for(w);
            let vals: std::vec::Vec<u128> = (0..n)
                .map(|i| {
                    let mut le = [0u8; 16];
                    le[..mb].copy_from_slice(&buf[HEAD_BYTES + i * mb..HEAD_BYTES + i * mb + mb]);
                    u128::from_le_bytes(le)
                })
                .collect();
            let r = reference(&vals, w, key_op(key), key_arity(key));

            assert_eq!(h, r, "key {key}: head arm disagrees with the oracle");
            assert_eq!(m, r, "key {key}: min arm disagrees with the oracle");
            assert_eq!(a64, r, "key {key}: acc64 arm disagrees with the oracle");
            assert_eq!(af, r, "key {key}: accfit arm disagrees with the oracle");
            assert_eq!(ad, r, "key {key}: accfit_dyn arm disagrees with the oracle");
            assert_eq!(
                al, r,
                "key {key}: min_lanes arm disagrees with the oracle, so the saturating \
                 reassociation is not legal and the whole lane-splitting result is void"
            );
        }
    }

    /// The check `141`'s void saturating run needed and did not have. If the
    /// answer does not depend on the data, LLVM may delete the loop and the
    /// timing measures nothing.
    ///
    /// The perturbation goes into a chunk whose sum is **below** the limit,
    /// because a chunk that clamps absorbs the change by design and finding
    /// that it does is not evidence either way. Both facts are asserted: a bit
    /// flipped in a non-clamping chunk moves the answer, and a bit flipped in a
    /// clamping chunk does not, which is the clamp doing its job.
    #[test]
    fn chunked_answer_depends_on_every_element_the_clamp_did_not_absorb() {
        for &key in ALL_KEYS
            .iter()
            .filter(|k| key_op(**k) == 0 && key_nc(**k) == 0)
        {
            let w = key_w(key);
            let n = key_n(key);
            let arity = key_arity(key);
            let mb = min_bytes_for(w);
            let hb = head_bytes_for(w);
            let l: u128 = (1u128 << w) - 1;
            let buf = build_bytes(key, 3);
            let vals: std::vec::Vec<u128> = (0..n)
                .map(|i| {
                    let mut le = [0u8; 16];
                    le[..mb].copy_from_slice(&buf[HEAD_BYTES + i * mb..HEAD_BYTES + i * mb + mb]);
                    u128::from_le_bytes(le)
                })
                .collect();
            let chunk_sum = |c: usize| -> u128 { vals[c * arity..(c + 1) * arity].iter().sum() };
            let chunks = n / arity;

            let below = (0..chunks).find(|&c| chunk_sum(c) + 1 < l).expect(
                "no chunk sits below the limit, so this key cannot distinguish a live \
                 workload from an absorbed one",
            );
            let above = (0..chunks)
                .find(|&c| chunk_sum(c) > l + 1)
                .expect("no chunk clamps, so this key is measuring a plain wrapping sum");

            let cols: &Cols = unsafe { &*(buf.as_ptr() as *const Cols) };
            let base = arms::accfit(key, cols);

            let flip = |b: &mut std::vec::Vec<u8>, idx: usize| {
                b[idx * hb] ^= 1;
                b[HEAD_BYTES + idx * mb] ^= 1;
            };

            let mut b1 = buf.clone();
            flip(&mut b1, below * arity);
            let c1: &Cols = unsafe { &*(b1.as_ptr() as *const Cols) };
            assert_ne!(
                base,
                arms::accfit(key, c1),
                "key {key}: flipping one bit in a chunk that does not clamp did not move \
                 the answer, so the workload is not measuring the data and the loop may \
                 be constant-folded"
            );

            let mut b2 = buf.clone();
            flip(&mut b2, above * arity);
            assert_eq!(
                base,
                arms::accfit(key, unsafe { &*(b2.as_ptr() as *const Cols) }),
                "key {key}: flipping one bit in a chunk that clamps moved the answer, so \
                 the clamp is not absorbing and the declared semantics is not what is \
                 being computed"
            );
        }
    }

    /// The complement of the test above. If the clamp never fires, the arms
    /// are all measuring a plain wrapping sum and the clamping question is not
    /// being asked at all. Asserted over every chunked key rather than
    /// sampled, because a distribution that silently drifts clamp-free would
    /// leave every number in this section looking reasonable and meaning
    /// nothing.
    #[test]
    fn the_clamp_fires_on_a_real_fraction_of_chunks_at_every_chunked_key() {
        for &key in ALL_KEYS.iter().filter(|k| key_op(**k) == 0) {
            let w = key_w(key);
            let n = key_n(key);
            let arity = key_arity(key);
            let mb = min_bytes_for(w);
            let buf = build_bytes(key, 3);
            let l: u128 = (1u128 << w) - 1;
            let vals: std::vec::Vec<u128> = (0..n)
                .map(|i| {
                    let mut le = [0u8; 16];
                    le[..mb].copy_from_slice(&buf[HEAD_BYTES + i * mb..HEAD_BYTES + i * mb + mb]);
                    u128::from_le_bytes(le)
                })
                .collect();
            let chunks = n / arity;
            let clamped = (0..chunks)
                .filter(|c| {
                    let s: u128 = vals[c * arity..(c + 1) * arity].iter().sum();
                    s > l
                })
                .count();
            let frac = clamped as f64 / chunks as f64;
            assert!(
                frac > 0.05 && frac < 0.95,
                "key {key}: the clamp fires on {:.1}% of chunks, so this row is either \
                 measuring a plain wrapping sum or an absorbed constant rather than \
                 clamping arithmetic",
                frac * 100.0
            );
        }
    }

    /// The two accidental controls this bench's noise floor is read from,
    /// asserted rather than inferred.
    ///
    /// At some cells two arms select the identical accumulator carrier, so they
    /// are the same instantiation of the same function and any measured spread
    /// between them is this harness's resolution on this workload rather than a
    /// result. The harness's own duplicate detector cannot establish this: it
    /// compares the generated `bench_entry` dispatcher, which is a 592-instruction
    /// shim identical across all six arms whatever the arm bodies do.
    ///
    /// So the identity is established here, where it is exact: it is a fact about
    /// which const the accumulator selector returns.
    #[test]
    fn the_noise_floor_controls_really_are_the_same_instantiation() {
        // `accfit` and `acc64` coincide wherever the derived width is 64.
        let mut coincide_32 = 0;
        for arity in [2usize, 4, 8, 16, 64, 256] {
            if accfit_bits(32, arity) == 64 {
                coincide_32 += 1;
            }
        }
        assert_eq!(
            coincide_32, 6,
            "the W=32 rows were quoted as an accfit-equals-acc64 control at every \
             swept arity; if that is not true at all six the quoted noise floor is \
             not measuring identical code"
        );

        // `accfit` and `minimum` coincide wherever the derived width equals the
        // minimum container's width.
        let mut coincide_60 = 0;
        for arity in [2usize, 4, 8, 16] {
            if accfit_bits(60, arity) == rung_bits(60) {
                coincide_60 += 1;
            }
        }
        assert_eq!(
            coincide_60, 4,
            "the W=60 rows at arity 2 through 16 were quoted as an \
             accfit-equals-minimum control; if that is not true at all four the \
             quoted noise floor is not measuring identical code"
        );

        // And the controls must not silently spread to cells quoted as results.
        assert_ne!(
            accfit_bits(8, 16),
            64,
            "W=8 arity=16 was quoted as a result for the accumulator-narrowing \
             lever; if accfit picks 64 there it is a control instead"
        );
        assert_ne!(
            accfit_bits(16, 256),
            rung_bits(16),
            "W=16 arity=256 was quoted as a result against the minimum container; \
             if accfit picks the minimum container there the arms are identical"
        );
    }

    /// The retraction lemma the whole bench rests on, asserted over the full
    /// swept matrix rather than at a chosen width: for non-negative terms,
    /// clamping every step equals clamping once.
    #[test]
    fn clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width() {
        for &w in SWEPT.iter() {
            let l: u128 = (1u128 << w) - 1;
            for arity in [2usize, 4, 16, 64, 256, 1024] {
                let mut rng = SplitMix64(0xDEAD_BEEF ^ (w as u64) ^ (arity as u64));
                for _ in 0..64 {
                    let terms: std::vec::Vec<u128> =
                        (0..arity).map(|_| (rng.next() as u128) & l).collect();
                    let eager = terms.iter().fold(0u128, |a, &x| {
                        let s = a + x;
                        if s > l {
                            l
                        } else {
                            s
                        }
                    });
                    let exact: u128 = terms.iter().sum();
                    let once = if exact > l { l } else { exact };
                    assert_eq!(
                        eager, once,
                        "w={w} arity={arity}: eager and deferred clamping disagree, so \
                         every arm in this bench is computing a different function"
                    );
                }
            }
        }
    }

    /// The interior-safety predicate is what selects an accumulator, so it is
    /// asserted over the whole matrix rather than sampled: the accumulator
    /// `accfit` picks must actually hold `arity * (2^W - 1)`.
    #[test]
    fn accfit_holds_the_exact_sum_at_every_swept_width_and_arity() {
        for &w in SWEPT.iter() {
            for arity in [2usize, 4, 16, 64, 256, 1024] {
                let bits = accfit_bits(w, arity);
                assert!(bits <= 128, "w={w} arity={arity}: no native rung fits");
                let worst: u128 = (arity as u128) * ((1u128 << w) - 1);
                let cap: u128 = if bits >= 128 {
                    u128::MAX
                } else {
                    (1u128 << bits) - 1
                };
                assert!(
                    worst <= cap,
                    "w={w} arity={arity}: accfit picked {bits} bits, which cannot hold \
                     the exact sum {worst}, so the interior clamps it deletes are live"
                );
            }
        }
    }

    /// The shipped container is not always interior-safe, which is the claim
    /// that the doubling is a bad approximation to the fold rule rather than
    /// an implementation of it. Asserted over the whole matrix.
    #[test]
    fn the_shipped_container_fails_interior_safety_at_arities_the_design_expects() {
        let mut failures = 0;
        for &w in SWEPT.iter() {
            let head_bits = (head_bytes_for(w) * 8) as u32;
            for arity in [2usize, 4, 16, 64, 256, 1024] {
                if accumulator_bits_needed(w, arity) > head_bits {
                    failures += 1;
                }
            }
        }
        assert!(
            failures > 0,
            "the shipped doubled container was interior-safe at every swept cell, which \
             would make it an implementation of the fold rule rather than an approximation"
        );
    }
}
