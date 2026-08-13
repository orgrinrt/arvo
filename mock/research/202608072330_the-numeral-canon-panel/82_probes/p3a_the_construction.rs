// PROBE p3a. The lifting, built.
//
// p2 established that for signed saturating addition, the predicate
// `LO >= 0 || HI <= 0` on a DECLARED OPERAND WINDOW matches associativity on
// that window's generated closure exactly, zero residue in both directions,
// over every interval at widths 2 through 6.
//
// This probe turns that into the thing 80 says nobody has constructed: a
// declaration a consumer writes, from which the law region is a fact known at
// monomorphisation. Four things have to hold for it to count.
//
//   1. The predicate is const and gates a trait bound, so selection happens at
//      monomorphisation and nothing reads a value.
//   2. The permission is NOT an author-written marker. 80 section 3.1 showed a
//      declared marker is checked by nothing and the licensed consumer then
//      returns wrong answers with no signal. So the permission is computed.
//   3. The computed permission is a CLOSED FORM, because 80 section 4.2 showed
//      a swept positive verdict does not compile at a shipped width. So the
//      closed form is cross-checked against the sweep over a model band, at
//      compile time, per 80 section 4.3, and the agreement is an assertion.
//   4. No forbidden feature. No `generic_const_exprs`, no `specialization`, no
//      `dyn`, no `TypeId`, and this file declares `#![no_std]` to make the
//      last of those checkable rather than asserted.
//
// Compiled on the pinned toolchain. The companion probes p3b and p3c are the
// two refusals: a straddling declaration, and a perturbed closed form.

#![no_std]
#![allow(dead_code)]

// ---------------------------------------------------------------------------
// The operation, at a modelled width and at the shipped width.
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

// ---------------------------------------------------------------------------
// The SWEPT verdict, at a model width: is the operation associative on the set
// a fold over operands from [lo, hi] can actually reach. The closure is
// computed to a fixpoint; nothing about it is assumed.
// ---------------------------------------------------------------------------

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

    // Fixpoint: close under the operation.
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

    // Associativity over the closure.
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

// ---------------------------------------------------------------------------
// The CLOSED FORM verdict. Constant time in the declaration, no enumeration,
// so it is available at any width. This is the thing an arm gates on.
// ---------------------------------------------------------------------------

const fn closed_verdict(lo: i32, hi: i32) -> bool {
    lo >= 0 || hi <= 0
}

// ---------------------------------------------------------------------------
// The cross-check. The closed form is a declaration, and 80 section 3.1 is
// that a declaration checked against nothing is worth nothing. So it is
// checked against the sweep at every interval of every width in the model
// band, at compile time, and the agreement is asserted.
//
// What stays unchecked afterwards is one named thing: the transfer of the
// agreement from the model band to the shipped width. That is 68's transfer
// proviso, and it is the whole residue rather than the whole verdict.
// ---------------------------------------------------------------------------

const MODEL_BAND: [u32; 3] = [2, 3, 4];

const fn cross_check_holds() -> bool {
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

// The count of intervals the cross-check actually visited, so the check cannot
// pass by never entering the loop. 80's own first-run failure was exactly an
// instrument that returned before reaching the expensive path.
const fn cross_check_population() -> u32 {
    let mut total = 0u32;
    let mut k = 0;
    while k < MODEL_BAND.len() {
        let w = MODEL_BAND[k];
        let maxv: i32 = (1 << (w - 1)) - 1;
        let minv: i32 = -(1 << (w - 1));
        let mut lo = minv;
        while lo <= maxv {
            let mut hi = lo;
            while hi <= maxv {
                total += 1;
                hi += 1;
            }
            lo += 1;
        }
        k += 1;
    }
    total
}

// How many of the visited intervals the closed form says YES to, and how many
// NO to. Both must be non-zero or the cross-check is vacuous on one side.
const fn cross_check_yes() -> u32 {
    let mut yes = 0u32;
    let mut k = 0;
    while k < MODEL_BAND.len() {
        let w = MODEL_BAND[k];
        let maxv: i32 = (1 << (w - 1)) - 1;
        let minv: i32 = -(1 << (w - 1));
        let mut lo = minv;
        while lo <= maxv {
            let mut hi = lo;
            while hi <= maxv {
                if closed_verdict(lo, hi) {
                    yes += 1;
                }
                hi += 1;
            }
            lo += 1;
        }
        k += 1;
    }
    yes
}

pub const CROSS_CHECK_POPULATION: u32 = cross_check_population();
pub const CROSS_CHECK_YES: u32 = cross_check_yes();

// ---------------------------------------------------------------------------
// The declaration a consumer writes. A window on the operand type.
// ---------------------------------------------------------------------------

pub trait Window {
    const LO: i32;
    const HI: i32;
}

/// A declared operand window, as a type.
pub struct Win<const LO: i32, const HI: i32>;

impl<const LO: i32, const HI: i32> Window for Win<LO, HI> {
    const LO: i32 = LO;
    const HI: i32 = HI;
}

// ---------------------------------------------------------------------------
// The permission. Not writable by an author: the blanket impl is the only one,
// and its associated const runs the verdict.
// ---------------------------------------------------------------------------

pub trait ReassociableFold {
    const PROOF: ();
}

impl<W: Window> ReassociableFold for W {
    const PROOF: () = {
        assert!(
            cross_check_holds(),
            "the closed-form law verdict disagrees with the swept verdict somewhere in \
             the model band, so the closed form is wrong and no arm may be gated on it"
        );
        assert!(
            CROSS_CHECK_POPULATION > 0 && CROSS_CHECK_YES > 0
                && CROSS_CHECK_YES < CROSS_CHECK_POPULATION,
            "the cross-check is vacuous: it visited no intervals, or the closed form \
             answered the same way on every one of them"
        );
        assert!(
            closed_verdict(<W as Window>::LO, <W as Window>::HI),
            "the declared operand window straddles zero, so signed saturating addition \
             is not associative on the values a fold over it can reach, and a \
             reassociating consumer may not be instantiated at it"
        );
    };
}

// ---------------------------------------------------------------------------
// The consumer. Bounded on the permission, so instantiating it at a window the
// verdict refuses is a compile error rather than a wrong answer.
// ---------------------------------------------------------------------------

#[inline]
fn sat_add_i8(x: i8, y: i8) -> i8 {
    x.saturating_add(y)
}

/// Sequential fold. Legal at any window. Present as the comparator.
pub fn fold_sequential(xs: &[i8]) -> i8 {
    let mut acc: i8 = 0;
    let mut i = 0;
    while i < xs.len() {
        acc = sat_add_i8(acc, xs[i]);
        i += 1;
    }
    acc
}

/// Reassociating fold. Legal only where the declared window licenses it.
/// Splits into four independent accumulators, which is a reassociation.
pub fn fold_reassociated<W: Window + ReassociableFold>(xs: &[i8]) -> i8 {
    let _ = <W as ReassociableFold>::PROOF;
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
// The windows a consumer would actually declare.
// ---------------------------------------------------------------------------

/// Costs, penalties, drawdowns: a signed quantity that is never positive.
/// There is no unsigned type for this, which is why the window is the
/// declaration rather than the sign of the container.
pub type NeverPositive = Win<-128, 0>;

/// Magnitudes accumulated into a signed accumulator because a later step
/// subtracts. Non-negative operands, signed container.
pub type NeverNegative = Win<0, 127>;

/// A narrower window inside the non-negative half, which the verdict also
/// licenses: the predicate reads the declaration, not the container.
pub type SmallGains = Win<3, 40>;

pub fn total_drawdown(xs: &[i8]) -> i8 {
    fold_reassociated::<NeverPositive>(xs)
}
pub fn total_magnitude(xs: &[i8]) -> i8 {
    fold_reassociated::<NeverNegative>(xs)
}
pub fn total_small_gains(xs: &[i8]) -> i8 {
    fold_reassociated::<SmallGains>(xs)
}

// ---------------------------------------------------------------------------
// The ingest boundary, which is where a declared window comes from. This is
// O-G option (c) made concrete: the value condition is checked ONCE, where
// values enter, and is a typestate fact from then on. It works here only
// because the window is CLOSED under the operation, so nothing has to re-check
// it as the fold proceeds.
// ---------------------------------------------------------------------------

/// A value carried with the window it was admitted into. Private field: the
/// only way to get one is through the checked constructor or through an
/// operation the window is closed under.
pub struct InWindow<W: Window> {
    v: i8,
    _w: core::marker::PhantomData<W>,
}

impl<W: Window> InWindow<W> {
    /// The one runtime check, at the boundary. Returns none if the value is
    /// outside the declared window.
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

/// The closure property, as a function that exists only where it is sound: the
/// sum of two values from a sign-uniform window is in that window's closure,
/// and the closure has the same sign uniformity, so the permission survives.
/// Note this returns the raw value rather than an `InWindow<W>`, because the
/// SUM is in the closure and not in `W` itself. What survives is the
/// permission, not the window.
#[inline]
pub fn add_within<W: Window + ReassociableFold>(a: InWindow<W>, b: InWindow<W>) -> i8 {
    let _ = <W as ReassociableFold>::PROOF;
    sat_add_i8(a.get(), b.get())
}

// ---------------------------------------------------------------------------
// A runtime agreement check, so the file is not only a compilation claim.
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn agreement_check(seed: u64) -> u64 {
    let mut s = seed | 1;
    let mut disagreements: u64 = 0;
    let mut checked: u64 = 0;
    let mut buf = [0i8; 64];

    let mut trial = 0;
    while trial < 20000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let len = (s % 64) as usize;
        // non-positive window
        let mut i = 0;
        while i < len {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            buf[i] = -((s % 129) as i64 as i8 as i16 as i8);
            i += 1;
        }
        let a = fold_sequential(&buf[..len]);
        let b = fold_reassociated::<NeverPositive>(&buf[..len]);
        checked += 1;
        if a != b {
            disagreements += 1;
        }
        // non-negative window
        let mut i = 0;
        while i < len {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            buf[i] = (s % 128) as i8;
            i += 1;
        }
        let a = fold_sequential(&buf[..len]);
        let b = fold_reassociated::<NeverNegative>(&buf[..len]);
        checked += 1;
        if a != b {
            disagreements += 1;
        }
        trial += 1;
    }
    (checked << 32) | disagreements
}

#[panic_handler]
fn ph(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
