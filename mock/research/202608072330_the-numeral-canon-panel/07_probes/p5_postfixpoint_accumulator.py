#!/usr/bin/env python3
"""p5. The fold's accumulator as a post-fixpoint, and what soundness costs.

The droplist records that growing an accumulator's own TYPE per iteration cannot
work in principle, since a type cannot depend on a runtime value, and that the
replacement is to fix the per-element type and check accumulator sufficiency at
compile time.  It does not say what sufficiency IS.

The adjunction frame supplies a candidate, and the candidate is the standard one
for reasoning about an unbounded loop with a fixed abstract element: A is
sufficient exactly when it is a POST-FIXPOINT of the abstract step,

    step#(A, P)  <=  A,

because then, by induction on the trip count and monotonicity of step#, the
concrete accumulation after any number of iterations lies inside g(A).  The trip
count never enters, which is exactly the property the droplist entry needs.

Two preconditions are doing work in that argument and this probe isolates both.

  (i)  step# must be MONOTONE, or the induction has nothing to run on.
  (ii) g must be the concretisation the design actually intends.  If the
       saturating top denotes only its own value, saturation is unsound; if it
       denotes "at least this", saturation is sound.  That is a choice about what
       a datum means, not about arithmetic, and the record does not appear to
       have made it.

Measured exhaustively at small widths, per resolution.  Wrapping, saturating and
substitute-zero, which are the three the record names.
"""

from fractions import Fraction as Q
import itertools

# ------------------------------------------------------------------ numerals


def vset(I, F):
    q = Q(1, 2**F)
    return [k * q for k in range(2 ** (I + F))]


# --------------------------------------------------------------- resolutions
# Each takes an exact rational and the accumulator's value list, and returns a
# member of that list.  These are the three the record names.


def res_wrap(x, V):
    n = len(V)
    step = V[1] - V[0]
    k = int(x / step)
    return V[k % n]


def res_sat(x, V):
    if x < V[0]:
        return V[0]
    if x > V[-1]:
        return V[-1]
    step = V[1] - V[0]
    return V[int(x / step)]


def res_zero(x, V):
    if x < V[0] or x > V[-1]:
        return V[0]
    step = V[1] - V[0]
    return V[int(x / step)]


RES = [("wrapping", res_wrap), ("saturating", res_sat), ("substitute zero", res_zero)]


# ------------------------------------------------------ Q1: monotone or not


def q1():
    print("=== Q1. Is the resolved step monotone in the accumulator? ===")
    print("    a |-> R(a + p, V) for each fixed p, checked over every pair a <= a'.")
    for I, F in [(3, 0), (2, 1), (4, 0)]:
        V = vset(I, F)
        print(f"  U<{I},{F}>, {len(V)} values")
        for name, R in RES:
            bad = 0
            wit = None
            for p in V:
                for a, b in itertools.combinations(V, 2):
                    ra, rb = R(a + p, V), R(b + p, V)
                    if not ra <= rb:
                        bad += 1
                        if wit is None:
                            wit = (a, b, p, ra, rb)
            tot = len(V) * len(list(itertools.combinations(V, 2)))
            msg = ""
            if wit:
                msg = (f"   first: a={wit[0]} <= a'={wit[1]}, p={wit[2]}, "
                       f"but R gives {wit[3]} > {wit[4]}")
            print(f"    {name:>16} | not monotone at {bad:>5} of {tot:>5}{msg}")


# --------------------------------------- Q2: the post-fixpoint, and soundness


def q2():
    print()
    print("=== Q2. Post-fixpoint holds; does soundness follow, and under which g? ===")
    print("    Point reading   : a datum denotes exactly its own value.")
    print("    Absorbing reading: the top denotes [top, infinity), everything else itself.")
    print()
    IA, FA = 3, 0          # accumulator U<3,0>: 0..7
    IP, FP = 2, 0          # element     U<2,0>: 0..3
    VA, VP = vset(IA, FA), vset(IP, FP)
    print(f"    accumulator U<{IA},{FA}> = {VA}")
    print(f"    element     U<{IP},{FP}> = {VP}")
    print()
    print(f"{'resolution':>16} | {'n':>2} | {'seqs':>6} | {'unsound, point':>14} | {'unsound, absorbing':>18}")
    print("-" * 74)
    for name, R in RES:
        # post-fixpoint check: is R(a+p) always in VA?  It is, by construction of
        # R, so the interesting quantity is soundness rather than membership.
        for n in range(1, 6):
            pt = ab = 0
            wit = None
            for seq in itertools.product(VP, repeat=n):
                acc = VA[0]
                for p in seq:
                    acc = R(acc + p, VA)
                exact = sum(seq)
                if acc != exact:
                    pt += 1
                    if wit is None and exact <= VA[-1]:
                        wit = ("in range", seq, exact, acc)
                # absorbing reading: acc is sound if exact == acc, or if acc is
                # the top and exact >= top.
                if not (acc == exact or (acc == VA[-1] and exact >= VA[-1])):
                    ab += 1
                    if wit is None:
                        wit = ("absorbing", seq, exact, acc)
            print(f"{name:>16} | {n:>2} | {len(VP)**n:>6} | {pt:>14} | {ab:>18}")
        if wit:
            print(f"{'':>16}   witness [{wit[0]}] seq={list(wit[1])} exact={wit[2]} abstract={wit[3]}")
        print()


# ------------------------------- Q3: n-independence, the point of the shape


def q3():
    print("=== Q3. Does the absorbing reading's soundness stay n-independent? ===")
    print("    A post-fixpoint argument's whole value is that the trip count never")
    print("    enters.  If the failure count under the absorbing reading is zero for")
    print("    every n tried, that is the shape holding; if it grows with n, it is not.")
    IA, FA = 3, 0
    IP, FP = 2, 0
    VA, VP = vset(IA, FA), vset(IP, FP)
    for name, R in RES:
        row = []
        for n in range(1, 9):
            bad = 0
            for seq in itertools.product(VP, repeat=n):
                acc = VA[0]
                for p in seq:
                    acc = R(acc + p, VA)
                exact = sum(seq)
                if not (acc == exact or (acc == VA[-1] and exact >= VA[-1])):
                    bad += 1
            row.append(f"{bad}/{len(VP)**n}")
        print(f"{name:>16} | " + "  ".join(f"n={i+1}:{v}" for i, v in enumerate(row)))


if __name__ == "__main__":
    q1()
    q2()
    q3()


# ------------------------------------------------------------------ appendix
# Q2 and Q3 leave one thing unmeasured: WHICH inequality on the declared members
# the post-fixpoint condition reduces to.  The range half is discharged by the
# saturating resolution under the absorbing reading, at zero failures for every n
# tried, so it appears to need no bound at all.  That leaves the grid half.
#
# Candidate: the accumulator is sufficient exactly when its grid is at least as
# fine as the element's, F_A >= F_P, because otherwise a + p leaves the
# accumulator's grid even while in range and the resolved answer is not an
# over-approximation of anything.  Measured by sweeping the two fraction widths
# against each other, which is a decisive test: if the candidate is right the
# failures should be exactly the cells with F_A < F_P.


def q4():
    print()
    print("=== Q4. Which inequality is the sufficiency condition? ===")
    print("    Saturating, absorbing reading, n = 4.  Cell shows unsound sequences.")
    print("    Accumulator U<3,FA>, element U<1,FP>.")
    print()
    hdr = "    " + "FA\\FP |" + "".join(f"{fp:>10}" for fp in range(0, 4))
    print(hdr)
    print("    " + "-" * (len(hdr) - 4))
    for FA in range(0, 4):
        VA = vset(3, FA)
        row = []
        for FP in range(0, 4):
            VP = vset(1, FP)
            bad = 0
            for seq in itertools.product(VP, repeat=4):
                acc = VA[0]
                for p in seq:
                    acc = res_sat(acc + p, VA)
                exact = sum(seq)
                if not (acc == exact or (acc == VA[-1] and exact >= VA[-1])):
                    bad += 1
            row.append(f"{bad:>10}")
        print(f"      {FA:>4} |" + "".join(row))
    print()
    print("    Predicted: zero exactly on and above the diagonal FA >= FP.")


q4()
