"""Instrument 5: does the multi-cover antichain stay narrow, and what does radix 6 do?

Q18 found the widest antichain of minimal upper bounds to be 2, which if it holds makes
a tie-break rule cheap. A width measured inside one box is exactly the kind of number the
prior panel found to be an artifact of its own bound, so it is measured again at three
box sizes and with more shape kinds present.

Q21 chases the one new failure mode instrument 4 turned up: radix 2 against radix 6
produced three pairs with upper bounds present and none least, which neither the
commensurable nor the incommensurable radix pairs did.
"""

from fractions import Fraction as F
from itertools import combinations
from i1_shape_space import Space, uf, sf_sym, fl


def minimal_uppers(sp, i, j):
    ks = list(sp.bits(sp.above[i] & sp.above[j]))
    return [k for k in ks if not any(m != k and sp.sets[m] < sp.sets[k] for m in ks)]


def q20():
    print("Q20 does the widest antichain of minimal upper bounds grow with the box?")
    for lim, pmax, elo, ehi in ((4, 2, -2, 1), (5, 3, -3, 2), (6, 4, -4, 3)):
        lab = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
        lab += [
            (f"F<p{p},e{a}..{b}>", fl(p, a, b))
            for p in range(1, pmax + 1)
            for a in range(elo, 1)
            for b in range(0, ehi + 1)
            if a <= b
        ]
        sp = Space(lab)
        hist = {}
        widest = None
        for i, j in combinations(range(sp.n), 2):
            mu = minimal_uppers(sp, i, j)
            n = len(mu)
            hist[n] = hist.get(n, 0) + 1
            if widest is None or n > widest[0]:
                widest = (n, sp.names[i], sp.names[j], [sp.names[k] for k in mu])
        print(f"    lim={lim} p<={pmax} e in [{elo},{ehi}]: points {sp.n}, pairs {sum(hist.values())}")
        print(f"      antichain widths: {dict(sorted(hist.items()))}")
        if widest and widest[0] > 1:
            print(f"      widest: {widest[1]} v {widest[2]} -> {widest[3]}")
    print()


def q20b():
    print("Q20b the same with three sign domains and two commensurable radices present")
    lim = 5
    lab = [(f"U<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
    lab += [(f"S<{I},{Fw}>", sf_sym(2, I, Fw)) for I in range(0, lim + 1) for Fw in range(0, lim + 1) if 0 <= I + Fw <= lim]
    lab += [(f"U4<{I},{Fw}>", uf(4, I, Fw)) for I in range(0, 3) for Fw in range(0, 3) if 0 <= I + Fw <= 3]
    lab += [
        (f"F<p{p},e{a}..{b}>", fl(p, a, b))
        for p in range(1, 4)
        for a in range(-3, 1)
        for b in range(0, 3)
        if a <= b
    ]
    sp = Space(lab)
    hist = {}
    widest = None
    for i, j in combinations(range(sp.n), 2):
        mu = minimal_uppers(sp, i, j)
        n = len(mu)
        hist[n] = hist.get(n, 0) + 1
        if widest is None or n > widest[0]:
            widest = (n, sp.names[i], sp.names[j], [sp.names[k] for k in mu])
    print(f"    points {sp.n}, pairs {sum(hist.values())}")
    print(f"    antichain widths: {dict(sorted(hist.items()))}")
    if widest:
        print(f"    widest: {widest[1]} v {widest[2]} -> {widest[3]}")
    print()


def q21():
    print("Q21 radix 2 against radix 6: the bounds-but-no-least pairs")
    lab = [(f"U2<{I},{Fw}>", uf(2, I, Fw)) for I in range(0, 5) for Fw in range(0, 5) if 0 <= I + Fw <= 4]
    lab += [(f"U6<{I},{Fw}>", uf(6, I, Fw)) for I in range(0, 4) for Fw in range(0, 4) if 0 <= I + Fw <= 3]
    sp = Space(lab)
    shown = 0
    for i, j in combinations(range(sp.n), 2):
        uc = sp.above[i] & sp.above[j]
        if uc and sp.least(uc) is None and shown < 4:
            shown += 1
            mu = minimal_uppers(sp, i, j)
            print(f"    {sp.names[i]} = {sorted(sp.sets[i])}")
            print(f"    {sp.names[j]} = {sorted(sp.sets[j])}")
            print(f"      minimal upper bounds: {[sp.names[k] for k in mu]}")
            for k in mu:
                print(f"        {sp.names[k]} = {sorted(sp.sets[k])[:8]}{' ...' if len(sp.sets[k]) > 8 else ''}")
    print("    two radices sharing a factor give the third failure mode: upper bounds exist")
    print("    and none is least, which is the fixed-point-against-float shape, not the")
    print("    incommensurable-radix shape.")
    print()


if __name__ == "__main__":
    print("instrument 5: scaling checks on instrument 4's two headline numbers")
    print("=" * 78)
    q20()
    q20b()
    q21()
