#!/usr/bin/env python3
"""Compile-cost comparison for two questions this dispatch answers.

  alias          BiasMulPP (file 41's bare-alias mechanism, raw operand
                 quads, unsealed tower)
  generic        BiasMulGeneric (probe 5's generic trait, genuine Bias
                 operands pre-reduced so they are well-typed, unsealed
                 tower)
  alias_sealed   BiasMulPP against the sealed tower (probe 3's fix),
                 same operands as `alias`, to price the seal itself

`generic` needs pre-reduced operands: `BPos<N, D>` is only well-typed
Bias for coprime N, D (matching the design's own value-uniqueness
discipline), so feeding it a random unreduced pair tests an ill-typed
input that could never arise as an actual Bias value. `alias` and
`alias_sealed` take raw Pos quads directly and need no pre-reduction
(BiasMulPP's own API shape, file 41 section 6's own open question about
whether that shape or a genuine-Bias-input shape is right for the
shipped crate).

Every instantiation is forced by a const assertion against a
Python-computed answer (fractions.Fraction, reduced), so nothing is
elided and correctness is checked at the same time as cost. Build shape:
`rustc --edition 2021 --crate-type lib --emit=metadata`, matching file
36's and file 41's own sweep shape (trait-solve-only, no codegen).
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


def quads(count: int, bits: int, seed: int = 20260803):
    rng = random.Random(seed + bits * 1000 + count)
    lo, hi = 1 << (bits - 1), (1 << bits) - 1
    out = []
    seen = set()
    while len(out) < count:
        n1, d1, n2, d2 = (rng.randint(lo, hi) for _ in range(4))
        f1, f2 = Fraction(n1, d1), Fraction(n2, d2)
        key = (f1.numerator, f1.denominator, f2.numerator, f2.denominator)
        if key in seen:
            continue
        seen.add(key)
        out.append(key)
    return out


HDR_ALIAS = """#![allow(dead_code)]
#![no_std]
#[path = "../../vu_bias.rs"]
mod bias;
use bias::nat::{H, I, O, Pos};
use bias::{Bias, BiasMulPP};
"""

HDR_ALIAS_SEALED = """#![allow(dead_code)]
#![no_std]
#[path = "../../vu_bias_sealed.rs"]
mod bias;
use bias::nat::{H, I, O, Pos};
use bias::{Bias, BiasMulPP};
"""

HDR_GENERIC = """#![allow(dead_code)]
#![no_std]
#[path = "../generic_biasmul_lib.rs"]
mod g;
use g::bias::nat::{H, I, O, Pos};
use g::bias::{Bias, BPos};
use g::BiasMulGeneric;
"""

HDR_ALIAS_SEALED_ADJ = """#![allow(dead_code)]
#![no_std]
#[path = "../../vu_bias_sealed_adj.rs"]
mod bias;
use bias::nat::{H, I, O, Pos};
use bias::{Bias, BiasMulPP};
"""

HDRS = {
    "alias": HDR_ALIAS,
    "alias_sealed": HDR_ALIAS_SEALED,
    "alias_sealed_adj": HDR_ALIAS_SEALED_ADJ,
    "generic": HDR_GENERIC,
}


def emit(kind: str, count: int, bits: int) -> str:
    qs = quads(count, bits)
    body = [HDRS[kind]]
    for i, (n1, d1, n2, d2) in enumerate(qs):
        prod = Fraction(n1, d1) * Fraction(n2, d2)
        rn, rd = prod.numerator, prod.denominator
        body.append(
            "pub type N1_%d = %s;\npub type D1_%d = %s;\n"
            "pub type N2_%d = %s;\npub type D2_%d = %s;\n"
            % (i, vu_pos(n1), i, vu_pos(d1), i, vu_pos(n2), i, vu_pos(d2))
        )
        if kind == "generic":
            body.append(
                "type P%d = <BPos<N1_%d, D1_%d> as BiasMulGeneric<BPos<N2_%d, D2_%d>>>::Out;\n"
                "const _: () = assert!(<P%d as Bias>::NUM == %d);\n"
                "const _: () = assert!(<P%d as Bias>::DEN == %d);\n"
                % (i, i, i, i, i, i, rn, i, rd)
            )
        else:
            body.append(
                "type P%d = BiasMulPP<N1_%d, D1_%d, N2_%d, D2_%d>;\n"
                "const _: () = assert!(<P%d as Bias>::NUM == %d);\n"
                "const _: () = assert!(<P%d as Bias>::DEN == %d);\n"
                % (i, i, i, i, i, i, rn, i, rd)
            )
    return "".join(body)


if __name__ == "__main__":
    kind, count, bits, out_path = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    with open(out_path, "w") as f:
        f.write(emit(kind, count, bits))
