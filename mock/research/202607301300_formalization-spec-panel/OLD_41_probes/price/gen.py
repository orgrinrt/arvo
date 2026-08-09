#!/usr/bin/env python3
"""Generate the compile-cost sweep sources for BiasMul.

Two shapes, same value quadruples in each:

  bias_mag   N instantiations of BiasMagN/BiasMagD alone (magnitude only:
             PMul then Reduce, no sign)
  bias_full  N instantiations of BiasMulPP (magnitude plus sign wrap), the
             full `bias = B1 * B2` a consumer actually names

Every instantiation is forced by a const assertion against a
Python-computed answer (Fraction, reduced), so nothing is elided and
correctness is checked at the same time the cost is measured. Operand
numerator and denominator are each drawn independently at the given bit
width, which is a harder distribution than dyadic biases (matching file
36's own sweep discipline: worst-case pairs, not the common case).
"""
import math
import random
import sys
from fractions import Fraction


def vu_pos(n: int) -> str:
    if n == 1:
        return "H"
    if n % 2 == 0:
        return "O<%s>" % vu_pos(n // 2)
    return "I<%s>" % vu_pos(n // 2)


def quads(count: int, bits: int, seed: int = 20260803):
    rng = random.Random(seed + bits * 1000 + count)
    lo, hi = 1 << (bits - 1), (1 << bits) - 1
    out = []
    seen = set()
    while len(out) < count:
        n1, d1, n2, d2 = (rng.randint(lo, hi) for _ in range(4))
        key = (n1, d1, n2, d2)
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
    return out


HDR = """#![allow(dead_code)]
#![no_std]
#[path = "../../vu_bias.rs"]
mod bias;
use bias::nat::{H, I, O, Pos};
use bias::{Bias, BiasMagD, BiasMagN, BiasMulPP};
"""


def emit(kind: str, count: int, bits: int) -> str:
    qs = quads(count, bits)
    body = [HDR]
    for i, (n1, d1, n2, d2) in enumerate(qs):
        prod = Fraction(n1, d1) * Fraction(n2, d2)
        rn, rd = prod.numerator, prod.denominator
        body.append(
            "pub type N1_%d = %s;\npub type D1_%d = %s;\n"
            "pub type N2_%d = %s;\npub type D2_%d = %s;\n"
            % (i, vu_pos(n1), i, vu_pos(d1), i, vu_pos(n2), i, vu_pos(d2))
        )
        if kind == "bias_mag":
            body.append(
                "const _: () = assert!(<BiasMagN<N1_%d, D1_%d, N2_%d, D2_%d> as Pos>::VAL == %d);\n"
                "const _: () = assert!(<BiasMagD<N1_%d, D1_%d, N2_%d, D2_%d> as Pos>::VAL == %d);\n"
                % (i, i, i, i, rn, i, i, i, i, rd)
            )
        elif kind == "bias_full":
            body.append(
                "type P%d = BiasMulPP<N1_%d, D1_%d, N2_%d, D2_%d>;\n"
                "const _: () = assert!(<P%d as Bias>::NUM == %d);\n"
                "const _: () = assert!(<P%d as Bias>::DEN == %d);\n"
                % (i, i, i, i, i, i, rn, i, rd)
            )
        else:
            raise SystemExit("unknown kind " + kind)
    return "".join(body)


if __name__ == "__main__":
    kind, count, bits, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    with open(out, "w") as f:
        f.write(emit(kind, count, bits))
