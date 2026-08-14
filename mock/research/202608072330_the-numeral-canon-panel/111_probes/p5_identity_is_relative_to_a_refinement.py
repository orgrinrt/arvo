#!/usr/bin/env python3
"""P5. Degeneracy is relative to a refinement, and the three notions in play are
one notion at three refinements.

Three results in this unit are about an axis "vanishing":

  109 P5   a carried range makes the completion unreachable, so the clamp is
           absent from the emitted code rather than selected.
  110 F5   radix at F = 0 has left the definition of R, so no term separates it.
  110 F6   rounding at F = 0 is read by R but no grid-closed term reaches where
           it is read.

They are stated as different things. This probe asks whether they are one thing
evaluated at three different refinements of the carrier:

  the trivial refinement       every value, every rational argument to R.
                               An axis degenerate here is 110's DEFINITIONAL.
  the signature's refinement   the image of the term algebra. An axis
                               degenerate here and not above is 110's
                               REACHABILITY.
  a declared refinement        a consumer-supplied predicate on the operands,
                               109's carried range being one.

If they are, then identity is not a single relation. It is a relation indexed by
a refinement, and the design's arms are indexed by which refinement has been
discharged. That is the shape I13 asks for, arrived at from the identity
question rather than from the rewriting question.

Two things get measured, and the second is what makes it a design statement
rather than an observation:

  1. the extent of the merge. For each declared refinement, do two primitives
     differing only in the completion have the same operation tables on it?
  2. the CONGRUENCE condition. A merge licenses substitution only if it survives
     the operations, which for a refinement means the refinement is closed under
     them. This is 109's "closure is prior to every law" and 110's P8 congruence
     check meeting on the same condition, and it is measured here rather than
     argued.
"""

from fractions import Fraction
from itertools import product

# --------------------------------------------------------------- the model


class Prim:
    """Unsigned W-bit, fraction width F, radix 2, one realisation map."""

    def __init__(self, W, F, policy, rounding="trunc"):
        self.W, self.F, self.policy, self.rounding = W, F, policy, rounding
        self.step = Fraction(1, 2 ** F)
        self.values = [Fraction(k) * self.step for k in range(1 << W)]
        self.lo, self.hi = self.values[0], self.values[-1]

    def R(self, q):
        k = q / self.step
        n, d = k.numerator, k.denominator
        g = Fraction(n // d) if self.rounding == "floor" else (
            Fraction(int(k)) if k >= 0 else -Fraction(int(-k)))
        if self.rounding == "near":
            fl = n // d
            rem = k - fl
            g = Fraction(fl + 1) if rem > Fraction(1, 2) else (
                Fraction(fl) if rem < Fraction(1, 2) else
                Fraction(fl if fl % 2 == 0 else fl + 1))
        v = g * self.step
        if self.lo <= v <= self.hi:
            return v
        if self.policy == "sat":
            return self.hi if v > self.hi else self.lo
        span = self.hi - self.lo + self.step
        return ((v - self.lo) % span) + self.lo

    def add(self, a, b):
        return self.R(a + b)

    def mul(self, a, b):
        return self.R(a * b)


# ------------------------------------------------- refinements and identity

def box(p, hi):
    """The declared refinement 109's carried range induces: operands at most
    `hi`. Returned as the extent, which is what identity is relative to."""
    return [v for v in p.values if v <= hi]


def closed_under(p, ext, ops):
    """Is the extent an invariant of the operations? A merge that is not a
    congruence licenses nothing, because the substituted value leaves the region
    the licence was granted on."""
    s = set(ext)
    for name in ops:
        for a, b in product(ext, repeat=2):
            if getattr(p, name)(a, b) not in s:
                return False
    return True


def same_on(p, q, ext, ops):
    """Do the two primitives have the same operation tables on the extent?"""
    n = 0
    for name in ops:
        for a, b in product(ext, repeat=2):
            if getattr(p, name)(a, b) != getattr(q, name)(a, b):
                n += 1
    return n


def total_pairs(ext, ops):
    return len(ops) * len(ext) ** 2


# ------------------------------------------------------------------- sweeps

def completion_sweep(W, ops):
    """wrap against saturate, over every declared box, under `ops`."""
    sat = Prim(W, 0, "sat")
    wrp = Prim(W, 0, "wrap")
    rows = []
    for hi_k in range(1 << W):
        hi = Fraction(hi_k)
        ext = box(sat, hi)
        mism = same_on(sat, wrp, ext, ops)
        rows.append((hi_k, len(ext), mism, total_pairs(ext, ops),
                     closed_under(sat, ext, ops), closed_under(wrp, ext, ops)))
    return rows


def rounding_sweep(W, F):
    """The same question one axis over: two rounding modes, and a declared
    refinement to a coarser sub-grid on which the product is exact."""
    a = Prim(W, F, "sat", "trunc")
    b = Prim(W, F, "sat", "near")
    rows = []
    for c in range(F + 1):
        # operands restricted to multiples of 2^-c, so a product carries 2c
        # fraction bits and is exact on the F-grid exactly when 2c <= F.
        stride = 1 << (F - c) if F >= c else 1
        ext = [v for v in a.values if (v / a.step).numerator % stride == 0]
        mism = same_on(a, b, ext, ["mul"])
        rows.append((c, len(ext), mism, total_pairs(ext, ["mul"]),
                     closed_under(a, ext, ["mul"])))
    return rows


def main():
    print("P5. identity relative to a refinement")
    print("=" * 78)

    for ops in (["add"], ["add", "mul"]):
        W = 4
        print(f"\ncompletion axis: saturate against wrap, W = {W}, F = 0, "
              f"signature {{{', '.join(ops)}}}")
        print("  declared operand bound  extent  disagreements  of  closed?")
        rows = completion_sweep(W, ops)
        merged_and_closed = []
        for hi_k, n, mism, tot, cl_s, cl_w in rows:
            mark = ""
            if mism == 0 and cl_s and cl_w:
                mark = "  <== merged, and the merge is a congruence"
                merged_and_closed.append(hi_k)
            elif mism == 0:
                mark = "  <== merged, but NOT closed: the merge licenses nothing"
            print(f"  operands <= {hi_k:<11} {n:>5}  {mism:>13}  {tot:>4}  "
                  f"{'yes' if cl_s and cl_w else 'no':<4}{mark}")
        if merged_and_closed:
            print(f"  largest sound declared bound: {max(merged_and_closed)}")
        print(f"  on the full carrier the two differ on "
              f"{rows[-1][2]} of {rows[-1][3]} applications, so nothing merges there")

    print("\n" + "-" * 78)
    W, F = 6, 2
    print(f"rounding axis: truncate against nearest, W = {W}, F = {F}, "
          f"signature {{mul}}")
    print("  operand grid 2^-c   extent  disagreements  of  closed?")
    for c, n, mism, tot, cl in rounding_sweep(W, F):
        mark = "  <== merged" if mism == 0 else ""
        print(f"  c = {c:<15} {n:>5}  {mism:>13}  {tot:>4}  "
              f"{'yes' if cl else 'no':<4}{mark}")
    print("  the rounding mode is unreachable exactly where the product of two")
    print("  operands is already on the grid, which is a declared refinement and")
    print("  not a property of the signature or of the definition")

    print("\n" + "-" * 78)
    print("what this says about the three notions:")
    print("  definitional   degenerate at the trivial refinement (the whole")
    print("                 carrier, and every rational argument to R)")
    print("  reachability   degenerate on the image of the term algebra")
    print("  declared       degenerate on a consumer-supplied extent")
    print("  All three are the same predicate over different extents. What")
    print("  differs is who supplies the extent and when it is known.")
    print()
    print("and the condition that makes a merge usable:")
    print("  the extent must be closed under the signature, or the merged")
    print("  identity is not a congruence and substitution walks out of the")
    print("  region the licence was granted on. That is one condition covering")
    print("  109's 'closure is prior to every law' and 110's P8 congruence")
    print("  check, and it is what a const predicate on an arm has to assert.")


if __name__ == "__main__":
    main()
