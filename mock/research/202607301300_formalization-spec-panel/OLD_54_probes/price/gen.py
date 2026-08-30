#!/usr/bin/env python3
"""Generator for file 54's radix-axis compile-cost sweep.

The question: `Implicit<E, A, B>` carries both an exponent and a rational adjustment, and
`A * radix^E` could be folded into a single rational, which would make `E` a redundant axis.
This generator emits the two spellings of the same grid so the fold's cost can be measured
rather than assumed.

  radix_exp   the grid as (radix = 10, exponent = -k, adjustment = 1). The `Pos` nest that
              has to be written is the one for k, of depth floor(log2 k) + 1.
  absorbed    the same grid as (radix = 2, exponent = 0, adjustment = 1 / 10^k). The `Pos`
              nest is the one for 10^k, of depth floor(k * log2 10) + 1.

Both spellings are forced through a const assertion reading the numeral's own consts, so
neither is left as an inert alias whose bound checks never run.
"""

import sys


def pos_type(n: int) -> str:
    """The value-unique `Pos` spelling of n. H = 1, O<P> = 2P, I<P> = 2P+1, outermost
    constructor is the least significant bit."""
    assert n >= 1
    bits = bin(n)[3:]  # drop the leading '1', which is H
    t = "H"
    for b in bits:
        t = f"{'I' if b == '1' else 'O'}<{t}>"
    return t


def depth(n: int) -> int:
    return n.bit_length()


HEADER = """#![allow(dead_code, unused_imports)]

#[path = "../../vu_bias_sealed_adj.rs"]
pub mod bias;
#[path = "../../numeral.rs"]
pub mod numeral;

use bias::nat::{Adjustment, Ratio, H, I, O};
use bias::BZero;
use numeral::*;

type PP8 = O<O<O<H>>>;
type A1 = Ratio<H, H>;
"""


def emit(kind: str, kmax: int, path: str) -> None:
    out = [HEADER]
    ks = [kmax] if kind.endswith("_one") else list(range(1, kmax + 1))
    for k in ks:
        if kind == "radix_exp":
            out.append(f"type K{k} = {pos_type(k)};")
            out.append(
                f"pub type G{k} = Fx<Ten, PP8, ENeg<K{k}>, A1, BZero, Symmetric>;"
            )
            out.append(f"const _: () = assert!(<G{k} as Numeral>::EMIN == -{k});")
            out.append(f"const _: () = assert!(<G{k} as Numeral>::R == 10);")
        elif kind == "absorbed":
            n = 10**k
            out.append(f"type D{k} = {pos_type(n)};")
            out.append(f"pub type AD{k} = Ratio<H, D{k}>;")
            out.append(
                f"pub type G{k} = Fx<Two, PP8, EZero, AD{k}, BZero, Symmetric>;"
            )
            out.append(f"const _: () = assert!(<AD{k} as Adjustment>::DEN == {n}u64);")
            out.append(f"const _: () = assert!(<G{k} as Numeral>::R == 2);")
        elif kind == "radix_one":
            out.append(f"type K{k} = {pos_type(k)};")
            out.append(
                f"pub type G{k} = Fx<Ten, PP8, ENeg<K{k}>, A1, BZero, Symmetric>;"
            )
            out.append(f"const _: () = assert!(<G{k} as Numeral>::EMIN == -{k});")
        elif kind == "absorbed_one":
            n = 10**k
            out.append(f"type D{k} = {pos_type(n)};")
            out.append(f"pub type AD{k} = Ratio<H, D{k}>;")
            out.append(
                f"pub type G{k} = Fx<Two, PP8, EZero, AD{k}, BZero, Symmetric>;"
            )
            out.append(f"const _: () = assert!(<G{k} as Numeral>::R == 2);")
        else:
            raise SystemExit(f"unknown kind {kind}")
    # force every declaration through a signature rather than leaving inert aliases
    out.append("pub fn forced() -> i64 {")
    out.append("    let mut acc = 0i64;")
    for k in ks:
        out.append(f"    acc += <G{k} as Numeral>::EMIN;")
    out.append("    acc")
    out.append("}")
    with open(path, "w") as fh:
        fh.write("\n".join(out) + "\n")


if __name__ == "__main__":
    if sys.argv[1] == "depth":
        for k in [1, 2, 3, 4, 6, 8, 12, 16, 19, 24, 32, 96, 398]:
            print(f"k={k:<4} depth(k)={depth(k):<3} depth(10^k)={depth(10**k)}")
        sys.exit(0)
    emit(sys.argv[1], int(sys.argv[2]), sys.argv[3])
