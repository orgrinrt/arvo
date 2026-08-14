// p7. Two expressibility claims nobody has compiled, and one of them is
// explicitly disclaimed in the record.
//
// CLAIM ONE, disclaimed by its own author.
// `111` section 19.2, having proposed the structural predicate, writes:
// "Expressibility rests on `112` rather than on me. Its `p8c` compiles a
// per-node check by recursing over the grade's own type structure... I did not
// compile it and I am not claiming I did."
//
// So the predicate `111` F111-15 offers, and which `p1` then measures over the
// whole term space, has never been shown to be writable. This file writes it.
// The wall it could hit is the one this workspace names repeatedly: condition
// (b) is a TOP-DOWN property ("no internal node has an ancestor multiplication
// whose sibling interval contains zero"), and the obvious spelling threads a
// boolean down through a const argument, `A: NoAnn<{ MASKED || B_HAS_ZERO }>`,
// which is arithmetic in a const argument and needs the forbidden feature.
//
// The repair is the one `a-refused-bound-wants-a-trait-not-a-feature.md`
// states: break the constraint into pieces that hold on their own. Condition
// (b) is equivalent to a purely LOCAL check at each multiplication node,
//
//     for each `mul` node, each child is a leaf or its sibling's interval
//     excludes zero
//
// because masking has no effect below a leaf, and a masked internal node fails
// at the moment it is reached. So the property is bottom-up after all, and it
// is an associated const with no arithmetic in type position.
//
// CLAIM TWO, which `p2` produced and nothing has compiled.
// `p2` establishes that a wrapping realisation map over ring operations is a
// ring homomorphism, so the arms agree exactly when the ROOT's exact value is in
// range, and the per-node discharge check is answering a question nobody asked.
// That makes the discharge check itself an arm selected on the overflow policy.
// This file writes that selector and checks it erases in both directions.
//
// PREDICTIONS, RECORDED BEFORE COMPILING
// --------------------------------------
// P1. Both compile with zero feature gates, no `dyn` and no `TypeId`.
// P2. The predicate agrees with `p1`'s model on every term below: it fires on
//     `(x + y) + z` and on `(x * y) + z`, and not on `(x + y) - y` (leaf repeat)
//     nor on `(x + y) * z` with `z` able to be zero.
// P3. Raising `z`'s declared lower bound off zero makes it fire on
//     `(x + y) * z`, which is `111` F111-17 arriving in the type system.
// P4. On `x - (y - z)` over an unsigned container the per-node check refuses and
//     the root-only check licenses, so the policy-selected arm differs by policy
//     on one term, which is what makes it an arm.
// P5. Both gated arms erase: the licensed body is the bare arithmetic and the
//     refused body is the saturating chain, with no residue of the check.
//
// NEGATIVE CONTROLS
// -----------------
// C1. A control term where the predicate must NOT fire, printed alongside, so a
//     predicate that returned `true` unconditionally would be visible.
// C2. A control where the per-node and root-only checks AGREE, so P4's
//     difference is not an artifact of the two checks always disagreeing.
// C3. Ungated reference functions compiled beside the gated ones, so "it erases"
//     is read off symbol aliasing rather than asserted.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// The container, carried as a type so the discharge check is generic over it
// and the overflow policy is a static bit rather than a runtime flag.
// ---------------------------------------------------------------------------

trait Ctr {
    const LO: i64;
    const HI: i64;
    /// True when the realisation map is a ring homomorphism, which `p2`
    /// establishes is exactly what makes the root decide everything.
    const RING_HOMOMORPHISM: bool;
}

struct U3Sat;
impl Ctr for U3Sat {
    const LO: i64 = 0;
    const HI: i64 = 7;
    const RING_HOMOMORPHISM: bool = false;
}

struct U3Wrap;
impl Ctr for U3Wrap {
    const LO: i64 = 0;
    const HI: i64 = 7;
    const RING_HOMOMORPHISM: bool = true;
}

struct U8Sat;
impl Ctr for U8Sat {
    const LO: i64 = 0;
    const HI: i64 = 255;
    const RING_HOMOMORPHISM: bool = false;
}

// ---------------------------------------------------------------------------
// A term is a type. Every quantity the predicate reads is an associated const,
// so every computation happens in an impl body.
// ---------------------------------------------------------------------------

trait Term {
    /// The corner-propagated interval at this node.
    const LO: i64;
    const HI: i64;
    /// Which declared leaves this subtree mentions, as an occupancy set.
    const LEAVES: u64;
    const IS_LEAF: bool;
    /// Condition (a): every leaf occurs at most once in this subtree.
    const LINEAR: bool;
    /// Condition (b): no internal node here sits under a multiplication whose
    /// sibling interval contains zero.
    const NO_ANNIHILATOR: bool;
}

/// A declared leaf: an identity and a two-endpoint declaration.
struct Leaf<const ID: u32, const L: i64, const H: i64>;

/// The three ring operations. Nothing else is in the signature, and `p2`'s C2
/// measures that adding a non-ring operation breaks the wrap arm.
struct Add<A, B>(PhantomData<(A, B)>);
struct Sub<A, B>(PhantomData<(A, B)>);
struct Mul<A, B>(PhantomData<(A, B)>);

impl<const ID: u32, const L: i64, const H: i64> Term for Leaf<ID, L, H> {
    const LO: i64 = L;
    const HI: i64 = H;
    const LEAVES: u64 = 1u64 << ID;
    const IS_LEAF: bool = true;
    const LINEAR: bool = true;
    const NO_ANNIHILATOR: bool = true;
}

impl<A: Term, B: Term> Term for Add<A, B> {
    const LO: i64 = A::LO + B::LO;
    const HI: i64 = A::HI + B::HI;
    const LEAVES: u64 = A::LEAVES | B::LEAVES;
    const IS_LEAF: bool = false;
    const LINEAR: bool = A::LINEAR && B::LINEAR && (A::LEAVES & B::LEAVES) == 0;
    const NO_ANNIHILATOR: bool = A::NO_ANNIHILATOR && B::NO_ANNIHILATOR;
}

impl<A: Term, B: Term> Term for Sub<A, B> {
    const LO: i64 = A::LO - B::HI;
    const HI: i64 = A::HI - B::LO;
    const LEAVES: u64 = A::LEAVES | B::LEAVES;
    const IS_LEAF: bool = false;
    const LINEAR: bool = A::LINEAR && B::LINEAR && (A::LEAVES & B::LEAVES) == 0;
    const NO_ANNIHILATOR: bool = A::NO_ANNIHILATOR && B::NO_ANNIHILATOR;
}

impl<A: Term, B: Term> Term for Mul<A, B> {
    const LO: i64 = {
        let c = [A::LO * B::LO, A::LO * B::HI, A::HI * B::LO, A::HI * B::HI];
        let mut m = c[0];
        let mut i = 1;
        while i < 4 {
            if c[i] < m {
                m = c[i];
            }
            i += 1;
        }
        m
    };
    const HI: i64 = {
        let c = [A::LO * B::LO, A::LO * B::HI, A::HI * B::LO, A::HI * B::HI];
        let mut m = c[0];
        let mut i = 1;
        while i < 4 {
            if c[i] > m {
                m = c[i];
            }
            i += 1;
        }
        m
    };
    const LEAVES: u64 = A::LEAVES | B::LEAVES;
    const IS_LEAF: bool = false;
    const LINEAR: bool = A::LINEAR && B::LINEAR && (A::LEAVES & B::LEAVES) == 0;
    // The local form of condition (b). A child that is a leaf has nothing below
    // it to be masked, so only an internal child under a zero-containing
    // sibling is a violation.
    const NO_ANNIHILATOR: bool = A::NO_ANNIHILATOR
        && B::NO_ANNIHILATOR
        && (A::IS_LEAF || !(B::LO <= 0 && 0 <= B::HI))
        && (B::IS_LEAF || !(A::LO <= 0 && 0 <= A::HI));
}

// ---------------------------------------------------------------------------
// The discharge checks, generic over the container, and the arm selected
// between them by the overflow policy.
// ---------------------------------------------------------------------------

trait Honest<C: Ctr> {
    /// `111` F111-15's structural predicate: when this holds and the per-node
    /// rule refuses, `p1` measures that an enumerating oracle also refuses, so
    /// the refusal is honest rather than conservative.
    const REFUSAL_IS_HONEST: bool;
}

impl<C: Ctr, T: Term> Honest<C> for T {
    const REFUSAL_IS_HONEST: bool = T::LINEAR && T::NO_ANNIHILATOR;
}

// The per-node fit check has to be seeded from the container, so it cannot live
// on `Term`, which knows nothing about a container. It recurses on its own trait
// instead, which is `112` p8c's `AllOk` shape generalised over the container.
trait AllFit<C: Ctr> {
    const OK: bool;
}
impl<C: Ctr, const ID: u32, const L: i64, const H: i64> AllFit<C> for Leaf<ID, L, H> {
    const OK: bool = L >= C::LO && H <= C::HI;
}
impl<C: Ctr, A: Term + AllFit<C>, B: Term + AllFit<C>> AllFit<C> for Add<A, B> {
    const OK: bool =
        A::OK && B::OK && <Add<A, B> as Term>::LO >= C::LO && <Add<A, B> as Term>::HI <= C::HI;
}
impl<C: Ctr, A: Term + AllFit<C>, B: Term + AllFit<C>> AllFit<C> for Sub<A, B> {
    const OK: bool =
        A::OK && B::OK && <Sub<A, B> as Term>::LO >= C::LO && <Sub<A, B> as Term>::HI <= C::HI;
}
impl<C: Ctr, A: Term + AllFit<C>, B: Term + AllFit<C>> AllFit<C> for Mul<A, B> {
    const OK: bool =
        A::OK && B::OK && <Mul<A, B> as Term>::LO >= C::LO && <Mul<A, B> as Term>::HI <= C::HI;
}

/// The honest selector, with `AllFit` supplying the per-node half.
trait Selected<C: Ctr> {
    const OK: bool;
}
impl<C: Ctr, T: Term + AllFit<C>> Selected<C> for T {
    const OK: bool = if C::RING_HOMOMORPHISM {
        T::LO >= C::LO && T::HI <= C::HI
    } else {
        <T as AllFit<C>>::OK
    };
}

// ---------------------------------------------------------------------------
// Terms. Leaf identities are distinct except where a repeat is the point.
// ---------------------------------------------------------------------------

type X = Leaf<0, 2, 3>;
type Y = Leaf<1, 0, 1>;
type Z0 = Leaf<2, 0, 3>; // can be zero
type Z1 = Leaf<2, 1, 3>; // declared away from zero, P3's term


type TAdd3 = Add<Add<X, Y>, Z0>; // predicate should fire
type TMulAdd = Add<Mul<X, Y>, Z0>; // predicate should fire: no masking ancestor
type TRepeat = Sub<Add<X, Y>, Y>; // (a) fails: leaf repeat
type TAnnih = Mul<Add<X, Y>, Z0>; // (b) fails: sibling can be zero
type TAnnihFixed = Mul<Add<X, Y>, Z1>; // P3: (b) holds once z excludes zero
type TNested = Sub<X, Sub<Y, Z0>>; // P4: per-node refuses, root licenses
type TFlat = Add<X, Y>; // C2: both checks agree

// ---------------------------------------------------------------------------
// Arms gated on the selected const, plus ungated controls for C3.
// ---------------------------------------------------------------------------

// A LOGICAL width narrower than the container, which is arvo's actual case and
// the only one where the wrap licence saves an instruction. Reduction at the
// logical width is a mask; the general arm masks after every node and the cheap
// arm masks once, or not at all.
//
// Three arms, and the distinction between the last two is the whole point:
//   general          reduce at every node. Correct by definition.
//   cheap_reduced    no intermediate reductions, one at the root.
//   cheap_unreduced  no reduction at all, the result used as a wide value.

const LOGICAL_HI: i64 = 31; // five bits in a u8 container
const MASK: u8 = 31;

struct L5Wrap;
impl Ctr for L5Wrap {
    const LO: i64 = 0;
    const HI: i64 = LOGICAL_HI;
    const RING_HOMOMORPHISM: bool = true;
}

struct L5Sat;
impl Ctr for L5Sat {
    const LO: i64 = 0;
    const HI: i64 = LOGICAL_HI;
    const RING_HOMOMORPHISM: bool = false;
}

// x in [12, 15], y in [0, 4], z in [0, 10]:
//   inner  y - z  -> [-10,   4]   leaves the logical range below zero
//   root x-(y-z)  -> [  8,  25]   stays inside it
type A5 = Leaf<0, 12, 15>;
type B5 = Leaf<1, 0, 4>;
type C5 = Leaf<2, 0, 10>;
type TW = Sub<A5, Sub<B5, C5>>;

#[inline(never)]
#[no_mangle]
pub fn general_masked(a: u8, b: u8, c: u8) -> u8 {
    (a.wrapping_sub(b.wrapping_sub(c) & MASK)) & MASK
}

#[inline(never)]
#[no_mangle]
pub fn cheap_reduced_ungated(a: u8, b: u8, c: u8) -> u8 {
    a.wrapping_sub(b.wrapping_sub(c)) & MASK
}

#[inline(never)]
#[no_mangle]
pub fn cheap_unreduced_ungated(a: u8, b: u8, c: u8) -> u8 {
    a.wrapping_sub(b.wrapping_sub(c))
}

/// Under wrap the root check licenses dropping the final reduction too, because
/// the root's exact value is already inside the logical range.
#[inline(never)]
#[no_mangle]
pub fn wrap_gated(a: u8, b: u8, c: u8) -> u8 {
    if const { <TW as Selected<L5Wrap>>::OK } {
        a.wrapping_sub(b.wrapping_sub(c))
    } else {
        (a.wrapping_sub(b.wrapping_sub(c) & MASK)) & MASK
    }
}

/// Under saturation the same declaration refuses, because the intermediate
/// leaves the range and a clamp is not a homomorphism.
#[inline(never)]
#[no_mangle]
pub fn sat_gated(a: u8, b: u8, c: u8) -> u8 {
    if const { <TW as Selected<L5Sat>>::OK } {
        a.wrapping_sub(b.wrapping_sub(c))
    } else {
        a.min(MASK).saturating_sub(b.min(MASK).saturating_sub(c.min(MASK)))
    }
}

#[inline(never)]
#[no_mangle]
pub fn general_saturating(a: u8, b: u8, c: u8) -> u8 {
    a.min(MASK).saturating_sub(b.min(MASK).saturating_sub(c.min(MASK)))
}

fn row<T, C>(name: &str)
where
    T: Term + AllFit<C> + Honest<C>,
    C: Ctr,
{
    println!(
        "  {:<26} [{:>4},{:>4}]  linear {:<5} noann {:<5} PRED {:<5} \
         per-node {:<5} root {:<5} SELECTED {:<5}",
        name,
        T::LO,
        T::HI,
        T::LINEAR,
        T::NO_ANNIHILATOR,
        <T as Honest<C>>::REFUSAL_IS_HONEST,
        <T as AllFit<C>>::OK,
        T::LO >= C::LO && T::HI <= C::HI,
        <T as Selected<C>>::OK,
    );
}

fn main() {
    println!("p7. The structural predicate, and the policy-selected discharge check");
    println!("{}", "=".repeat(78));
    println!();
    println!("Container [0, 7], SATURATING (not a ring homomorphism)");
    println!();
    row::<TAdd3, U3Sat>("(x + y) + z");
    row::<TMulAdd, U3Sat>("(x * y) + z");
    row::<TRepeat, U3Sat>("(x + y) - y          [C1]");
    row::<TAnnih, U3Sat>("(x + y) * z, z >= 0  [C1]");
    row::<TAnnihFixed, U3Sat>("(x + y) * z, z >= 1  [P3]");
    row::<TNested, U3Sat>("x - (y - z)          [P4]");
    row::<TFlat, U3Sat>("x + y                [C2]");
    println!();
    println!("Container [0, 7], WRAPPING (a ring homomorphism)");
    println!();
    row::<TAdd3, U3Wrap>("(x + y) + z");
    row::<TRepeat, U3Wrap>("(x + y) - y          [C1]");
    row::<TNested, U3Wrap>("x - (y - z)          [P4]");
    row::<TFlat, U3Wrap>("x + y                [C2]");
    println!();
    println!("P4 is the pair of `x - (y - z)` rows: the per-node check refuses");
    println!("because the inner difference reaches below zero, and the root-only");
    println!("check licenses because the root does not. C2 is the `x + y` pair,");
    println!("where the two checks agree, so the difference is about the term.");
    println!();
    println!("Container [0, 255], SATURATING, the width the gated arms use");
    println!();
    row::<TNested, U8Sat>("x - (y - z)");
    println!();
    println!();
    println!("The gated arms: logical width 5 in a u8 container, x in [12,15],");
    println!("y in [0,4], z in [0,10], term x - (y - z)");
    println!();
    row::<TW, L5Sat>("x - (y - z)  SAT");
    row::<TW, L5Wrap>("x - (y - z)  WRAP");
    println!();
    println!("  wrap_gated selected: {}", <TW as Selected<L5Wrap>>::OK);
    println!("  sat_gated  selected: {}", <TW as Selected<L5Sat>>::OK);
    println!();
    println!("The predicate's own claim, restated: where PRED is true and the");
    println!("per-node check refuses, `p1` measures that an enumerating oracle");
    println!("also refuses, on 63037 of 63037 such cells at uW3/sat.");
}
