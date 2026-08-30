//! p4b: the difference verdict as the thing an arm actually gates on, at const
//! time, at the gated width itself.
//!
//! This is 80's p2c construction with the load-bearing substitution: the gate
//! no longer evaluates a hand-written closed form whose connection to the law
//! is asserted. It evaluates THE LAW, at width 64, through the same map the
//! sweep would use, via the finite-difference criterion p4 validated: an
//! equation law of degree d over the wrapping fragment holds at width W
//! exactly when all d+1 forward differences at 0 vanish mod 2^W. Cost at
//! width 64: d+1 map evaluations. There is no model band, no transfer, and no
//! declared verdict anywhere in the construction.
//!
//! The band survives in one place, demoted to the job it can do: a rung-0
//! crate-level const asserts that the difference verdict agrees with the
//! exhaustive sweep at widths 2..=8, for the laws whose sweeps are affordable.
//! That validates the IMPLEMENTATION of the criterion, not the transfer;
//! at width 64 the criterion carries its own weight.
//!
//!   rustc --edition 2021 -O p4b_certificate_gate.rs -o p4b       (builds)
//!   rustc --edition 2021 -O --cfg use_l16 p4b_... .rs            (refused: L_16 false at 64)
//!   rustc --edition 2021 -O --cfg use_l64 p4b_... .rs            (refused: L_64 false at 64)
//!
//! The default build licenses the two laws that are true at width 64: L_128
//! (threshold 127) and arity-3 wrapping associativity (an integer identity),
//! the latter through the trivariate mixed-difference form of the criterion.
//!
//! Toolchain: pinned nightly-2026-05-28. No feature gates.

const fn mask_of(w: u32) -> u64 {
    if w >= 64 {
        u64::MAX
    } else {
        (1u64 << w) - 1
    }
}

/// The law family through the map: (x)(x-1)...(x-k+1) wrapping at width w.
const fn falling_prod(x: u64, k: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let mut acc: u64 = 1;
    let mut i: u64 = 0;
    while i < k {
        acc = acc.wrapping_mul(x.wrapping_sub(i) & m) & m;
        i += 1;
    }
    acc
}

/// Univariate difference verdict for L_k at width w: k+1 evaluations of the
/// map at x = 0..=k, forward-difference triangle mod 2^w, all entries zero.
/// Constant in w. MAXK bounds the scratch array; k <= MAXK.
const MAXK: usize = 128;
const fn l_k_diff_verdict(k: u64, w: u32) -> bool {
    let m = mask_of(w);
    let d = k as usize; // degree of the product is k
    let mut vals = [0u64; MAXK + 1];
    let mut i = 0usize;
    while i <= d {
        vals[i] = falling_prod(i as u64, k, w);
        i += 1;
    }
    let mut j = 1usize;
    while j <= d {
        let mut t = d;
        while t >= j {
            vals[t] = vals[t].wrapping_sub(vals[t - 1]) & m;
            t -= 1;
        }
        j += 1;
    }
    let mut s = 0usize;
    while s <= d {
        if vals[s] != 0 {
            return false;
        }
        s += 1;
    }
    true
}

/// Trivariate mixed-difference verdict for arity-3 wrapping-mul associativity
/// at width w: 27 evaluations over the box {0,1,2}^3, difference transform
/// along each axis, all entries zero. Constant in w.
const fn assoc_mul_residual(a: u64, b: u64, c: u64, w: u32) -> u64 {
    let m = mask_of(w);
    let l = (a.wrapping_mul(b) & m).wrapping_mul(c) & m;
    let r = a.wrapping_mul(b.wrapping_mul(c) & m) & m;
    l.wrapping_sub(r) & m
}
const fn assoc_mul_diff_verdict(w: u32) -> bool {
    let m = mask_of(w);
    let mut t = [[[0u64; 3]; 3]; 3];
    let mut a = 0usize;
    while a < 3 {
        let mut b = 0usize;
        while b < 3 {
            let mut c = 0usize;
            while c < 3 {
                t[a][b][c] = assoc_mul_residual(a as u64, b as u64, c as u64, w);
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    // difference transform along each axis
    let mut j = 1usize;
    while j < 3 {
        let mut i = 2usize;
        while i >= j {
            let mut b = 0usize;
            while b < 3 {
                let mut c = 0usize;
                while c < 3 {
                    t[i][b][c] = t[i][b][c].wrapping_sub(t[i - 1][b][c]) & m;
                    c += 1;
                }
                b += 1;
            }
            i -= 1;
        }
        j += 1;
    }
    let mut j = 1usize;
    while j < 3 {
        let mut i = 2usize;
        while i >= j {
            let mut a = 0usize;
            while a < 3 {
                let mut c = 0usize;
                while c < 3 {
                    t[a][i][c] = t[a][i][c].wrapping_sub(t[a][i - 1][c]) & m;
                    c += 1;
                }
                a += 1;
            }
            i -= 1;
        }
        j += 1;
    }
    let mut j = 1usize;
    while j < 3 {
        let mut i = 2usize;
        while i >= j {
            let mut a = 0usize;
            while a < 3 {
                let mut b = 0usize;
                while b < 3 {
                    t[a][b][i] = t[a][b][i].wrapping_sub(t[a][b][i - 1]) & m;
                    b += 1;
                }
                a += 1;
            }
            i -= 1;
        }
        j += 1;
    }
    let mut a = 0usize;
    while a < 3 {
        let mut b = 0usize;
        while b < 3 {
            let mut c = 0usize;
            while c < 3 {
                if t[a][b][c] != 0 {
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

/// Exhaustive sweep for L_k, affordable widths only. Used by the rung-0
/// implementation check, never by the gate.
const fn l_k_swept_verdict(k: u64, w: u32) -> bool {
    let n = 1u64 << w;
    let mut x = 0u64;
    while x < n {
        if falling_prod(x, k, w) != 0 {
            return false;
        }
        x += 1;
    }
    true
}

/// Rung 0: the criterion's implementation agrees with the sweep where the
/// sweep exists. This validates the checker, not the transfer.
const IMPLEMENTATION_CHECK: () = {
    let mut w = 2u32;
    while w <= 8 {
        assert!(
            l_k_diff_verdict(4, w) == l_k_swept_verdict(4, w),
            "difference criterion disagrees with the sweep for L_4"
        );
        assert!(
            l_k_diff_verdict(6, w) == l_k_swept_verdict(6, w),
            "difference criterion disagrees with the sweep for L_6"
        );
        assert!(
            l_k_diff_verdict(16, w) == l_k_swept_verdict(16, w),
            "difference criterion disagrees with the sweep for L_16"
        );
        w += 1;
    }
};

/// The gate: an arm is licensed at width 64 exactly when the law it rewrites
/// through HOLDS at width 64, decided by the criterion, at the gated width.
struct LawL<const K: u64>;
trait Licensed {
    const CHECK: ();
}
impl<const K: u64> Licensed for LawL<K> {
    const CHECK: () = {
        let () = IMPLEMENTATION_CHECK;
        assert!(
            l_k_diff_verdict(K, 64),
            "this law is FALSE at width 64, decided at width 64 by the difference \
             criterion in K+1 evaluations; no arm may rewrite through it"
        );
    };
}

struct AssocMul64;
impl Licensed for AssocMul64 {
    const CHECK: () = {
        let () = IMPLEMENTATION_CHECK;
        assert!(
            assoc_mul_diff_verdict(64),
            "wrapping-mul associativity failed its own criterion, which would \
             mean the implementation is broken"
        );
    };
}

fn consumer<L: Licensed>() -> &'static str {
    let () = <L as Licensed>::CHECK;
    "licensed"
}

// Reached instantiations, forced through top-level consts so the gate cannot
// be skipped by dead code (the rung-3 hazard 82's F8 measured).
const _LICENSE_L128: () = <LawL<128> as Licensed>::CHECK;
const _LICENSE_ASSOC: () = <AssocMul64 as Licensed>::CHECK;
#[cfg(use_l16)]
const _LICENSE_L16: () = <LawL<16> as Licensed>::CHECK;
#[cfg(use_l64)]
const _LICENSE_L64: () = <LawL<64> as Licensed>::CHECK;

fn main() {
    println!("p4b: arms gated on the law itself at width 64, no band, no transfer");
    println!(
        "  consumer::<LawL<128>>()  = {} (threshold 127, true at 64)",
        consumer::<LawL<128>>()
    );
    println!(
        "  consumer::<AssocMul64>() = {} (integer identity)",
        consumer::<AssocMul64>()
    );
    println!(
        "  l_k_diff_verdict(16, 64) = {}   l_k_diff_verdict(64, 64) = {}",
        const { l_k_diff_verdict(16, 64) },
        const { l_k_diff_verdict(64, 64) }
    );
    println!("  build with --cfg use_l16 or --cfg use_l64 for the refusals");
}
