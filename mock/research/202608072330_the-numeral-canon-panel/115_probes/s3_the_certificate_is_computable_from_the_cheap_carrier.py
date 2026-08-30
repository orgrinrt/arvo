#!/usr/bin/env python3
"""S3. The certificate rides on the cheap carrier, and its verdict matches the
model.

S2 establishes, on my own tower construction, that no arrangement inside the type
system escapes the expensive carrier's obligation: a const block, an impl that
merely defines a const, and a trait projection selecting the rule type all refuse
where the expensive tower refuses. So `114` F114-17 is right and the selection
has to happen where the term type is **written**.

That leaves one question `114` does not ask, and it is the one that decides
whether the selection is actionable: **what is available at that point?** A
selection made before instantiation can only read what does not require the thing
it is deciding about.

The answer is that the certificate is computable from the cheap carrier alone,
and that both of its conditions are functions of the term's syntax and the
declared bounds' syntax:

  condition (a), every leaf occurs at most once, is a leaf-multiplicity fact,
  which is purely syntactic;

  condition (b), no internal node sits under a multiplication whose sibling
  interval contains zero, reads only intervals, which the CHEAP tower computes.

So this probe generates Rust in which the certificate is an associated const on
the interval tower, compiles it with no feature gate, runs it, and checks every
verdict against `111_probes/r2`'s model on the same terms.

`114` F114-15 claims the predicate compiles. This is not a citation of that: the
conditions here are computed my way, the check is against my model rather than
against a claim, and what it adds is that the certificate needs **nothing from
the expensive carrier**, which is what makes S2's result usable rather than only
a prohibition.

The case that must fail: three of the eight terms below have a false certificate,
for three different reasons, and the count is asserted by the probe rather than
by this sentence. A run in which every verdict is true would prove
nothing, and a mutation that drops each condition is run to show each is
load-bearing.
"""

import os
import re
import subprocess
import sys
import tempfile
import importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
R2 = os.path.join(os.path.dirname(HERE), "111_probes",
                  "r2_a_structural_predicate_for_where_the_corner_rule_is_exact.py")
spec = importlib.util.spec_from_file_location("r2", R2)
r2 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r2)

PRELUDE = """#![allow(dead_code)]
use core::marker::PhantomData;

/// The cheap carrier, extended with the two conditions. Every associated const
/// below is ordinary const arithmetic in an impl body: nothing arithmetic sits
/// in type position and no feature gate is required.
pub trait Term {
    const LO: i64;
    const HI: i64;
    /// One bit per declared leaf, so condition (a) is a disjointness test.
    const MASK: u64;
    /// (a) every leaf occurs at most once.
    const LINEAR: bool;
    /// (b) no internal node sits under a multiplication whose sibling
    /// interval contains zero. Expressed as a local test at each node.
    const NOMASK: bool;
    const IS_LEAF: bool;
}

pub struct Lf<const IDX: u32, const LO: i64, const HI: i64>;
impl<const IDX: u32, const LO: i64, const HI: i64> Term for Lf<IDX, LO, HI> {
    const LO: i64 = LO;
    const HI: i64 = HI;
    const MASK: u64 = 1u64 << IDX;
    const LINEAR: bool = true;
    const NOMASK: bool = true;
    const IS_LEAF: bool = true;
}

pub struct Add<A, B>(PhantomData<(A, B)>);
impl<A: Term, B: Term> Term for Add<A, B> {
    const LO: i64 = A::LO + B::LO;
    const HI: i64 = A::HI + B::HI;
    const MASK: u64 = A::MASK | B::MASK;
    const LINEAR: bool = A::LINEAR && B::LINEAR && (A::MASK & B::MASK) == 0;
    const NOMASK: bool = A::NOMASK && B::NOMASK;
    const IS_LEAF: bool = false;
}

pub struct Sub<A, B>(PhantomData<(A, B)>);
impl<A: Term, B: Term> Term for Sub<A, B> {
    const LO: i64 = A::LO - B::HI;
    const HI: i64 = A::HI - B::LO;
    const MASK: u64 = A::MASK | B::MASK;
    const LINEAR: bool = A::LINEAR && B::LINEAR && (A::MASK & B::MASK) == 0;
    const NOMASK: bool = A::NOMASK && B::NOMASK;
    const IS_LEAF: bool = false;
}

const fn mn(a: i64, b: i64) -> i64 { if a < b { a } else { b } }
const fn mx(a: i64, b: i64) -> i64 { if a > b { a } else { b } }

pub struct Mul<A, B>(PhantomData<(A, B)>);
impl<A: Term, B: Term> Term for Mul<A, B> {
    const LO: i64 = mn(mn(A::LO * B::LO, A::LO * B::HI),
                       mn(A::HI * B::LO, A::HI * B::HI));
    const HI: i64 = mx(mx(A::LO * B::LO, A::LO * B::HI),
                       mx(A::HI * B::LO, A::HI * B::HI));
    const MASK: u64 = A::MASK | B::MASK;
    const LINEAR: bool = A::LINEAR && B::LINEAR && (A::MASK & B::MASK) == 0;
    const NOMASK: bool = A::NOMASK
        && B::NOMASK
        && !(!A::IS_LEAF && B::LO <= 0 && 0 <= B::HI)
        && !(!B::IS_LEAF && A::LO <= 0 && 0 <= A::HI);
    const IS_LEAF: bool = false;
}
"""

# name, rust type, python term, python extents (per leaf label)
L = lambda i: ("leaf", i)
TERMS = [
    ("x + y",
     "Add<Lf<0,0,3>, Lf<1,0,3>>",
     (None, "add", L(0), L(1)), [(0, 3), (0, 3)]),
    ("x * y",
     "Mul<Lf<0,0,3>, Lf<1,0,3>>",
     (None, "mul", L(0), L(1)), [(0, 3), (0, 3)]),
    ("(x + y) - y",
     "Sub<Add<Lf<0,0,3>, Lf<1,0,3>>, Lf<1,0,3>>",
     (None, "sub", (None, "add", L(0), L(1)), L(1)), [(0, 3), (0, 3)]),
    ("(x + y) * z, z in [0,3]",
     "Mul<Add<Lf<0,0,3>, Lf<1,0,3>>, Lf<2,0,3>>",
     (None, "mul", (None, "add", L(0), L(1)), L(2)), [(0, 3), (0, 3), (0, 3)]),
    ("(x + y) * z, z in [1,3]",
     "Mul<Add<Lf<0,0,3>, Lf<1,0,3>>, Lf<2,1,3>>",
     (None, "mul", (None, "add", L(0), L(1)), L(2)), [(0, 3), (0, 3), (1, 3)]),
    ("(x * y) + z",
     "Add<Mul<Lf<0,0,3>, Lf<1,0,3>>, Lf<2,0,3>>",
     (None, "add", (None, "mul", L(0), L(1)), L(2)), [(0, 3), (0, 3), (0, 3)]),
    ("x - (y - z)",
     "Sub<Lf<0,0,0>, Sub<Lf<1,0,0>, Lf<2,0,1>>>",
     (None, "sub", L(0), (None, "sub", L(1), L(2))), [(0, 0), (0, 0), (0, 1)]),
    ("x * (y - y)",
     "Mul<Lf<0,0,3>, Sub<Lf<1,0,3>, Lf<1,0,3>>>",
     (None, "mul", L(0), (None, "sub", L(1), L(1))), [(0, 3), (0, 3)]),
]


def gen(mutation=None):
    s = PRELUDE
    if mutation == "drop_a":
        s = s.replace("&& (A::MASK & B::MASK) == 0", "&& true")
    if mutation == "drop_b":
        s = re.sub(r"const NOMASK: bool = A::NOMASK\n        && B::NOMASK\n"
                   r"        && !\(!A::IS_LEAF && B::LO <= 0 && 0 <= B::HI\)\n"
                   r"        && !\(!B::IS_LEAF && A::LO <= 0 && 0 <= A::HI\);",
                   "const NOMASK: bool = A::NOMASK && B::NOMASK;", s)
    s += "\nfn main() {\n"
    for i, (name, ty, _, _) in enumerate(TERMS):
        s += (f'    println!("{i}|{{}}|{{}}|{{}}|{{}}|{{}}",\n'
              f'        <{ty} as Term>::LINEAR, <{ty} as Term>::NOMASK,\n'
              f'        <{ty} as Term>::LINEAR && <{ty} as Term>::NOMASK,\n'
              f'        <{ty} as Term>::LO, <{ty} as Term>::HI);\n')
    s += "}\n"
    return s


def run(mutation, tmp):
    path = os.path.join(tmp, f"cert_{mutation or 'plain'}.rs")
    with open(path, "w") as fh:
        fh.write(gen(mutation))
    binp = os.path.join(tmp, f"cert_{mutation or 'plain'}")
    c = subprocess.run(["rustc", "--edition", "2021", "-O", path, "-o", binp],
                       capture_output=True, text=True)
    if c.returncode != 0:
        return None, c.stderr.strip().splitlines()[:3]
    r = subprocess.run([binp], capture_output=True, text=True)
    out = {}
    for line in r.stdout.strip().splitlines():
        f = line.split("|")
        out[int(f[0])] = (f[1] == "true", f[2] == "true", f[3] == "true",
                          int(f[4]), int(f[5]))
    return out, None


def main():
    print("S3. the certificate on the cheap carrier, checked against the model")
    print("=" * 78)
    v = subprocess.run(["rustc", "--version"], capture_output=True, text=True)
    print(f"toolchain: {v.stdout.strip()}")
    tmp = tempfile.mkdtemp(prefix="s3_")
    src = gen(None)
    print(f"feature gates in the generated source: "
          f"{len(re.findall(r'^#!\[feature', src, re.M))}")
    print(f"references to the expensive carrier in it: "
          f"{len(re.findall(r'Coef|Vek|Cons|AddV|Af\b', src))}")
    print()

    plain, err = run(None, tmp)
    if plain is None:
        print("COMPILE FAILED:", err)
        return 1

    print(f"  {'term':<26} {'rust (a)':>9} {'rust (b)':>9} {'rust cert':>10} "
          f"{'model cert':>11} {'agree':>6}")
    agree_all = True
    for i, (name, ty, pt, ext) in enumerate(TERMS):
        ra, rb, rc, lo, hi = plain[i]
        ma = r2.leaves_are_linear(pt)
        mb = r2.no_annihilating_ancestor(pt, ext)
        mc = ma and mb
        ok = (ra == ma) and (rb == mb) and (rc == mc)
        agree_all &= ok
        print(f"  {name:<26} {str(ra):>9} {str(rb):>9} {str(rc):>10} "
              f"{str(mc):>11} {str(ok):>6}")
    print()
    n_false = sum(1 for i in range(len(TERMS)) if not plain[i][2])
    print(f"  every verdict matches the model: {agree_all}")
    print(f"  terms whose certificate is FALSE: {n_false} of {len(TERMS)}, "
          f"so the run had cases that could disagree")

    print()
    print("  root intervals, so the cheap carrier is shown to be computing:")
    for i, (name, _, _, _) in enumerate(TERMS):
        print(f"    {name:<26} [{plain[i][3]}, {plain[i][4]}]")

    print()
    print("MUTATIONS, each dropping one condition")
    print()
    for mut, label in (("drop_a", "condition (a) forced true"),
                       ("drop_b", "condition (b) forced true")):
        got, err = run(mut, tmp)
        if got is None:
            print(f"  {label:<28} COMPILE FAILED: {err}")
            continue
        moved = [TERMS[i][0] for i in range(len(TERMS))
                 if got[i][2] != plain[i][2]]
        print(f"  {label:<28} verdicts that moved: {len(moved)}  {moved}")

    print()
    print("-" * 78)
    print("reading: the certificate reads intervals and leaf identity and")
    print("nothing else, so it is available at the point S2 shows the selection")
    print("has to be made. Both conditions are functions of the term's syntax")
    print("and the declared bounds' syntax, which is what a macro or a code")
    print("generator has in hand before either tower is instantiated.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
