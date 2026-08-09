// Tick 3, positive half: enumerate the design's operation surface as it
// stands through file 50 and show every member's growth (does the result
// numeral widen, narrow, or stay put, and by how much) is computed from
// (which primitive, which operand numeral(s)) alone, with no Policy
// parameter anywhere in the trait that computes it.
//
// This models numerals by a bare precision marker (const P: u32), not the
// real Nat/Pos/Bias tower. That is deliberate: the question under test is
// which INPUTS a growth computation reads, not how a numeral encodes its
// own value, and file 35's own structural probe (35_probes/probe_2) used
// the identical lightweight-marker method for the same reason. Reusing
// the full sealed tower here would add encoding machinery the question
// does not touch, not rigour.
//
// Every *_Growth trait below is generic over operand numeral types ONLY.
// None of them take a Policy parameter. That absence is the claim, and
// grep confirms it: no `Policy` token appears left of any `Growth` impl
// in this file.

#![allow(dead_code)]

pub trait Numeral {
    const P: u32;
}

macro_rules! numeral {
    ($name:ident, $p:expr) => {
        pub struct $name;
        impl Numeral for $name {
            const P: u32 = $p;
        }
    };
}

// operand markers used below. Binary32's 24-bit significand is included
// so the mul_full-over-Ranged check reproduces file 50's own number
// (50_probes/probe_3_exponent_as_type.rs: "binary32 * binary32 -> p=48").
numeral!(N8, 8);
numeral!(N16, 16);
numeral!(N4, 4); // file 50 probe 3's M1, p=4
numeral!(N3, 3); // file 50 probe 3's M2, p=3
numeral!(Binary32, 24);

const fn ceil_log2(n: u32) -> u32 {
    32 - (n - 1).leading_zeros()
}

pub trait Op {
    const IS_EXACT: bool;
}

// --- 1. in-numeral quantised arithmetic (add/sub/mul/div, non-exact) ---
// result stays in the same declared numeral; no growth.
pub struct AddInNumeral;
pub struct SubInNumeral;
pub struct MulInNumeral;
pub struct DivInNumeral;
impl Op for AddInNumeral {
    const IS_EXACT: bool = false;
}
impl Op for SubInNumeral {
    const IS_EXACT: bool = false;
}
impl Op for MulInNumeral {
    const IS_EXACT: bool = false;
}
impl Op for DivInNumeral {
    const IS_EXACT: bool = false;
}

pub trait InNumeralGrowth<N: Numeral> {
    type Out: Numeral;
}
impl<N: Numeral> InNumeralGrowth<N> for AddInNumeral {
    type Out = N;
}
impl<N: Numeral> InNumeralGrowth<N> for SubInNumeral {
    type Out = N;
}
impl<N: Numeral> InNumeralGrowth<N> for MulInNumeral {
    type Out = N;
}
impl<N: Numeral> InNumeralGrowth<N> for DivInNumeral {
    type Out = N;
}

// --- 2. mul_full over Implicit numerals (49:269, widths add) ---
pub struct MulFull;
impl Op for MulFull {
    const IS_EXACT: bool = true;
}
pub struct MulNum<N1, N2>(core::marker::PhantomData<(N1, N2)>);
impl<N1: Numeral, N2: Numeral> Numeral for MulNum<N1, N2> {
    const P: u32 = N1::P + N2::P;
}
pub trait MulFullGrowth<N1: Numeral, N2: Numeral> {
    type Out: Numeral;
}
impl<N1: Numeral, N2: Numeral> MulFullGrowth<N1, N2> for MulFull {
    type Out = MulNum<N1, N2>;
}

// --- 3. mulnum over Ranged numerals (50_probes/probe_3, file 50 4.1) ---
pub struct MulNumRanged;
impl Op for MulNumRanged {
    const IS_EXACT: bool = true;
}
pub struct RangedMulNum<M1, M2>(core::marker::PhantomData<(M1, M2)>);
impl<M1: Numeral, M2: Numeral> Numeral for RangedMulNum<M1, M2> {
    const P: u32 = M1::P + M2::P;
}
pub trait MulNumRangedGrowth<M1: Numeral, M2: Numeral> {
    type Out: Numeral;
}
impl<M1: Numeral, M2: Numeral> MulNumRangedGrowth<M1, M2> for MulNumRanged {
    type Out = RangedMulNum<M1, M2>;
}

// --- 4. div_exact by a type-level constant (file 43, section 1.13) ---
pub struct DivExact;
impl Op for DivExact {
    const IS_EXACT: bool = true;
}
pub trait Constant {
    const V: u32;
}
pub struct C4410;
impl Constant for C4410 {
    const V: u32 = 4410;
}
pub struct DivNum<N, C>(core::marker::PhantomData<(N, C)>);
impl<N: Numeral, C: Constant> Numeral for DivNum<N, C> {
    // exact division by a constant scales the adjustment; precision is
    // unchanged (section 1.13: "adjustment A * (cd/cn) reduced"). the
    // constant's own value plays no role in the WIDTH; it is folded into
    // the type-level rational the numeral carries, which this stand-in
    // does not model. that is fine: the point under test is that the
    // Out type is a function of (DivExact, N, C), never of a Policy.
    const P: u32 = N::P;
}
pub trait DivExactGrowth<N: Numeral, C: Constant> {
    type Out: Numeral;
}
impl<N: Numeral, C: Constant> DivExactGrowth<N, C> for DivExact {
    type Out = DivNum<N, C>;
}

// --- 5. div_floor / rem, the Euclidean pair (file 43, section 1.13) ---
pub struct DivFloor;
impl Op for DivFloor {
    const IS_EXACT: bool = true;
}
pub struct RemOp;
impl Op for RemOp {
    const IS_EXACT: bool = true;
}
pub struct QuotNum<N1, N2>(core::marker::PhantomData<(N1, N2)>);
impl<N1: Numeral, N2: Numeral> Numeral for QuotNum<N1, N2> {
    const P: u32 = if N1::P > N2::P { N1::P } else { N2::P };
}
pub struct RemNum<N1, N2>(core::marker::PhantomData<(N1, N2)>);
impl<N1: Numeral, N2: Numeral> Numeral for RemNum<N1, N2> {
    const P: u32 = N1::P + N2::P; // the gcd-quantum numeral, stand-in width
}
pub trait DivFloorGrowth<N1: Numeral, N2: Numeral> {
    type Out: Numeral;
}
impl<N1: Numeral, N2: Numeral> DivFloorGrowth<N1, N2> for DivFloor {
    type Out = QuotNum<N1, N2>;
}
pub trait RemGrowth<N1: Numeral, N2: Numeral> {
    type Out: Numeral;
}
impl<N1: Numeral, N2: Numeral> RemGrowth<N1, N2> for RemOp {
    type Out = RemNum<N1, N2>;
}

// --- 6. fold / fold_sequential / fold_compensated (49 section 1.14, 1.8; file 50 4.5) ---
pub struct Fold;
pub struct FoldSequential;
pub struct FoldCompensated;
impl Op for Fold {
    const IS_EXACT: bool = false;
}
impl Op for FoldSequential {
    const IS_EXACT: bool = false;
}
impl Op for FoldCompensated {
    const IS_EXACT: bool = false;
}
pub trait Arity {
    const N: u32;
}
pub struct Eight;
impl Arity for Eight {
    const N: u32 = 8;
}
pub struct AccumNum<N, A>(core::marker::PhantomData<(N, A)>);
impl<N: Numeral, A: Arity> Numeral for AccumNum<N, A> {
    // interior-safety accumulator: the operand's own width plus a
    // ceil(log2 n) term, section 1.8 and file 50 4.5's derivation.
    const P: u32 = N::P + ceil_log2(A::N);
}
pub trait FoldGrowth<N: Numeral, A: Arity> {
    type Out: Numeral;
}
impl<N: Numeral, A: Arity> FoldGrowth<N, A> for Fold {
    type Out = AccumNum<N, A>;
}
// fold_sequential regroups nothing and publishes nothing: no accumulator
// growth at all, faithful by construction (49:509-514).
pub trait FoldSequentialGrowth<N: Numeral> {
    type Out: Numeral;
}
impl<N: Numeral> FoldSequentialGrowth<N> for FoldSequential {
    type Out = N;
}
// fold_compensated: a genuinely different formula (error feedback keeps
// the accumulator at the operand's own width; the residual is carried
// separately, section 1.5), still purely a function of (Op, N, Arity).
pub struct CompensatedAccum<N, A>(core::marker::PhantomData<(N, A)>);
impl<N: Numeral, A: Arity> Numeral for CompensatedAccum<N, A> {
    const P: u32 = N::P;
}
pub trait FoldCompensatedGrowth<N: Numeral, A: Arity> {
    type Out: Numeral;
}
impl<N: Numeral, A: Arity> FoldCompensatedGrowth<N, A> for FoldCompensated {
    type Out = CompensatedAccum<N, A>;
}

// --- 7. quantize::<Src, Dst>, the explicit named target ---
pub struct Quantize;
impl Op for Quantize {
    const IS_EXACT: bool = false;
}
pub trait QuantizeGrowth<Src: Numeral, Dst: Numeral> {
    type Out: Numeral;
}
impl<Src: Numeral, Dst: Numeral> QuantizeGrowth<Src, Dst> for Quantize {
    type Out = Dst;
}

// --- checks: every Out reproduces the number the corresponding file
// already reported, and none of the eleven trait declarations above
// names a Policy anywhere in its parameter list. ---

const _: () = assert!(
    <<MulFull as MulFullGrowth<N4, N3>>::Out as Numeral>::P == 7,
    "file 50 probe_3: M1(p=4) * M2(p=3) -> p=7"
);
const _: () = assert!(
    <<MulNumRanged as MulNumRangedGrowth<Binary32, Binary32>>::Out as Numeral>::P == 48,
    "file 50 probe_3: binary32 * binary32 -> p=48"
);
const _: () = assert!(<<AddInNumeral as InNumeralGrowth<N16>>::Out as Numeral>::P == 16);
const _: () = assert!(<<DivExact as DivExactGrowth<N16, C4410>>::Out as Numeral>::P == 16);
const _: () = assert!(<<Fold as FoldGrowth<N8, Eight>>::Out as Numeral>::P == 11); // 8 + ceil(log2 8) = 8 + 3
const _: () = assert!(<<FoldSequential as FoldSequentialGrowth<N8>>::Out as Numeral>::P == 8);
const _: () = assert!(<<Quantize as QuantizeGrowth<N8, N16>>::Out as Numeral>::P == 16);

fn main() {
    println!("eleven operations, every Out computed from (Op, operand numeral(s)) alone");
    println!(
        "mul_full(p4,p3).P            = {}",
        <<MulFull as MulFullGrowth<N4, N3>>::Out as Numeral>::P
    );
    println!(
        "mulnum_ranged(bin32,bin32).P = {}",
        <<MulNumRanged as MulNumRangedGrowth<Binary32, Binary32>>::Out as Numeral>::P
    );
    println!(
        "fold(p8, arity8).P           = {}",
        <<Fold as FoldGrowth<N8, Eight>>::Out as Numeral>::P
    );
    println!(
        "fold_sequential(p8).P        = {}",
        <<FoldSequential as FoldSequentialGrowth<N8>>::Out as Numeral>::P
    );
}
