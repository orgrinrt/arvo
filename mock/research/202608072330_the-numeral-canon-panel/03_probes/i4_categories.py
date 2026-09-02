"""Instrument 4: measurements for the readings op's three do not cover.

Tonight is a breadth pass, so these are the categories rather than variants inside one.

  Q17  is "one family per radix" a real seam? Measured over radix pairs that ARE
       commensurable (2 and 4, 2 and 8, 3 and 9) and pairs that are not (2 and 3,
       2 and 6). If the commensurable pairs behave as one family, the seam is the
       step set rather than the radix, and the radix carves nothing.

  Q18  the multi-cover reading. Instead of demanding a unique least upper bound,
       ask how many MINIMAL upper bounds a pair has. If that number is always small,
       a named tie-break makes a total operation out of a partial one, and the
       closure question stops deciding totality.

  Q19  the uniform-grid seam. Split the space by whether a shape's value set is an
       arithmetic progression rather than by what the design calls the shape.
"""

from fractions import Fraction as F
from itertools import combinations
from i1_shape_space import Space, uf, fl, shape_params, report


def q17():
    print("Q17 is the radix the seam, or is the step set the seam?")
    pairs = [(2, 4, "commensurable, 4 = 2^2"), (2, 8, "commensurable, 8 = 2^3"),
             (3, 9, "commensurable, 9 = 3^2"), (2, 3, "incommensurable"),
             (2, 6, "shares a factor, 6 = 2*3")]
    for r1, r2, note in pairs:
        lab = []
        for r, lim in ((r1, 4), (r2, 3)):
            for I in range(0, lim + 1):
                for Fw in range(0, lim + 1):
                    if 0 <= I + Fw <= lim:
                        lab.append((f"U{r}<{I},{Fw}>", uf(r, I, Fw)))
        sp = Space(lab)
        cross = nolst = nobound = ok = 0
        for i, j in combinations(range(sp.n), 2):
            ni, nj = sp.names[i], sp.names[j]
            a1, a2 = f"U{r1}<" in ni, f"U{r2}<" in ni
            b1, b2 = f"U{r1}<" in nj, f"U{r2}<" in nj
            if not ((a1 and b2) or (a2 and b1)):
                continue
            cross += 1
            uc = sp.above[i] & sp.above[j]
            if uc == 0:
                nobound += 1
            elif sp.least(uc) is None:
                nolst += 1
            else:
                ok += 1
        print(f"    radix {r1} with radix {r2} ({note})")
        print(f"      cross pairs {cross}: join present {ok}, no upper bound {nobound}, bounds-but-no-least {nolst}")
    print("    note the shape of the failure: across radices the join dies for want of any")
    print("    upper bound, not for want of a least one. Those are different repairs.")
    print()


def minimal_uppers(sp, i, j):
    uc = sp.above[i] & sp.above[j]
    ks = list(sp.bits(uc))
    return [k for k in ks if not any(m != k and sp.sets[m] < sp.sets[k] for m in ks)]


def q18():
    print("Q18 the multi-cover reading: how many MINIMAL upper bounds does a pair have?")
    lim = 5
    fx = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
    fls = [
        (f"F<p{p},e{a}..{b}>", fl(p, a, b))
        for p in range(1, 4)
        for a in range(-3, 1)
        for b in range(0, 3)
        if a <= b
    ]
    for tag, lab in (("fixed-point alone", fx), ("fixed-point and float", fx + fls)):
        sp = Space(lab)
        hist = {}
        for i, j in combinations(range(sp.n), 2):
            n = len(minimal_uppers(sp, i, j))
            hist[n] = hist.get(n, 0) + 1
        tot = sum(hist.values())
        print(f"    {tag}: pairs {tot}")
        for n in sorted(hist):
            label = "no upper bound at all" if n == 0 else ("a join" if n == 1 else f"{n} minimal upper bounds")
            print(f"      {hist[n]:>5}  {label}")
        big = max(hist)
        print(f"      widest antichain of minimal upper bounds seen: {big}")
    print("    a tie-break rule over a small antichain makes a total operation. what it")
    print("    costs is that the answer is chosen rather than forced.")
    print()


def q19():
    print("Q19 the uniform-grid seam, measured instead of assumed")
    lim = 5
    fx = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
    fls = [
        (f"F<p{p},e{a}..{b}>", fl(p, a, b))
        for p in range(1, 4)
        for a in range(-3, 1)
        for b in range(0, 3)
        if a <= b
    ]
    both = Space(fx + fls)
    uni = sum(1 for s in both.sets if shape_params(s) is not None)
    print(f"    points {both.n}: value sets that ARE an arithmetic progression {uni}, that are not {both.n - uni}")
    fl_uniform = [n for n, s in zip(both.names, both.sets) if "F<" in n and shape_params(s) is not None]
    print(f"    float-declared shapes whose value set is nevertheless uniform: {len(fl_uniform)}")
    print(f"      {fl_uniform[:6]}")
    print("    so the declared kind and the geometric shape are not the same partition, and")
    print("    a family boundary drawn on the declaration cuts through numerals that agree.")
    fail_pairs = {"uni-uni": [0, 0], "uni-non": [0, 0], "non-non": [0, 0]}
    for i, j in combinations(range(both.n), 2):
        ui = shape_params(both.sets[i]) is not None
        uj = shape_params(both.sets[j]) is not None
        key = "uni-uni" if ui and uj else ("non-non" if not ui and not uj else "uni-non")
        uc = both.above[i] & both.above[j]
        fail_pairs[key][1] += 1
        if uc == 0 or both.least(uc) is None:
            fail_pairs[key][0] += 1
    print("    join failures split by whether each side's value set is uniform:")
    for k, (bad, tot) in fail_pairs.items():
        print(f"      {k}: {bad} of {tot} pairs have no join")
    print()


if __name__ == "__main__":
    print("instrument 4: the categories op's three readings do not cover")
    print("=" * 78)
    q17()
    q18()
    q19()
