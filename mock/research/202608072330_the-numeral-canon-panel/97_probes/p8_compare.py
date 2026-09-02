#!/usr/bin/env python3
"""Reads the emitted assembly for p8 and compares entry bodies and aliases.

An alias is a stronger result than matching bodies: the linker proved the two are
the same code rather than two copies that happen to agree. Both are reported.
"""
import re, subprocess, sys, os

SRC = "p8_does_a_computed_demand_erase.s"
src = open(os.path.join(".p8", SRC)).read()

alias = dict(re.findall(r'^(_\w+) = (\S+)$', src, re.M))
bodies = {}
for m in re.finditer(r'^(_\w+):\n(.*?)^\s*\.cfi_endproc', src, re.S | re.M):
    name, body = m.group(1), m.group(2)
    ins = [l.strip() for l in body.split("\n")
           if l.strip() and not l.strip().startswith((".", ";", "L"))]
    bodies[name] = ins

def resolve(sym):
    seen = set()
    while sym in alias and sym not in seen:
        seen.add(sym)
        sym = alias[sym]
    return sym

names = ["entry_speed", "entry_residency", "entry_joined",
         "entry_handwritten", "entry_different"]
print("P8. does a selector on a COMPUTED demand set cost anything")
print()
print("%-20s %-10s %s" % ("entry", "instrs", "resolves to"))
for n in names:
    s = "_" + n
    tgt = resolve(s)
    ins = bodies.get(tgt) or bodies.get(s)
    print("%-20s %-10s %s" % (n, len(ins) if ins else "alias", tgt if tgt != s else "(own body)"))
print()

def same(a, b):
    ra, rb = resolve("_" + a), resolve("_" + b)
    if ra == rb:
        return "SAME SYMBOL"
    ia, ib = bodies.get(ra), bodies.get(rb)
    if ia is None or ib is None:
        return "one side has no body"
    return "identical body" if ia == ib else "different"

pairs = [
    ("entry_joined", "entry_handwritten",
     "the computed demand against a hand-written call to the arm it resolves to"),
    ("entry_joined", "entry_speed",
     "NEGATIVE CONTROL: the join must not collapse onto its first operand"),
    ("entry_joined", "entry_residency",
     "NEGATIVE CONTROL: nor onto its second"),
    ("entry_joined", "entry_different",
     "NEGATIVE CONTROL: nor onto an unrelated arm"),
]
for a, b, why in pairs:
    print("%-40s %-16s %s" % (a + " vs " + b, same(a, b), why))
print()
verdict_ok = (same("entry_joined", "entry_handwritten") in ("SAME SYMBOL", "identical body")
              and all(same("entry_joined", x) == "different"
                      for x in ("entry_speed", "entry_residency", "entry_different")))
print("VERDICT: %s" % ("the computed demand costs nothing and resolves correctly"
                       if verdict_ok else "NOT established, read the table above"))
