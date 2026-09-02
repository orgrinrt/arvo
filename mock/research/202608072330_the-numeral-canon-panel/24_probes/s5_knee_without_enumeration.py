#!/usr/bin/env python3
"""
s5: the knee is not affine, but it does not need a list either.

Section 5.2 of the deliverable listed, as the general form's largest cost, that s3's grid
carries the canonical exponent as one affine piece, which covers constant and slope one and
slope two but not gradual underflow. 08:392-396 records that going to the full function
space collides with a refusal of enumeration ratified four times.

This probe attacks that. The claim under test:

    Every canonical exponent the design names is
        f(e) = max(K, e + I)
    for two integers K and I. Constant is the case where the sloped piece never wins over
    the reach; slope one is the case where the floor never wins; the knee is the case where
    both win somewhere. So the design's admitted set is a two-integer shape and not a list.

And the consequence, which is the interesting half:

    Q1. Every design shape is expressible in that form.
    Q2. The form is CLOSED under the meet, because the pointwise max of two of them is
        one of them:
            max(max(K1, e+I1), max(K2, e+I2)) = max(max(K1,K2), e + max(I1,I2))
        checked exhaustively rather than only argued.
    Q3. The form is NOT closed under the join, because the pointwise min of two of them is
        generally not of the form. That is 08's measured meet-closed-not-join-closed
        asymmetry (08:603-607), and here it falls out of the shape rather than being
        surveyed.
    Q4. The join's result is the mirror shape 08:436-440 describes, "slope one at the
        bottom and slope zero above", which is min(K, e+I). So the family closed under both
        is max-of-min-of-affine, which is where the list worry comes back, and the design
        does not need it unless it wants the join total.

08:625-628 is the source for the meet being the pointwise maximum: "every pairwise
intersection in the pool has canonical exponent slopes drawn from {0, 1} ... Intersection
takes the pointwise maximum".

Run:  python3 s5_knee_without_enumeration.py
"""

from itertools import product

EMIN, EMAX = -40, 40  # the binade window every check runs over


def f_maxaffine(K, I, e):
    return max(K, e + I)


def sample(K, I):
    return tuple(f_maxaffine(K, I, e) for e in range(EMIN, EMAX + 1))


def in_family(vals):
    """Is this sampled function of the form max(K, e + I) over the window? Recover the
    parameters if so. Returns (K, I) or None."""
    # I is fixed by the last point if the sloped piece wins at the top; K by the first if
    # the floor wins at the bottom. Rather than reason, search the finite candidate set.
    lo, hi = min(vals), max(vals)
    for K in range(lo - 1, hi + 2):
        for I in range(lo - EMAX - 1, hi - EMIN + 2):
            if sample(K, I) == vals:
                return (K, I)
    return None


def main():
    print("s5: expressing every named canonical exponent as max(K, e + I)")
    print(f"window: binades {EMIN}..{EMAX}")
    print()

    # ---- Q1: the design's shapes ---------------------------------------------------
    print("Q1: the design's named shapes in the two-integer form")
    shapes = [
        # (label, K, I, what it should look like over the window)
        ("fixed point, f == -F, F=8",        -8, -1000, "constant"),
        ("fixed point, f == -F, F=0",         0, -1000, "constant"),
        ("float, p=24, f(e)=e-23",        -10000,   -23, "slope one"),
        ("float, p=53, f(e)=e-52",        -10000,   -52, "slope one"),
        ("gradual underflow, p=24, emin=-14", -37,   -23, "knee"),
        ("gradual underflow, p=11, emin=-14", -24,   -10, "knee"),
    ]
    ok_q1 = 0
    for label, K, I, want in shapes:
        vals = sample(K, I)
        slopes = {vals[i + 1] - vals[i] for i in range(len(vals) - 1)}
        if slopes == {0}:
            got = "constant"
        elif slopes == {1}:
            got = "slope one"
        elif slopes == {0, 1}:
            got = "knee"
        else:
            got = f"other {sorted(slopes)}"
        agree = got == want
        ok_q1 += agree
        print(f"   {'ok ' if agree else 'XXX'} {label:<38} K={K:<7} I={I:<7} -> {got}")
    print(f"   -> {ok_q1}/{len(shapes)}")
    print()

    # ---- Q2 and Q3: closure ---------------------------------------------------------
    # a pool spanning all three shape kinds, with the sentinel-ish extremes replaced by
    # values inside the window so the closure question is non-degenerate.
    pool = [(K, I) for K, I in product(range(-12, 5, 2), range(-12, 5, 2))]
    print(f"Q2/Q3: closure over a pool of {len(pool)} grids, {len(pool)**2} ordered pairs")

    meet_in = meet_out = 0
    join_in = join_out = 0
    join_witness = None
    identity_holds = True
    for (K1, I1), (K2, I2) in product(pool, pool):
        a, b = sample(K1, I1), sample(K2, I2)

        # meet: pointwise maximum
        m = tuple(max(x, y) for x, y in zip(a, b))
        # the identity, checked rather than assumed
        if m != sample(max(K1, K2), max(I1, I2)):
            identity_holds = False
        if in_family(m) is not None:
            meet_in += 1
        else:
            meet_out += 1

        # join: pointwise minimum
        j = tuple(min(x, y) for x, y in zip(a, b))
        if in_family(j) is not None:
            join_in += 1
        else:
            join_out += 1
            if join_witness is None:
                join_witness = ((K1, I1), (K2, I2), j)

    total = len(pool) ** 2
    print(f"   meet (pointwise max) stays in the family : {meet_in}/{total}")
    print(f"   the closed form max(max(K1,K2), e+max(I1,I2)) holds : {identity_holds}")
    print(f"   join (pointwise min) stays in the family : {join_in}/{total}")
    print(f"   join leaves the family                   : {join_out}/{total}")
    print()

    def slope_runs(vals):
        slopes = [vals[k + 1] - vals[k] for k in range(len(vals) - 1)]
        runs = []
        for sl in slopes:
            if runs and runs[-1][0] == sl:
                runs[-1][1] += 1
            else:
                runs.append([sl, 1])
        return [tuple(r) for r in runs]

    print("Q4: what the join lands in, for the two cases that differ")
    print()

    # Case A: the one 08:436-440 measured, a pure fixed shape against a pure float.
    # Pure fixed: floor wins everywhere. Pure float: floor never wins.
    fixed = sample(-8, -10000)      # f == -8 throughout
    flt = sample(-10000, -23)       # f == e - 23 throughout
    jA = tuple(min(x, y) for x, y in zip(fixed, flt))
    print("   A. join of a pure fixed shape (f == -8) and a pure float (f == e-23)")
    print(f"      slope runs: {slope_runs(jA)}")
    print(f"      in the family max(K, e+I)? {in_family(jA) is not None}")
    print("      slope one at the bottom then slope zero above, which is 08:436-440's")
    print("      mirror shape. It is min(K, e+I), one flip of the same two integers.")
    print()

    # Case B: the join of two knees, which is what the pool witness above actually is.
    if join_witness:
        (p1, p2, j) = join_witness
        print(f"   B. join of two knees, {p1} and {p2}")
        print(f"      slope runs: {slope_runs(j)}")
        print(f"      in the family max(K, e+I)? {in_family(j) is not None}")
        print("      four segments, so it is not the mirror shape either. The join of two")
        print("      knees is strictly worse than the join 08 measured, and this is the")
        print("      case 08 did not have because its pool's join question was asked")
        print("      across kinds rather than within the knee family.")
    print()

    print("Reading:")
    print("  The knee needs two integers, not a list. The design's whole admitted set is")
    print("  the two-integer form, so the enumeration collision 08:392-396 names does not")
    print("  arise for the shapes the design actually has.")
    print("  Meet-closure is an algebraic identity on those two integers rather than a")
    print("  survey result, and non-closure under the join is the same identity failing")
    print("  in the other direction. 08 measured both; this says why.")
    print("  A design wanting the join total needs at least min-of-max-of-affine, and the")
    print("  segment count grows with each join, which is where the function-space")
    print("  question actually starts. Refusing the join keeps the shape at two integers.")


if __name__ == "__main__":
    main()
