//! Emitted-code inspection for the two `Layout::Bitpacked` readings' single-element
//! extraction, at logical field width 13 (matching file 32's own model and file 73's
//! `probe_3`). Both extraction functions are byte-for-byte the transform the bench
//! crates in `mock/benches/variants/bitpack-shared/src/lib.rs` use (`extract_aligned`,
//! `extract_zeropad`), copied here rather than linked so this probe stays a single
//! standalone file, per the panel's own probe convention (`32_probes/OUTCOMES.md`).
//!
//! `extract_aligned` / `extract_zeropad` are `#[inline(always)]`, matching the real
//! bench source. Each has a `#[unsafe(no_mangle)] #[inline(never)]` one-call wrapper
//! (`extract_aligned_standalone` / `extract_zeropad_standalone`) whose disassembly
//! shows the extraction body inlined into its own symbol (answers "what does one
//! extraction cost in isolation"), and each also feeds a sequential column-sum loop
//! (`sum_aligned` / `sum_zeropad`) that inlines the same body directly into the loop
//! (answers "what does the loop LLVM actually emits look like, autovectorised or
//! not"). Same source, two disassembly questions.

const LOGICAL_BITS: u32 = 13;
const MASK13: u32 = (1u32 << LOGICAL_BITS) - 1;

/// Byte-aligned-slot extraction: address is `i * 2` (a shift), one 2-byte
/// unaligned load, mask.
#[inline(always)]
fn extract_aligned(buf: &[u8], i: usize) -> u16 {
    let off = i * 2;
    let raw = u16::from_le_bytes([buf[off], buf[off + 1]]);
    raw & (MASK13 as u16)
}

/// Zero-inter-value-padding extraction: address is `(i * 13) >> 3` (a multiply
/// then a shift), one 4-byte unaligned load (wide enough that the worst-case
/// bit shift, 7, plus the 13-bit field never exceeds the 32 loaded bits), a
/// variable shift by `(i * 13) & 7`, mask.
#[inline(always)]
fn extract_zeropad(buf: &[u8], i: usize) -> u16 {
    let bit_off = i * LOGICAL_BITS as usize;
    let byte_off = bit_off >> 3;
    let bit_shift = (bit_off & 7) as u32;
    let w = u32::from_le_bytes([
        buf[byte_off],
        buf[byte_off + 1],
        buf[byte_off + 2],
        buf[byte_off + 3],
    ]);
    ((w >> bit_shift) & MASK13) as u16
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn extract_aligned_standalone(buf: &[u8], i: usize) -> u16 {
    extract_aligned(buf, i)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn extract_zeropad_standalone(buf: &[u8], i: usize) -> u16 {
    extract_zeropad(buf, i)
}

/// Sequential column-sum over N elements, byte-aligned reading. `N` is a
/// runtime-length slice bound (not a const generic), so the loop body is a
/// single monomorphisation whose disassembly is directly comparable to
/// `sum_zeropad` below: same shape of function, same calling convention,
/// differing only in which extraction each one calls per iteration.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn sum_aligned(buf: &[u8], n: usize) -> u64 {
    let mut s: u64 = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(extract_aligned(buf, i) as u64);
        i += 1;
    }
    s
}

/// Sequential column-sum over N elements, zero-inter-value-padding reading.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn sum_zeropad(buf: &[u8], n: usize) -> u64 {
    let mut s: u64 = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(extract_zeropad(buf, i) as u64);
        i += 1;
    }
    s
}

/// Random-access column-sum, byte-aligned reading: `idx[i]` breaks the
/// linear relation between the loop counter and the extraction address, so
/// unlike `sum_aligned` above LLVM cannot strength-reduce the address
/// computation into a running accumulator across iterations.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn sum_aligned_rand(buf: &[u8], n: usize, idx: &[u32]) -> u64 {
    let mut s: u64 = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(extract_aligned(buf, idx[i] as usize) as u64);
        i += 1;
    }
    s
}

/// Random-access column-sum, zero-inter-value-padding reading: same
/// data-dependent-index property as `sum_aligned_rand` above, applied to the
/// extraction whose address computation is a multiply rather than a shift.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn sum_zeropad_rand(buf: &[u8], n: usize, idx: &[u32]) -> u64 {
    let mut s: u64 = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(extract_zeropad(buf, idx[i] as usize) as u64);
        i += 1;
    }
    s
}

/// A native-array Dense reading: `[u16; _]`, no byte-buffer modelling at all.
/// This is what `Layout::Dense` at `StoredWidth` rounded to a native register
/// actually compiles to once the carrier is genuinely typed as `u16` rather
/// than reconstructed from two bounds-checked byte loads (`extract_aligned`
/// above models the byte-buffer case because `Layout::Bitpacked`'s own
/// extraction has no native-register form to compare against; Dense does,
/// and this probe measures it honestly rather than assuming the byte-buffer
/// model was already the cheapest expression of Dense).
#[inline(always)]
fn extract_native(buf: &[u16], i: usize) -> u16 {
    buf[i] & (MASK13 as u16)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn extract_native_standalone(buf: &[u16], i: usize) -> u16 {
    extract_native(buf, i)
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn sum_native(buf: &[u16], n: usize) -> u64 {
    let mut s: u64 = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(extract_native(buf, i) as u64);
        i += 1;
    }
    s
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn sum_native_rand(buf: &[u16], n: usize, idx: &[u32]) -> u64 {
    let mut s: u64 = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add(extract_native(buf, idx[i] as usize) as u64);
        i += 1;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cross-checks both loop forms against a slow reference decode over a
    /// pseudo-random buffer, 4096 elements, packed by hand here (not via the
    /// bench crate, to keep this probe single-file). Confirms the standalone
    /// extraction shapes above are the same transform the disassembly probes
    /// below are reading, before trusting any instruction count from them.
    #[test]
    fn sums_match_reference() {
        const N: usize = 4096;
        let mut logical = [0u16; N];
        let mut state: u64 = 0x1234_5678;
        for v in logical.iter_mut() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            *v = ((state >> 33) as u16) & MASK13 as u16;
        }
        let mut aligned = std::vec![0u8; N * 2];
        for (i, &v) in logical.iter().enumerate() {
            let b = v.to_le_bytes();
            aligned[i * 2] = b[0];
            aligned[i * 2 + 1] = b[1];
        }
        let mut zeropad = std::vec![0u8; (N * 13) / 8 + 4];
        for (i, &v) in logical.iter().enumerate() {
            let bit_off = i * 13;
            let byte_off = bit_off >> 3;
            let bit_shift = (bit_off & 7) as u32;
            let field = (v as u32) & MASK13;
            let w = u32::from_le_bytes([
                zeropad[byte_off],
                zeropad[byte_off + 1],
                zeropad[byte_off + 2],
                zeropad[byte_off + 3],
            ]) | (field << bit_shift);
            let b = w.to_le_bytes();
            zeropad[byte_off] = b[0];
            zeropad[byte_off + 1] = b[1];
            zeropad[byte_off + 2] = b[2];
            zeropad[byte_off + 3] = b[3];
        }
        let expect: u64 = logical.iter().map(|&v| v as u64).sum();
        assert_eq!(sum_aligned(&aligned, N), expect);
        assert_eq!(sum_zeropad(&zeropad, N), expect);
        for i in 0..N {
            assert_eq!(extract_aligned_standalone(&aligned, i), logical[i]);
            assert_eq!(extract_zeropad_standalone(&zeropad, i), logical[i]);
        }
        // identity permutation is a valid (if uninteresting) index array;
        // this checks the *_rand entry points thread the index through
        // correctly, not that they behave differently from the sequential
        // ones (the disassembly, not this test, is what shows the codegen
        // difference random indices make).
        let idx: std::vec::Vec<u32> = (0..N as u32).collect();
        assert_eq!(sum_aligned_rand(&aligned, N, &idx), expect);
        assert_eq!(sum_zeropad_rand(&zeropad, N, &idx), expect);
    }
}
