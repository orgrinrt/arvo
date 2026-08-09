//! Third emitted-code probe for file 81. `decoders2.rs` put the period and the
//! per-lane shifts where the design would put them (associated consts), and the
//! group loop then unrolled against a literal trip count with every shift a
//! literal `ubfx` operand. What it did not fix is the loads: LLVM narrowed each
//! of the eight 8-byte reads back into individual byte reads recombined with
//! `orr`, because it could prove only two or three bytes per field are live.
//! Thirteen byte loads and eleven `orr`s per group is not the cost of the
//! packing, it is the cost of asking for eight overlapping windows the compiler
//! then has to reconcile.
//!
//! So this file states the windows explicitly. A group of `P` fields spans
//! `G` bytes; a 64-bit window placed at a byte offset covers every field whose
//! bit offset within that window leaves `W` bits inside it. Greedily assigning
//! lanes to windows gives, for W = 13, two windows per eight fields instead of
//! eight. The window count, the window offsets, and the per-lane (window,
//! shift) pairs are all functions of W alone and all computed in const
//! position.
//!
//! Everything here is a probe, not arvo source.

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

/// Greedy window assignment. Returns the window byte offsets, padded to 8.
const fn windows_of(w: usize) -> [u8; 8] {
    let mut out = [0u8; 8];
    let mut nw = 0usize;
    let mut cur = 0usize; // current window's byte offset
    let mut j = 0usize;
    while j < period_of(w) {
        let bit = j * w;
        if j == 0 {
            cur = 0;
            out[0] = 0;
            nw = 1;
        } else if bit - cur * 8 + w > 64 {
            cur = bit >> 3;
            out[nw] = cur as u8;
            nw += 1;
        }
        j += 1;
    }
    out
}

const fn window_count_of(w: usize) -> usize {
    let mut nw = 0usize;
    let mut cur = 0usize;
    let mut j = 0usize;
    while j < period_of(w) {
        let bit = j * w;
        if j == 0 {
            cur = 0;
            nw = 1;
        } else if bit - cur * 8 + w > 64 {
            cur = bit >> 3;
            nw += 1;
        }
        j += 1;
    }
    nw
}

/// Per-lane (window index, shift within that window).
const fn lanes_of(w: usize) -> [(u8, u8); 8] {
    let mut out = [(0u8, 0u8); 8];
    let mut cur = 0usize;
    let mut wi = 0usize;
    let mut j = 0usize;
    while j < period_of(w) {
        let bit = j * w;
        if j == 0 {
            cur = 0;
            wi = 0;
        } else if bit - cur * 8 + w > 64 {
            cur = bit >> 3;
            wi += 1;
        }
        out[j] = (wi as u8, (bit - cur * 8) as u8);
        j += 1;
    }
    out
}

pub trait Packing {
    const W: usize;
    const P: usize;
    const G: usize;
    const MASK: u64;
    const NWIN: usize;
    const WINDOWS: [u8; 8];
    const LANES: [(u8, u8); 8];
    /// Read headroom past the last group, in bytes: the last window starts at
    /// most `G - 1` bytes into the group and reads 8.
    const HEADROOM: usize;
}

pub struct Pack<const W: usize>;

impl<const W: usize> Packing for Pack<W> {
    const W: usize = W;
    const P: usize = period_of(W);
    const G: usize = group_bytes_of(W);
    const MASK: u64 = mask_of(W);
    const NWIN: usize = window_count_of(W);
    const WINDOWS: [u8; 8] = windows_of(W);
    const LANES: [(u8, u8); 8] = lanes_of(W);
    const HEADROOM: usize = 8;
}

#[inline(always)]
unsafe fn ldu64(buf: &[u8], at: usize) -> u64 {
    unsafe { u64::from_le(core::ptr::read_unaligned(buf.as_ptr().add(at) as *const u64)) }
}

/// # Safety
/// `buf` holds `n * W` bits plus `K::HEADROOM` bytes; `n` is a multiple of
/// `K::P`.
#[inline(always)]
pub unsafe fn sum_windowed<K: Packing>(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    let mut base = 0usize;
    let groups = n / K::P;
    for _ in 0..groups {
        let mut win = [0u64; 8];
        let mut w = 0usize;
        while w < K::NWIN {
            win[w] = unsafe { ldu64(buf, base + K::WINDOWS[w] as usize) };
            w += 1;
        }
        let mut j = 0usize;
        while j < K::P {
            let (wi, sh) = K::LANES[j];
            s = s.wrapping_add((win[wi as usize] >> sh) & K::MASK);
            j += 1;
        }
        base += K::G;
    }
    s
}

macro_rules! probe {
    ($w:literal, $f:ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $f(buf: *const u8, len: usize, n: usize) -> u64 {
            let b = unsafe { core::slice::from_raw_parts(buf, len) };
            unsafe { sum_windowed::<Pack<$w>>(b, n) }
        }
    };
}

probe!(13, win_w13);
probe!(11, win_w11);
probe!(3, win_w3);
probe!(12, win_w12);
probe!(20, win_w20);
probe!(16, win_w16);
probe!(27, win_w27);
probe!(57, win_w57);

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
        unsafe { sum_windowed::<Pack<W>>(&buf, n) },
        expect,
        "windowed W={W}"
    );
    println!(
        "W={W:3} P={:2} G={:3} windows={:?} lanes={:?}",
        <Pack<W> as Packing>::P,
        <Pack<W> as Packing>::G,
        &<Pack<W> as Packing>::WINDOWS[..<Pack<W> as Packing>::NWIN],
        &<Pack<W> as Packing>::LANES[..<Pack<W> as Packing>::P],
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
    println!("windowed decode agrees with the packer at every probed width");
}
