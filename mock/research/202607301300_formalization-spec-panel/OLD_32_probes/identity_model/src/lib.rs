//! A minimal, faithful model of the settled identity contract from
//! `31_arntzen_settling_the_identity_contract.md` section 4, built for
//! codegen and compile-time measurement rather than for shipping.
//!
//! Scope: `Numeral` (Radix, Precision, ExponentForm nesting Adjustment,
//! Bias, Underflow, Specials; Domain), and `Encoding` nested inside
//! `Lowering` (SignIndexing, FieldLayout, Canonicalisation), matching
//! `31:326-359` and `202607301200_topic.the-formalization-spec.md:38-186`.
//! `Policy` is modelled only deep enough to call one arithmetic operation
//! through the full stack; the multiplicative half and the fold-law
//! apparatus are out of this probe's scope (already measured on the
//! `Policy`/`Lowering` side by file 08).
#![feature(const_trait_impl)]
#![no_std]

use core::marker::PhantomData;

// --- Identity axes -----------------------------------------------------

pub const trait Radix {
    const VALUE: u32;
}
pub struct Two;
const impl Radix for Two {
    const VALUE: u32 = 2;
}
pub struct Ten;
const impl Radix for Ten {
    const VALUE: u32 = 10;
}

pub const trait Precision {
    const DIGITS: u32;
}
pub struct P<const N: u32>;
const impl<const N: u32> Precision for P<N> {
    const DIGITS: u32 = N;
}

pub const trait Underflow {}
pub struct Unbounded;
const impl Underflow for Unbounded {}
pub struct Gradual;
const impl Underflow for Gradual {}
pub struct Flushed;
const impl Underflow for Flushed {}

pub const trait Specials {
    const HAS_INFINITY: bool;
    const HAS_NAN: bool;
}
pub struct WithInfNaN;
const impl Specials for WithInfNaN {
    const HAS_INFINITY: bool = true;
    const HAS_NAN: bool = true;
}

pub const trait Adjustment {}
pub struct Unit;
const impl Adjustment for Unit {}
pub struct FullRange<const F: u32>;
const impl<const F: u32> Adjustment for FullRange<F> {}

pub const trait Bias {}
pub struct ZeroBias;
const impl Bias for ZeroBias {}

pub const trait ExponentForm {
    // true for `Ranged` (Specials structurally reachable), false for `Implicit`.
    const CARRIES_SPECIALS: bool;

    /// The settled quantiser pipeline (`31:378-384`): round on the
    /// unbounded-exponent extension of the grid (assumed already done by
    /// the caller; `exact` is the rounded value), then classify against
    /// the range and resolve. This is where identity's `Specials` member
    /// meets the common path: `Implicit`'s impl has no arm that can
    /// produce a specials sentinel, because `Implicit` carries no
    /// `Specials` type parameter to name one from. Not a runtime branch
    /// eliminated by DCE: a branch that has no source text to write.
    fn classify(exact: i64, min_repr: i64, max_repr: i64) -> i64;
}

/// The exponent is a type-level constant. No `Underflow`, no `Specials`:
/// there is no field for either, matching the topic file's own reasoning
/// ("a constant exponent has no bottom to fall off").
pub struct Implicit<const E: i32, A: Adjustment, B: Bias>(PhantomData<(A, B)>);
const impl<const E: i32, A: Adjustment, B: Bias> ExponentForm for Implicit<E, A, B> {
    const CARRIES_SPECIALS: bool = false;

    #[inline]
    fn classify(exact: i64, min_repr: i64, max_repr: i64) -> i64 {
        if exact > max_repr {
            max_repr
        } else if exact < min_repr {
            min_repr
        } else {
            exact
        }
    }
}

/// The exponent is stored per-value, in a range with real edges.
pub struct Ranged<const EMIN: i32, const EMAX: i32, U: Underflow, S: Specials>(PhantomData<(U, S)>);
const impl<const EMIN: i32, const EMAX: i32, U: Underflow, S: Specials> ExponentForm
    for Ranged<EMIN, EMAX, U, S>
{
    const CARRIES_SPECIALS: bool = true;

    #[inline]
    fn classify(exact: i64, min_repr: i64, max_repr: i64) -> i64 {
        const POS_INF: i64 = i64::MAX;
        const NEG_INF: i64 = i64::MIN;
        // `S::HAS_INFINITY` is a const known at monomorphisation, so this
        // is not a runtime dispatch on whether specials exist; it is
        // constant-folded to whichever arm applies before codegen.
        if S::HAS_INFINITY {
            if exact > max_repr {
                POS_INF
            } else if exact < min_repr {
                NEG_INF
            } else {
                exact
            }
        } else if exact > max_repr {
            max_repr
        } else if exact < min_repr {
            min_repr
        } else {
            exact
        }
    }
}

pub const trait SignDomain {}
pub struct NonNegative;
const impl SignDomain for NonNegative {}
pub struct Symmetric;
const impl SignDomain for Symmetric {}
pub struct AsymmetricLow;
const impl SignDomain for AsymmetricLow {}

/// `31:328-333`. Four members: Radix, Precision, Exponent, Domain.
pub const trait Numeral {
    type Radix: Radix;
    type Precision: Precision;
    type Exponent: ExponentForm;
    type Domain: SignDomain;
}

/// A fixed-point composition: `Implicit` exponent form, so `Specials` is
/// structurally absent (see `probe_2_specials_structurally_absent.rs`).
///
/// `PBITS` and `NEG_F` are carried as independent const parameters rather
/// than derived from `I + F` / `-F` inside the impl: computing a const
/// generic from another const generic hits the same wall the
/// consolidation already names and rejects
/// (`26_consolidation_two.md` droplist, "Computing type-level width
/// arithmetic as a const generic under `min_generic_const_args`").
/// That wall is orthogonal to what this probe measures, so it is sidestepped
/// by carrying both quantities explicitly at the call site (exactly the
/// discipline the consolidation's own multiplicative half already adopted:
/// type-level values are provided, not computed in const position).
pub struct FixNumeral<const I: u32, const F: u32, const PBITS: u32, const NEG_F: i32, D: SignDomain>(
    PhantomData<D>,
);
const impl<const I: u32, const F: u32, const PBITS: u32, const NEG_F: i32, D: SignDomain> Numeral
    for FixNumeral<I, F, PBITS, NEG_F, D>
{
    type Radix = Two;
    type Precision = P<PBITS>;
    type Exponent = Implicit<NEG_F, Unit, ZeroBias>;
    type Domain = D;
}

/// A float-shaped composition: `Ranged` exponent form, `Specials` present.
pub struct FloatNumeral<const EMIN: i32, const EMAX: i32, const PBITS: u32, D: SignDomain>(
    PhantomData<D>,
);
const impl<const EMIN: i32, const EMAX: i32, const PBITS: u32, D: SignDomain> Numeral
    for FloatNumeral<EMIN, EMAX, PBITS, D>
{
    type Radix = Two;
    type Precision = P<PBITS>;
    type Exponent = Ranged<EMIN, EMAX, Gradual, WithInfNaN>;
    type Domain = D;
}

// --- Encoding, nested inside Lowering -----------------------------------

pub const trait SignIndexing {}
pub struct UnsignedIdx;
const impl SignIndexing for UnsignedIdx {}
pub struct TwosComplement;
const impl SignIndexing for TwosComplement {}
pub struct SignMagnitude;
const impl SignIndexing for SignMagnitude {}
pub struct OnesComplement;
const impl SignIndexing for OnesComplement {}

pub const trait FieldLayout {
    const HIDDEN_BIT: bool;
    const ENCODING_BIAS: i64;
    const RESERVED_CODES: u32;
}
/// The common fixed-point case: nothing to derive, nothing reserved.
pub struct PlainFields;
const impl FieldLayout for PlainFields {
    const HIDDEN_BIT: bool = false;
    const ENCODING_BIAS: i64 = 0;
    const RESERVED_CODES: u32 = 0;
}
/// The IEEE case: a hidden leading bit, an exponent bias, reserved codes.
pub struct IEEEFields<const BIAS: i64>;
const impl<const BIAS: i64> FieldLayout for IEEEFields<BIAS> {
    const HIDDEN_BIT: bool = true;
    const ENCODING_BIAS: i64 = BIAS;
    const RESERVED_CODES: u32 = 2; // inf, nan
}

pub const trait Canonicalisation {
    /// `encode . decode` idempotent; `31:372-374`. The identity case is
    /// what a datum with no cohorts and no signed zero needs; the NaN
    /// case is what a datum with payload multiplicity needs.
    fn canonicalize(x: i64) -> i64;
}
pub struct IdentityCanon;
const impl Canonicalisation for IdentityCanon {
    #[inline]
    fn canonicalize(x: i64) -> i64 {
        x
    }
}
/// Models NaN-payload canonicalisation: any bit pattern in the NaN band
/// (here, magic marker range) collapses to one canonical representative.
/// Not `#[inline(always)]`: the point is to measure what a real branch
/// costs, not to force the compiler's hand either way.
pub struct NaNCanon;
const impl Canonicalisation for NaNCanon {
    #[inline]
    fn canonicalize(x: i64) -> i64 {
        const NAN_LOW: i64 = 0x7FF0_0000_0000_0001;
        const NAN_HIGH: i64 = 0x7FF7_FFFF_FFFF_FFFF;
        const CANONICAL_NAN: i64 = 0x7FF8_0000_0000_0000;
        if x >= NAN_LOW && x <= NAN_HIGH {
            CANONICAL_NAN
        } else {
            x
        }
    }
}

/// `31:354-359`.
pub const trait Encoding {
    type SignIndexing: SignIndexing;
    type Fields: FieldLayout;
    type Canonical: Canonicalisation;
}

pub struct FixEncoding<Si: SignIndexing>(PhantomData<Si>);
const impl<Si: SignIndexing> Encoding for FixEncoding<Si> {
    type SignIndexing = Si;
    type Fields = PlainFields;
    type Canonical = IdentityCanon;
}

pub struct FloatEncoding<const BIAS: i64>;
const impl<const BIAS: i64> Encoding for FloatEncoding<BIAS> {
    type SignIndexing = SignMagnitude;
    type Fields = IEEEFields<BIAS>;
    type Canonical = NaNCanon;
}

// --- Lowering ------------------------------------------------------------

pub const trait StoredWidth {}
pub struct Minimum;
const impl StoredWidth for Minimum {}
pub struct DoubleLogical;
const impl StoredWidth for DoubleLogical {}

pub const trait Widening {}
pub struct NoWiden;
const impl Widening for NoWiden {}
pub struct InContainer;
const impl Widening for InContainer {}
pub struct PerOperation;
const impl Widening for PerOperation {}

pub const trait StorageLayout {}
pub struct Dense;
const impl StorageLayout for Dense {}
pub struct Bitpacked;
const impl StorageLayout for Bitpacked {}

/// `topic.the-formalization-spec.md:55-59`, with `Encoding` nested in per
/// `31:346-352`.
pub const trait Lowering {
    type Encoding: Encoding;
    type StoredWidth: StoredWidth;
    type Widening: Widening;
    type Layout: StorageLayout;
}

pub struct HotLowering<Si: SignIndexing>(PhantomData<Si>);
const impl<Si: SignIndexing> Lowering for HotLowering<Si> {
    type Encoding = FixEncoding<Si>;
    type StoredWidth = Minimum;
    type Widening = NoWiden;
    type Layout = Dense;
}

pub struct ColdLowering<Si: SignIndexing>(PhantomData<Si>);
const impl<Si: SignIndexing> Lowering for ColdLowering<Si> {
    type Encoding = FixEncoding<Si>;
    type StoredWidth = Minimum;
    type Widening = PerOperation;
    type Layout = Bitpacked;
}

pub struct FloatLowering<const BIAS: i64>;
const impl<const BIAS: i64> Lowering for FloatLowering<BIAS> {
    type Encoding = FloatEncoding<BIAS>;
    type StoredWidth = Minimum;
    type Widening = NoWiden;
    type Layout = Dense;
}

// --- Minimal Policy, enough to route one op through the stack -----------
//
// `S: Policy + Lowering` is the fused two-parameter form the consolidation
// settled on (`26_consolidation_two.md:34`); each `*Lowering` marker below
// also carries the (here trivial) `Policy` impl so it can stand in for `S`
// directly, matching how a real preset (`Hot`, `Warm`, `Cold`, `Precise`)
// is one type implementing both traits.

pub const trait Policy {}
pub struct WrapPolicy;
const impl Policy for WrapPolicy {}
const impl<Si: SignIndexing> Policy for HotLowering<Si> {}
const impl<Si: SignIndexing> Policy for ColdLowering<Si> {}
const impl<const BIAS: i64> Policy for FloatLowering<BIAS> {}

// --- The composed number, and one operation through the full stack ------

#[repr(transparent)]
pub struct Number<N: Numeral, S>(pub i64, PhantomData<(N, S)>)
where
    S: Policy + Lowering;

/// One arithmetic op through the full ten(+)-axis stack. The body pays
/// the canonicalisation obligation the crossing contract states
/// (`31:372-374`): every value-producing step whose result could carry
/// a non-canonical datum is followed by `Encoding::Canonical` before the
/// datum is observed (`31:263-270`, `DatumDeterministic`).
#[inline(never)]
pub fn add<N: Numeral, S: Policy + Lowering>(a: i64, b: i64) -> i64 {
    let raw = a.wrapping_add(b);
    <S::Encoding as Encoding>::Canonical::canonicalize(raw)
}

/// Sum-reduce over a slice, same stack, used for the array-workload
/// probes (vectorisation, canonicalisation-on-the-hot-path).
#[inline(never)]
pub fn sum_reduce<N: Numeral, S: Policy + Lowering>(xs: &[i64]) -> i64 {
    let mut acc: i64 = 0;
    for &x in xs {
        acc = add::<N, S>(acc, x);
    }
    acc
}

/// Elementwise (no fold, no cross-lane dependency) add over two slices
/// into an output slice, the shape a real column-store consumer writes.
#[inline(never)]
pub fn elementwise_add<N: Numeral, S: Policy + Lowering>(a: &[i64], b: &[i64], out: &mut [i64]) {
    let n = a.len().min(b.len()).min(out.len());
    let mut i = 0;
    while i < n {
        out[i] = add::<N, S>(a[i], b[i]);
        i += 1;
    }
}

// --- Codegen-inspection wrappers, distinct exported symbols ---------------
//
// `#[no_mangle] pub extern "C" fn` + `#[inline(never)]` so each is its own
// disassemblable symbol under `objdump -d`, per the consolidation's own
// check-build discipline (`26_consolidation_two.md` section 1.6): defeat
// the inliner deliberately so the question "what did this axis generate"
// has an answer that survives to the object file.

/// `classify` on the fixed-point (`Implicit`) side: the common path that
/// never has specials.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_classify_implicit(exact: i64, min_repr: i64, max_repr: i64) -> i64 {
    Implicit::<0, Unit, ZeroBias>::classify(exact, min_repr, max_repr)
}

/// `classify` on the float-shaped (`Ranged`, `Specials = WithInfNaN`)
/// side.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_classify_ranged(exact: i64, min_repr: i64, max_repr: i64) -> i64 {
    Ranged::<-100, 100, Gradual, WithInfNaN>::classify(exact, min_repr, max_repr)
}

/// `sum_reduce` over the fixed-point stack: `Canonical = IdentityCanon`.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_sum_reduce_fixed(xs: &[i64]) -> i64 {
    sum_reduce::<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>(xs)
}

/// `sum_reduce` over the float-shaped stack: `Canonical = NaNCanon`.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_sum_reduce_float(xs: &[i64]) -> i64 {
    sum_reduce::<FloatNumeral<-100, 100, 52, Symmetric>, FloatLowering<1023>>(xs)
}

/// Raw baseline: a plain wrapping-add fold with no identity/encoding
/// machinery at all, the floor every comparison above is measured
/// against.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_sum_reduce_raw_baseline(xs: &[i64]) -> i64 {
    let mut acc: i64 = 0;
    for &x in xs {
        acc = acc.wrapping_add(x);
    }
    acc
}

/// `elementwise_add` over the fixed-point stack, the shape a real
/// column-store consumer writes (no fold, no cross-lane dependency).
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_elementwise_add_fixed(a: &[i64], b: &[i64], out: &mut [i64]) {
    elementwise_add::<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>(a, b, out)
}

/// Raw baseline for the elementwise case.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_elementwise_add_raw_baseline(a: &[i64], b: &[i64], out: &mut [i64]) {
    let n = a.len().min(b.len()).min(out.len());
    let mut i = 0;
    while i < n {
        out[i] = a[i].wrapping_add(b[i]);
        i += 1;
    }
}

// --- Bitpacked column: extraction through `Encoding::Fields` -------------
//
// Models a Cold (`Bitpacked` layout) column of `Q(13,3)`-shaped fields
// packed contiguously with no padding, matching arvo's own `Bits<N,S>` /
// `bitfield!` shape. `FIELD_BITS = 16` here (13 + 3, a realistic
// non-power-of-two width) packed into a `u64` word, four fields per word
// with 0 bits of slack, so extraction needs a real shift-and-mask against
// a per-field bit offset, not a byte-aligned load.

pub const FIELD_BITS: u32 = 16;
pub const FIELDS_PER_WORD: u32 = 4;
pub const FIELD_MASK: u64 = (1u64 << FIELD_BITS) - 1;

/// Extract field `idx` (0..4) from a packed word, honouring
/// `Encoding::Fields::HIDDEN_BIT` / `ENCODING_BIAS` from the trivial
/// (`PlainFields`) case: both are zero/false, so this is exactly the
/// shift-and-mask a hand-written extractor would write, with the
/// `FieldLayout` associated consts read at compile time (`Fi` is a
/// concrete type, so `Fi::HIDDEN_BIT` / `Fi::ENCODING_BIAS` are
/// monomorphised constants, not runtime reads).
#[inline]
pub fn extract_field<Fi: FieldLayout>(word: u64, idx: u32) -> i64 {
    let shift = idx * FIELD_BITS;
    let raw = (word >> shift) & FIELD_MASK;
    let raw = if Fi::HIDDEN_BIT {
        raw | (1u64 << (FIELD_BITS - 1))
    } else {
        raw
    };
    (raw as i64) + Fi::ENCODING_BIAS
}

/// Sum-reduce over a packed column: four fields per word, through the
/// full `Encoding::Fields` projection.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_bitpacked_column_sum(words: &[u64]) -> i64 {
    let mut acc: i64 = 0;
    for &w in words {
        let mut idx = 0u32;
        while idx < FIELDS_PER_WORD {
            acc = acc.wrapping_add(extract_field::<PlainFields>(w, idx));
            idx += 1;
        }
    }
    acc
}

/// Hand-rolled baseline: the same shift-and-mask, written directly with
/// no `FieldLayout` trait in the way.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_bitpacked_column_sum_raw_baseline(words: &[u64]) -> i64 {
    let mut acc: i64 = 0;
    for &w in words {
        let mut idx = 0u32;
        while idx < FIELDS_PER_WORD {
            let shift = idx * FIELD_BITS;
            let raw = (w >> shift) & FIELD_MASK;
            acc = acc.wrapping_add(raw as i64);
            idx += 1;
        }
    }
    acc
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --- Inlinable variant: the shape a real consumer actually ships -------
//
// `add` above is `#[inline(never)]` so its own body disassembles cleanly
// in isolation (the canonicalisation-cost probes). That same
// `#[inline(never)]` defeats vectorisation across the call boundary in a
// loop, which is an artefact of isolating the callee, not a property of
// the design: arvo's own shipped hot paths are `#[inline]` (or left to
// LTO) precisely so the optimiser can see through them. This is the
// inlinable twin, used only for the vectorisation question.
#[inline]
pub fn add_inlinable<N: Numeral, S: Policy + Lowering>(a: i64, b: i64) -> i64 {
    let raw = a.wrapping_add(b);
    <S::Encoding as Encoding>::Canonical::canonicalize(raw)
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_elementwise_add_fixed_inlinable(a: &[i64], b: &[i64], out: &mut [i64]) {
    let n = a.len().min(b.len()).min(out.len());
    let mut i = 0;
    while i < n {
        out[i] = add_inlinable::<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>(
            a[i], b[i],
        );
        i += 1;
    }
}

/// Same as `probe_elementwise_add_fixed_inlinable`, but written in the
/// equal-length idiom (`for i in 0..a.len()`, asserting the lengths
/// agree) instead of taking `min()` of three lengths. This isolates
/// whether the identity contract's richness affects vectorisation, or
/// whether the loop idiom does.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_elementwise_add_fixed_equal_len_idiom(
    a: &[i64],
    b: &[i64],
    out: &mut [i64],
) {
    assert!(a.len() == b.len() && b.len() == out.len());
    for i in 0..a.len() {
        out[i] = add_inlinable::<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>(
            a[i], b[i],
        );
    }
}

/// Ablation: identical to `probe_elementwise_add_fixed_equal_len_idiom`
/// (same signature, same assert, same crate, same build flags) but the
/// body is a bare `wrapping_add`, no generic identity/encoding call at
/// all. Isolates whether the `assert!` or the generic dispatch chain is
/// what defeats vectorisation above.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_elementwise_add_ablation_no_generic(a: &[i64], b: &[i64], out: &mut [i64]) {
    assert!(a.len() == b.len() && b.len() == out.len());
    for i in 0..a.len() {
        out[i] = a[i].wrapping_add(b[i]);
    }
}

/// The corrected vectorisation probe: identical to the control
/// (`for i in 0..a.len()`, no `assert!`, out-of-bounds simply panics via
/// the normal bounds check like the control) but through the full
/// identity contract (`add_inlinable::<FixNumeral<...>, HotLowering<...>>`)
/// rather than a bare `wrapping_add`.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_elementwise_add_fixed_no_assert(a: &[i64], b: &[i64], out: &mut [i64]) {
    for i in 0..a.len() {
        out[i] = add_inlinable::<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>(
            a[i], b[i],
        );
    }
}

/// Verbatim copy of the standalone control (`/tmp/vec_control.rs`),
/// co-located in this crate to isolate whether the crate's own build
/// environment (LTO, function count, panic handler, cdylib-adjacent
/// settings) is what differs, rather than `add_inlinable` itself.
#[no_mangle]
#[inline(never)]
pub extern "C" fn probe_vectorises_verbatim_control(a: &[i64], b: &[i64], out: &mut [i64]) {
    for i in 0..a.len() {
        out[i] = a[i].wrapping_add(b[i]);
    }
}

// --- Layout: does the identity/encoding richness cost bytes? -----------

const _: () = assert!(
    core::mem::size_of::<Number<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>>(
    ) == core::mem::size_of::<i64>(),
    "Number<N,S> must be exactly its raw payload width; Radix, Precision, \
     ExponentForm nesting, Domain, SignIndexing, FieldLayout and \
     Canonicalisation are all zero-sized types erased at compile time"
);
const _: () = assert!(
    core::mem::align_of::<Number<FixNumeral<8, 5, 13, -5, Symmetric>, HotLowering<TwosComplement>>>(
    ) == core::mem::align_of::<i64>()
);
const _: () = assert!(
    core::mem::size_of::<Number<FloatNumeral<-100, 100, 52, Symmetric>, FloatLowering<1023>>>()
        == core::mem::size_of::<i64>(),
    "the same holds for the Ranged/Specials-carrying side: WithInfNaN, \
     Gradual, IEEEFields, NaNCanon are all zero-sized"
);
const _: () = assert!(core::mem::size_of::<Two>() == 0);
const _: () = assert!(core::mem::size_of::<P<13>>() == 0);
const _: () = assert!(core::mem::size_of::<Implicit<-5, Unit, ZeroBias>>() == 0);
const _: () = assert!(core::mem::size_of::<Ranged<-100, 100, Gradual, WithInfNaN>>() == 0);
const _: () = assert!(core::mem::size_of::<FixEncoding<TwosComplement>>() == 0);
const _: () = assert!(core::mem::size_of::<FloatEncoding<1023>>() == 0);
const _: () = assert!(core::mem::size_of::<IdentityCanon>() == 0);
const _: () = assert!(core::mem::size_of::<NaNCanon>() == 0);
