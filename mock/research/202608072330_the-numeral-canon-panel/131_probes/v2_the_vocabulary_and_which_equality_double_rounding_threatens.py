#!/usr/bin/env python3
"""v2. Two holes: the vocabulary fork, and what double rounding actually
threatens.

HOLE A: THE VOCABULARY
----------------------
`125` section 7 and `128` argue for retiring "truncation" for a six-mode
vocabulary, on the ground that on signed domains the word is ambiguous between
bit-drop, which is floor, and toward-zero, which is Rust's `/`. `126` kept the
older spelling and its reply `129` does not address it. `130` carries "the
toward_zero vocabulary confirmation" without re-deriving it.

So the disagreement is located and both parties have now seen the evidence. It is
settled here by counting, because the question "are these two modes the same
operation" has an exact answer and does not need anybody's preference.

The dispatch cites 21,204 of 32,768 signed negative cases at `W = 8`. That is a
specific number about a specific sweep and it is checked rather than repeated.

HOLE B: DOUBLE ROUNDING
------------------------
`126` Finding 5 measures that directed modes compose exactly across precisions
and nearest modes do not, and names double rounding across chained carrier widths
as "a hazard the panel's carrier/discharge apparatus has not yet named".

**I think that names the wrong equality, and this probe is how I find out.**
`122` 4.6's rule is an equality between two arms that BOTH round at every node:
one applies the range part at each node, the other defers it to the root while
applying the grid part at every node whose result leaves the grid. If both arms
round at every node, double rounding happens identically in both and cancels out
of the comparison.

What double rounding threatens is a DIFFERENT equality, between staged narrowing
and direct narrowing, which no clause in the preceding topic states. If that is
right, `126`'s hazard is real and is not a gap in 4.6; it is a missing clause
about a relation 4.6 never claimed.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. Bit-drop equals floor on both signednesses, exhaustively.
P2. Bit-drop differs from toward-zero on exactly the negative values with a
    nonzero dropped part, and on nothing else. The count at `W = 8, F = 1` is
    whatever it is; the dispatch's figure is checked against a stated sweep
    rather than assumed to match.
P3. `122` 4.6's equality holds under every mode, including the nearest ones, when
    carriers chain. Double rounding does not threaten it.
P4. Staged narrowing equals direct narrowing for floor, ceil and toward_zero and
    fails for half_up and half_even, reproducing `125` T4 and `126` Finding 5 on
    a third instrument.
P5. Therefore the missing clause is about staged narrowing, not about 4.6.

NEGATIVE CONTROLS
-----------------
C1. On unsigned domains bit-drop and toward-zero must agree everywhere, or the
    ambiguity is not about sign and the vocabulary argument is misdiagnosed.
C2. P3's sweep must contain cells where the two arms of 4.6 could have differed,
    that is cells where a node's result leaves the grid, or the equality is being
    read off a region where nothing rounds.
C3. P4 must show a nonzero failure count for the nearest modes, or the sweep does
    not reach the double-rounding region at all.
"""

import sys
from fractions import Fraction
from itertools import product


def frac_floor(x, F):
    import math
    s = Fraction(1, 2 ** F)
    return Fraction(math.floor(x / s)) * s


def frac_ceil(x, F):
    import math
    s = Fraction(1, 2 ** F)
    return Fraction(math.ceil(x / s)) * s


def q(mode, x, F):
    s = Fraction(1, 2 ** F)
    if mode == "floor":
        return frac_floor(x, F)
    if mode == "ceil":
        return frac_ceil(x, F)
    if mode == "toward_zero":
        return frac_floor(x, F) if x >= 0 else frac_ceil(x, F)
    if mode == "half_up":
        return frac_floor(x + s / 2, F)
    if mode == "half_even":
        lo = frac_floor(x, F)
        d = x - lo
        if d < s / 2:
            return lo
        if d > s / 2:
            return lo + s
        return lo if int(lo / s) % 2 == 0 else lo + s
    raise ValueError(mode)


DIRECTED = ("floor", "ceil", "toward_zero")
NEAREST = ("half_up", "half_even")
MODES = DIRECTED + NEAREST


# ------------------------------------------------------------------- HOLE A


def bit_drop(k, W, F, signed):
    """Two's complement bit-drop: an arithmetic right shift by F of the raw
    scaled integer. This is what hardware gives away free."""
    if signed:
        return k >> F
    return (k & ((1 << W) - 1)) >> F


def toward_zero_div(k, F):
    """Rust's integer `/`, which truncates toward zero."""
    d = 1 << F
    qout = abs(k) // d
    return qout if k >= 0 else -qout


def hole_a():
    print("=" * 92)
    print("HOLE A. The vocabulary: is 'truncation' one mode or two?")
    print("=" * 92)
    print()
    print(f"  {'W':>3} {'F':>2} {'signedness':<10} {'values':>8} "
          f"{'bit-drop != floor':>18} {'bit-drop != toward_zero':>24}")
    for W in (4, 6, 8):
        for F in (1, 2):
            for signed in (True, False):
                lo = -(1 << (W - 1)) if signed else 0
                hi = (1 << (W - 1)) - 1 if signed else (1 << W) - 1
                nf = nt = tot = 0
                for k in range(lo, hi + 1):
                    x = Fraction(k, 1 << F)
                    tot += 1
                    bd = bit_drop(k, W, F, signed)
                    fl = int(q("floor", x, 0))
                    tz = toward_zero_div(k, F)
                    if bd != fl:
                        nf += 1
                    if bd != tz:
                        nt += 1
                print(f"  {W:>3} {F:>2} {'signed' if signed else 'unsigned':<10} "
                      f"{tot:>8} {nf:>18} {nt:>24}")
    print()
    print("  P1 holds when the 'bit-drop != floor' column is zero everywhere.")
    print("  C1 holds when the last column is zero on every unsigned row.")

    print()
    print("  The dispatch cites 21,204 of 32,768 at W = 8. Checked against the")
    print("  sweep that could produce a count of that size:")
    print()
    for W, F in ((8, 1), (8, 2), (8, 7)):
        lo, hi = -(1 << (W - 1)), (1 << (W - 1)) - 1
        pairs = 0
        diff = 0
        for a in range(lo, hi + 1):
            for b in range(lo, hi + 1):
                if b == 0:
                    continue
                pairs += 1
                fl = a // b
                tz = abs(a) // abs(b) * (1 if (a >= 0) == (b >= 0) else -1)
                if fl != tz:
                    diff += 1
        print(f"    W = {W}, op = integer division over all nonzero-divisor pairs: "
              f"{diff} of {pairs} differ")
    print()
    print("    and over single values at each fraction width:")
    for F in (1, 2, 4, 7):
        W = 8
        lo, hi = -(1 << (W - 1)), (1 << (W - 1)) - 1
        d = sum(1 for k in range(lo, hi + 1) if bit_drop(k, W, F, True) != toward_zero_div(k, F))
        print(f"      W = 8, F = {F}: {d} of {hi - lo + 1} values differ")


# ------------------------------------------------------------------- HOLE B


def rng(W, signed, F):
    s = Fraction(1, 2 ** F)
    lo = -(1 << (W - 1)) if signed else 0
    hi = (1 << (W - 1)) - 1 if signed else (1 << W) - 1
    return lo * s, hi * s


def reduce_at(mode, x, F, lo, hi, policy="wrap"):
    """The full realisation: quantise to the grid, then range-reduce.

    The range is passed in rather than derived from `F`. The first version of
    this derived it, so coarsening the fraction width between nodes silently
    changed the RANGE as well as the grid, and the two arms of the comparison
    below were reduced against different spans. That produced 66 differences on
    one chain for every mode including the directed ones, which read as a
    refutation of P3 and was an artifact of my own bookkeeping. The defect is
    recorded here rather than repaired silently."""
    g = q(mode, x, F)
    step = Fraction(1, 2 ** F)
    if policy == "sat":
        return lo if g < lo else (hi if g > hi else g)
    span = (hi - lo) + step
    return ((g - lo) % span) + lo


def hole_b():
    """Rewritten after two false starts, both recorded rather than hidden.

    First version derived the range from the fraction width, so coarsening the
    grid silently changed the range too and both arms were reduced against
    different spans. Second version, with the range fixed, deferred the range
    part past a MULTIPLICATION, which `122` 4.6 does not license: 4.6 requires
    the range part at the operands of every node the map is not a homomorphism
    for, and wrapping is not a homomorphism for multiplication at F > 0. So the
    arm being tested was a strawman that deferred more than the rule permits,
    and its failures said nothing about 4.6.

    This version implements 4.6 as written and separates the two cases the rule
    does and does not speak to."""
    print()
    print("=" * 92)
    print("HOLE B. Which equality does double rounding threaten?")
    print("=" * 92)

    W, signed = 4, False

    def is_hom(op, F):
        """`122` 4.4 as revised: wrapping is a homomorphism for add and sub at
        any fraction width and for mul only at F = 0."""
        return op != "mul" or F == 0

    def leaves_grid(op, F):
        return op == "mul" and F > 0

    def arm_every_node(mode, a, b, Fs, lo, hi):
        """Reduce fully at every node. `Fs` is the output grid per node."""
        n1 = reduce_at(mode, a * b, Fs[0], lo, hi)
        return reduce_at(mode, n1 + a, Fs[1], lo, hi)

    def arm_46(mode, a, b, Fs, lo, hi):
        """`122` 4.6 as written: the range part at the operands of every
        non-homomorphic node, the grid part at the result of every node whose
        exact result can leave the grid, both deferred elsewhere, and a full
        reduction at the root."""
        x, y = a, b
        if not is_hom("mul", Fs[0]):
            x, y = reduce_at(mode, x, Fs[0], lo, hi), reduce_at(mode, y, Fs[0], lo, hi)
        v1 = x * y
        if leaves_grid("mul", Fs[0]):
            v1 = q(mode, v1, Fs[0])
        v2 = v1 + a
        if not is_hom("add", Fs[1]):
            raise AssertionError("add is a homomorphism under wrap at every F")
        return reduce_at(mode, v2, Fs[1], lo, hi)

    print()
    print("CASE A, the control. One grid for the whole chain, which is the")
    print("setting `122` 4.6 is stated in. The two arms must agree.")
    print()
    print(f"  {'mode':<12} {'F':<4} {'cells':>6} {'left the grid':>14} "
          f"{'arms differ':>12}")
    for mode in MODES:
        for F in (1, 2):
            lo, hi = rng(W, signed, F)
            vals = [Fraction(k, 1 << F) for k in range(0, 1 << W)]
            cells = differ = left = 0
            for a, b in product(vals, repeat=2):
                cells += 1
                if (a * b) * (1 << F) != int((a * b) * (1 << F)):
                    left += 1
                if arm_every_node(mode, a, b, (F, F), lo, hi) != \
                        arm_46(mode, a, b, (F, F), lo, hi):
                    differ += 1
            print(f"  {mode:<12} {F:<4} {cells:>6} {left:>14} {differ:>12}")

    print()
    print("CASE B. The grid coarsens between nodes, one container throughout.")
    print("`122` 4.6 says 'the grid part must be applied at the result of every")
    print("node whose exact result can leave the grid'. With two grids in play")
    print("the phrase does not say WHICH grid, and the two readings differ.")
    print()
    print(f"  {'mode':<12} {'F chain':<10} {'cells':>6} "
          f"{'read as each node grid':>23} {'read as final grid':>19}")
    for mode in MODES:
        for Fa, Fb in ((2, 1), (3, 1)):
            lo, hi = rng(W, signed, Fa)
            vals = [Fraction(k, 1 << Fa) for k in range(0, 1 << W)]
            cells = d_each = d_final = 0
            for a, b in product(vals, repeat=2):
                cells += 1
                base = arm_every_node(mode, a, b, (Fa, Fb), lo, hi)
                if arm_46(mode, a, b, (Fa, Fb), lo, hi) != base:
                    d_each += 1
                if arm_46(mode, a, b, (Fb, Fb), lo, hi) != base:
                    d_final += 1
            print(f"  {mode:<12} {f'{Fa}->{Fb}':<10} {cells:>6} "
                  f"{d_each:>23} {d_final:>19}")
    print()
    print("  The two columns are the two readings of one sentence. Where they")
    print("  differ, the sentence is ambiguous rather than wrong, and the")
    print("  ambiguity is exactly what `126` names as unaddressed.")

    # --- P4: staged vs direct narrowing
    print()
    print("-" * 92)
    print("P4 and C3. Staged narrowing against direct narrowing, which is a")
    print("different equality and is the one `126` Finding 5 measures.")
    print()
    print(f"  {'mode':<12} {'F_exact -> F_acc -> F_final':<28} {'cells':>6} "
          f"{'staged != direct':>17}")
    for mode in MODES:
        for Fe, Fa, Ff in ((4, 3, 1), (4, 2, 1), (5, 3, 2)):
            cells = differ = 0
            for k in range(-(1 << 6), (1 << 6) + 1):
                x = Fraction(k, 1 << Fe)
                cells += 1
                if q(mode, q(mode, x, Fa), Ff) != q(mode, x, Ff):
                    differ += 1
            print(f"  {mode:<12} {f'{Fe} -> {Fa} -> {Ff}':<28} {cells:>6} "
                  f"{differ:>17}")
    print()
    print("  P4 holds when the directed modes read zero and the nearest modes do")
    print("  not, which is C3.")


def main():
    hole_a()
    hole_b()
    print()
    print("=" * 92)
    print(
        """
  READING IT

  Hole A is settled by the second and third columns of the first table. If
  bit-drop is floor everywhere and differs from toward-zero on signed rows only,
  then "truncation" names two different operations on signed domains and a
  predicate spelled with it does not name a mode. That is a fact about the
  operations rather than a preference between spellings.

  Hole B is settled by comparing P3's column to P4's. They are two different
  equalities and only one of them is at risk.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
