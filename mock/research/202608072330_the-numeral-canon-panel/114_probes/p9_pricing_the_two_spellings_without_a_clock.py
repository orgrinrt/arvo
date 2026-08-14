#!/usr/bin/env python3
"""p9. The compile-time question, attacked rather than reported.

THE QUESTION, AS THE PANEL LEFT IT
-----------------------------------
`111` section 26 carries it as the one open item that "a harness rather than an
argument closes":

    Disjunction against static selection. `112` section 9 says sound rules
    disjoin and a design carries as many as it can afford. Section 21 says the
    choice is decidable from the term's structure, so a design can carry one.
    Both are arms and the difference between them is a compile-time cost that
    nothing has priced.

WHY IT CANNOT BE PRICED AS A DURATION, STATED PRECISELY
--------------------------------------------------------
`evidence-lives-in-the-repo-or-it-never-happened.md` is explicit: where mockspace
is used, a measurement outside `mock/benches/` on the bench harness "can not be
named 'bench' or 'benchmark' or anything similar", and a compile-time figure
taken anywhere else is "an ad-hoc quick spike with no substance".

The harness cannot take this measurement. Its schema is entirely runtime. The
committed CSV header, read from `mock/benches/bitpack-carrier-width_n16384.csv`:

    run,pass,cooldown_ms,mode,variant,batch_idx,e2e_ns,algo_ns,bridge_ns,
    batch_count,score,input_tag,instructions,cycles,setup_ns,first_ns,digest

and the meta it emits beside it records `cpu`, `os`, `rustc`, `git_commit`,
`timestamp`, `counter_freq`, `framework`. There is no compile-time column, no
compile-time mode, and no build phase the harness times: it loads each variant as
a prebuilt cdylib and measures calls into it. So a compile-time comparison is not
merely absent from the harness, it is outside what the harness is shaped to hold.

That is the honest answer to half the question. `cargo build --timings`,
`-Ztime-passes` and a shell `time` all produce durations, all of them outside the
harness, and none of them may be called a measurement here.

WHAT CAN BE ESTABLISHED WITHOUT A CLOCK
----------------------------------------
Two things, and both are exact rather than measured, which is the distinction
`108` section 7 draws between a computed and a measured cost coordinate.

ONE, a COUNT. The number of associated-const evaluations each spelling forces is
a function of the term alone. For a fold of L leaves, with 2L-1 nodes:

    corner spelling   2 consts per node          -> O(L)
    affine spelling   an L-cell vector per node  -> O(L^2)

`111` F111-18 reports the affine STATE as "one coefficient per distinct leaf",
which is the state at ONE node. The type-level tower instantiates a vector at
every node, so the compile-side quantity is the product and not the factor. That
is a correction to how F111-18 will be read, and it is arithmetic rather than a
measurement.

TWO, a REFUSAL. Whether a spelling compiles at a given fold length is a
deterministic, reproducible fact about the source and the pinned toolchain, and
it is not a duration. If the affine tower stops compiling at some L, that is a
compile-time cost expressed in the one unit this workspace already accepts as
exact: the compiler's own verdict. This probe measures that.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. The corner spelling compiles at every length attempted.
P2. The affine spelling refuses at some length, and the refusal is a limit
    rather than a type error, so it names the resource that ran out.
P3. The disjunction refuses at the same length as the affine spelling alone,
    because it forces the same tower.
P4. The static selection compiles wherever the corner spelling does, whenever
    the structural predicate fires, because it never forces the affine tower.
    That is the whole claim of `111` section 21 turned into a compile check.

NEGATIVE CONTROLS
-----------------
C1. A spelling that forces NEITHER tower must compile at every length, or the
    refusals below are about the generator rather than about the towers.
C2. The affine tower's associated const must be genuinely forced: a variant that
    declares the types and never reads a const must compile where the reading
    one refuses. If both refuse, the cost is in type construction and the
    finding is about that instead, which is a different and equally reportable
    answer.
"""

import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).parent

PRELUDE = """
#![allow(dead_code)]
use core::marker::PhantomData;

// ---- the corner spelling: two consts per node -----------------------------
trait Iv { const LO: i64; const HI: i64; }
struct ILeaf<const L: i64, const H: i64>;
impl<const L: i64, const H: i64> Iv for ILeaf<L, H> { const LO: i64 = L; const HI: i64 = H; }
struct IAdd<A, B>(PhantomData<(A, B)>);
impl<A: Iv, B: Iv> Iv for IAdd<A, B> { const LO: i64 = A::LO + B::LO; const HI: i64 = A::HI + B::HI; }
struct ISub<A, B>(PhantomData<(A, B)>);
impl<A: Iv, B: Iv> Iv for ISub<A, B> { const LO: i64 = A::LO - B::HI; const HI: i64 = A::HI - B::LO; }

trait IvAll { const OK: bool; }
impl<const L: i64, const H: i64> IvAll for ILeaf<L, H> { const OK: bool = L >= 0 && H <= CHI; }
impl<A: Iv + IvAll, B: Iv + IvAll> IvAll for IAdd<A, B> {
    const OK: bool = A::OK && B::OK && <IAdd<A, B> as Iv>::LO >= 0 && <IAdd<A, B> as Iv>::HI <= CHI;
}
impl<A: Iv + IvAll, B: Iv + IvAll> IvAll for ISub<A, B> {
    const OK: bool = A::OK && B::OK && <ISub<A, B> as Iv>::LO >= 0 && <ISub<A, B> as Iv>::HI <= CHI;
}

// ---- the affine spelling: an L-cell coefficient vector at every node ------
trait Coeff { const V: i64; }
struct Lit<const N: i64>;
impl<const N: i64> Coeff for Lit<N> { const V: i64 = N; }
struct CSum<A, B>(PhantomData<(A, B)>);
impl<A: Coeff, B: Coeff> Coeff for CSum<A, B> { const V: i64 = A::V + B::V; }
struct CDiff<A, B>(PhantomData<(A, B)>);
impl<A: Coeff, B: Coeff> Coeff for CDiff<A, B> { const V: i64 = A::V - B::V; }

trait Coeffs { const RADIUS: i64; }
struct Nil;
struct Cons<H, T>(PhantomData<(H, T)>);
impl Coeffs for Nil { const RADIUS: i64 = 0; }
impl<H: Coeff, T: Coeffs> Coeffs for Cons<H, T> {
    const RADIUS: i64 = { let v = H::V; (if v < 0 { -v } else { v }) + T::RADIUS };
}

trait AddC<O> { type Out: Coeffs; }
impl AddC<Nil> for Nil { type Out = Nil; }
impl<H1: Coeff, T1: AddC<T2>, H2: Coeff, T2> AddC<Cons<H2, T2>> for Cons<H1, T1> {
    type Out = Cons<CSum<H1, H2>, <T1 as AddC<T2>>::Out>;
}
trait SubC<O> { type Out: Coeffs; }
impl SubC<Nil> for Nil { type Out = Nil; }
impl<H1: Coeff, T1: SubC<T2>, H2: Coeff, T2> SubC<Cons<H2, T2>> for Cons<H1, T1> {
    type Out = Cons<CDiff<H1, H2>, <T1 as SubC<T2>>::Out>;
}

trait Grade { type Centre: Coeff; type Vec: Coeffs;
    const LO: i64 = <Self::Centre as Coeff>::V - <Self::Vec as Coeffs>::RADIUS;
    const HI: i64 = <Self::Centre as Coeff>::V + <Self::Vec as Coeffs>::RADIUS;
}
struct Aff<C, V>(PhantomData<(C, V)>);
impl<C: Coeff, V: Coeffs> Grade for Aff<C, V> { type Centre = C; type Vec = V; }
struct GAdd<A, B>(PhantomData<(A, B)>);
impl<A: Grade, B: Grade> Grade for GAdd<A, B> where A::Vec: AddC<B::Vec> {
    type Centre = CSum<A::Centre, B::Centre>;
    type Vec = <A::Vec as AddC<B::Vec>>::Out;
}
struct GSub<A, B>(PhantomData<(A, B)>);
impl<A: Grade, B: Grade> Grade for GSub<A, B> where A::Vec: SubC<B::Vec> {
    type Centre = CDiff<A::Centre, B::Centre>;
    type Vec = <A::Vec as SubC<B::Vec>>::Out;
}

trait GAll { const OK: bool; }
impl<C: Coeff, V: Coeffs> GAll for Aff<C, V> {
    const OK: bool = <Aff<C, V> as Grade>::LO >= 0 && <Aff<C, V> as Grade>::HI <= CHI;
}
impl<A: Grade + GAll, B: Grade + GAll> GAll for GAdd<A, B> where A::Vec: AddC<B::Vec> {
    const OK: bool = A::OK && B::OK && <GAdd<A, B> as Grade>::LO >= 0 && <GAdd<A, B> as Grade>::HI <= CHI;
}
impl<A: Grade + GAll, B: Grade + GAll> GAll for GSub<A, B> where A::Vec: SubC<B::Vec> {
    const OK: bool = A::OK && B::OK && <GSub<A, B> as Grade>::LO >= 0 && <GSub<A, B> as Grade>::HI <= CHI;
}

const CHI: i64 = 1_000_000;
"""


def gen_corner(n):
    leaves = [f"ILeaf<0, 3>" for _ in range(n)]
    t = leaves[0]
    for i in range(1, n):
        t = f"IAdd<{t}, {leaves[i]}>"
    return f"type CornerTerm = {t};\n"


def gen_affine_types(n):
    """Leaf i has coefficient 3 in cell i and 0 elsewhere: an n-cell vector."""
    out = []
    for i in range(n):
        cells = "Nil"
        for j in reversed(range(n)):
            cells = f"Cons<Lit<{3 if j == i else 0}>, {cells}>"
        out.append(f"type AL{i} = Aff<Lit<3>, {cells}>;")
    t = "AL0"
    for i in range(1, n):
        t = f"GAdd<{t}, AL{i}>"
    out.append(f"type AffTerm = {t};")
    return "\n".join(out) + "\n"


VARIANTS = {
    # C1: neither tower is forced.
    "neither": lambda n: gen_corner(n) + gen_affine_types(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 { a }
""",
    # P1: the corner tower's const is read.
    "corner": lambda n: gen_corner(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 { if const { <CornerTerm as IvAll>::OK } { a } else { a ^ 1 } }
""",
    # C2: the affine TYPES are constructed and no const of theirs is read.
    "affine-types-only": lambda n: gen_affine_types(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 { let _x: Option<AffTerm> = None; a }
""",
    # P2: the affine tower's const is read.
    "affine": lambda n: gen_affine_types(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 { if const { <AffTerm as GAll>::OK } { a } else { a ^ 1 } }
""",
    # P3: both towers are read, which is `112` section 9's disjunction.
    "disjunction": lambda n: gen_corner(n) + gen_affine_types(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 {
    if const { <CornerTerm as IvAll>::OK || <AffTerm as GAll>::OK } { a } else { a ^ 1 }
}
""",
    # P4: the structural predicate fires on a fold, so only the corner tower is
    # read. The affine types are still DECLARED, which is the honest shape: a
    # library ships both and instantiates one.
    "selection": lambda n: gen_corner(n) + gen_affine_types(n) + """
const PREDICATE_FIRES: bool = true; // a fold has distinct leaves and no multiply
#[no_mangle] pub fn f(a: u8) -> u8 {
    if const { if PREDICATE_FIRES { <CornerTerm as IvAll>::OK } else { <AffTerm as GAll>::OK } } { a }
    else { a ^ 1 }
}
""",
    # ---- Added after the first run, which found the wall is in TRAIT SOLVING
    # ---- rather than in const evaluation, and that `selection` did not dodge it.
    # ---- These four ask where exactly the obligation is forced and whether the
    # ---- one-attribute remedy moves the wall.
    #
    # Does DEFINING an impl whose const body reads the affine tower force it,
    # even when nothing reads that const?
    "affine-impl-only": lambda n: gen_affine_types(n) + """
struct AffineArm;
trait Chk { const OK: bool; }
impl Chk for AffineArm { const OK: bool = <AffTerm as GAll>::OK; }
#[no_mangle] pub fn f(a: u8) -> u8 { a }
""",
    # Selection through an ASSOCIATED TYPE, so the unselected arm is never named
    # at the site. If the obligation is proven only after normalisation, this
    # compiles where `selection` refuses.
    "selection-assoc": lambda n: gen_corner(n) + gen_affine_types(n) + """
trait Chk { const OK: bool; }
struct CornerArm;
impl Chk for CornerArm { const OK: bool = <CornerTerm as IvAll>::OK; }
struct AffineArm;
impl Chk for AffineArm { const OK: bool = <AffTerm as GAll>::OK; }
trait Pick { type Arm; }
struct Cond<const C: bool>;
impl Pick for Cond<true> { type Arm = CornerArm; }
impl Pick for Cond<false> { type Arm = AffineArm; }
const PREDICATE_FIRES: bool = true;
#[no_mangle] pub fn f(a: u8) -> u8 {
    if const { <<Cond<PREDICATE_FIRES> as Pick>::Arm as Chk>::OK } { a } else { a ^ 1 }
}
""",
    # The one-attribute remedy, on the spelling that refused.
    # Selection BEFORE instantiation: the affine machinery is fully present in
    # the crate and is never instantiated at this term. If this compiles where
    # `selection` refuses, the composition is real and the const `if` at the use
    # site was simply the wrong place to put it.
    "selection-early": lambda n: gen_corner(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 { if const { <CornerTerm as IvAll>::OK } { a } else { a ^ 1 } }
// the affine machinery is in the prelude above and is never instantiated here
""",
    "affine-limit-1024": lambda n: "#![recursion_limit = \"1024\"]\n" + gen_affine_types(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 { if const { <AffTerm as GAll>::OK } { a } else { a ^ 1 } }
""",
    "disjunction-limit-1024": lambda n: "#![recursion_limit = \"1024\"]\n" + gen_corner(n) + gen_affine_types(n) + """
#[no_mangle] pub fn f(a: u8) -> u8 {
    if const { <CornerTerm as IvAll>::OK || <AffTerm as GAll>::OK } { a } else { a ^ 1 }
}
""",
}


def compile_variant(name, n, tmp):
    src = tmp / f"{name}_{n}.rs"
    body = VARIANTS[name](n)
    if body.startswith("#!["):
        head, rest = body.split("\n", 1)
        src.write_text(head + "\n" + PRELUDE + rest)
    else:
        src.write_text(PRELUDE + body)
    r = subprocess.run(
        ["rustc", "--edition", "2021", "--crate-type", "lib", "-O", "--emit", "metadata",
         "-o", str(tmp / f"{name}_{n}.rmeta"), str(src)],
        capture_output=True, text=True,
    )
    err = r.stderr
    tag = "ok"
    if r.returncode != 0:
        first = [l for l in err.splitlines() if l.startswith("error")]
        tag = first[0][:96] if first else "error (no error line)"
    return r.returncode, tag


def main():
    print("=" * 96)
    print("p9. Pricing the two spellings without a clock")
    print("=" * 96)
    print()
    print(subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip())
    print()
    print("ONE. The exact counts, which are arithmetic rather than a measurement.")
    print()
    print(f"  {'fold L':>7} {'nodes':>7} {'corner consts':>15} {'affine cells':>14} {'ratio':>8}")
    for L in (2, 4, 8, 16, 32, 64, 256, 1024):
        nodes = 2 * L - 1
        print(f"  {L:>7} {nodes:>7} {2 * nodes:>15} {L * nodes:>14} {L / 2:>8.1f}")
    print()
    print("  `111` F111-18 reports the affine STATE as one coefficient per leaf,")
    print("  which is the state at ONE node. The tower carries a vector at every")
    print("  node, so the compile-side quantity is the product.")

    print()
    print("TWO. The refusal, which is deterministic and is not a duration.")
    print()
    lengths = [2, 8, 16, 24, 32, 48, 64, 96, 128]
    with tempfile.TemporaryDirectory() as td:
        tmp = Path(td)
        print(f"  {'variant':<20} " + " ".join(f"{L:>5}" for L in lengths))
        firstfail = {}
        for name in VARIANTS:
            row = []
            for L in lengths:
                rc, tag = compile_variant(name, L, tmp)
                row.append("ok" if rc == 0 else "FAIL")
                if rc != 0 and name not in firstfail:
                    firstfail[name] = (L, tag)
            print(f"  {name:<20} " + " ".join(f"{c:>5}" for c in row), flush=True)
        print()
        if firstfail:
            print("  The first refusal for each variant, with the diagnostic:")
            for name, (L, tag) in firstfail.items():
                print(f"    {name:<20} first fails at L = {L}")
                print(f"      {tag}")
        else:
            print("  No variant refused at any length attempted. That is itself the")
            print("  answer: the compile-side difference does not reach a limit here,")
            print("  so the only remaining unit is a duration and the question stays")
            print("  unpriced.")

    print()
    print("=" * 96)
    print(
        """
  READING IT

  C1 is the `neither` row. It must be `ok` throughout, or the refusals below are
  about the generator's source size rather than about a const tower.

  C2 is the `affine-types-only` row against the `affine` row. If types-only
  compiles where affine refuses, the cost is const EVALUATION. If both refuse at
  the same length, the cost is type CONSTRUCTION, which is a different answer and
  changes what a design can do about it.

  What is NOT established here, stated plainly: no duration, for any spelling, at
  any length. Every figure above is a count or a compiler verdict. The harness
  cannot take the duration and nothing else may be called a measurement, so the
  duration remains unpriced and the shape of the instrument that would price it
  is a compile-time mode the bench harness does not have.
"""
    )


if __name__ == "__main__":
    main()
