#!/usr/bin/env python3
"""P6. The repair for P5, and it is the result rather than the repair.

P5 asked whether a declared extent on which two completions agree is closed
under the operations, because a merge that is not a congruence licenses nothing.
The answer was no, comprehensively: at W = 4 unsigned under {add}, the merge
holds for every operand bound up to 7 and the only closed extents are {0} and
the whole carrier, on which the merge does not hold. **The two regions are
almost disjoint.** So the naive form of "identity relative to a refinement" is
dead, and P5's own table killed it.

The diagnosis is that I asked for the wrong object. Demanding a closed extent
demands an invariant subalgebra, and a subalgebra is what you need when the
equivalence must survive an operation whose result type is the same as its
operand type. That is exactly the endomorphism assumption `109` section 8
already refuted from the chain side: `mul : P x P -> P` is what forces the
quantisation, and the moment the result is allowed to be a different primitive
the problem changes shape.

The refined form does the same thing to the extent. An operation does not
preserve the operand's refinement, it TRANSFORMS it:

    add : {v <= a} x {v <= b} -> {v <= a + b}

which is `109` P5's `RSum` rule, stated as a typing rather than as a bound. Then
there is no invariant extent to find. There is a derivation, each node carries
its own extent, and the merge is licensed at each node against that node's
extent rather than globally.

So the congruence question becomes: is the family of local equivalences closed
under composition, given that the extents propagate? This probe measures it
over whole terms rather than over single applications, which is what P5 should
have done.

Measured:

  1. For every declared operand bound B and every chain length k, do the two
     completions agree on EVERY term of that shape? Exhaustively, over all
     operand tuples in the box.
  2. Does the propagated bound predict the boundary exactly, in both
     directions? A rule that only over-approximates is sound and useless; a rule
     that under-approximates is unsound. Both mispredictions are counted
     separately.
  3. The same for the rounding axis, where the propagated quantity is the
     fraction width rather than the magnitude, so the two are not one mechanism
     wearing two names.
"""

from fractions import Fraction
from itertools import product


class Prim:
    def __init__(self, W, F, policy, rounding="trunc"):
        self.W, self.F, self.policy, self.rounding = W, F, policy, rounding
        self.step = Fraction(1, 2 ** F)
        self.values = [Fraction(k) * self.step for k in range(1 << W)]
        self.lo, self.hi = self.values[0], self.values[-1]

    def R(self, q):
        k = q / self.step
        n, d = k.numerator, k.denominator
        if self.rounding == "near":
            fl = n // d
            rem = k - fl
            g = Fraction(fl + 1) if rem > Fraction(1, 2) else (
                Fraction(fl) if rem < Fraction(1, 2) else
                Fraction(fl if fl % 2 == 0 else fl + 1))
        else:
            g = Fraction(int(k))
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


def left_fold(p, xs, op):
    acc = xs[0]
    for x in xs[1:]:
        acc = getattr(p, op)(acc, x)
    return acc


def completion_terms(W, B, k, op):
    """Every left-nested term of arity k over operands bounded by B, evaluated
    under both completions. Returns (disagreements, total)."""
    sat = Prim(W, 0, "sat")
    wrp = Prim(W, 0, "wrap")
    ext = [v for v in sat.values if v <= Fraction(B)]
    bad = 0
    tot = 0
    for xs in product(ext, repeat=k):
        tot += 1
        if left_fold(sat, list(xs), op) != left_fold(wrp, list(xs), op):
            bad += 1
    return bad, tot


def propagated_bound(B, k, op):
    """109 P5's rule, applied k - 1 times. Addition sums the bounds; multiply
    multiplies them."""
    if op == "add":
        return B * k
    b = 1
    for _ in range(k):
        b *= B
    return b


def main():
    print("P6. the refinement propagates, and the propagated bound is the predicate")
    print("=" * 78)

    for W, op, ks, bs in ((4, "add", [2, 3, 4], range(0, 16)),
                          (4, "mul", [2, 3], range(0, 16)),
                          (5, "add", [2, 3], range(0, 32))):
        hi = (1 << W) - 1
        print(f"\nW = {W}, F = 0, unsigned, carrier 0..={hi}, "
              f"signature {{{op}}}, saturate against wrap")
        print("  arity  bound B  propagated  fits?  disagreements  of      verdict")
        sound_mispred = loose_mispred = 0
        for k in ks:
            for B in bs:
                bad, tot = completion_terms(W, B, k, op)
                pb = propagated_bound(B, k, op)
                fits = pb <= hi
                merged = bad == 0
                if fits and not merged:
                    sound_mispred += 1
                    verdict = "UNSOUND PREDICTION"
                elif merged and not fits:
                    loose_mispred += 1
                    verdict = "conservative"
                else:
                    verdict = "exact"
                if B in (0, 1, 2, 3, 4, 5, 7, 8, 15) or verdict != "exact":
                    print(f"  {k:>5}  {B:>7}  {pb:>10}  {'yes' if fits else 'no ':>5}"
                          f"  {bad:>13}  {tot:>6}  {verdict}")
        print(f"  unsound predictions (rule says merge, answers differ): {sound_mispred}")
        print(f"  conservative        (rule refuses, answers agree)    : {loose_mispred}")

    # ------------------------------------------------------------------ rounding
    print("\n" + "-" * 78)
    print("the rounding axis: the propagated quantity is the fraction width")
    print("W = 6, F = 2, signature {mul}, truncate against nearest")
    print("  operand fraction bits c  arity  propagated cF  <= F?  disagreements  of")
    tr = Prim(6, 2, "sat", "trunc")
    ne = Prim(6, 2, "sat", "near")
    unsound = conservative = 0
    for c in range(0, 3):
        stride = 1 << (2 - c)
        ext = [v for v in tr.values if (v / tr.step).numerator % stride == 0]
        for k in (2, 3):
            bad = tot = 0
            for xs in product(ext, repeat=k):
                tot += 1
                if left_fold(tr, list(xs), "mul") != left_fold(ne, list(xs), "mul"):
                    bad += 1
            pf = c * k
            fits = pf <= 2
            if fits and bad:
                unsound += 1
            if bad == 0 and not fits:
                conservative += 1
            print(f"  c = {c:<21} {k:>5}  {pf:>13}  {'yes' if fits else 'no ':>5}"
                  f"  {bad:>13}  {tot:>6}")
    print(f"  unsound predictions: {unsound}, conservative: {conservative}")

    print("\n" + "-" * 78)
    print("reading:")
    print("  There is no invariant extent to find, and P5 was wrong to look for")
    print("  one. The equivalence is indexed by a DERIVATION: each node carries")
    print("  the extent its operands' extents imply, and the merge is checked")
    print("  against that node's extent. Composition works because the extent")
    print("  propagates, not because it is preserved.")
    print()
    print("  That is the same move 109 section 8 makes for chain accuracy, one")
    print("  level down: the operator is not an endomorphism, and once its")
    print("  result is allowed to be a different primitive the property falls")
    print("  out of the typing instead of needing a policy.")


if __name__ == "__main__":
    main()
