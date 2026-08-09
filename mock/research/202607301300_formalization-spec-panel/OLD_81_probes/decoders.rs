//! Emitted-code probe for file 81: what a bitpacked column read costs, and
//! which parts of that cost are forced by the packing rather than by the way
//! one particular loop was written.
//!
//! Four decoder shapes over the same zero-inter-value-padding buffer, plus the
//! dense native baseline, each monomorphised at several logical widths.
//! Every probed symbol is `#[unsafe(no_mangle)] extern "C"` so it survives to
//! the disassembly; nothing here is arvo source.
//!
//! The shapes:
//!
//! - `naive`: file 75's `extract_zeropad` loop verbatim in shape. Byte offset
//!   and bit shift are both computed from the running index at runtime; the
//!   load width is fixed at 32 bits regardless of W.
//! - `period`: the same buffer read with the loop unrolled by the packing's own
//!   period, `P = 8 / gcd(W, 8)` elements per `W * P / 8` whole bytes. Every
//!   byte offset and every bit shift inside a group is then a compile-time
//!   constant, because it depends only on the element's position within the
//!   group and on W, both of which are known at monomorphisation.
//! - `stream`: a 64-bit bit-reader window, refilled when it runs short. The
//!   shift is a running value, not a constant, but no multiply and no
//!   per-element address computation exist at all.
//! - `native`: the dense `[u16]` baseline, the thing the multiple is measured
//!   against.
//!
//! `period` reads with `get_unchecked`, and the precondition is stated rather
//! than assumed: the buffer carries `LOAD_BYTES` bytes of read headroom past
//! the last field's last byte. That headroom is a function of W alone, so it
//! is a fact a capacity type can carry rather than a runtime check. The `_ck`
//! companion is the same shape with ordinary checked indexing, so the cost of
//! the check is separated from the cost of the packing.

// gcd, and the two quantities every claim in file 81 keys on. Both are const
// fns of the logical width alone: nothing here needs a value.
const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Elements per whole-byte group. 8 for odd W, 4 for W = 2 mod 4, 2 for
/// W = 4 mod 8, 1 for W a multiple of 8.
pub const fn period(w: usize) -> usize {
    8 / gcd(w, 8)
}

/// Bytes per group. Exactly `W * period(W) / 8`, always a whole number.
pub const fn group_bytes(w: usize) -> usize {
    w * period(w) / 8
}

/// The widest bit shift a field can start at is 7, so a load of `W + 7` bits
/// always covers a field. This picks the narrowest machine load width that
/// does, which is what makes the second load unnecessary rather than
/// unavoidable. Returns 0 when no single load suffices (W > 121).
pub const fn load_bytes(w: usize) -> usize {
    if w + 7 <= 8 {
        1
    } else if w + 7 <= 16 {
        2
    } else if w + 7 <= 32 {
        4
    } else if w + 7 <= 64 {
        8
    } else if w + 7 <= 128 {
        16
    } else {
        0
    }
}

#[inline(always)]
fn mask(w: usize) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
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

#[inline(always)]
fn ld64_ck(buf: &[u8], at: usize) -> u64 {
    u64::from_le_bytes([
        buf[at],
        buf[at + 1],
        buf[at + 2],
        buf[at + 3],
        buf[at + 4],
        buf[at + 5],
        buf[at + 6],
        buf[at + 7],
    ])
}

// ---------------------------------------------------------------------------
// shape 1: naive, file 75's own extraction, index-driven
// ---------------------------------------------------------------------------

/// File 75's `extract_zeropad` shape: byte offset and bit shift both derived
/// from the running index at runtime. The load is 64 bits rather than that
/// function's hardwired 32, because a hardwired 32-bit load is silently wrong
/// for W > 25 (`OUTCOMES.md`, finding N1) and the comparison below is meant to
/// isolate the constant-folding, not to re-measure a defect.
#[inline(always)]
pub fn extract_naive<const W: usize>(buf: &[u8], i: usize) -> u64 {
    let bit_off = i * W;
    let byte_off = bit_off >> 3;
    let bit_shift = (bit_off & 7) as u32;
    let w = u64::from_le_bytes([
        buf[byte_off],
        buf[byte_off + 1],
        buf[byte_off + 2],
        buf[byte_off + 3],
        buf[byte_off + 4],
        buf[byte_off + 5],
        buf[byte_off + 6],
        buf[byte_off + 7],
    ]);
    (w >> bit_shift) & mask(W)
}

/// The hardwired-32-bit-load form exactly as file 75 wrote it, kept so the
/// defect is reproducible rather than only described.
#[inline(always)]
pub fn extract_naive_u32<const W: usize>(buf: &[u8], i: usize) -> u64 {
    let bit_off = i * W;
    let byte_off = bit_off >> 3;
    let bit_shift = (bit_off & 7) as u32;
    let w = u32::from_le_bytes([
        buf[byte_off],
        buf[byte_off + 1],
        buf[byte_off + 2],
        buf[byte_off + 3],
    ]);
    ((w >> bit_shift) as u64) & mask(W)
}

#[inline(always)]
pub fn sum_naive_u32<const W: usize>(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add(extract_naive_u32::<W>(buf, i));
    }
    s
}

#[inline(always)]
pub fn sum_naive<const W: usize>(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add(extract_naive::<W>(buf, i));
    }
    s
}

// ---------------------------------------------------------------------------
// shape 2: period-unrolled, every offset and shift a compile-time constant
// ---------------------------------------------------------------------------

/// # Safety
/// `buf` must hold `n * W` bits plus 8 bytes of read headroom, and `n` must be
/// a multiple of `period(W)`. Both are functions of W and the declared
/// capacity, not of any value.
#[inline(always)]
pub unsafe fn sum_period<const W: usize>(buf: &[u8], n: usize) -> u64 {
    let p = period(W);
    let g = group_bytes(W);
    let m = mask(W);
    let mut s = 0u64;
    let mut base = 0usize;
    let groups = n / p;
    for _ in 0..groups {
        // `p` is a compile-time constant after monomorphisation, so this loop
        // fully unrolls and every `bo` / `sh` below folds to a literal.
        for j in 0..p {
            let bit = j * W;
            let bo = bit >> 3;
            let sh = (bit & 7) as u32;
            let word = unsafe { ld64(buf, base + bo) };
            s = s.wrapping_add((word >> sh) & m);
        }
        base += g;
    }
    s
}

/// Same shape with ordinary checked indexing, to separate the bounds check
/// from the packing.
#[inline(always)]
pub fn sum_period_ck<const W: usize>(buf: &[u8], n: usize) -> u64 {
    let p = period(W);
    let g = group_bytes(W);
    let m = mask(W);
    let mut s = 0u64;
    let mut base = 0usize;
    let groups = n / p;
    for _ in 0..groups {
        for j in 0..p {
            let bit = j * W;
            let bo = bit >> 3;
            let sh = (bit & 7) as u32;
            let word = ld64_ck(buf, base + bo);
            s = s.wrapping_add((word >> sh) & m);
        }
        base += g;
    }
    s
}

// ---------------------------------------------------------------------------
// shape 3: streaming bit-reader
// ---------------------------------------------------------------------------

/// # Safety
/// As `sum_period`.
#[inline(always)]
pub unsafe fn sum_stream<const W: usize>(buf: &[u8], n: usize) -> u64 {
    let m = mask(W);
    let mut s = 0u64;
    let mut bitpos = 0usize;
    for _ in 0..n {
        let byte = bitpos >> 3;
        let sh = (bitpos & 7) as u32;
        let word = unsafe { ld64(buf, byte) };
        s = s.wrapping_add((word >> sh) & m);
        bitpos += W;
    }
    s
}

// ---------------------------------------------------------------------------
// shape 4: dense native baseline
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn sum_native_u16(vals: &[u16], n: usize, m: u16) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add((vals[i] & m) as u64);
    }
    s
}

// ---------------------------------------------------------------------------
// monomorphic exported entry points
// ---------------------------------------------------------------------------

macro_rules! probe_width {
    ($w:literal, $naive:ident, $period:ident, $period_ck:ident, $stream:ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $naive(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            sum_naive::<$w>(b, n)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn $period(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            unsafe { sum_period::<$w>(b, n) }
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn $period_ck(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            sum_period_ck::<$w>(b, n)
        }
        #[unsafe(no_mangle)]
        pub extern "C" fn $stream(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            unsafe { sum_stream::<$w>(b, n) }
        }
    };
}

probe_width!(13, naive_w13, period_w13, period_ck_w13, stream_w13);
probe_width!(11, naive_w11, period_w11, period_ck_w11, stream_w11);
probe_width!(3, naive_w3, period_w3, period_ck_w3, stream_w3);
probe_width!(12, naive_w12, period_w12, period_ck_w12, stream_w12);
probe_width!(20, naive_w20, period_w20, period_ck_w20, stream_w20);
probe_width!(16, naive_w16, period_w16, period_ck_w16, stream_w16);
probe_width!(27, naive_w27, period_w27, period_ck_w27, stream_w27);
probe_width!(57, naive_w57, period_w57, period_ck_w57, stream_w57);

#[unsafe(no_mangle)]
pub extern "C" fn native_u16(vals: *const u16, len: usize, n: usize) -> u64 {
    let v = unsafe { core::slice::from_raw_parts(vals, len) };
    sum_native_u16(v, n, 0x1fff)
}

// ---------------------------------------------------------------------------
// correctness: every shape decodes the same stream the packer wrote
// ---------------------------------------------------------------------------

pub fn pack<const W: usize>(vals: &[u64], out: &mut [u8]) {
    for (i, &v) in vals.iter().enumerate() {
        let bit = i * W;
        let mut byte = bit >> 3;
        let mut sh = (bit & 7) as u32;
        let mut left = W;
        let mut field = v & mask(W);
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
        vals.push(rng & mask(W));
    }
    let bytes = (n * W).div_ceil(8) + 16;
    let mut buf = vec![0u8; bytes];
    pack::<W>(&vals, &mut buf);
    let expect: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
    assert_eq!(sum_naive::<W>(&buf, n), expect, "naive W={W}");
    let u32_ok = sum_naive_u32::<W>(&buf, n) == expect;
    assert_eq!(
        u32_ok,
        W + 7 <= 32,
        "the hardwired 32-bit load is correct exactly when W + 7 <= 32, W={W}"
    );
    assert_eq!(unsafe { sum_period::<W>(&buf, n) }, expect, "period W={W}");
    assert_eq!(sum_period_ck::<W>(&buf, n), expect, "period_ck W={W}");
    assert_eq!(unsafe { sum_stream::<W>(&buf, n) }, expect, "stream W={W}");
    println!(
        "W={W:3} period={:2} group_bytes={:3} load_bytes={:3} ok (n={n})",
        period(W),
        group_bytes(W),
        load_bytes(W)
    );
}

fn main() {
    // n is a multiple of 8, so it is a multiple of every period this file
    // instantiates.
    check::<13>(1024);
    check::<11>(1024);
    check::<3>(1024);
    check::<12>(1024);
    check::<20>(1024);
    check::<16>(1024);
    check::<27>(1024);
    check::<57>(1024);
    println!("all shapes agree with the packer at every probed width");
    for w in 1..=64usize {
        println!(
            "w={w:2} period={:2} group_bytes={:3} load_bytes={:2}",
            period(w),
            group_bytes(w),
            load_bytes(w)
        );
    }
}
