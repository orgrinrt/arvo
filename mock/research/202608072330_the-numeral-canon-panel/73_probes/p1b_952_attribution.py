#!/usr/bin/env python3
"""p1b. Which operation does the panel's 952 belong to?

Not part of the assigned question. Built because p1 returned 952 for signed
saturating ADDITION at w = 4 and two panel files attribute that number to two
different operations:

  `63:230`  "signed two's-complement saturation induces a unital commutative
             magma that is not a semigroup, 952 associativity failures on Q
             itself"                                    (operation unnamed)
  `66:458`  "the panel's measured 952 of a comparable space for signed
             saturating MULTIPLICATION at width 4"      (operation named)

Both cannot be right unless the two operations coincidentally agree. This
measures both, exhaustively over Q^3 at w = 4, Q = [-8, 7].

I did not open `55_probes/p4`, so this settles which operation carries the
number and does not settle what `55` measured.
"""

W = 4
Q = list(range(-(1 << (W - 1)), 1 << (W - 1)))
LO, HI = min(Q), max(Q)


def sat(e):
    return LO if e < LO else (HI if e > HI else e)


def fails(op):
    bad = 0
    for a in Q:
        for b in Q:
            for c in Q:
                if op(op(a, b), c) != op(a, op(b, c)):
                    bad += 1
    return bad


sadd = lambda a, b: sat(a + b)
smul = lambda a, b: sat(a * b)

fa, fm = fails(sadd), fails(smul)
total = len(Q) ** 3

print("Q = [%d, %d], |Q|^3 = %d" % (LO, HI, total))
print("signed saturating ADDITION      associativity failures: %d" % fa)
print("signed saturating MULTIPLICATION associativity failures: %d" % fm)
print()
assert fa != fm, "the two operations agree; the attribution question is moot"
print("the two differ, so the number belongs to exactly one of them:")
print("  952 is %s" % ("ADDITION" if fa == 952 else ("MULTIPLICATION" if fm == 952 else "NEITHER")))
