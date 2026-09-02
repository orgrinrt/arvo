// PROBE p7. The construction rebuilt in the shape op's own sentence names.
//
//   > Making the predicates const expressions for example, allows using const
//   > functions and pipe in some data that is outside the typestate. However,
//   > being const time expressions, typestate is usable there too
//
// p3d put the declared window in the TYPESTATE, as two shapes whose const
// parameters make a straddling bound unspellable. That is the "typestate is
// usable there too" half. This probe builds the other half: the window is an
// ordinary `const` in the consumer's own module, outside the typestate
// entirely, piped through a const function into an inline const block.
//
// Three questions, and the third is the one worth the probe.
//
//   1. Does it work at all?
//   2. Does a straddling window refuse?
//   3. WHEN does it refuse? p3b established that the typestate route's
//      const-assert is a monomorphisation-time refusal that fires only where
//      the generic function is instantiated in codegen, so a straddling
//      declaration inside unreached code compiled clean. The arm here is NOT
//      generic, so there is nothing to monomorphise. That may make the refusal
//      earlier and unconditional, or it may make it later. Measured, not
//      assumed.

#![no_std]
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// The verdict, as a const function. Identical to p3a and p3d.
// ---------------------------------------------------------------------------

const fn model_sat_add(x: i32, y: i32, minv: i32, maxv: i32) -> i32 {
    let s = x + y;
    if s > maxv {
        maxv
    } else if s < minv {
        minv
    } else {
        s
    }
}

const MODEL_CAP: usize = 32;

const fn swept_verdict(lo: i32, hi: i32, w: u32) -> bool {
    let maxv: i32 = (1 << (w - 1)) - 1;
    let minv: i32 = -(1 << (w - 1));
    let n = (maxv - minv + 1) as usize;
    let mut present = [false; MODEL_CAP];
    let mut v = lo;
    while v <= hi {
        present[(v - minv) as usize] = true;
        v += 1;
    }
    let mut changed = true;
    while changed {
        changed = false;
        let mut i = 0;
        while i < n {
            if present[i] {
                let mut j = 0;
                while j < n {
                    if present[j] {
                        let z = model_sat_add(minv + i as i32, minv + j as i32, minv, maxv);
                        let zi = (z - minv) as usize;
                        if !present[zi] {
                            present[zi] = true;
                            changed = true;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
    }
    let mut ia = 0;
    while ia < n {
        if present[ia] {
            let a = minv + ia as i32;
            let mut ib = 0;
            while ib < n {
                if present[ib] {
                    let b = minv + ib as i32;
                    let mut ic = 0;
                    while ic < n {
                        if present[ic] {
                            let c = minv + ic as i32;
                            let l = model_sat_add(model_sat_add(a, b, minv, maxv), c, minv, maxv);
                            let r = model_sat_add(a, model_sat_add(b, c, minv, maxv), minv, maxv);
                            if l != r {
                                return false;
                            }
                        }
                        ic += 1;
                    }
                }
                ib += 1;
            }
        }
        ia += 1;
    }
    true
}

const fn closed_verdict(lo: i32, hi: i32) -> bool {
    lo >= -1 || hi <= 0
}

const MODEL_BAND: [u32; 3] = [2, 3, 4];

const fn cross_check() -> bool {
    let mut k = 0;
    while k < MODEL_BAND.len() {
        let w = MODEL_BAND[k];
        let maxv: i32 = (1 << (w - 1)) - 1;
        let minv: i32 = -(1 << (w - 1));
        let mut lo = minv;
        while lo <= maxv {
            let mut hi = lo;
            while hi <= maxv {
                if swept_verdict(lo, hi, w) != closed_verdict(lo, hi) {
                    return false;
                }
                hi += 1;
            }
            lo += 1;
        }
        k += 1;
    }
    true
}

const _: () = {
    assert!(
        cross_check(),
        "the closed-form verdict disagrees with the swept verdict in the model band"
    );
};

/// The whole predicate, as one const function of data that need not come from
/// any type. This is the thing op's sentence describes.
pub const fn reassociation_licensed(lo: i32, hi: i32) -> bool {
    closed_verdict(lo, hi)
}

// ---------------------------------------------------------------------------
// The consumer's own data, outside the typestate. An ordinary module const.
// ---------------------------------------------------------------------------

/// A ledger of drawdowns. The bounds are the consumer's, in the consumer's
/// module, and no type carries them.
mod ledger {
    pub const COST_LO: i32 = -128;
    pub const COST_HI: i32 = 0;
}

/// A second consumer whose bounds are computed by a const function from other
/// const data rather than written as literals, since op's sentence says const
/// functions are in scope for this.
mod budget {
    pub const CATEGORIES: i32 = 8;
    pub const PER_CATEGORY_MAX: i32 = 15;
    pub const fn upper() -> i32 {
        CATEGORIES * PER_CATEGORY_MAX
    }
    pub const LO: i32 = 0;
    pub const HI: i32 = upper();
}

#[inline(always)]
fn sat(x: i8, y: i8) -> i8 {
    x.saturating_add(y)
}

fn reassociated(xs: &[i8]) -> i8 {
    let mut acc = [0i8; 16];
    let mut ch = xs.chunks_exact(16);
    for c in &mut ch {
        let mut k = 0;
        while k < 16 {
            acc[k] = sat(acc[k], c[k]);
            k += 1;
        }
    }
    let mut t: i8 = 0;
    for &x in ch.remainder() {
        t = sat(t, x);
    }
    let mut k = 0;
    while k < 16 {
        t = sat(t, acc[k]);
        k += 1;
    }
    t
}

// ---------------------------------------------------------------------------
// The arms. Not generic. The gate is an inline const block reading data that
// lives outside any type.
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn total_cost(p: *const i8, n: usize) -> i8 {
    const {
        assert!(
            reassociation_licensed(ledger::COST_LO, ledger::COST_HI),
            "the ledger's declared cost window straddles zero, so the reassociated \
             fold may not be used for it"
        )
    };
    reassociated(unsafe { core::slice::from_raw_parts(p, n) })
}

#[no_mangle]
pub extern "C" fn total_budget(p: *const i8, n: usize) -> i8 {
    const {
        assert!(
            reassociation_licensed(budget::LO, budget::HI),
            "the budget's declared window straddles zero, so the reassociated fold \
             may not be used for it"
        )
    };
    reassociated(unsafe { core::slice::from_raw_parts(p, n) })
}

/// The dead-code case, which is where the typestate route failed in p3b. This
/// function is a plain `pub fn` that nothing reaches, and it is NOT generic.
pub fn total_cost_dead_code(xs: &[i8]) -> i8 {
    const {
        assert!(
            reassociation_licensed(ledger::COST_LO, ledger::COST_HI),
            "dead-code arm: the declared window straddles zero"
        )
    };
    reassociated(xs)
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
