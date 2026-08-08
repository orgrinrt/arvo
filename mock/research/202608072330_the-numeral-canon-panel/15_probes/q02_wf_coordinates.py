#!/usr/bin/env python3
# q02. If the numeral is keyed on (total width, fraction width) instead of
# (integer width, fraction width), does every shape rule stay inside the
# naturals, and is every rule expressible with the operations 13's nat ladder
# already has?
#
# q01 established that over the product's 6561 pairs the integer width goes
# negative at 15 and the total width and fraction width never do. This probe
# asks the follow-on question, which is whether the RULES can be written in the
# new coordinates using only:
#
#     successor / addition        (13's Add, AddC)
#     comparison                  (13's Cmp)
#     natural subtraction where the subtrahend is provably smaller
#
# and nothing else. Anything needing a general subtraction, a multiplication of
# widths, or an exponential is a wall and is reported as one.
#
# Reproduce:  python3 q02_wf_coordinates.py
# Toolchain:  python3 only. Exact integer arithmetic, not a measurement.

BOX = 9


def bits_for(units):
    return units.bit_length()


def box(n):
    return [(I, F) for I in range(n) for F in range(n)]


# ---- ground truth in (I, F), from q01's instrument B --------------------------

def prod_IF(a, b):
    (I1, F1), (I2, F2) = a, b
    W1, W2 = I1 + F1, I2 + F2
    if W1 == 0 or W2 == 0:
        return (0, 0)
    return (bits_for((2**W1 - 1) * (2**W2 - 1)) - (F1 + F2), F1 + F2)


def add_IF(a, b):
    (I1, F1), (I2, F2) = a, b
    F = max(F1, F2)
    m = (2 ** (I1 + F1) - 1) * 2 ** (F - F1) + (2 ** (I2 + F2) - 1) * 2 ** (F - F2)
    return (bits_for(m) - F, F)


# ---- candidate rules in (W, F), using only the permitted operations -----------

def prod_WF(w1f1, w2f2):
    """Claim: W_out depends on the two TOTAL widths alone, by one comparison.
    06 section 7.3 states the predicate is min(W1, W2) == 1 for widths at least
    one. Written here with no multiplication and no exponential."""
    (W1, F1), (W2, F2) = w1f1, w2f2
    if W1 == 0 or W2 == 0:
        return (0, 0)
    if W1 == 1 or W2 == 1:
        W = W1 + W2 - 1  # a natural, since both are at least 1
    else:
        W = W1 + W2
    return (W, F1 + F2)


def add_WF(w1f1, w2f2):
    """Claim: align to the finer fraction, take the max, add one.
    d_i = F_out - F_i is a natural because F_out is the max.
    A_i = W_i + d_i is a natural sum."""
    (W1, F1), (W2, F2) = w1f1, w2f2
    F = max(F1, F2)
    A1 = W1 + (F - F1)
    A2 = W2 + (F - F2)
    return (max(A1, A2) + 1, F)


def to_WF(IF):
    return (IF[0] + IF[1], IF[1])


def main():
    shapes = box(BOX)
    pairs = [(a, b) for a in shapes for b in shapes]
    print(f"pairs: {len(pairs)}")

    # ---- product ------------------------------------------------------------
    bad = []
    for a, b in pairs:
        want = to_WF(prod_IF(a, b))
        got = prod_WF(to_WF(a), to_WF(b))
        if want != got:
            bad.append((a, b, want, got))
    print(f"product: (W,F) rule disagrees with ground truth at {len(bad)} of {len(pairs)}")
    for a, b, w, g in bad[:8]:
        print(f"   U<{a[0]},{a[1]}> * U<{b[0]},{b[1]}>: want {w} got {g}")

    # ---- addition -----------------------------------------------------------
    bad = []
    for a, b in pairs:
        want = to_WF(add_IF(a, b))
        got = add_WF(to_WF(a), to_WF(b))
        if want != got:
            bad.append((a, b, want, got))
    print(f"addition: (W,F) rule disagrees with ground truth at {len(bad)} of {len(pairs)}")
    for a, b, w, g in bad[:8]:
        print(f"   U<{a[0]},{a[1]}> + U<{b[0]},{b[1]}>: want {w} got {g}")

    # ---- where the natural subtraction sits, and whether it is ever negative --
    negd = 0
    for a, b in pairs:
        (W1, F1), (W2, F2) = to_WF(a), to_WF(b)
        F = max(F1, F2)
        if F - F1 < 0 or F - F2 < 0:
            negd += 1
    print(f"addition's alignment subtraction F_out - F_i negative at {negd} of {len(pairs)}")

    # ---- and the coordinate that actually goes negative, for the record ------
    negI = sum(1 for a, b in pairs if prod_IF(a, b)[0] < 0)
    negW = sum(1 for a, b in pairs if to_WF(prod_IF(a, b))[0] < 0)
    print(f"product: I < 0 at {negI}, W < 0 at {negW}")

    # ---- the join and the meet in the new coordinates ------------------------
    # join in (I,F) is coordinatewise max. In (W,F) that is
    #   F = max(F1,F2);  W = max(W1 + (F-F1), W2 + (F-F2))
    # which is add_WF without the carry.
    bad = []
    for a, b in pairs:
        want = to_WF((max(a[0], b[0]), max(a[1], b[1])))
        (W1, F1), (W2, F2) = to_WF(a), to_WF(b)
        F = max(F1, F2)
        got = (max(W1 + (F - F1), W2 + (F - F2)), F)
        if want != got:
            bad.append((a, b, want, got))
    print(f"join: (W,F) rule disagrees at {len(bad)} of {len(pairs)}")

    # meet in (I,F) is coordinatewise min. In (W,F):
    #   F = min(F1,F2);  W = min(W1 - (F1-F), W2 - (F2-F))
    # both subtractions are naturals since F is the min. But W itself can be
    # negative when an I is negative, which cannot happen among ADMITTED shapes.
    bad = []
    for a, b in pairs:
        want = to_WF((min(a[0], b[0]), min(a[1], b[1])))
        (W1, F1), (W2, F2) = to_WF(a), to_WF(b)
        F = min(F1, F2)
        got = (min(W1 - (F1 - F), W2 - (F2 - F)), F)
        if want != got:
            bad.append((a, b, want, got))
    print(f"meet: (W,F) rule disagrees at {len(bad)} of {len(pairs)}")

    # ---- the real question for the meet: does it need a general subtraction? --
    # W_i - (F_i - F) = I_i + F. If some admitted shape had I < 0 this could go
    # below zero. Measure it over the box including the 15 product outputs fed
    # back in, which is what repeated multiplication does.
    outs = {prod_IF(a, b) for a, b in pairs}
    withneg = shapes + [s for s in outs if s[0] < 0]
    worst = 0
    for a in withneg:
        for b in withneg:
            (W1, F1), (W2, F2) = to_WF(a), to_WF(b)
            F = min(F1, F2)
            worst = min(worst, min(W1 - (F1 - F), W2 - (F2 - F)))
    print(f"meet over the box PLUS the negative-I outputs: least W reached = {worst}")

    # ---- repeated multiplication: does the corner compound? ------------------
    # 06 named this as untested. Square the worst shape repeatedly.
    s = (0, 1)
    print()
    print("repeated squaring of U<0,1>, in (I,F) then (W,F):")
    for k in range(6):
        W, F = to_WF(s)
        print(f"  step {k}: I={s[0]} F={s[1]}   W={W} F={F}")
        s = prod_IF(s, s)


if __name__ == "__main__":
    main()


# ---- appended: characterise the addition overshoot exactly -------------------
def addendum():
    shapes = box(BOX)
    pairs = [(a, b) for a in shapes for b in shapes]
    bad = [(a, b) for a, b in pairs
           if to_WF(add_IF(a, b)) != add_WF(to_WF(a), to_WF(b))]
    print()
    print(f"ADDENDUM. addition overshoot: {len(bad)} pairs")
    deg = [(a, b) for a, b in bad if a[0] + a[1] == 0 or b[0] + b[1] == 0]
    print(f"  of which at least one operand is the zero-only numeral W=0: {len(deg)}")
    rest = [(a, b) for a, b in bad if (a, b) not in set(deg)]
    print(f"  the remainder: {len(rest)}")
    for a, b in rest[:12]:
        print(f"    U<{a[0]},{a[1]}> + U<{b[0]},{b[1]}>: "
              f"want {to_WF(add_IF(a,b))} got {add_WF(to_WF(a), to_WF(b))}")
    nd = [(a, b) for a, b in pairs if a[0] + a[1] >= 1 and b[0] + b[1] >= 1]
    ndbad = [(a, b) for a, b in nd
             if to_WF(add_IF(a, b)) != add_WF(to_WF(a), to_WF(b))]
    print(f"  restricted to both operands non-degenerate: "
          f"{len(ndbad)} of {len(nd)} disagree")


addendum()
