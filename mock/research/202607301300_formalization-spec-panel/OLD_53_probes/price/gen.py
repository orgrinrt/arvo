#!/usr/bin/env python3
"""Aggregate compile-cost sweep: the multiplication nobody ran.

The review's per-composition figures (49:327-335) are milliseconds and
favourable, and files 41/42 already measured linearity at scale in their
own CSVs without any prose ever stating the aggregate. This sweep fills
the profile gaps those CSVs leave:

  dyadic      full BiasMulPP over dyadic magnitudes (2^a / 2^b), the shape
              every shipped fixed-point numeral actually uses (49:331-333
              quotes 1.55 ms/composition for this case; here it is priced
              at scale, on the SEALED tower, min-of-2)
  distinct16  16-bit random quads, sealed tower (validation spot against
              file 41's bias_full 16-bit slope of ~159 ms/composition,
              which was measured on the unsealed tower)
  repeat16    N instantiation SITES over exactly 5 distinct 16-bit
              compositions (aliases + re-assertions), against distinct16
              at the same N: does the solver cache make repeated
              compositions free, i.e. is the aggregate driven by DISTINCT
              compositions or by call sites?
  headline    the design's own headline constants (49:583-585: 44100,
              48000, 4096): div_exact-shaped compositions Q0.15 * (1/C),
              denominators up to 32768*48000 ~ 2^30.6, the band 49 prices
              as "roughly an order of magnitude more" without a number.

Same discipline as 41/42's sweeps: every instantiation forced by a const
assertion against a Python-computed reduced Fraction, `rustc --edition
2021 --crate-type lib --emit=metadata`, trait-solve-only, no codegen.
Tower: the sealed copy (42's fix, carried unmodified through 46/50/52).
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


def quads_random(count: int, bits: int, seed: int = 20260803):
    rng = random.Random(seed + bits * 1000 + count)
    lo, hi = 1 << (bits - 1), (1 << bits) - 1
    out, seen = [], set()
    while len(out) < count:
        n1, d1, n2, d2 = (rng.randint(lo, hi) for _ in range(4))
        key = (n1, d1, n2, d2)
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
    return out


def quads_dyadic(count: int, seed: int = 20260803):
    rng = random.Random(seed + count)
    out, seen = [], set()
    while len(out) < count:
        key = tuple(1 << rng.randint(0, 15) for _ in range(4))
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
    return out


HDR = """#![allow(dead_code)]
#![no_std]
#[path = "../../vu_bias_sealed.rs"]
mod bias;
use bias::nat::{H, I, O, Pos};
use bias::{Bias, BiasMulPP};
"""

HEADLINE = [
    # (name, quantum denominator, constant) : (1/qd) * (1/c), the
    # div_exact bias path B * (cd/cn) with cd = 1, B = 1/qd.
    ("q15_by_44100", 32768, 44100),
    ("q15_by_48000", 32768, 48000),
    ("q15_by_4096", 32768, 4096),
]

# The chained case: a Q0.15 bias through a 48000/44100 sample-rate
# conversion, (1/32768) * (48000/44100). Both magnitudes large and
# sharing factors, so the Stein gcd does real work: the shape that
# actually inhabits the expensive band.
HEADLINE_CHAINED = [(1, 32768, 48000, 44100)]


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


def emit(kind: str, count: int) -> str:
    body = [HDR]
    if kind == "dyadic":
        for i, q in enumerate(quads_dyadic(count)):
            body.append(one_comp(i, *q))
    elif kind == "distinct16":
        for i, q in enumerate(quads_random(count, 16)):
            body.append(one_comp(i, *q))
    elif kind == "repeat16":
        base = quads_random(5, 16)
        for i in range(count):
            body.append(one_comp(i, *base[i % 5]))
    elif kind == "headline":
        # count is ignored; the profile is the three named constants.
        for i, (_, qd, c) in enumerate(HEADLINE):
            body.append(one_comp(i, 1, qd, 1, c))
    elif kind == "chained":
        for i, q in enumerate(HEADLINE_CHAINED):
            body.append(one_comp(i, *q))
    else:
        raise SystemExit("unknown kind %r" % kind)
    return "".join(body)


if __name__ == "__main__":
    kind, count, out_path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    with open(out_path, "w") as f:
        f.write(emit(kind, count))
