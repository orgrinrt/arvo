#!/usr/bin/env python3
"""i2e: the shape count again, with a defect in i2b/i2c/i2d repaired.

THE DEFECT. i2b bounded the canonical exponent by `fexp(e) >= e - depth`, a
bound that moves with the binade. That silently forbids a constant, because a
constant would have to be at least `W-1-depth` at the top binade and at most 0
at the bottom. i2d reported `fixed alone n=0` at windows 6 and 8 and I read
that as a fact about fixed-point formats when it was a fact about my own
enumeration bound. `RULES.md` records the predecessor panel doing exactly this:
"one expert's headline counts turned out to be an artifact of its own
enumeration bound".

The three earlier instruments are kept unchanged with their outputs, because
the defect is worth more visible than repaired in place. What survives from
them is the shape of the answer; the counts below supersede theirs.

THE REPAIR. Bound the canonical exponent by a fixed precision floor,
`-P <= fexp(e) <= e`, which does not move with the binade. A constant is then
admitted at every window, and the question is asked of a set that contains all
three named shapes at every size.
"""

import itertools


def shapes(W, P):
    es = list(range(W))
    out = set()
    for combo in itertools.product(*[range(-P, e + 1) for e in es]):
        if any(combo[i + 1] < combo[i] for i in range(len(combo) - 1)):
            continue
        out.add(combo)
    return out


def families(W, P, A):
    es = list(range(W))
    fixed = {v for v in ({tuple(c for _ in es) for c in range(-P, 1)}) if v in A}
    flt = {v for v in ({tuple(e - k for e in es) for k in range(0, P + W)}) if v in A}
    knee = set()
    for k in range(0, P + W):
        for K in range(-P, W):
            v = tuple(max(e - k, K) for e in es)
            if v in A:
                knee.add(v)
    return fixed, flt, knee


def close_under(op, seed, A):
    cur = set(seed)
    while True:
        new = set()
        for a, b in itertools.combinations(list(cur), 2):
            m = tuple(op(x, y) for x, y in zip(a, b))
            if m in A and m not in cur:
                new.add(m)
        if not new:
            return cur
        cur |= new


def describe(v):
    slopes = [v[i + 1] - v[i] for i in range(len(v) - 1)]
    seq = []
    for s in slopes:
        if not seq or seq[-1][0] != s:
            seq.append([s, 1])
        else:
            seq[-1][1] += 1
    return " then ".join(f"slope {s} x{n}" for s, n in seq) or "single binade"


if __name__ == "__main__":
    print("Shapes over W binades with a precision floor P, monotone, inhabited.")
    print(f"{'W':>3} {'P':>3} {'all':>9} {'fixed':>6} {'float':>6} {'knee':>6} "
          f"{'named':>6} {'meet-cl':>8} {'join-cl':>8} {'tapered':>8}")
    for W, P in ((4, 4), (5, 5), (6, 5), (6, 6), (7, 6)):
        A = shapes(W, P)
        fx, fl, kn = families(W, P, A)
        N = fx | fl | kn
        M = close_under(max, N, A)
        J = close_under(min, N, A)
        tap = sum(1 for v in A if any(v[i + 1] - v[i] >= 2 for i in range(len(v) - 1)))
        print(f"{W:>3} {P:>3} {len(A):>9} {len(fx):>6} {len(fl):>6} {len(kn):>6} "
              f"{len(N):>6} {len(M):>8} {len(J):>8} {tap:>8}")
    print()

    W, P = 5, 5
    A = shapes(W, P)
    fx, fl, kn = families(W, P, A)
    N = fx | fl | kn
    print(f"At W={W}, P={P}: {len(N)} named of {len(A)} shapes.")
    M = close_under(max, N, A)
    J = close_under(min, N, A)
    print(f"    meet closure adds {len(M) - len(N)}")
    print(f"    join closure adds {len(J) - len(N)}")
    print()
    print("Per family, both operations:")
    for name, S in (("fixed alone", fx), ("float alone", fl),
                    ("float with underflow alone", kn),
                    ("fixed + float", fx | fl), ("all three", N)):
        m = close_under(max, S, A)
        j = close_under(min, S, A)
        print(f"    {name:28s} n={len(S):>3}  meet={len(m):>3}  join={len(j):>3}"
              + ("" if len(j) == len(S) and len(m) == len(S) else "   <- leaks"))
    print()
    print("Three joins of a fixed shape with a float shape, in full:")
    shown = 0
    for a in sorted(fx):
        for b in sorted(fl):
            m = tuple(min(x, y) for x, y in zip(a, b))
            if m in A and m not in N:
                print(f"    fixed {a}")
                print(f"  v float {b}")
                print(f"  =       {m}   -> {describe(m)}, unnamed")
                shown += 1
                if shown == 3:
                    break
        if shown == 3:
            break
    if shown == 0:
        print("    none: every such join is already a named shape.")
    print()
    print("Three meets of a fixed shape with a float shape, in full:")
    shown = 0
    for a in sorted(fx):
        for b in sorted(fl):
            m = tuple(max(x, y) for x, y in zip(a, b))
            if m in A and m not in N:
                print(f"    UNNAMED MEET {a} ^ {b} = {m} -> {describe(m)}")
                shown += 1
                if shown == 3:
                    break
        if shown == 3:
            break
    if shown == 0:
        print("    none: every pointwise maximum of a fixed and a float shape is named.")
