#!/usr/bin/env python3
"""Generate the compile-cost sweep sources.

Three shapes, same value pairs in each so the comparison is like for like:

  vu_gcd    N instantiations of the value-unique encoding's Stein gcd
  tn_gcd    N instantiations of typenum's `Gcf`, the named prior art
  vu_reduce N instantiations of the full reduction (gcd + exact division),
            which is what an adjustment actually costs; typenum has no
            counterpart, so this one is measured rather than compared

Every instantiation is forced by a const assertion against a Python-computed
value, so nothing is elided and correctness is checked at the same time.
"""
import math
import random
import sys


def vu_pos(n: int) -> str:
    if n == 1:
        return "H"
    if n % 2 == 0:
        return "O<%s>" % vu_pos(n // 2)
    return "I<%s>" % vu_pos(n // 2)


def tn_uint(n: int) -> str:
    if n == 0:
        return "UTerm"
    return "UInt<%s, %s>" % (tn_uint(n // 2), "B1" if n % 2 else "B0")


def pairs(count: int, bits: int, seed: int = 20260803):
    rng = random.Random(seed + bits * 1000 + count)
    lo, hi = 1 << (bits - 1), (1 << bits) - 1
    out = []
    seen = set()
    while len(out) < count:
        a = rng.randint(lo, hi)
        b = rng.randint(lo, hi)
        if (a, b) in seen:
            continue
        seen.add((a, b))
        out.append((a, b))
    return out


HDR_VU = """#![allow(dead_code)]
#![no_std]
#[path = "../../vu_nat.rs"]
mod nat;
use nat::{H, I, O, Pos, Ratio, Reduce, Gcd};
use nat::maxmin::GcdMM;
"""

HDR_TN = """#![allow(dead_code)]
#![no_std]
use typenum::{B0, B1, UInt, UTerm, Unsigned, Gcf};
"""


def emit(kind: str, count: int, bits: int) -> str:
    ps = pairs(count, bits)
    if kind == "tn_gcd":
        body = [HDR_TN]
        for i, (a, b) in enumerate(ps):
            g = math.gcd(a, b)
            body.append(
                "pub type X%d = %s;\npub type Y%d = %s;\n"
                "const _: () = assert!(<Gcf<X%d, Y%d> as Unsigned>::U64 == %d);\n"
                % (i, tn_uint(a), i, tn_uint(b), i, i, g)
            )
        return "".join(body)

    body = [HDR_VU]
    for i, (a, b) in enumerate(ps):
        g = math.gcd(a, b)
        body.append("pub type X%d = %s;\npub type Y%d = %s;\n" % (i, vu_pos(a), i, vu_pos(b)))
        if kind == "vu_gcd_maxmin":
            body.append(
                "const _: () = assert!(<<X%d as GcdMM<Y%d>>::Out as Pos>::VAL == %d);\n"
                % (i, i, g)
            )
        elif kind == "vu_gcd":
            body.append(
                "const _: () = assert!(<<X%d as Gcd<Y%d>>::Out as Pos>::VAL == %d);\n"
                % (i, i, g)
            )
        elif kind == "vu_reduce":
            body.append(
                "const _: () = assert!(<<Ratio<X%d, Y%d> as Reduce>::N as Pos>::VAL == %d);\n"
                "const _: () = assert!(<<Ratio<X%d, Y%d> as Reduce>::D as Pos>::VAL == %d);\n"
                % (i, i, a // g, i, i, b // g)
            )
        else:
            raise SystemExit("unknown kind " + kind)
    return "".join(body)


if __name__ == "__main__":
    kind, count, bits, out = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
    with open(out, "w") as f:
        f.write(emit(kind, count, bits))
