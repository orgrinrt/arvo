#!/usr/bin/env python3
"""S2. Reproduce `114` F114-17's mechanism, then test the one route it did not,
and find out where a selection between two propagation rules can actually live.

`114` F114-17 reports that `111` section 21's composition has its compile-time
motivation refuted: `selection` refuses at the same fold length as `disjunction`,
`selection-assoc` refuses identically, and an impl that merely **defines** a
const reading the expensive tower refuses too. Only selecting before
instantiation compiles as far as the cheap arm alone.

Reproducing before conceding. The tower shapes here are mine rather than `114`'s
and the recursion limit is lowered deliberately, so the fold lengths will not
match its table; what is being reproduced is the **ordering and the mechanism**,
which is what the claim is about. `114` itself records that raising the limit to
1024 moves every wall, so the wall is a limit artifact and the ordering is the
finding.

Eight variants, at each fold length:

  neither      names neither tower                         CONTROL, must compile
  cheap        requires the interval tower's obligation
  expensive    requires the coefficient tower's obligation
  const_if     a const block reading both consts
  impl_only    an impl whose const body reads the expensive tower, never read
  types_only   names both types, requires only the cheap obligation
  select_proj  a trait picking the rule type by a const bool, then reading the
               const of the projection                     the route 114 did not try
  early_cheap  the expensive machinery is present in the crate and this term is
               written with the cheap tower                114's selection-early

Predicted before running, recorded so it can be wrong:

  P1. `neither` compiles at every length.
  P2. `cheap` compiles further than `expensive`.
  P3. `const_if` and `impl_only` refuse where `expensive` refuses, which is
      `114`'s mechanism: the obligation is forced where the reading code is
      DEFINED rather than where it is evaluated.
  P4. `types_only` compiles as far as `cheap`, which is `114`'s
      affine-types-only.
  P5. `select_proj` refuses with `expensive`, because normalising the projection
      still requires an impl whose body reads the expensive tower, so routing
      the choice through a type does not escape a defect that lands at
      definition. If P5 is wrong, `114`'s conclusion needs narrowing and mine
      needs less repair than it looks.
  P6. `early_cheap` compiles as far as `cheap`.
"""

import os
import re
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
LIMIT = 32
LENGTHS = [2, 4, 6, 8, 12, 16, 24]

PRELUDE = """#![allow(dead_code)]
#![recursion_limit = "%d"]
use core::marker::PhantomData;

// ---------------- the cheap carrier: an interval per node
pub trait Iv { const LO: i64; const HI: i64; }
pub struct Leaf<const LO: i64, const HI: i64>;
impl<const LO: i64, const HI: i64> Iv for Leaf<LO, HI> {
    const LO: i64 = LO;
    const HI: i64 = HI;
}
pub struct AddI<A, B>(PhantomData<(A, B)>);
impl<A: Iv, B: Iv> Iv for AddI<A, B> {
    const LO: i64 = A::LO + B::LO;
    const HI: i64 = A::HI + B::HI;
}

// ---------------- the expensive carrier: a coefficient vector per node
pub trait Coef { const V: i64; }
pub struct K<const N: i64>;
impl<const N: i64> Coef for K<N> { const V: i64 = N; }
pub struct Plus<A, B>(PhantomData<(A, B)>);
impl<A: Coef, B: Coef> Coef for Plus<A, B> { const V: i64 = A::V + B::V; }

pub trait Vek { const RAD: i64; }
pub struct Nil;
impl Vek for Nil { const RAD: i64 = 0; }
pub struct Cons<H, T>(PhantomData<(H, T)>);
impl<H: Coef, T: Vek> Vek for Cons<H, T> {
    const RAD: i64 = (if H::V < 0 { -H::V } else { H::V }) + T::RAD;
}

pub trait AddV<O> { type Out: Vek; }
impl AddV<Nil> for Nil { type Out = Nil; }
impl<H1, T1, H2, T2> AddV<Cons<H2, T2>> for Cons<H1, T1>
where
    H1: Coef, H2: Coef, T1: Vek + AddV<T2>, T2: Vek,
{
    type Out = Cons<Plus<H1, H2>, <T1 as AddV<T2>>::Out>;
}

pub trait Af { const RAD: i64; type V: Vek; }
pub struct AfLeaf<V>(PhantomData<V>);
impl<V: Vek> Af for AfLeaf<V> { const RAD: i64 = V::RAD; type V = V; }
pub struct AddA<A, B>(PhantomData<(A, B)>);
impl<A: Af, B: Af> Af for AddA<A, B>
where
    A::V: AddV<B::V>,
{
    type V = <A::V as AddV<B::V>>::Out;
    const RAD: i64 = <<A::V as AddV<B::V>>::Out as Vek>::RAD;
}
""" % LIMIT


def cheap_term(n):
    t = "Leaf<0, 1>"
    for _ in range(n - 1):
        t = f"AddI<{t}, Leaf<0, 1>>"
    return t


def vec_lit(n, i):
    """The i-th basis vector of length n, as a type."""
    out = "Nil"
    for j in reversed(range(n)):
        out = f"Cons<K<{1 if j == i else 0}>, {out}>"
    return out


def expensive_term(n):
    t = f"AfLeaf<{vec_lit(n, 0)}>"
    for i in range(1, n):
        t = f"AddA<{t}, AfLeaf<{vec_lit(n, i)}>>"
    return t


def source(variant, n):
    s = PRELUDE
    s += f"pub type Chp = {cheap_term(n)};\n"
    s += f"pub type Exp = {expensive_term(n)};\n\n"
    if variant == "neither":
        s += "pub fn f() -> i64 { 0 }\n"
    elif variant == "cheap":
        s += "pub const R: i64 = <Chp as Iv>::HI;\n"
    elif variant == "expensive":
        s += "pub const R: i64 = <Exp as Af>::RAD;\n"
    elif variant == "const_if":
        s += ("pub const R: i64 = {\n"
              "    if <Chp as Iv>::HI < 1000 { <Chp as Iv>::HI }\n"
              "    else { <Exp as Af>::RAD }\n"
              "};\n")
    elif variant == "impl_only":
        s += ("pub trait Q { const X: i64; }\n"
              "pub struct S;\n"
              "impl Q for S { const X: i64 = <Exp as Af>::RAD; }\n"
              "pub const R: i64 = <Chp as Iv>::HI;\n")
    elif variant == "types_only":
        s += ("pub type Both = (Chp, Exp);\n"
              "pub const R: i64 = <Chp as Iv>::HI;\n")
    elif variant == "select_proj":
        s += ("pub trait Rule { const R: i64; }\n"
              "pub struct ChpW;\n"
              "impl Rule for ChpW { const R: i64 = <Chp as Iv>::HI; }\n"
              "pub struct ExpW;\n"
              "impl Rule for ExpW { const R: i64 = <Exp as Af>::RAD; }\n"
              "pub trait Pick { type Out: Rule; }\n"
              "pub struct Cond<const B: bool>;\n"
              "impl Pick for Cond<true> { type Out = ChpW; }\n"
              "impl Pick for Cond<false> { type Out = ExpW; }\n"
              "pub const R: i64 =\n"
              "    <<Cond<{ <Chp as Iv>::HI < 1000 }> as Pick>::Out as Rule>::R;\n")
    elif variant == "early_cheap":
        # the expensive machinery is fully present in the crate (the PRELUDE and
        # the `Exp` alias above) and this term is simply written with the cheap
        # tower, which is 114's selection-early.
        s += "pub const R: i64 = <Chp as Iv>::HI;\n"
    else:
        raise ValueError(variant)
    return s


VARIANTS = ["neither", "cheap", "expensive", "const_if", "impl_only",
            "types_only", "select_proj", "early_cheap"]


def compile_one(variant, n, tmp):
    path = os.path.join(tmp, f"{variant}_{n}.rs")
    with open(path, "w") as fh:
        fh.write(source(variant, n))
    r = subprocess.run(
        ["rustc", "--edition", "2021", "--crate-type", "lib",
         "--emit", "metadata", "-o", os.path.join(tmp, "out.rmeta"), path],
        capture_output=True, text=True)
    if r.returncode == 0:
        return "ok", ""
    m = re.search(r"^error(\[[A-Z0-9]+\])?: (.*)$", r.stderr, re.M)
    code = m.group(1) or "" if m else ""
    msg = (m.group(2) if m else r.stderr.strip().splitlines()[0])[:58]
    return "FAIL", f"error{code}: {msg}"


def main():
    print("S2. where a selection between two propagation rules can live")
    print("=" * 78)
    v = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
    print(f"toolchain: {v.stdout.strip()}")
    print(f"recursion_limit lowered to {LIMIT} so the walls arrive at small "
          f"fold lengths;")
    print("114's absolute lengths are not being reproduced, its ordering is.")
    print()

    hdr = "  " + f"{'variant':<14}" + "".join(f"{n:>6}" for n in LENGTHS)
    print(hdr)
    first_fail = {}
    diags = {}
    tmp = tempfile.mkdtemp(prefix="s2_")
    for variant in VARIANTS:
        row = f"  {variant:<14}"
        for n in LENGTHS:
            st, msg = compile_one(variant, n, tmp)
            row += f"{st:>6}"
            if st == "FAIL" and variant not in first_fail:
                first_fail[variant] = n
                diags[variant] = msg
        print(row)

    print()
    print("first refusal, with the diagnostic:")
    for variant in VARIANTS:
        if variant in first_fail:
            print(f"  {variant:<14} L = {first_fail[variant]:<4} {diags[variant]}")
        else:
            print(f"  {variant:<14} compiles at every length swept")

    print()
    print("-" * 78)
    print("predictions, checked:")
    ok = lambda b: "CONFIRMED" if b else "REFUTED"
    fc = first_fail.get("cheap")
    fe = first_fail.get("expensive")
    print(f"  P1 neither compiles everywhere              {ok('neither' not in first_fail)}")
    print(f"  P2 cheap outlasts expensive                 "
          f"{ok(fe is not None and (fc is None or fc > fe))}")
    print(f"  P3 const_if and impl_only refuse with expensive  "
          f"{ok(first_fail.get('const_if') == fe and first_fail.get('impl_only') == fe)}")
    print(f"  P4 types_only matches cheap                 "
          f"{ok(first_fail.get('types_only') == fc)}")
    print(f"  P5 select_proj refuses with expensive       "
          f"{ok(first_fail.get('select_proj') == fe)}")
    print(f"  P6 early_cheap matches cheap                "
          f"{ok(first_fail.get('early_cheap') == fc)}")


if __name__ == "__main__":
    sys.exit(main())
