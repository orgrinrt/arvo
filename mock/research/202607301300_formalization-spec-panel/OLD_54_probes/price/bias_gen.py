#!/usr/bin/env python3
"""Where a decimal numeral's own bias compositions land on file 53's cost curve.

File 53 measured two ends of the compile-cost model (`53:129-137`): dyadic magnitudes at
2.1 ms per composition, 16-bit random rational pairs at 143 ms. Radix ten was not in that
sweep, and the brief for file 54 says to price it rather than assume it, because powers of
ten are not dyadic and could plausibly land on the expensive side.

Three decimal-shaped profiles, against the two file 53 already priced as controls, so the
comparison is against a number this host produced rather than against a quoted one:

  dec_quantum   (1/10^a) * (1/10^b), unit numerators. The currency and sensor-scale shape:
                a decimal fixed-point quantum composed with another.
  dec_slope     (n1/10^a) * (n2/10^b) with small non-unit numerators. The MATLAB
                slope-and-bias shape written in decimal, where the gcd has real work.
  dec_wide      (n1/d1) * (n2/d2) with all four drawn from powers of ten times small
                factors, so both magnitudes are large and share factors.

The generator, the forcing discipline (every composition asserted against a Python-computed
reduced Fraction so nothing is an inert alias) and the build shape are file 53's, reused
rather than reinvented. The tower is the sealed copy carried through 42, 46, 50, 52.
"""

import random
import sys
from fractions import Fraction


def vu_pos(n: int) -> str:
    if n == 1:
        return "H"
    if n % 2 == 0:
        return "O<%s>" % vu_pos(n // 2)
    return "I<%s>" % vu_pos(n // 2)


HDR = """#![allow(dead_code)]
#[path = "../../vu_bias_sealed_adj.rs"]
mod bias;
use bias::nat::{H, I, O, Pos};
use bias::{Bias, BiasMulPP};
"""


def one_comp(i, n1, d1, n2, d2):
    prod = Fraction(n1, d1) * Fraction(n2, d2)
    return (
        "pub type N1_%d = %s;\npub type D1_%d = %s;\n"
        "pub type N2_%d = %s;\npub type D2_%d = %s;\n"
        "type P%d = BiasMulPP<N1_%d, D1_%d, N2_%d, D2_%d>;\n"
        "const _: () = assert!(<P%d as Bias>::NUM == %d);\n"
        "const _: () = assert!(<P%d as Bias>::DEN == %d);\n"
        % (
            i, vu_pos(n1), i, vu_pos(d1), i, vu_pos(n2), i, vu_pos(d2),
            i, i, i, i, i, i, prod.numerator, i, prod.denominator,
        )
    )


def quads(kind: str, count: int, seed: int = 20260804):
    rng = random.Random(seed + count)
    out, seen = [], set()
    while len(out) < count:
        if kind == "dec_quantum":
            key = (1, 10 ** rng.randint(1, 9), 1, 10 ** rng.randint(1, 9))
        elif kind == "dec_quantum6":
            key = (1, 10 ** rng.randint(1, 6), 1, 10 ** rng.randint(1, 6))
        elif kind == "dec_slope":
            key = (
                rng.randint(1, 99),
                10 ** rng.randint(1, 5),
                rng.randint(1, 99),
                10 ** rng.randint(1, 5),
            )
        elif kind == "dec_wide":
            key = (
                rng.randint(1, 999) * 10 ** rng.randint(0, 3),
                10 ** rng.randint(3, 6),
                rng.randint(1, 999) * 10 ** rng.randint(0, 3),
                10 ** rng.randint(3, 6),
            )
        elif kind == "dyadic":
            key = tuple(1 << rng.randint(0, 15) for _ in range(4))
        elif kind == "distinct16":
            lo, hi = 1 << 15, (1 << 16) - 1
            key = tuple(rng.randint(lo, hi) for _ in range(4))
        else:
            raise SystemExit("unknown kind %r" % kind)
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
    return out


if __name__ == "__main__":
    kind, count, out_path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    body = [HDR]
    for i, q in enumerate(quads(kind, count)):
        body.append(one_comp(i, *q))
    with open(out_path, "w") as f:
        f.write("".join(body))
