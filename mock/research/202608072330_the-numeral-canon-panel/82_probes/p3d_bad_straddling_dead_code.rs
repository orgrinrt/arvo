// PROBE p3d. The construction rebuilt so the refusal happens at TYPE CHECK.
//
// Three results forced this rewrite.
//
//   p3b: the const-assert permission is a monomorphisation-time refusal. A
//        straddling declaration inside an unreached `pub fn` compiled clean,
//        and `nm` on the archive shows the function was never emitted. Only
//        when the same function was made `#[no_mangle] extern "C"` did the
//        refusal fire (p3b2, p3_compile_output.txt and the p3b2 run).
//
//   p3e: lifting the predicate into a bound is refused outright. Both
//        spellings give "generic parameters may not be used in const
//        operations" with rustc pointing at `generic_const_exprs`, which the
//        workspace forbids (p3e_compile_output.txt).
//
//   The workspace's standing move on a refused bound: decompose the constraint
//   into smaller named contracts that each hold on their own, rather than
//   forcing an expression into a position the language does not allow it.
//
// The decomposition here is available because the predicate
// `LO >= 0 || HI <= 0` is not an arbitrary arithmetic condition. It is a
// DISJUNCTION OF TWO SHAPES, and a shape can be a type. So sign uniformity
// stops being computed from the declaration and starts being carried by which
// declaration was written, with the unsigned const-parameter type making a
// straddling bound unspellable inside either licensed shape.
//
// What that buys, and it is the whole point: the refusal is now a trait bound,
// so it fires during type checking whether or not the call is ever reached,
// and the verdict cross-check moves to a crate-level const that is evaluated
// unconditionally rather than per instantiation.

#![no_std]
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// Stage minus one, restated in the compiler: the swept verdict and the closed
// form, identical to p3a. Unchanged so the two constructions are comparable.
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
    lo >= 0 || hi <= 0
}

const MODEL_BAND: [u32; 3] = [2, 3, 4];

const fn cross_check(sink: &mut [u32; 3]) -> bool {
    let mut k = 0;
    while k < MODEL_BAND.len() {
        let w = MODEL_BAND[k];
        let maxv: i32 = (1 << (w - 1)) - 1;
        let minv: i32 = -(1 << (w - 1));
        let mut lo = minv;
        while lo <= maxv {
            let mut hi = lo;
            while hi <= maxv {
                sink[0] += 1;
                let cf = closed_verdict(lo, hi);
                let sv = swept_verdict(lo, hi, w);
                if cf {
                    sink[1] += 1;
                } else {
                    sink[2] += 1;
                }
                if cf != sv {
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

// UNCONDITIONAL. A crate-level const is evaluated whether or not any generic
// function is ever instantiated, which is the property the p3a shape lacked.
// The population counters are asserted non-degenerate so the check cannot pass
// by never entering the loop or by answering one way on every interval, which
// is the "setup that helps" failure 80 hit twice.
const CROSS_CHECK: ([u32; 3], bool) = {
    let mut counts = [0u32; 3];
    let ok = cross_check(&mut counts);
    (counts, ok)
};

const _: () = {
    assert!(
        CROSS_CHECK.1,
        "the closed-form sign-uniformity verdict disagrees with the swept verdict \
         somewhere in the model band, so no arm may be gated on the shape split below"
    );
    assert!(
        CROSS_CHECK.0[0] > 0,
        "the cross-check visited no intervals and therefore established nothing"
    );
    assert!(
        CROSS_CHECK.0[1] > 0 && CROSS_CHECK.0[2] > 0,
        "the cross-check answered the same way on every interval it visited, so it \
         exercised only one side of the verdict"
    );
};

pub const CROSS_CHECK_INTERVALS: u32 = CROSS_CHECK.0[0];
pub const CROSS_CHECK_LICENSED: u32 = CROSS_CHECK.0[1];
pub const CROSS_CHECK_REFUSED: u32 = CROSS_CHECK.0[2];

// ---------------------------------------------------------------------------
// The declarations. Three shapes, of which two are sign-uniform BY
// CONSTRUCTION because their bounds are unsigned, and one is general.
// ---------------------------------------------------------------------------

pub trait Window {
    const LO: i32;
    const HI: i32;
}

/// A window `[LO, HI]` with both bounds non-negative. A negative bound cannot
/// be written here, because the const parameters are unsigned.
pub struct NonNeg<const LO: u8, const HI: u8>;

/// A window `[-MAG_HI, -MAG_LO]`, entirely non-positive for the same reason.
pub struct NonPos<const MAG_LO: u8, const MAG_HI: u8>;

/// A general window, which may straddle zero.
pub struct Win<const LO: i32, const HI: i32>;

impl<const LO: u8, const HI: u8> Window for NonNeg<LO, HI> {
    const LO: i32 = LO as i32;
    const HI: i32 = HI as i32;
}
impl<const MAG_LO: u8, const MAG_HI: u8> Window for NonPos<MAG_LO, MAG_HI> {
    const LO: i32 = -(MAG_HI as i32);
    const HI: i32 = -(MAG_LO as i32);
}
impl<const LO: i32, const HI: i32> Window for Win<LO, HI> {
    const LO: i32 = LO;
    const HI: i32 = HI;
}

// ---------------------------------------------------------------------------
// The permission. Implemented for the two sign-uniform shapes and for nothing
// else. There is no const assertion in it, because there is nothing left to
// assert: the shape IS the predicate.
// ---------------------------------------------------------------------------

#[diagnostic::on_unimplemented(
    message = "the declared operand window `{Self}` may straddle zero",
    label = "signed saturating addition is not associative on the values a fold over a straddling window can reach",
    note = "declare the window as `NonNeg<LO, HI>` or `NonPos<MAG_LO, MAG_HI>` if the operands are sign-uniform, or use the sequential fold"
)]
pub trait ReassociableFold: Window {}

impl<const LO: u8, const HI: u8> ReassociableFold for NonNeg<LO, HI> {}
impl<const MAG_LO: u8, const MAG_HI: u8> ReassociableFold for NonPos<MAG_LO, MAG_HI> {}

// ---------------------------------------------------------------------------
// The arms.
// ---------------------------------------------------------------------------

#[inline]
fn sat_add_i8(x: i8, y: i8) -> i8 {
    x.saturating_add(y)
}

/// Legal at any window.
#[no_mangle]
pub extern "C" fn fold_sequential(p: *const i8, n: usize) -> i8 {
    let xs = unsafe { core::slice::from_raw_parts(p, n) };
    let mut acc: i8 = 0;
    let mut i = 0;
    while i < xs.len() {
        acc = sat_add_i8(acc, xs[i]);
        i += 1;
    }
    acc
}

/// Legal only where the declared window shape licenses it.
pub fn fold_reassociated<W: ReassociableFold>(xs: &[i8]) -> i8 {
    let mut a0: i8 = 0;
    let mut a1: i8 = 0;
    let mut a2: i8 = 0;
    let mut a3: i8 = 0;
    let mut ch = xs.chunks_exact(4);
    for c in &mut ch {
        a0 = sat_add_i8(a0, c[0]);
        a1 = sat_add_i8(a1, c[1]);
        a2 = sat_add_i8(a2, c[2]);
        a3 = sat_add_i8(a3, c[3]);
    }
    let mut tail: i8 = 0;
    for &x in ch.remainder() {
        tail = sat_add_i8(tail, x);
    }
    let l = sat_add_i8(a0, a1);
    let r = sat_add_i8(a2, a3);
    sat_add_i8(sat_add_i8(l, r), tail)
}

// ---------------------------------------------------------------------------
// The declarations a consumer writes.
// ---------------------------------------------------------------------------

/// Drawdowns, costs, penalties. Never positive, and there is no unsigned type
/// for this, which is why the window is the declaration rather than the sign of
/// the container.
pub type Drawdown = NonPos<0, 128>;

/// Magnitudes accumulated into a signed accumulator because a later step
/// subtracts.
pub type Magnitude = NonNeg<0, 127>;

/// A narrower non-negative window, licensed for the same reason.
pub type SmallGain = NonNeg<3, 40>;


/// A straddling window, used only from a plain `pub fn` that nothing reaches.
/// Under the const-assert construction this compiled clean (p3b). Under the
/// structural construction it must not.
pub type Delta = Win<-128, 127>;
pub fn total_delta_dead_code(xs: &[i8]) -> i8 {
    fold_reassociated::<Delta>(xs)
}

#[no_mangle]
pub extern "C" fn total_drawdown(p: *const i8, n: usize) -> i8 {
    fold_reassociated::<Drawdown>(unsafe { core::slice::from_raw_parts(p, n) })
}
#[no_mangle]
pub extern "C" fn total_magnitude(p: *const i8, n: usize) -> i8 {
    fold_reassociated::<Magnitude>(unsafe { core::slice::from_raw_parts(p, n) })
}
#[no_mangle]
pub extern "C" fn total_small_gain(p: *const i8, n: usize) -> i8 {
    fold_reassociated::<SmallGain>(unsafe { core::slice::from_raw_parts(p, n) })
}

// ---------------------------------------------------------------------------
// The second obligation, which has a different binding time from the first and
// is worth separating rather than folding in.
//
// Sign uniformity is structural and refuses at type check. Whether the declared
// window FITS the container is a different claim, it is arithmetic on the
// bounds, and it is back to a const assertion, so it refuses at
// monomorphisation and only where reached. One declaration, two obligations,
// two strengths.
// ---------------------------------------------------------------------------

pub trait FitsI8: Window {
    const FITS: ();
}
impl<W: Window> FitsI8 for W {
    const FITS: () = {
        assert!(
            <W as Window>::LO >= -128 && <W as Window>::HI <= 127,
            "the declared operand window does not fit the container"
        );
    };
}

#[no_mangle]
pub extern "C" fn total_checked_fit(p: *const i8, n: usize) -> i8 {
    let _ = <Magnitude as FitsI8>::FITS;
    fold_reassociated::<Magnitude>(unsafe { core::slice::from_raw_parts(p, n) })
}

// ---------------------------------------------------------------------------
// The ingest boundary. O-G option (c), and the reason it works here is that the
// licensed windows are CLOSED under the operation: the value condition is
// checked once, where values enter, and nothing has to re-check it as the fold
// proceeds. That closure is what p1's route lacked.
// ---------------------------------------------------------------------------

pub struct InWindow<W: Window> {
    v: i8,
    _w: core::marker::PhantomData<W>,
}

impl<W: Window> InWindow<W> {
    #[inline]
    pub fn admit(v: i8) -> Option<Self> {
        if (v as i32) >= <W as Window>::LO && (v as i32) <= <W as Window>::HI {
            Some(InWindow {
                v,
                _w: core::marker::PhantomData,
            })
        } else {
            None
        }
    }
    #[inline]
    pub fn get(&self) -> i8 {
        self.v
    }
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
