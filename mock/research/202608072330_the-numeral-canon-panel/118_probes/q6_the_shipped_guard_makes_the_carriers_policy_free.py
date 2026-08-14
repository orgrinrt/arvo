#!/usr/bin/env python3
"""q6. Correcting `114` section 6.4 against `116` F116-6.

WHAT `114` SECTION 6.4 CLAIMED
-------------------------------
That `warm-clamp-shared`'s `fold_chunked` "is arm W1 already": a const predicate
selects between a wrapping fold reduced once at the end of the chunk and a fold
reduced at every node, and the predicate `accumulator_bits_needed(W, ARITY) <=
A::BITS` is the corner rule at the root of a fold. The source reading is right and
`115` section 0.2 verified it independently at `warm-clamp-shared/src/lib.rs:288`,
`:291`, `:296-306` and `:158-160`.

WHAT `116` F116-6 SAYS ABOUT IT
--------------------------------
On a discharged extent the realisation map is the identity, and the identity is
both a ring homomorphism and monotone, so a discharged declaration escapes the
trade that no policy can escape. If the shipped guard makes the carrier's map the
identity on the reachable range, then the kernel is an instance of **that**,
rather than of arm W1, and the difference is not cosmetic: arm W1 needs the
carrier to be **wrapping**, and the identity needs nothing of the carrier at all.

The guard is `accumulator_bits_needed(W, ARITY) = W + ceil_log2(ARITY)`, which is
the bit width of `ARITY * (2^W - 1)`, which is the corner rule's upper endpoint
for a fold of `ARITY` leaves each declared at the full width. So under the guard
no accumulator addition can leave the carrier's range, and the carrier's map is
never applied to anything outside it.

THE TESTABLE CONSEQUENCE
-------------------------
If that is right, **the carrier's overflow policy is free under the guard**: the
safe branch computes the same answer whether the accumulator wraps or saturates.
That is a stronger and more useful statement than "this is arm W1", because it
says the kernel does not depend on the property arm W1 rests on, and a design
reading it as arm W1 would think it had to keep the carrier wrapping.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. Under the guard, the safe branch agrees with the oracle at every arity and
    width swept, with a wrapping carrier and with a saturating carrier, and the
    two carriers agree with each other.
P2. Without the guard, the two carriers disagree, and the wrapping one disagrees
    with the oracle. That is the control that makes P1 a result.
P3. The guard is exactly the corner rule at the root: the smallest accumulator
    width at which the safe branch is correct equals `W + ceil_log2(ARITY)` at
    every arity swept, neither larger nor smaller.
P4. The unsafe branch is correct at every accumulator width, since it reduces at
    every node, so the guard is about the safe branch alone.

NEGATIVE CONTROLS
-----------------
C1. P2 is the primary control. If the carriers agree without the guard too, the
    guard is not what makes them agree.
C2. A guard off by one in the permissive direction must produce disagreement, or
    P3's "neither larger nor smaller" is untested on the side that matters.
C3. The sweep must contain inputs whose accumulator sum exceeds the carrier at
    the widths where the guard refuses, or the whole comparison runs where
    nothing could differ. The count is reported.
"""

from itertools import product
import random
import sys


def ceil_log2(n):
    k = 0
    while (1 << k) < n:
        k += 1
    return k


def accumulator_bits_needed(w, arity):
    """warm-clamp-shared/src/lib.rs:158-160, transcribed."""
    return w + ceil_log2(arity)


def carrier_reduce(v, bits, policy):
    hi = (1 << bits) - 1
    if policy == "wrap":
        return v % (1 << bits)
    return 0 if v < 0 else (hi if v > hi else v)


def limit(w):
    """warm-clamp-shared's `A::limit(W)`: the largest value of the logical width."""
    return (1 << w) - 1


def safe_branch(chunk, w, abits, policy):
    """The shipped `safe` branch: wrapping (or whatever the carrier does) adds
    through the chunk, then one clamp to the logical width."""
    acc = 0
    for x in chunk:
        acc = carrier_reduce(acc + x, abits, policy)
    return min(acc, limit(w))


def unsafe_branch(chunk, w, abits, policy):
    """The shipped `!safe` branch: saturating add and clamp at every node."""
    acc = 0
    lim = limit(w)
    for x in chunk:
        acc = carrier_reduce(acc + x, abits, "sat")
        acc = min(acc, lim)
    return acc


def oracle(chunk, w):
    """Exact sum, clamped once to the logical width. What both branches mean."""
    return min(sum(chunk), limit(w))


def sweep(w, arity, abits, policy, trials, rng):
    """Returns (disagreements with the oracle, cells, cells where the exact sum
    exceeds the carrier)."""
    bad = over = 0
    hi = limit(w)
    for _ in range(trials):
        chunk = [rng.randint(0, hi) for _ in range(arity)]
        if safe_branch(chunk, w, abits, policy) != oracle(chunk, w):
            bad += 1
        if sum(chunk) > (1 << abits) - 1:
            over += 1
    return bad, trials, over


def main():
    print("=" * 104)
    print("q6. The shipped guard makes the carrier's policy free")
    print("=" * 104)

    rng = random.Random(20260814)
    print()
    print("The guard, transcribed from warm-clamp-shared/src/lib.rs:158-160:")
    print("    accumulator_bits_needed(w, arity) = w + ceil_log2(arity)")
    print()
    print(f"  {'W':>3} {'arity':>6} {'needed':>7}   " +
          "  ".join(f"{h:>26}" for h in
                    ("guard met: wrap / sat / agree", "guard short by 1: wrap / sat")))
    for w, arity in ((3, 4), (3, 8), (4, 4), (4, 16), (5, 8), (6, 4)):
        need = accumulator_bits_needed(w, arity)
        bw, n, ov = sweep(w, arity, need, "wrap", 4000, rng)
        bs, _, _ = sweep(w, arity, need, "sat", 4000, rng)
        agree = 0
        rng2 = random.Random(99)
        for _ in range(4000):
            chunk = [rng2.randint(0, limit(w)) for _ in range(arity)]
            if safe_branch(chunk, w, need, "wrap") == safe_branch(chunk, w, need, "sat"):
                agree += 1
        bw2, _, ov2 = sweep(w, arity, need - 1, "wrap", 4000, rng)
        bs2, _, _ = sweep(w, arity, need - 1, "sat", 4000, rng)
        print(f"  {w:>3} {arity:>6} {need:>7}   "
              f"{bw:>8} / {bs:<6} / {agree}/{n:<6}   {bw2:>10} / {bs2:<12}")
    print()
    print("  Columns are disagreements with the oracle out of 4000 random chunks.")
    print("  'agree' counts chunks where the two carriers give the same answer.")
    print("  The last pair is C2: the guard lowered by one bit, where the")
    print("  disagreement must appear or 'neither larger nor smaller' is untested.")

    # ------------------------------------------------------------------- P3
    print()
    print("-" * 104)
    print("P3. Is the guard exactly the corner rule at the root of the fold?")
    print("The smallest accumulator width at which the safe branch is correct,")
    print("found by search, against what the shipped formula says.")
    print()
    print(f"  {'W':>3} {'arity':>6} {'formula':>8} {'smallest correct, wrap':>24} "
          f"{'smallest correct, sat':>23}")
    for w, arity in ((3, 4), (3, 8), (4, 4), (4, 16), (5, 8), (2, 32)):
        need = accumulator_bits_needed(w, arity)
        found = {}
        for policy in ("wrap", "sat"):
            for abits in range(1, need + 4):
                r = random.Random(4242)
                ok = True
                for _ in range(3000):
                    chunk = [r.randint(0, limit(w)) for _ in range(arity)]
                    if safe_branch(chunk, w, abits, policy) != oracle(chunk, w):
                        ok = False
                        break
                if ok:
                    found[policy] = abits
                    break
        print(f"  {w:>3} {arity:>6} {need:>8} {found.get('wrap', '-'):>24} "
              f"{found.get('sat', '-'):>23}")
    print()
    print("  A saturating carrier can be correct at a NARROWER width than a")
    print("  wrapping one, because saturation and the final clamp agree above the")
    print("  logical limit while wrapping does not. Where the two columns differ,")
    print("  the shipped guard is the WRAPPING one, which is the conservative")
    print("  choice and is what the formula computes.")

    # ------------------------------------------------------------------- P4
    print()
    print("-" * 104)
    print("P4 and C3. The unsafe branch, and whether the sweep could see anything.")
    print()
    print(f"  {'W':>3} {'arity':>6} {'abits':>6} {'unsafe branch wrong':>21} "
          f"{'chunks whose sum exceeds the carrier':>38}")
    for w, arity, abits in ((3, 8, 3), (3, 8, 5), (4, 16, 4), (4, 16, 8)):
        r = random.Random(7)
        bad = over = 0
        for _ in range(4000):
            chunk = [r.randint(0, limit(w)) for _ in range(arity)]
            if unsafe_branch(chunk, w, abits, "sat") != oracle(chunk, w):
                bad += 1
            if sum(chunk) > (1 << abits) - 1:
                over += 1
        print(f"  {w:>3} {arity:>6} {abits:>6} {bad:>21} {over:>38}")

    print()
    print("=" * 104)
    print(
        """
  READING IT

  P1 holds when the guard-met columns are zero for BOTH carriers and 'agree' is
  the full count. That says the kernel's correctness does not read the carrier's
  overflow policy, so it is `116` F116-6's identity rather than `114`'s arm W1.

  P2 and C1 are the guard-short-by-one columns. They must be nonzero, or the
  guard is not what is doing the work.

  P3's two columns differing is the finding underneath: the shipped formula
  computes the width a WRAPPING carrier needs, which is conservative for a
  saturating one. That is a real slack a design could take.

  C3 is the last column: the sweep must contain chunks whose sum exceeds the
  carrier at the narrow widths, or nothing could have differed anywhere.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
