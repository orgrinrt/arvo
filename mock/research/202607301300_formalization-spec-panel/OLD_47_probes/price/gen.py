#!/usr/bin/env python3
"""Generators for file 47's two price sweeps.

  alias_table   : N type aliases naming Nats in the ratified encoding, the cost
                  of shipping a decimal-literal table.
  grade_declared: N fold call sites in file 37's const form (grade declared by
                  the caller as a const bitmask).
  grade_projected: N fold call sites in file 47 probe 3's form (grade projected).

Every emitted instantiation is const-asserted against a Python-computed value,
so a wrong sweep fails to compile rather than reporting a number.
"""
import sys

def pos(n):
    assert n >= 1
    if n == 1: return "H"
    return ("I<" if n & 1 else "O<") + pos(n >> 1) + ">"

def nat(n):
    return "Z" if n == 0 else "Pz<%s>" % pos(n)

def alias_table_bare(n):
    """The table with no per-row check, to separate the alias's own cost from
    the cost of the assertion that makes it trustworthy."""
    out = ['#![allow(dead_code)]', 'pub mod n {', '    use tower::nat::{Z, Pz, H, O, I};']
    for i in range(n):
        out.append("    pub type N%d = %s;" % (i, nat(i)))
    out.append('}')
    return "\n".join(out) + "\n"

def alias_table(n):
    out = ['#![allow(dead_code)]', 'use tower::nat::{Nat, Z, Pz, H, O, I};', 'pub mod n {',
           '    use tower::nat::{Z, Pz, H, O, I};']
    for i in range(n):
        out.append("    pub type N%d = %s;" % (i, nat(i)))
    out.append('}')
    for i in range(n):
        out.append("const _: () = assert!(<n::N%d as Nat>::VAL == %d);" % (i, i))
    return "\n".join(out) + "\n"

def grade_projected(n):
    out = ['#![allow(dead_code)]', '#[path = "../probe_3_the_grade_is_projected.rs"] mod m;',
           'use m::*;', 'use tower::nat::{H, I, O};',
           'type A = I<H>; type Hd = H; type Wide = O<O<H>>;']
    comps = [("ReduceModulo, ReduceModulo, Unsigned, Hd, A", "Faithful"),
             ("ReduceModulo, ReduceModulo, Signed, Hd, A", "EventsTransferred"),
             ("Refuse, Refuse, Signed, Hd, A", "RefusalsTransferred"),
             ("Refuse, ReduceModulo, Signed, Hd, A", "BothTransferred")]
    for i in range(n):
        c, g = comps[i % 4]
        out.append("pub const C%d: Folded<%s> = regroup_fold::<%s>(&[1,2,3,4]);" % (i, g, c))
    return "\n".join(out) + "\n"


def grade_declared(n):
    """File 37's own mechanism, unmodified, at N call sites. The caller writes
    the published bitmask; the const assertion checks it."""
    out = ['#![allow(dead_code)]',
           '#[path = "../../37_probes/probe_4_view_as_a_return_type_and_the_transfer.rs"] mod m;',
           'use m::*;']
    comps = [("2, 2, 0, 4, 0", 0), ("2, 2, 1, 4, 0", 2), ("0, 0, 1, 4, 0", 1), ("0, 2, 1, 4, 0", 3)]
    for i in range(n):
        c, p = comps[i % 4]
        out.append("pub const C%d: Folded<%d> = regroup_fold::<%s, %d>([1,2,3,4]);" % (i, p, c, p))
    return "\n".join(out) + "\n"

if __name__ == "__main__":
    kind, n = sys.argv[1], int(sys.argv[2])
    sys.stdout.write({"alias_table": alias_table,
                      "alias_table_bare": alias_table_bare,
                      "grade_projected": grade_projected,
                      "grade_declared": grade_declared}[kind](n))
