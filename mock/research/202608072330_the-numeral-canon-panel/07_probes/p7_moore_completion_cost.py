#!/usr/bin/env python3
"""p7. If a best abstraction is wanted across kinds, what does it cost?

File 03 measures that fixed-point against float has minimal upper bounds of
width 2 and no least one, and argues by a counting argument that no uniform-grid
shape sits between.  In the adjunction frame that is exactly "no best
abstraction over the union of the two domains", and the literature's standard
response is to build the REDUCED PRODUCT, that is, to close the family under
intersection so the Moore condition holds again.

Nobody has priced that.  This probe does, structurally: it computes the Moore
closure of the union family over a box and reports how many sets the closure
adds, what they look like, and whether any of them is a shape either family
already names.

If the closure adds a handful of nameable shapes, reading A across kinds is a
design with a size.  If it adds a population that is essentially "arbitrary
finite sets", it is not a family and the option is dead on its own terms.  The
count is the whole content, and it is a count rather than a magnitude, so no
bench bears on it.
"""

import itertools
from fractions import Fraction as Q


def ufixed(I, F):
    q = Q(1, 2**F)
    return frozenset(k * q for k in range(2 ** (I + F)))


def flt(p, elo, ehi):
    """A minimal float family: significands of p bits, exponents elo..ehi, plus
    zero.  The shape file 03 uses for its witness."""
    out = {Q(0)}
    for e in range(elo, ehi + 1):
        base = Q(2) ** e
        for m in range(2 ** (p - 1), 2**p):
            out.add(Q(m, 2 ** (p - 1)) * base)
    return frozenset(out)


def moore_closure(fam, cap=200000):
    """Close a family under pairwise intersection until fixed, or give up."""
    cur = set(fam)
    added = set()
    changed = True
    while changed:
        changed = False
        new = set()
        for a, b in itertools.combinations(sorted(cur, key=lambda s: (len(s), sorted(s))), 2):
            c = a & b
            if c not in cur and c not in new:
                new.add(c)
        if new:
            added |= new
            cur |= new
            changed = True
        if len(cur) > cap:
            return cur, added, False
    return cur, added, True


def run():
    for WMAX, PMAX in [(3, 2), (4, 3)]:
        fixed = {ufixed(I, F) for F in range(WMAX + 1) for I in range(0, WMAX + 1)
                 if 0 <= I + F <= WMAX}
        floats = set()
        for p in range(1, PMAX + 1):
            for elo in range(-2, 2):
                for ehi in range(elo, 2):
                    floats.add(flt(p, elo, ehi))
        print(f"=== box: fixed-point total width <= {WMAX}, float precision <= {PMAX} ===")
        for label, fam in [("fixed-point alone", fixed),
                           ("float alone", floats),
                           ("both kinds in one order", fixed | floats)]:
            closed, added, done = moore_closure(fam)
            share = (len(added) / len(fam)) if fam else 0
            print(f"  {label:>26} | start {len(fam):>4} | closure adds {len(added):>5} "
                  f"| {'fixed point reached' if done else 'ABORTED at cap'} "
                  f"| growth x{1+share:.2f}")
            if added and label.startswith("both"):
                # Is any added set already a shape either family names?
                overlap = sum(1 for s in added if s in fixed or s in floats)
                print(f"  {'':>26}   of the added sets, {overlap} are already named by a family")
                sample = sorted(added, key=lambda s: (len(s), sorted(s)))[:3]
                for s in sample:
                    print(f"  {'':>26}   added: {sorted(s)}")
        print()

    # -- the specific witness file 03 names, checked directly.
    print("=== 03's witness, checked in this instrument's own representation ===")
    a = ufixed(0, 1)          # {0, 1/2}
    b = ufixed(2, 0)          # {0,1,2,3}
    print(f"  U<0,1> = {sorted(a)}")
    print(f"  U<2,0> = {sorted(b)}")
    union = a | b
    print(f"  union  = {sorted(union)}  ({len(union)} values)")
    fixed = {ufixed(I, F): (I, F) for F in range(6) for I in range(0, 6)
             if 0 <= I + F <= 6}
    ups_f = [s for s in fixed if union <= s]
    least_f = [s for s in ups_f if not any(o < s for o in ups_f)]
    print(f"  minimal fixed-point upper bounds: {[fixed[s] for s in least_f]}")
    fl = {}
    for p in range(1, 4):
        for elo in range(-3, 3):
            for ehi in range(elo, 3):
                fl[flt(p, elo, ehi)] = (p, elo, ehi)
    both = dict(list(fixed.items()) + list(fl.items()))
    ups = [s for s in both if union <= s]
    minimal = [s for s in ups if not any(o < s for o in ups)]
    print(f"  minimal upper bounds across BOTH kinds: {len(minimal)}")
    for s in sorted(minimal, key=lambda s: (len(s), sorted(s))):
        print(f"    {both[s]}  size {len(s)}")
    print(f"  is the union itself named by either family? "
          f"{union in fixed or union in fl}")


if __name__ == "__main__":
    run()


# ---------------------------------------------------------------- growth sweep
# The counts above are small, which is only informative if the growth is known.
# A completion that adds a handful at every box is a third family with a size; one
# that grows faster than the family it completes is "arbitrary finite sets" wearing
# a name.  Swept here rather than argued.


def sweep():
    print()
    print("=== growth of the Moore completion with the box ===")
    print(f"{'W':>3} {'P':>3} | {'fixed':>6} {'float':>6} {'union':>6} | "
          f"{'added':>6} | {'added/union':>11}")
    print("-" * 60)
    for WMAX in range(2, 7):
        for PMAX in (2, 3):
            fixed = {ufixed(I, F) for F in range(WMAX + 1) for I in range(0, WMAX + 1)
                     if 0 <= I + F <= WMAX}
            floats = set()
            for p in range(1, PMAX + 1):
                for elo in range(-2, 2):
                    for ehi in range(elo, 2):
                        floats.add(flt(p, elo, ehi))
            fam = fixed | floats
            closed, added, done = moore_closure(fam, cap=60000)
            ratio = len(added) / len(fam) if fam else 0
            print(f"{WMAX:>3} {PMAX:>3} | {len(fixed):>6} {len(floats):>6} {len(fam):>6} | "
                  f"{len(added):>6} | {ratio:>11.3f}"
                  + ("" if done else "  ABORTED"))


sweep()


# The first sweep held the float exponent range fixed at -2..1, so the saturation
# it shows is a property of that choice rather than of the completion.  The free
# parameter is swept here, because a completion whose size is set by the float
# family's own span is a very different object from one set by the fixed-point box.


def sweep_exponents():
    print()
    print("=== growth with the FLOAT exponent span, fixed-point box held at W=5 ===")
    print(f"{'espan':>6} {'P':>3} | {'float':>6} {'union':>6} | {'added':>6} | {'ratio':>7}")
    print("-" * 50)
    WMAX = 5
    fixed = {ufixed(I, F) for F in range(WMAX + 1) for I in range(0, WMAX + 1)
             if 0 <= I + F <= WMAX}
    for span in range(1, 6):
        for PMAX in (2, 3):
            floats = set()
            for p in range(1, PMAX + 1):
                for elo in range(-span, span):
                    for ehi in range(elo, span):
                        floats.add(flt(p, elo, ehi))
            fam = fixed | floats
            closed, added, done = moore_closure(fam, cap=60000)
            print(f"{span:>6} {PMAX:>3} | {len(floats):>6} {len(fam):>6} | {len(added):>6} | "
                  f"{len(added)/len(fam):>7.3f}" + ("" if done else "  ABORTED"))


sweep_exponents()
