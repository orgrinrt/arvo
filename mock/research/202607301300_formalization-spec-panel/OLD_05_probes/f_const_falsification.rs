//! PROBE F: 03 section 3 sketched a compile-time brute-force falsification
//! check and said plainly it had not compiled one. This compiles four, at
//! nightly-2026-05-28, and they run at const-eval time on every build.
//!
//! The property checked is 01's translation stability:
//!     phi(phi(x) + c) == phi(x + c)   for every exact x and representable c
//! which 01 proposes as the replacement for `Faithful`.
#![allow(dead_code)]

const MAX: i32 = 7; // 3-bit unsigned representable set [0, 7]
const SMIN: i32 = -8; // 4-bit signed [-8, 7]
const SMAX: i32 = 7;

// --- recovery maps, one per resolution. Two or three lines each. ------------
const fn phi_clamp_u(x: i32) -> i32 {
    if x > MAX {
        MAX
    } else if x < 0 {
        0
    } else {
        x
    }
}
const fn phi_zero_u(x: i32) -> i32 {
    if x > MAX || x < 0 {
        0
    } else {
        x
    }
}
const fn phi_mod_u(x: i32) -> i32 {
    x.rem_euclid(MAX + 1)
}
const fn phi_clamp_s(x: i32) -> i32 {
    if x > SMAX {
        SMAX
    } else if x < SMIN {
        SMIN
    } else {
        x
    }
}

// NOTE: a `const fn` cannot call through a `fn` pointer ("function pointer
// calls are not allowed in constant functions"), so the oracle cannot be a
// parameter. A macro is the gate-free way to keep one statement of the check
// and instantiate it per resolution.
macro_rules! stable_unsigned {
    ($name:ident, $phi:ident) => {
        const fn $name() -> bool {
            let mut x = 0;
            while x <= 2 * MAX {
                let mut c = 0;
                while c <= MAX {
                    if $phi($phi(x) + c) != $phi(x + c) {
                        return false;
                    }
                    c += 1;
                }
                x += 1;
            }
            true
        }
    };
}
macro_rules! stable_signed {
    ($name:ident, $phi:ident) => {
        const fn $name() -> bool {
            let mut x = 2 * SMIN;
            while x <= 2 * SMAX {
                let mut c = SMIN;
                while c <= SMAX {
                    if $phi($phi(x) + c) != $phi(x + c) {
                        return false;
                    }
                    c += 1;
                }
                x += 1;
            }
            true
        }
    };
}
stable_unsigned!(clamp_u, phi_clamp_u);
stable_unsigned!(zero_u, phi_zero_u);
stable_unsigned!(mod_u, phi_mod_u);
stable_signed!(clamp_s, phi_clamp_s);

pub const CLAMP_U: bool = clamp_u();
pub const ZERO_U: bool = zero_u();
pub const MOD_U: bool = mod_u();
pub const CLAMP_S: bool = clamp_s();

const _: () = assert!(CLAMP_U, "unsigned clamping should be translation-stable");
const _: () = assert!(!ZERO_U, "01 finding 1: SubstituteZero is NOT stable");
const _: () = assert!(MOD_U, "modular reduction should be translation-stable");
const _: () = assert!(!CLAMP_S, "signed clamping is NOT stable");

// --- the same machinery applied to the poison (absorbing bottom) delivery ---
// Kleene/bottom semantics: BOT absorbs. Encoded as Option-free sentinel.
const BOT: i32 = i32::MIN;
const fn add_bot_u(a: i32, b: i32) -> i32 {
    if a == BOT || b == BOT {
        return BOT;
    }
    let s = a + b;
    if s > MAX {
        BOT
    } else {
        s
    }
}
const fn add_bot_s(a: i32, b: i32) -> i32 {
    if a == BOT || b == BOT {
        return BOT;
    }
    let s = a + b;
    if s > SMAX || s < SMIN {
        BOT
    } else {
        s
    }
}

const fn assoc_u() -> bool {
    let mut a = 0;
    while a <= MAX {
        let mut b = 0;
        while b <= MAX {
            let mut c = 0;
            while c <= MAX {
                if add_bot_u(add_bot_u(a, b), c) != add_bot_u(a, add_bot_u(b, c)) {
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
const fn assoc_s() -> bool {
    let mut a = SMIN;
    while a <= SMAX {
        let mut b = SMIN;
        while b <= SMAX {
            let mut c = SMIN;
            while c <= SMAX {
                if add_bot_s(add_bot_s(a, b), c) != add_bot_s(a, add_bot_s(b, c)) {
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

pub const ASSOC_BOT_U: bool = assoc_u();
pub const ASSOC_BOT_S: bool = assoc_s();

const _: () = assert!(
    ASSOC_BOT_U,
    "bottom-extended unsigned addition is a semigroup"
);
const _: () = assert!(
    !ASSOC_BOT_S,
    "bottom-extended signed addition is NOT: (7+1)+(-1) vs 7+(1-1)"
);
