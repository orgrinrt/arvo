#!/usr/bin/env python3
"""i2c: the named shapes are closed under one operation and not the other.

i2b measured that closing the three named exponent shapes under pointwise
maximum adds nothing at every window tried. That was not what I expected and it
sends the question the other way: intersection takes the pointwise maximum, and
union takes the pointwise minimum. So this instrument asks the same closure
question for the minimum, and reports what the shapes the minimum produces
actually look like.

If the answer is that the named set is closed under maximum and not under
minimum, then two things in the record are the same fact:

  `SETTLED_laws.md:278-288`  meets are exact, joins strictly overshoot
  `03` section 3.2          the cross-kind join has no least upper bound

and the overshoot is the design rounding an unnamed shape up to a named one.
"""

import itertools

WINDOWS = [(4, 3), (6, 4), (8, 4), (10, 5)]


def shapes(window, depth):
    es = list(range(window))
    out = set()
    for combo in itertools.product(*[range(e - depth, e + 1) for e in es]):
        if any(combo[i + 1] < combo[i] for i in range(len(combo) - 1)):
            continue
        out.add(combo)
    return out


def named(window, depth, allshapes):
    es = list(range(window))
    n = {}
    for c in range(-depth, window):
        v = tuple(c for _ in es)
        if v in allshapes:
            n[v] = "fixed (constant)"
    for c in range(0, depth + 1):
        v = tuple(e - c for e in es)
        if v in allshapes:
            n[v] = "float (slope one)"
    for c in range(0, depth + 1):
        for knee in es:
            v = tuple(max(e - c, knee - c) for e in es)
            if v in allshapes and v not in n:
                n[v] = "float with gradual underflow (constant, then slope one)"
    return n


def close_under(op, seed, allshapes):
    cur = set(seed)
    while True:
        new = set()
        for a, b in itertools.combinations(list(cur), 2):
            m = tuple(op(x, y) for x, y in zip(a, b))
            if m in allshapes and m not in cur:
                new.add(m)
        if not new:
            return cur
        cur |= new


def describe(v):
    slopes = [v[i + 1] - v[i] for i in range(len(v) - 1)]
    if all(s == 0 for s in slopes):
        return "constant"
    if all(s == 1 for s in slopes):
        return "slope one"
    seq = []
    for s in slopes:
        if not seq or seq[-1][0] != s:
            seq.append([s, 1])
        else:
            seq[-1][1] += 1
    return " then ".join(f"slope {s} x{n}" for s, n in seq)


if __name__ == "__main__":
    print(f"{'window':>7} {'depth':>6} {'all':>8} {'named':>6} "
          f"{'meet-closed':>12} {'join-closed':>12}")
    for window, depth in WINDOWS:
        A = shapes(window, depth)
        N = named(window, depth, A)
        M = close_under(max, N.keys(), A)
        J = close_under(min, N.keys(), A)
        print(f"{window:>7} {depth:>6} {len(A):>8} {len(N):>6} "
              f"{len(M):>12} {len(J):>12}")
    print()

    window, depth = 6, 4
    A = shapes(window, depth)
    N = named(window, depth, A)
    J = close_under(min, N.keys(), A)
    added = sorted(J - set(N))
    print(f"At window {window}, depth {depth}: the join closure adds "
          f"{len(added)} shapes to {len(N)} named.")
    kinds = {}
    for v in added:
        kinds[describe(v)] = kinds.get(describe(v), 0) + 1
    print("The added shapes, by the run-length of their slopes:")
    for k in sorted(kinds, key=lambda k: -kinds[k]):
        print(f"    {kinds[k]:>4}  {k}")
    print()
    print("Three added shapes in full, with the pair that produced each:")
    shown = 0
    for a, b in itertools.combinations(sorted(N.keys()), 2):
        m = tuple(min(x, y) for x, y in zip(a, b))
        if m in A and m not in N:
            print(f"    {a}  ({N[a]})")
            print(f"  v {b}  ({N[b]})")
            print(f"  = {m}  -> {describe(m)}, unnamed")
            shown += 1
            if shown == 3:
                break
    print()
    print("And the same question asked of the meet, for symmetry:")
    shownm = 0
    for a, b in itertools.combinations(sorted(N.keys()), 2):
        m = tuple(max(x, y) for x, y in zip(a, b))
        if m in A and m not in N:
            print(f"    UNNAMED MEET: {a} ^ {b} = {m}")
            shownm += 1
            if shownm == 3:
                break
    if shownm == 0:
        print("    none: every pointwise maximum of two named shapes is named.")
