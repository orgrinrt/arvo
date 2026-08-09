//! Second emitted-code probe for file 81, after the first one found its own
//! defect: `period(W)` and `group_bytes(W)` written as `const fn` calls in
//! *value* position are not const-evaluated. rustc only guarantees const
//! evaluation in a const position, and LLVM failed to fold the recursive gcd,
//! so `decoders.rs`'s `period` shape emitted a runtime `udiv`/`msub` loop
//! computing the period on every call, plus a live per-element guard against
//! the runtime period. Half the shifts still folded (jump threading found
//! them), which is exactly the kind of partial success that reads as working.
//!
//! This file puts the same quantities where the design would put them: on a
//! trait, as associated consts, monomorphised per width. An associated const
//! is const-evaluated by construction, so `P`, `G`, `MASK`, and the per-lane
//! `(byte offset, bit shift)` plan are literals in the emitted IR and the
//! group loop unrolls against a literal trip count.
//!
//! The plan array is `[(u8, u8); 8]` at every width, not `[(u8, u8); P]`,
//! because a length that is an expression of a generic parameter needs
//! `generic_const_exprs`, forbidden. 8 is the maximum period over every
//! width (`P = 8 / gcd(W, 8) <= 8`), so a fixed-length-8 plan with `P` live
//! entries costs nothing at runtime and needs no gate. This is the same shape
//! the review's own capacity work landed on: a const the type carries, and a
//! literal the language's own grammar forces.

// ---------------------------------------------------------------------------
// the packing plan, as associated consts
// ---------------------------------------------------------------------------

const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
const fn period_of(w: usize) -> usize {
    8 / gcd(w, 8)
}
const fn group_bytes_of(w: usize) -> usize {
    w * period_of(w) / 8
}
const fn mask_of(w: usize) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}
const fn plan_of(w: usize) -> [(u8, u8); 8] {
    let mut out = [(0u8, 0u8); 8];
    let mut j = 0;
    while j < period_of(w) {
        let bit = j * w;
        out[j] = ((bit >> 3) as u8, (bit & 7) as u8);
        j += 1;
    }
    out
}

/// What a `Layout::Bitpacked` column of a given logical width knows about
/// itself before any value exists. Everything here is a function of W alone.
pub trait Packing {
    /// The logical field width in bits.
    const W: usize;
    /// Elements per whole-byte group, `8 / gcd(W, 8)`.
    const P: usize;
    /// Bytes per group, `W * P / 8`.
    const G: usize;
    /// The field mask.
    const MASK: u64;
    /// Per-lane byte offset and bit shift within a group. Only the first `P`
    /// entries are live.
    const PLAN: [(u8, u8); 8];
    /// Bytes of read headroom a whole-word load needs past the last field.
    const HEADROOM: usize;
}

pub struct Pack<const W: usize>;

impl<const W: usize> Packing for Pack<W> {
    const W: usize = W;
    const P: usize = period_of(W);
    const G: usize = group_bytes_of(W);
    const MASK: u64 = mask_of(W);
    const PLAN: [(u8, u8); 8] = plan_of(W);
    const HEADROOM: usize = 8;
}

#[inline(always)]
unsafe fn ld64(buf: &[u8], at: usize) -> u64 {
    unsafe {
        u64::from_le_bytes([
            *buf.get_unchecked(at),
            *buf.get_unchecked(at + 1),
            *buf.get_unchecked(at + 2),
            *buf.get_unchecked(at + 3),
            *buf.get_unchecked(at + 4),
            *buf.get_unchecked(at + 5),
            *buf.get_unchecked(at + 6),
            *buf.get_unchecked(at + 7),
        ])
    }
}

// ---------------------------------------------------------------------------
// the period-unrolled decode, driven by the associated consts
// ---------------------------------------------------------------------------

/// # Safety
/// `buf` holds `n * W` bits plus `K::HEADROOM` bytes, and `n` is a multiple of
/// `K::P`. Both are declaration-time facts about the column, not runtime ones.
#[inline(always)]
pub unsafe fn sum_plan<K: Packing>(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    let mut base = 0usize;
    let groups = n / K::P;
    for _ in 0..groups {
        for j in 0..K::P {
            let (bo, sh) = K::PLAN[j];
            let word = unsafe { ld64(buf, base + bo as usize) };
            s = s.wrapping_add((word >> sh) & K::MASK);
        }
        base += K::G;
    }
    s
}

/// The same, with the group loop hand-written against `K::P` as a literal
/// match rather than an indexed plan, to separate "the plan array costs a
/// load" from "the shifts are constants".
#[inline(always)]
pub unsafe fn sum_plan_direct<K: Packing>(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    let mut base = 0usize;
    let groups = n / K::P;
    for _ in 0..groups {
        let mut j = 0usize;
        while j < K::P {
            let bit = j * K::W;
            let word = unsafe { ld64(buf, base + (bit >> 3)) };
            s = s.wrapping_add((word >> (bit & 7)) & K::MASK);
            j += 1;
        }
        base += K::G;
    }
    s
}

// ---------------------------------------------------------------------------
// exported monomorphic entry points
// ---------------------------------------------------------------------------

macro_rules! probe {
    ($w:literal, $plan:ident, $direct:ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $plan(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            unsafe { sum_plan::<Pack<$w>>(b, n) }
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn $direct(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            unsafe { sum_plan_direct::<Pack<$w>>(b, n) }
        }
    };
}

probe!(13, plan_w13, direct_w13);
probe!(11, plan_w11, direct_w11);
probe!(3, plan_w3, direct_w3);
probe!(12, plan_w12, direct_w12);
probe!(20, plan_w20, direct_w20);
probe!(16, plan_w16, direct_w16);
probe!(27, plan_w27, direct_w27);
probe!(57, plan_w57, direct_w57);

// ---------------------------------------------------------------------------
// correctness
// ---------------------------------------------------------------------------

fn pack<const W: usize>(vals: &[u64], out: &mut [u8]) {
    for (i, &v) in vals.iter().enumerate() {
        let bit = i * W;
        let mut byte = bit >> 3;
        let mut sh = (bit & 7) as u32;
        let mut left = W;
        let mut field = v & mask_of(W);
        while left > 0 {
            let room = 8 - sh;
            let take = if left < room as usize {
                left
            } else {
                room as usize
            };
            let chunk = (field & ((1u64 << take) - 1)) as u8;
            out[byte] |= chunk << sh;
            field >>= take;
            left -= take;
            sh = 0;
            byte += 1;
        }
    }
}

fn check<const W: usize>(n: usize) {
    let mut rng = 0x243F_6A88_85A3_08D3u64;
    let mut vals = Vec::new();
    for _ in 0..n {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        vals.push(rng & mask_of(W));
    }
    let bytes = (n * W).div_ceil(8) + 16;
    let mut buf = vec![0u8; bytes];
    pack::<W>(&vals, &mut buf);
    let expect: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
    assert_eq!(
        unsafe { sum_plan::<Pack<W>>(&buf, n) },
        expect,
        "plan W={W}"
    );
    assert_eq!(
        unsafe { sum_plan_direct::<Pack<W>>(&buf, n) },
        expect,
        "direct W={W}"
    );
    println!(
        "W={W:3} P={:2} G={:3} plan={:?} ok",
        <Pack<W> as Packing>::P,
        <Pack<W> as Packing>::G,
        &<Pack<W> as Packing>::PLAN[..<Pack<W> as Packing>::P]
    );
}

fn main() {
    check::<13>(1024);
    check::<11>(1024);
    check::<3>(1024);
    check::<12>(1024);
    check::<20>(1024);
    check::<16>(1024);
    check::<27>(1024);
    check::<57>(1024);
    println!("all plan-driven shapes agree with the packer");
}
