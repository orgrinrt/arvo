//! An independent computation of the same answer, in a different radix.
//!
//! The arms work in 64-bit limbs. This works in 128-bit limbs, with its own
//! carry chain, its own projection and its own multiplication algorithm: the
//! arms multiply by three with a limb-wise widening multiply, and this one
//! does it as `v + (v << 1)`. Nothing here shares a line with `wide.rs`.
//!
//! That is what makes it worth having. A bug in the 64-bit carry chain, in the
//! top-limb mask, or in the multiplication is visible to validation rather
//! than agreed with by it, because the harness's cross-variant byte comparison
//! only establishes that the arms agree **with each other**. `20` section 2.1
//! is what that check looks like when every arm computes the same wrong thing:
//! six committed cells passed it while returning an input-independent
//! constant.
//!
//! Independence is not total and the limit is worth stating. Both this and the
//! arms are little-endian limb representations of the same integer, so a
//! misunderstanding of what the declared width **means** would be shared. What
//! it does catch is every implementation error in the limb arithmetic, which
//! is where the bugs actually are.

/// Limbs of 128 bits needed to hold `w` bits.
pub const fn limbs128_of(w: u32) -> usize {
    (w as usize).div_ceil(128)
}

/// Mask for the most significant 128-bit limb.
pub const fn top_mask128(w: u32) -> u128 {
    if w.is_multiple_of(128) {
        u128::MAX
    } else {
        (1u128 << (w % 128)) - 1
    }
}

fn mask(mut v: [u128; 2], w: u32) -> [u128; 2] {
    let n = limbs128_of(w);
    v[n - 1] &= top_mask128(w);
    let mut i = n;
    while i < 2 {
        v[i] = 0;
        i += 1;
    }
    v
}

fn add(a: [u128; 2], b: [u128; 2], w: u32) -> [u128; 2] {
    let n = limbs128_of(w);
    let mut r = [0u128; 2];
    let mut carry = 0u128;
    for i in 0..n {
        let (s1, c1) = a[i].overflowing_add(b[i]);
        let (s2, c2) = s1.overflowing_add(carry);
        r[i] = s2;
        carry = (c1 as u128) + (c2 as u128);
    }
    mask(r, w)
}

fn xor(a: [u128; 2], b: [u128; 2], w: u32) -> [u128; 2] {
    let n = limbs128_of(w);
    let mut r = [0u128; 2];
    for i in 0..n {
        r[i] = a[i] ^ b[i];
    }
    mask(r, w)
}

/// `v << 1` across the limbs, which the arms never compute.
fn shl1(a: [u128; 2], w: u32) -> [u128; 2] {
    let n = limbs128_of(w);
    let mut r = [0u128; 2];
    let mut carry = 0u128;
    for i in 0..n {
        r[i] = (a[i] << 1) | carry;
        carry = a[i] >> 127;
    }
    mask(r, w)
}

/// `v * 3` as `v + (v << 1)`, a different algorithm from the arms'.
fn mul3(a: [u128; 2], w: u32) -> [u128; 2] {
    add(shl1(a, w), a, w)
}

/// Four 64-bit limbs to two 128-bit limbs.
pub fn from_u64s(v: [u64; 4]) -> [u128; 2] {
    [
        (v[0] as u128) | ((v[1] as u128) << 64),
        (v[2] as u128) | ((v[3] as u128) << 64),
    ]
}

/// Two 128-bit limbs back to four 64-bit limbs.
pub fn to_u64s(v: [u128; 2]) -> [u64; 4] {
    [
        v[0] as u64,
        (v[0] >> 64) as u64,
        v[1] as u64,
        (v[1] >> 64) as u64,
    ]
}

/// The reference answer for a whole column.
pub fn reference(values: &[[u64; 4]], w: u32, d: usize) -> [u64; 4] {
    let k = from_u64s(crate::wide::operand_for(w));
    let x2 = from_u64s(crate::wide::xor_operand_for(w));
    let mut acc = [0u128; 2];
    for raw in values {
        let mut v = from_u64s(*raw);
        for j in 0..d {
            v = match j % 3 {
                0 => add(v, k, w),
                1 => xor(v, x2, w),
                _ => mul3(v, w),
            };
        }
        acc = add(acc, v, w);
    }
    to_u64s(acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::SWEPT_WIDTHS;
    use crate::wide::{mask_w_dyn, operand_for, steps, xor_operand_for};

    fn rnd(seed: u64, w: u32) -> [u64; 4] {
        let mut s = seed;
        let mut nxt = || {
            s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = s;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        mask_w_dyn([nxt(), nxt(), nxt(), nxt()], w)
    }

    /// The two radices agree on the step cycle, at every swept width and every
    /// operation count the manifest declares. This is the check that makes the
    /// oracle worth calling an oracle.
    ///
    /// The width and the operation count are both const generics on `check`,
    /// so this sweeps the exact monomorphisations the bench times rather than a
    /// dynamic-width stand-in for them.
    #[test]
    fn the_two_radices_agree_on_the_step_cycle() {
        fn check<const W: u32, const D: usize>() {
            let w = W;
            let k64 = operand_for(w);
            let x64 = xor_operand_for(w);
            let k = from_u64s(k64);
            let x2 = from_u64s(x64);
            for s in 0u64..128 {
                let v = rnd(s, w);
                let arm = steps::<W, D>(v, k64, x64);
                let mut o = from_u64s(v);
                for j in 0..D {
                    o = match j % 3 {
                        0 => add(o, k, w),
                        1 => xor(o, x2, w),
                        _ => mul3(o, w),
                    };
                }
                assert_eq!(
                    arm,
                    to_u64s(o),
                    "the 64-bit arm and the 128-bit oracle disagree at W={w} D={D} seed={s}"
                );
            }
        }
        macro_rules! all_d {
            ($w:literal) => {
                check::<$w, 1>();
                check::<$w, 2>();
                check::<$w, 3>();
                check::<$w, 4>();
                check::<$w, 8>();
            };
        }
        all_d!(129);
        all_d!(160);
        all_d!(192);
        all_d!(200);
        all_d!(232);
        all_d!(256);
    }

    /// The oracle's own projection is the projection. Checked directly rather
    /// than through the agreement above, so a shared misunderstanding of the
    /// width cannot hide inside a mutual check.
    #[test]
    fn the_oracle_projection_keeps_exactly_the_declared_bits() {
        for w in SWEPT_WIDTHS {
            let all_ones = [u128::MAX, u128::MAX];
            let m = mask(all_ones, w);
            let bits: u32 = m.iter().map(|l| l.count_ones()).sum();
            assert_eq!(
                bits, w,
                "the oracle keeps {bits} bits at a declared width of {w}"
            );
        }
    }

    /// Shifting left by one is multiplication by two, checked against the
    /// arms' own multiply-by-three minus one copy. Cheap, and it pins the
    /// carry across the 128-bit limb boundary, which is the one place this
    /// radix has a seam and the other does not.
    #[test]
    fn the_oracle_carries_across_its_limb_boundary() {
        for w in SWEPT_WIDTHS {
            if limbs128_of(w) < 2 {
                continue;
            }
            // A value with the top bit of limb zero set must carry into limb one.
            let v = [1u128 << 127, 0u128];
            let s = shl1(v, w);
            assert_eq!(s[0], 0, "at W={w} the low limb did not clear");
            assert_eq!(
                s[1] & 1,
                1,
                "at W={w} the carry did not reach the high limb"
            );
        }
    }
}
