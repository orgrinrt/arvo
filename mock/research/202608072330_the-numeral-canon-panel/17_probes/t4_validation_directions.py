#!/usr/bin/env python3
"""t4. Clause three says "then validate". What would certify that, and in which direction?

A validation claim is quantified over TWO sets, and a suite built from one of them says
nothing about the other:

  under-refusal  the design accepts a declaration it cannot serve
  over-refusal   the design refuses a declaration it should serve

A compile-fail suite (the usual instrument) tests the first. A "these all compile" suite
tests the second. Neither tests both, and the second is the one nobody writes, because a
test for a declaration nobody thought of is a test nobody thinks to write.

Method: take a typestate with a stated admissibility rule, seed it with one validation
defect at a time, compile every declaration in a bounded domain, and compare the accept
set against an independently written predicate. Report both error directions separately.

  python3 t4_validation_directions.py

Toolchain: rustc +nightly-2026-05-28, pinned. No features anywhere.
Nothing here is timed. Counts are counts.

Spike. Presume it flawed. The admissibility rule is one I chose for the probe and is not
the design's; what the probe checks is that the two error directions are separately
observable and separately missed, not what arvo should admit.
"""
import subprocess
import sys
import tempfile
import os

RUSTC = ["rustc", "+nightly-2026-05-28", "--edition", "2021",
         "--crate-type", "lib", "--emit", "metadata"]

# The independently written admissibility predicate. Written from the intent:
# a declaration is admissible when it names at least one bit and fits the widest rung
# the design dispatches through.
CEILING = 64


def spec_admissible(w: int) -> bool:
    return 1 <= w <= CEILING


# The typestate under test. The rung is carried in the type, per the panel's structural
# keying result; a where-clause bound does not disambiguate impls (E0119).
TEMPLATE = r"""
#![no_std]
#![allow(dead_code)]

pub trait Rung {{ type Carrier: Copy; }}
pub struct R8; pub struct R16; pub struct R32; pub struct R64;
impl Rung for R8  {{ type Carrier = u8;  }}
impl Rung for R16 {{ type Carrier = u16; }}
impl Rung for R32 {{ type Carrier = u32; }}
impl Rung for R64 {{ type Carrier = u64; }}

/// Admissible<W> is implemented exactly for the widths the design serves. Its presence
/// is the validation: a declaration whose width has no impl does not compile.
pub trait Admissible {{ type At: Rung; }}
pub struct Decl<const W: usize>;

{impls}

pub struct Numeral<const W: usize>(<Decl<W> as Admissible>::At)
where
    Decl<W>: Admissible;

pub fn use_it(_: Numeral<{w}>) where Decl<{w}>: Admissible {{}}
"""


def impls_for(defect: str) -> str:
    """Emit one Admissible impl per admitted width, per the defect under test."""
    out = []
    for w in range(0, 130):
        admit = 1 <= w <= CEILING
        if defect == "under_zero" and w == 0:
            admit = True                      # accepts a zero-width numeral
        if defect == "under_over_ceiling" and w in (65, 66):
            admit = True                      # accepts past the widest rung
        if defect == "over_off_by_one" and w == CEILING:
            admit = False                     # refuses the top rung
        if defect == "over_boundary_band" and w in (33, 34, 35):
            admit = False                     # refuses a band inside the served range
        if not admit:
            continue
        rung = "R8" if w <= 8 else "R16" if w <= 16 else "R32" if w <= 32 else "R64"
        out.append(f"impl Admissible for Decl<{w}> {{ type At = {rung}; }}")
    return "\n".join(out)


def compiles(w: int, impls: str, tmp: str) -> bool:
    src = os.path.join(tmp, f"d{w}.rs")
    with open(src, "w") as f:
        f.write(TEMPLATE.format(impls=impls, w=w))
    r = subprocess.run(RUSTC + [src, "--out-dir", tmp],
                       capture_output=True, text=True)
    return r.returncode == 0


DEFECTS = [
    ("none (control)", "none"),
    ("under-refusal: admits W = 0", "under_zero"),
    ("under-refusal: admits W = 65, 66", "under_over_ceiling"),
    ("over-refusal: refuses the top rung W = 64", "over_off_by_one"),
    ("over-refusal: refuses W = 33..35", "over_boundary_band"),
]

# the domain the probe sweeps. Stated, because a coverage claim without its domain is
# not a coverage claim.
DOMAIN = list(range(0, 70))


def main():
    print("t4. the two directions a validation claim is quantified over")
    print(f"    domain swept: W in {DOMAIN[0]}..={DOMAIN[-1]}, {len(DOMAIN)} declarations")
    print(f"    the independently written rule: admissible iff 1 <= W <= {CEILING}")
    print()
    print(f"{'defect':44}{'under':>8}{'over':>8}{'caught by':>28}")
    print("-" * 88)

    with tempfile.TemporaryDirectory() as tmp:
        for label, defect in DEFECTS:
            impls = impls_for(defect)
            under, over = [], []
            for w in DOMAIN:
                ok = compiles(w, impls, tmp)
                want = spec_admissible(w)
                if ok and not want:
                    under.append(w)
                if want and not ok:
                    over.append(w)
            # which suite shape would have observed it
            seen_by = []
            if under:
                seen_by.append("compile-fail suite")
            if over:
                seen_by.append("these-all-compile suite")
            print(f"{label:44}{len(under):>8}{len(over):>8}{', '.join(seen_by) or 'nothing':>28}")

    print()
    print("reading:")
    print("  A compile-fail suite observes the under-refusal rows and is blind to the")
    print("  over-refusal rows. A these-all-compile suite is blind the other way. Neither")
    print("  shape observes both, and only a sweep against a separately written predicate")
    print("  does, because only the predicate knows which widths SHOULD have compiled.")
    print()
    print("  The over-refusal rows are the ones with no natural test author: a consumer")
    print("  who never wrote W = 34 never files a bug, and the suite stays green because")
    print("  every declaration it contains is one somebody thought of.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
