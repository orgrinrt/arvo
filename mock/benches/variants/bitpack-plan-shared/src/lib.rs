//! Shared data model for the second `Layout::Bitpacked` bench, the one file 81
//! runs to establish whether the multiple file 75 measured is inherent to the
//! packing or a property of the loop that read it.
//!
//! Two things differ from `bench-bitpack-shared`, and both are the point.
//!
//! First, a third decoder. `extract_naive` is file 75's shape, kept verbatim in
//! structure: the byte offset and the bit shift are both derived from the
//! running index at runtime. `sum_windowed` is the same buffer read through a
//! plan the type carries: `P = 8 / gcd(W, 8)` fields occupy exactly
//! `G = W * P / 8` whole bytes, so the byte offsets and bit shifts inside a
//! group depend on nothing but W, and W is known at monomorphisation. The plan
//! lives in associated consts because a `const fn` called in value position is
//! not const-evaluated (`81_probes/OUTCOMES.md`, finding C1: the recursive gcd
//! emitted a runtime `udiv` loop when written that way).
//!
//! Second, the sizes. File 75's sweep ran 256, 4096 and 16384 elements, whose
//! largest dense footprint is 32 KB. This host's performance cores have a
//! 131072-byte L1 data cache (`sysctl hw.perflevel0.l1dcachesize`), so all
//! three of those sizes are L1-resident and neither layout ever pays for its
//! own footprint. The sizes here bracket that boundary: 65536 elements is
//! 128 KB dense (at L1) against 106.5 KB packed (inside it), and 262144 is
//! well past it for both.
//!
//! Bench infrastructure, not shipping arvo source: `std` used freely, matching
//! every sibling variant crate here.

use mockspace_bench_core::Routine;

/// Logical field width, 13 bits, the same non-power-of-two shape file 75 and
/// file 32 both used.
pub const LOGICAL_BITS: usize = 13;
pub const MASK13: u64 = (1u64 << LOGICAL_BITS) - 1;

/// Largest column this bench sweeps.
pub const MAX_N: usize = 262144;
/// Packed bytes at `MAX_N`, plus 16 bytes of read headroom for the widest
/// window a group's last lane can open. The headroom is a function of the
/// width alone, which is why it is a declaration-time constant rather than a
/// runtime bound check.
pub const MAX_PACKED_BYTES: usize = (MAX_N * LOGICAL_BITS) / 8 + 16;

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
const fn windows_of(w: usize) -> [u8; 8] {
    let mut out = [0u8; 8];
    let mut nw = 0usize;
    let mut cur = 0usize;
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

/// Everything a bitpacked column of a given logical width knows about itself
/// before any value exists.
pub trait Packing {
    const W: usize;
    const P: usize;
    const G: usize;
    const MASK: u64;
    const NWIN: usize;
    const WINDOWS: [u8; 8];
    const LANES: [(u8, u8); 8];
    /// Refuses at monomorphisation when a 64-bit window cannot hold a field of
    /// this width. A field starts at one of eight bit offsets, so the window
    /// must be at least `W + 7` bits; the general rule picks the window width
    /// from `W` (u64 to 57 bits, u128 to 121), and this bench only builds the
    /// u64 case, so it refuses rather than silently mis-addressing a lane.
    const WINDOW_FITS: ();
}

pub struct Pack<const W: usize>;

impl<const W: usize> Packing for Pack<W> {
    const W: usize = W;
    const P: usize = period_of(W);
    const G: usize = group_bytes_of(W);
    const MASK: u64 = if W >= 64 { u64::MAX } else { (1u64 << W) - 1 };
    const NWIN: usize = window_count_of(W);
    const WINDOWS: [u8; 8] = windows_of(W);
    const LANES: [(u8, u8); 8] = lanes_of(W);
    const WINDOW_FITS: () = assert!(
        W + 7 <= 64,
        "a 64-bit window cannot hold a field wider than 57 bits: a field may \
         start at any of eight bit offsets within a byte"
    );
}

/// Packs the logical values with no inter-value padding at all.
pub fn pack(vals: &[u16], out: &mut [u8]) {
    for (i, &v) in vals.iter().enumerate() {
        let bit = i * LOGICAL_BITS;
        let mut byte = bit >> 3;
        let mut sh = (bit & 7) as u32;
        let mut left = LOGICAL_BITS;
        let mut field = (v as u64) & MASK13;
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

/// The dense reading: the carrier is a native `[u16]`, one value per slot.
#[inline(always)]
pub fn sum_native(vals: &[u16], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add((vals[i] & (MASK13 as u16)) as u64);
    }
    s
}

/// File 75's shape: offset and shift derived from the running index.
#[inline(always)]
pub fn sum_naive(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        let bit_off = i * LOGICAL_BITS;
        let byte_off = bit_off >> 3;
        let bit_shift = (bit_off & 7) as u32;
        let w = u32::from_le_bytes([
            buf[byte_off],
            buf[byte_off + 1],
            buf[byte_off + 2],
            buf[byte_off + 3],
        ]);
        s = s.wrapping_add(((w >> bit_shift) as u64) & MASK13);
    }
    s
}

#[inline(always)]
unsafe fn ldu64(buf: &[u8], at: usize) -> u64 {
    unsafe { u64::from_le(core::ptr::read_unaligned(buf.as_ptr().add(at) as *const u64)) }
}

/// The same buffer read through the width's own plan.
///
/// # Safety
/// `buf` holds `n * W` bits plus 8 bytes of headroom and `n` is a multiple of
/// `K::P`. Both are declaration-time facts about the column.
#[inline(always)]
pub unsafe fn sum_windowed<K: Packing>(buf: &[u8], n: usize) -> u64 {
    // forces the width refusal at monomorphisation rather than at a read
    let () = K::WINDOW_FITS;
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

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Sum {
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PlanColumn<const N: usize> {
    pub logical: [u16; MAX_N],
    pub packed: [u8; MAX_PACKED_BYTES],
}

impl<const N: usize> Default for PlanColumn<N> {
    fn default() -> Self {
        PlanColumn {
            logical: [0u16; MAX_N],
            packed: [0u8; MAX_PACKED_BYTES],
        }
    }
}

impl<const N: usize> Routine for PlanColumn<N> {
    type Input = PlanColumn<N>;
    type Output = Sum;

    fn build_input(seed: u64) -> Self::Input {
        let mut col = PlanColumn::<N>::default();
        let mut rng = SplitMix64(seed ^ 0xB179_ACC0_0001_5EED);
        for i in 0..N {
            col.logical[i] = (rng.next() & MASK13) as u16;
        }
        let bytes = (N * LOGICAL_BITS) / 8 + 16;
        pack(&col.logical[..N], &mut col.packed[..bytes]);
        col
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let mut expect: u64 = 0;
        for &v in input.logical[..N].iter() {
            expect = expect.wrapping_add(v as u64);
        }
        if output.value != expect {
            return Err("column sum mismatch: this decoder produced a different \
                 value stream than the logical ground truth");
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }
}

// ---------------------------------------------------------------------------
// the same two decoders feeding a heavier per-element kernel
// ---------------------------------------------------------------------------

/// A stand-in for per-element consumer work: one 32-bit multiply, a shift and
/// an xor. Chosen so both decoders can vectorise it identically (aarch64 has a
/// 32-bit lane multiply and no 64-bit one), so the only thing that differs
/// between the variants below is how the value reached the kernel.
#[inline(always)]
pub fn kernel(v: u16) -> u64 {
    let t = (v as u32).wrapping_mul(2_654_435_761);
    ((t >> 11) ^ t) as u64 & 0xffff
}

#[inline(always)]
pub fn mac_native(vals: &[u16], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add(kernel(vals[i] & (MASK13 as u16)));
    }
    s
}

#[inline(always)]
pub fn mac_naive(buf: &[u8], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        let bit_off = i * LOGICAL_BITS;
        let byte_off = bit_off >> 3;
        let bit_shift = (bit_off & 7) as u32;
        let w = u32::from_le_bytes([
            buf[byte_off],
            buf[byte_off + 1],
            buf[byte_off + 2],
            buf[byte_off + 3],
        ]);
        s = s.wrapping_add(kernel((((w >> bit_shift) as u64) & MASK13) as u16));
    }
    s
}

/// # Safety
/// As `sum_windowed`.
#[inline(always)]
pub unsafe fn mac_windowed<K: Packing>(buf: &[u8], n: usize) -> u64 {
    let () = K::WINDOW_FITS;
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
            s = s.wrapping_add(kernel(((win[wi as usize] >> sh) & K::MASK) as u16));
            j += 1;
        }
        base += K::G;
    }
    s
}

/// The same column, scored by the heavier kernel rather than by a plain sum.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MacColumn<const N: usize>;

impl<const N: usize> Routine for MacColumn<N> {
    type Input = PlanColumn<N>;
    type Output = Sum;

    fn build_input(seed: u64) -> Self::Input {
        <PlanColumn<N> as Routine>::build_input(seed)
    }

    fn validate_output(input: &Self::Input, output: &Self::Output) -> Result<(), &'static str> {
        let mut expect: u64 = 0;
        for &v in input.logical[..N].iter() {
            expect = expect.wrapping_add(kernel(v));
        }
        if output.value != expect {
            return Err("kernel accumulation mismatch: this decoder produced a \
                 different value stream than the logical ground truth");
        }
        Ok(())
    }

    fn ops_per_call(_input: &Self::Input) -> u64 {
        N as u64
    }
}

/// The same plan-driven decode, but materialising each group into a
/// natural-width lane array before the kernel runs over it.
///
/// The reason this shape exists: `mac_windowed` hands the kernel values living
/// in 64-bit lanes, because that is the width of the window they were shifted
/// out of. The dense path hands the kernel 16-bit lanes. On a 128-bit vector
/// that is two lanes against eight, so any subsequent per-element work runs at
/// a quarter of the dense path's vector throughput, and the decode's advantage
/// is spent before the consumer's first instruction. Narrowing at the end of
/// the group is what puts both paths on the same lane width.
///
/// # Safety
/// As `sum_windowed`.
#[inline(always)]
pub unsafe fn mac_windowed_narrow<K: Packing>(buf: &[u8], n: usize) -> u64 {
    let () = K::WINDOW_FITS;
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
        let mut lane = [0u16; 8];
        let mut j = 0usize;
        while j < K::P {
            let (wi, sh) = K::LANES[j];
            lane[j] = ((win[wi as usize] >> sh) & K::MASK) as u16;
            j += 1;
        }
        let mut j = 0usize;
        while j < K::P {
            s = s.wrapping_add(kernel(lane[j]));
            j += 1;
        }
        base += K::G;
    }
    s
}

// ---------------------------------------------------------------------------
// the vector unpack: fields gathered straight into natural lanes
// ---------------------------------------------------------------------------

/// Byte-gather indices for one four-lane output vector starting at group lane
/// `l0`. Lane `j` of that vector takes the four bytes beginning at
/// `((l0 + j) * W) / 8`, little-endian, which is where its field starts.
/// Correct while a field plus its worst-case bit offset fits in four bytes,
/// that is `W <= 25`.
pub const fn tbl_indices(w: usize, l0: usize) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut j = 0usize;
    while j < 4 {
        let start = ((l0 + j) * w) / 8;
        let mut k = 0usize;
        while k < 4 {
            out[j * 4 + k] = (start + k) as u8;
            k += 1;
        }
        j += 1;
    }
    out
}

/// Per-lane right shifts for the same vector, as the negative left shifts
/// `USHL` wants.
pub const fn tbl_shifts(w: usize, l0: usize) -> [i32; 4] {
    let mut out = [0i32; 4];
    let mut j = 0usize;
    while j < 4 {
        out[j] = -(((l0 + j) * w % 8) as i32);
        j += 1;
    }
    out
}

/// The widest field this vector shape admits: a field may start at any of
/// eight bit offsets and the gather window is four bytes.
pub const fn simd_width_fits(w: usize) -> bool {
    w + 7 <= 32
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use core::arch::aarch64::*;

    use super::{tbl_indices, tbl_shifts, Packing};

    /// Decodes one group of eight fields into eight 16-bit lanes.
    ///
    /// # Safety
    /// `at` addresses a group start and the buffer holds sixteen readable
    /// bytes from there. Both are declaration-time facts: the group stride and
    /// the trailing headroom are functions of the width.
    #[inline(always)]
    pub unsafe fn decode_group<K: Packing>(buf: &[u8], at: usize) -> uint16x8_t {
        unsafe {
            let src = vld1q_u8(buf.as_ptr().add(at));
            let i0 = vld1q_u8(tbl_indices(K::W, 0).as_ptr());
            let i1 = vld1q_u8(tbl_indices(K::W, 4).as_ptr());
            let s0 = vld1q_s32(tbl_shifts(K::W, 0).as_ptr());
            let s1 = vld1q_s32(tbl_shifts(K::W, 4).as_ptr());
            let m = vdupq_n_u32(K::MASK as u32);
            let g0 = vreinterpretq_u32_u8(vqtbl1q_u8(src, i0));
            let g1 = vreinterpretq_u32_u8(vqtbl1q_u8(src, i1));
            let v0 = vandq_u32(vshlq_u32(g0, s0), m);
            let v1 = vandq_u32(vshlq_u32(g1, s1), m);
            vcombine_u16(vmovn_u32(v0), vmovn_u32(v1))
        }
    }
}

/// Vector sum over the packed column.
///
/// # Safety
/// As `sum_windowed`, plus sixteen rather than eight bytes of headroom.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn sum_simd<K: Packing>(buf: &[u8], n: usize) -> u64 {
    use core::arch::aarch64::*;
    assert!(K::P == 8 && simd_width_fits(K::W));
    unsafe {
        let mut acc = vdupq_n_u64(0);
        let mut base = 0usize;
        for _ in 0..(n / 8) {
            let lanes = neon::decode_group::<K>(buf, base);
            let lo = vmovl_u16(vget_low_u16(lanes));
            let hi = vmovl_high_u16(lanes);
            acc = vaddq_u64(acc, vaddl_u32(vget_low_u32(lo), vget_high_u32(lo)));
            acc = vaddq_u64(acc, vaddl_u32(vget_low_u32(hi), vget_high_u32(hi)));
            base += K::G;
        }
        vgetq_lane_u64(acc, 0).wrapping_add(vgetq_lane_u64(acc, 1))
    }
}

/// Vector decode feeding the heavier per-element kernel.
///
/// # Safety
/// As `sum_simd`.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn mac_simd<K: Packing>(buf: &[u8], n: usize) -> u64 {
    use core::arch::aarch64::*;
    assert!(K::P == 8 && simd_width_fits(K::W));
    unsafe {
        let mut s = 0u64;
        let mut base = 0usize;
        let mut lane = [0u16; 8];
        for _ in 0..(n / 8) {
            vst1q_u16(lane.as_mut_ptr(), neon::decode_group::<K>(buf, base));
            let mut j = 0usize;
            while j < 8 {
                s = s.wrapping_add(kernel(lane[j]));
                j += 1;
            }
            base += K::G;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `PlanColumn<262144>` is roughly 950 KB and `build_input` returns it by
    /// value, so an unoptimised build transits several copies of it through the
    /// frame and overflows the 2 MB test-thread stack. The release build elides
    /// those copies and does not, which is exactly the kind of difference that
    /// makes a suite pass under one profile and abort under another. Running
    /// the check on a thread with a stated stack size makes both profiles agree
    /// rather than making the assertion smaller.
    fn check<const N: usize>() {
        std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(check_inner::<N>)
            .expect("spawning the wide-stack checker thread")
            .join()
            .expect("the wide-stack checker thread panicked");
    }

    fn check_inner<const N: usize>() {
        for seed in 0u64..4 {
            let col = <PlanColumn<N> as Routine>::build_input(seed);
            let expect = col.logical[..N]
                .iter()
                .fold(0u64, |a, &b| a.wrapping_add(b as u64));
            assert_eq!(sum_native(&col.logical, N), expect, "native N={N}");
            assert_eq!(sum_naive(&col.packed, N), expect, "naive N={N}");
            assert_eq!(
                unsafe { sum_windowed::<Pack<13>>(&col.packed, N) },
                expect,
                "windowed N={N}"
            );
            let kexpect = col.logical[..N]
                .iter()
                .fold(0u64, |a, &b| a.wrapping_add(kernel(b)));
            assert_eq!(mac_native(&col.logical, N), kexpect, "mac native N={N}");
            assert_eq!(mac_naive(&col.packed, N), kexpect, "mac naive N={N}");
            assert_eq!(
                unsafe { mac_windowed::<Pack<13>>(&col.packed, N) },
                kexpect,
                "mac windowed N={N}"
            );
            assert_eq!(
                unsafe { mac_windowed_narrow::<Pack<13>>(&col.packed, N) },
                kexpect,
                "mac windowed narrow N={N}"
            );
            #[cfg(target_arch = "aarch64")]
            {
                assert_eq!(
                    unsafe { sum_simd::<Pack<13>>(&col.packed, N) },
                    expect,
                    "simd sum N={N}"
                );
                assert_eq!(
                    unsafe { mac_simd::<Pack<13>>(&col.packed, N) },
                    kexpect,
                    "simd mac N={N}"
                );
            }
        }
    }

    /// The plan is what it claims to be at width 13: eight fields per thirteen
    /// bytes, two windows, and every lane's shift distinct.
    #[test]
    fn plan_at_width_13_is_the_period_and_two_windows() {
        assert_eq!(<Pack<13> as Packing>::P, 8);
        assert_eq!(<Pack<13> as Packing>::G, 13);
        assert_eq!(<Pack<13> as Packing>::NWIN, 2);
        assert_eq!(&<Pack<13> as Packing>::WINDOWS[..2], &[0u8, 6u8]);
        let lanes = <Pack<13> as Packing>::LANES;
        assert_eq!(
            &lanes[..8],
            &[
                (0, 0),
                (0, 13),
                (0, 26),
                (0, 39),
                (1, 4),
                (1, 17),
                (1, 30),
                (1, 43)
            ]
        );
        // every lane's window read stays inside 64 bits
        for &(_, sh) in lanes[..8].iter() {
            assert!(
                sh as usize + 13 <= 64,
                "lane shift {sh} overflows the window"
            );
        }
    }

    /// The period is `8 / gcd(W, 8)` at every width the design admits, not
    /// only at the one this bench measures. A law asserted at one width is
    /// not a law.
    #[test]
    fn period_and_group_hold_at_every_width_to_64() {
        macro_rules! at {
            ($($w:literal),*) => { $(
                {
                    const W: usize = $w;
                    assert_eq!(<Pack<W> as Packing>::P, 8 / gcd(W, 8), "period W={W}");
                    assert_eq!(
                        <Pack<W> as Packing>::G * 8,
                        W * <Pack<W> as Packing>::P,
                        "group bytes W={W}"
                    );
                    assert!(<Pack<W> as Packing>::NWIN <= <Pack<W> as Packing>::P);
                    let lanes = <Pack<W> as Packing>::LANES;
                    for j in 0..<Pack<W> as Packing>::P {
                        let (wi, sh) = lanes[j];
                        assert!((wi as usize) < <Pack<W> as Packing>::NWIN, "W={W} lane {j}");
                        assert!(sh as usize + W <= 64, "W={W} lane {j} window overflow");
                        // the lane addresses the bit the packer wrote it to
                        let abs = <Pack<W> as Packing>::WINDOWS[wi as usize] as usize * 8
                            + sh as usize;
                        assert_eq!(abs, j * W, "W={W} lane {j} addresses the wrong bit");
                    }
                }
            )* };
        }
        at!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        at!(17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
        at!(33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48);
        at!(49, 50, 51, 52, 53, 54, 55, 56, 57);
    }

    #[test]
    fn column16384_decoders_agree() {
        check::<16384>();
    }

    #[test]
    fn column65536_decoders_agree() {
        check::<65536>();
    }

    #[test]
    fn column262144_decoders_agree() {
        check::<262144>();
    }
}
