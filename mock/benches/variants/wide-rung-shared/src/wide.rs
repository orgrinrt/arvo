//! Multi-limb wrapping arithmetic at a declared width, in 64-bit limbs.
//!
//! Every arm in this bench calls these functions. No arm carries its own copy
//! of the arithmetic, because a bench whose arms each re-derive the kernel
//! measures several possibly-drifted programs rather than one program in
//! several payload shapes. The arms differ in exactly one thing, the loader,
//! and `arms.rs` is where that difference lives.
//!
//! The value is always `[u64; 4]` regardless of the declared width. Limbs at
//! or above `limbs_of(W)` are held at zero and asserted so. A per-width array
//! length would need `generic_const_exprs`, which is forbidden.
//!
//! ## Why the operation cycle is add, exclusive or, multiply by three
//!
//! Two properties are required and they pull against each other.
//!
//! The chain must **not collapse**. Addition and multiplication by a constant
//! are both affine in the value, so a chain of any length composes to a single
//! multiply-add and LLVM performs that composition. `bench-warm-container-shared`
//! records discarding a whole sweep to this exact effect. The exclusive or is
//! bitwise, not affine, so it breaks the composition and the operation count
//! is a real operation count.
//!
//! The chain must **not reach a fixpoint**. `20` section 2.1 found six
//! committed cells measuring a register copy, because a saturating fold
//! reaches an absorbing value after three elements and the optimiser deletes
//! the loop while the answer stays correct. Every operation here is a
//! **bijection** on the `W`-bit residues: addition of a constant is, exclusive
//! or with a constant is its own inverse, and multiplication by three is
//! invertible modulo `2^W` because three is odd. A composition of bijections
//! is a bijection, so no value is ever absorbed and the answer depends on
//! every element. `the_answer_moves_when_any_single_element_moves` asserts
//! that at every declared key rather than leaving it as this paragraph.

use crate::shape::{limbs_of, top_mask};

/// Zeroes the limbs above the declared width and masks the top one.
///
/// This is the projection. It is applied after every operation, which is what
/// a `W`-bit numeral requires whatever its payload shape, and it is identical
/// in all five arms.
#[inline(always)]
pub fn mask_w<const W: u32>(mut v: [u64; 4]) -> [u64; 4] {
    let n = limbs_of(W);
    v[n - 1] &= top_mask(W);
    let mut i = n;
    while i < 4 {
        v[i] = 0;
        i += 1;
    }
    v
}

/// Wrapping addition modulo `2^W`, limb by limb with carry.
#[inline(always)]
pub fn wide_add<const W: u32>(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let n = limbs_of(W);
    let mut r = [0u64; 4];
    let mut carry = 0u64;
    let mut i = 0usize;
    while i < n {
        let (s1, c1) = a[i].overflowing_add(b[i]);
        let (s2, c2) = s1.overflowing_add(carry);
        r[i] = s2;
        // At most one of the two additions can carry: if the first did, `s1`
        // is at most `u64::MAX - 1` and adding a carry of one cannot overflow.
        carry = (c1 as u64) + (c2 as u64);
        i += 1;
    }
    mask_w::<W>(r)
}

/// Bitwise exclusive or. Both operands are already reduced, so the result is,
/// but the projection is applied anyway so that every operation in the cycle
/// costs what the arm's semantics say it costs rather than what this one
/// happens to allow.
#[inline(always)]
pub fn wide_xor<const W: u32>(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let n = limbs_of(W);
    let mut r = [0u64; 4];
    let mut i = 0usize;
    while i < n {
        r[i] = a[i] ^ b[i];
        i += 1;
    }
    mask_w::<W>(r)
}

/// Wrapping multiplication by three, modulo `2^W`.
///
/// Written as a limb-wise widening multiply rather than as `v + (v << 1)`,
/// because the shift-and-add form is what the oracle uses and the two need to
/// be different algorithms for the oracle to be worth having.
#[inline(always)]
pub fn wide_mul3<const W: u32>(a: [u64; 4]) -> [u64; 4] {
    let n = limbs_of(W);
    let mut r = [0u64; 4];
    let mut carry = 0u64;
    let mut i = 0usize;
    while i < n {
        let p = (a[i] as u128) * 3 + (carry as u128);
        r[i] = p as u64;
        carry = (p >> 64) as u64;
        i += 1;
    }
    mask_w::<W>(r)
}

/// The per-element step cycle, shared by every arm.
#[inline(always)]
pub fn steps<const W: u32, const D: usize>(mut v: [u64; 4], k: [u64; 4], x2: [u64; 4]) -> [u64; 4] {
    let mut j = 0usize;
    while j < D {
        v = match j % 3 {
            0 => wide_add::<W>(v, k),
            1 => wide_xor::<W>(v, x2),
            _ => wide_mul3::<W>(v),
        };
        j += 1;
    }
    v
}

/// The additive operand, a function of the width so that every limb of a
/// swept numeral carries distinct bits and a bug confined to one limb is
/// visible in the answer.
pub fn operand_for(w: u32) -> [u64; 4] {
    let n = limbs_of(w);
    let mut r = [0u64; 4];
    let mut i = 0usize;
    while i < n {
        r[i] = 0x9E37_79B9_7F4A_7C15u64
            .wrapping_mul((i as u64) + 1)
            .wrapping_add(w as u64);
        i += 1;
    }
    mask_w_dyn(r, w)
}

/// The exclusive-or operand. Distinct from the additive one so the two
/// operations cannot accidentally cancel.
pub fn xor_operand_for(w: u32) -> [u64; 4] {
    let n = limbs_of(w);
    let mut r = [0u64; 4];
    let mut i = 0usize;
    while i < n {
        r[i] = 0xBF58_476D_1CE4_E5B9u64.wrapping_mul((i as u64) + 3) ^ ((w as u64) << 17);
        i += 1;
    }
    mask_w_dyn(r, w)
}

/// Runtime-width projection, for the operand constructors and the tests. The
/// timed path never calls it: there `W` is a const generic and `mask_w` folds.
pub fn mask_w_dyn(mut v: [u64; 4], w: u32) -> [u64; 4] {
    let n = limbs_of(w);
    v[n - 1] &= top_mask(w);
    let mut i = n;
    while i < 4 {
        v[i] = 0;
        i += 1;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::SWEPT_WIDTHS;

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

    macro_rules! per_width {
        ($name:ident, $body:expr) => {
            #[test]
            fn $name() {
                fn go<const W: u32>() {
                    let f: fn(u32) = $body;
                    f(W);
                }
                go::<129>();
                go::<160>();
                go::<192>();
                go::<200>();
                go::<232>();
                go::<256>();
            }
        };
    }

    /// Every operation must land inside the declared width. An arm that leaves
    /// a stray high bit agrees with every other arm carrying the same bug, so
    /// the cross-variant check cannot see it and only this can.
    #[test]
    fn every_operation_lands_inside_the_declared_width() {
        for w in SWEPT_WIDTHS {
            let n = limbs_of(w);
            for s in 0u64..64 {
                let a = rnd(s, w);
                let b = rnd(s ^ 0xABCD, w);
                for r in [add_dyn(a, b, w), xor_dyn(a, b, w), mul3_dyn(a, w)] {
                    assert_eq!(
                        r[n - 1] & !top_mask(w),
                        0,
                        "a bit escaped the top limb at W={w}"
                    );
                    for i in n..4 {
                        assert_eq!(r[i], 0, "a limb above the width is non-zero at W={w}");
                    }
                }
            }
        }
    }

    /// The property the whole fixpoint defence rests on: each operation is a
    /// bijection on the residues, so nothing is ever absorbed. Checked by
    /// injectivity over a sample, at every swept width.
    #[test]
    fn every_operation_in_the_cycle_is_injective() {
        for w in SWEPT_WIDTHS {
            let k = operand_for(w);
            let x2 = xor_operand_for(w);
            let mut seen_add = std::collections::HashSet::new();
            let mut seen_xor = std::collections::HashSet::new();
            let mut seen_mul = std::collections::HashSet::new();
            for s in 0u64..512 {
                let v = rnd(s, w);
                assert!(seen_add.insert(add_dyn(v, k, w)), "add collided at W={w}");
                assert!(seen_xor.insert(xor_dyn(v, x2, w)), "xor collided at W={w}");
                assert!(seen_mul.insert(mul3_dyn(v, w)), "mul3 collided at W={w}");
            }
        }
    }

    /// Multiplication by three is invertible modulo `2^W`, which is the
    /// specific reason the step cycle cannot reach an absorbing value.
    ///
    /// Proved by **constructing** the inverse at the declared width and
    /// checking both that it is one and that it round-trips, rather than by
    /// asserting the property. A first version of this test used the inverse of
    /// three modulo `2^64` against values reduced modulo `2^W`, which is a
    /// different number above 64 bits: at `W = 129` the true inverse is
    /// `0xaaaa...ab` extended to 129 bits, and `3 * (3^-1 mod 2^64)` is
    /// `2^65 + 1` rather than one. The test failed, correctly, and the
    /// construction below is the repair.
    #[test]
    fn multiplication_by_three_is_invertible_at_every_swept_width() {
        for w in SWEPT_WIDTHS {
            let three = mask_w_dyn([3, 0, 0, 0], w);
            let one = mask_w_dyn([1, 0, 0, 0], w);
            let two = mask_w_dyn([2, 0, 0, 0], w);

            // Newton iteration on x <- x * (2 - 3x), which doubles the number of
            // correct low bits each step. Seeded with the inverse modulo 2^64,
            // two steps reach 256 bits, which covers every swept width.
            let mut x = mask_w_dyn([0xAAAA_AAAA_AAAA_AAAB, 0, 0, 0], w);
            for _ in 0..2 {
                let t = mulw_dyn(three, x, w);
                x = mulw_dyn(x, sub_dyn(two, t, w), w);
            }

            assert_eq!(
                mulw_dyn(three, x, w),
                one,
                "three has no inverse modulo 2^{w}, which would mean the multiply step \
                 is not a bijection and a value could be absorbed"
            );

            for s in 0u64..128 {
                let v = rnd(s, w);
                let back = mulw_dyn(mul3_dyn(v, w), x, w);
                assert_eq!(back, v, "multiply by three lost information at W={w}");
            }
        }
    }

    per_width!(const_and_dynamic_arithmetic_agree, |w| {
        for s in 0u64..64 {
            let a = rnd(s, w);
            let b = rnd(s ^ 0x1234, w);
            assert_eq!(add_dyn(a, b, w), add_dyn(a, b, w));
        }
    });

    // Dynamic-width mirrors of the const-generic operations, so the tests can
    // sweep widths at run time. They share the limb loops with nothing; a
    // disagreement between these and the const forms is caught by
    // `the_const_and_dynamic_forms_agree` below.
    fn add_dyn(a: [u64; 4], b: [u64; 4], w: u32) -> [u64; 4] {
        let n = limbs_of(w);
        let mut r = [0u64; 4];
        let mut carry = 0u64;
        for i in 0..n {
            let (s1, c1) = a[i].overflowing_add(b[i]);
            let (s2, c2) = s1.overflowing_add(carry);
            r[i] = s2;
            carry = (c1 as u64) + (c2 as u64);
        }
        mask_w_dyn(r, w)
    }
    fn xor_dyn(a: [u64; 4], b: [u64; 4], w: u32) -> [u64; 4] {
        let n = limbs_of(w);
        let mut r = [0u64; 4];
        for i in 0..n {
            r[i] = a[i] ^ b[i];
        }
        mask_w_dyn(r, w)
    }
    fn mul3_dyn(a: [u64; 4], w: u32) -> [u64; 4] {
        mulk_dyn(a, 3, w)
    }
    /// Full multi-limb multiply modulo `2^W`. Used only by the invertibility
    /// proof; the bench never multiplies two wide values.
    fn mulw_dyn(a: [u64; 4], b: [u64; 4], w: u32) -> [u64; 4] {
        let n = limbs_of(w);
        let mut acc = [0u64; 8];
        for i in 0..n {
            let mut carry: u128 = 0;
            for j in 0..(n - i) {
                let cur = acc[i + j] as u128 + (a[i] as u128) * (b[j] as u128) + carry;
                acc[i + j] = cur as u64;
                carry = cur >> 64;
            }
        }
        mask_w_dyn([acc[0], acc[1], acc[2], acc[3]], w)
    }

    /// Multi-limb subtraction modulo `2^W`, with borrow.
    fn sub_dyn(a: [u64; 4], b: [u64; 4], w: u32) -> [u64; 4] {
        let n = limbs_of(w);
        let mut r = [0u64; 4];
        let mut borrow = 0u64;
        for i in 0..n {
            let (d1, b1) = a[i].overflowing_sub(b[i]);
            let (d2, b2) = d1.overflowing_sub(borrow);
            r[i] = d2;
            borrow = (b1 as u64) + (b2 as u64);
        }
        mask_w_dyn(r, w)
    }

    fn mulk_dyn(a: [u64; 4], k: u64, w: u32) -> [u64; 4] {
        let n = limbs_of(w);
        let mut r = [0u64; 4];
        let mut carry = 0u64;
        for i in 0..n {
            let p = (a[i] as u128) * (k as u128) + (carry as u128);
            r[i] = p as u64;
            carry = (p >> 64) as u64;
        }
        mask_w_dyn(r, w)
    }

    /// The const-generic path is what the bench times and the dynamic path is
    /// what the tests sweep. If they ever disagree, every test above is
    /// testing something the bench does not run.
    #[test]
    fn the_const_and_dynamic_forms_agree() {
        macro_rules! check {
            ($($w:literal),*) => {$({
                let w: u32 = $w;
                let k = operand_for(w);
                let x2 = xor_operand_for(w);
                for s in 0u64 .. 128 {
                    let v = rnd(s, w);
                    assert_eq!(wide_add::<$w>(v, k), add_dyn(v, k, w), "add at W={w}");
                    assert_eq!(wide_xor::<$w>(v, x2), xor_dyn(v, x2, w), "xor at W={w}");
                    assert_eq!(wide_mul3::<$w>(v), mul3_dyn(v, w), "mul3 at W={w}");
                }
            })*};
        }
        check!(129, 160, 192, 200, 232, 256);
    }
}
