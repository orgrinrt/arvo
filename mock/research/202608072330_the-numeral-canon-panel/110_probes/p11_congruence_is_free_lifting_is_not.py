#!/usr/bin/env python3
"""P11. Congruence and lifting are different obligations, and only one is free.

`112` section 8 took my P7/P8 composite results further and found a hazard I had
not looked for: if a declared extent discharges a construction's base predicate,
what is the lifting rule? The obvious componentwise answer is **unsound**, at
26 of 81 pairs for complex multiplication, because complex mixes its components
and a componentwise bound does not bound a mixed result.

That raises a question about my own F13, which said denotational sameness is a
congruence with respect to the four constructions. If a construction can fail to
carry a predicate, can it also fail to carry an equality?

This probe answers it, and the answer separates two obligations that look like
one:

  CONGRUENCE.  P and Q the same primitive  =>  C(P) and C(Q) the same primitive.
  LIFTING.     a predicate true of P       =>  the matching predicate true of C(P).

FIRST VERSION WRONG IN TWO PLACES, both caught by the run rather than by review,
and both are the failures this workspace's test gate names first:

  - The lifting predicate demanded the base be CLOSED on the extent under the
    reduced operations. At `uW3` only the extent `{0}` satisfies that, so the
    rule fired once per base and could not expose anything. The predicate wanted
    is EXACTNESS: no reduction fires, which is what `112` means by the
    propagated bound fitting. That is setup that helps, in the strong form where
    the interesting region is never entered.
  - The congruence arm compared bases differing only in an `enc` field that
    nothing in the class ever read, so it compared each base with itself. Zero
    failures out of twelve, and it could not have reported anything else. That
    is the same dead-branch shape `111` found in my `p2`, written a second time
    by the same author in the file conceding the first.

Both are fixed below, and the congruence arm now carries a negative control that
must fail, because an arm with no failing case proves nothing.
"""

from itertools import product

# --------------------------------------------------------------- base algebra


class Base:
    def __init__(self, W, signed, policy, name=None):
        self.W, self.signed, self.policy = W, signed, policy
        n = 1 << W
        self.values = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self._name = name

    def label(self):
        s = "i" if self.signed else "u"
        return self._name or f"{s}W{self.W}/{self.policy}"

    def R(self, k):
        n = 1 << self.W
        if self.policy == "wrap":
            return ((k + n // 2) % n) - n // 2 if self.signed else k % n
        lo, hi = self.values[0], self.values[-1]
        return lo if k < lo else (hi if k > hi else k)

    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)


class Relabelled(Base):
    """Denotationally identical to its source: same value set, same operations,
    reached by a different construction path. This is what a second spelling of
    one primitive looks like, and congruence says its composites must match."""

    def __init__(self, src):
        self.W, self.signed, self.policy = src.W, src.signed, src.policy
        self.values = list(src.values)
        self._name = f"{src.label()}#relabelled"
        self._src = src

    def R(self, k):
        # the same map, computed the long way round: reduce by repeated
        # correction rather than by one modulus or one clamp.
        n = 1 << self.W
        if self.policy == "wrap":
            lo, hi = self.values[0], self.values[-1]
            while k > hi:
                k -= n
            while k < lo:
                k += n
            return k
        lo, hi = self.values[0], self.values[-1]
        return lo if k < lo else (hi if k > hi else k)


class Sabotaged(Base):
    """The negative control: same value set, one operation genuinely different.
    Congruence must FAIL against its source, or the congruence arm is vacuous."""

    def __init__(self, src):
        self.W, self.signed, self.policy = src.W, src.signed, src.policy
        self.values = list(src.values)
        self._name = f"{src.label()}#sabotaged"

    def mul(self, a, b):
        return self.R(a * b + 1)


# --------------------------------------------------------------- constructions

class Product2:
    name = "product2"
    def __init__(self, b): self.b = b
    def carrier(self, vs): return [(x, y) for x in vs for y in vs]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v): return (self.b.mul(u[0], v[0]), self.b.mul(u[1], v[1]))


class Complex:
    name = "complex"
    def __init__(self, b): self.b = b
    def carrier(self, vs): return [(x, y) for x in vs for y in vs]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v):
        a, bb = u; c, d = v
        return (self.b.sub(self.b.mul(a, c), self.b.mul(bb, d)),
                self.b.add(self.b.mul(a, d), self.b.mul(bb, c)))


class Dual:
    name = "dual"
    def __init__(self, b): self.b = b
    def carrier(self, vs): return [(x, y) for x in vs for y in vs]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v):
        a, bb = u; c, d = v
        return (self.b.mul(a, c), self.b.add(self.b.mul(a, d), self.b.mul(bb, c)))


class Interval:
    name = "interval"
    def __init__(self, b): self.b = b
    def carrier(self, vs): return [(x, y) for x in vs for y in vs if x <= y]
    def add(self, u, v): return (self.b.add(u[0], v[0]), self.b.add(u[1], v[1]))
    def mul(self, u, v):
        cs = [self.b.mul(u[0], v[0]), self.b.mul(u[0], v[1]),
              self.b.mul(u[1], v[0]), self.b.mul(u[1], v[1])]
        return (min(cs), max(cs))


CONSTRUCTIONS = [Product2, Complex, Dual, Interval]


def tables(C, base):
    c = C(base)
    carrier = c.carrier(base.values)
    return (tuple(carrier),
            tuple(c.add(u, v) for u, v in product(carrier, repeat=2)),
            tuple(c.mul(u, v) for u, v in product(carrier, repeat=2)))


def base_tables(b):
    return (tuple(b.values),
            tuple(b.add(x, y) for x, y in product(b.values, repeat=2)),
            tuple(b.mul(x, y) for x, y in product(b.values, repeat=2)))


# ------------------------------------------------------------------- lifting

class Tracked:
    """Wraps a base and records whether any reduction actually fired.

    This is the difference between the third and fourth versions of this file.
    Asking whether the RESULT stays inside the extent conflates two things: the
    base's own extent not being closed, and the construction leaving a region the
    base preserved. The lifting question is the second one, and the way to ask it
    is to watch whether a reduction fires anywhere inside the construction.
    """

    def __init__(self, base):
        self.b = base
        self.values = base.values
        self.reduced = False

    def _chk(self, exact):
        lo, hi = self.b.values[0], self.b.values[-1]
        if not (lo <= exact <= hi):
            self.reduced = True
        return self.b.R(exact)

    def add(self, a, b): return self._chk(a + b)
    def sub(self, a, b): return self._chk(a - b)
    def mul(self, a, b): return self._chk(a * b)


def base_exact_on(base, e):
    """Is the base EXACT on the extent {0..e}: does no reduction fire?"""
    E = [v for v in base.values if 0 <= v <= e]
    lo, hi = base.values[0], base.values[-1]
    for x, y in product(E, repeat=2):
        if not (lo <= x + y <= hi) or not (lo <= x * y <= hi):
            return False
    return True


def componentwise_holds(C, base, e):
    """Does the construction stay EXACT on the componentwise extent?

    Counts the pairs on which some base operation inside the construction had to
    reduce. The componentwise rule claims this is zero whenever the base itself
    is exact on the extent.
    """
    E = [v for v in base.values if 0 <= v <= e]
    bad = 0
    total = 0
    proto = C(base)
    carrier = proto.carrier(E)
    for u, v in product(carrier, repeat=2):
        t = Tracked(base)
        c = C(t)
        total += 1
        c.add(u, v)
        c.mul(u, v)
        if t.reduced:
            bad += 1
    return bad, total


def main():
    print("P11. congruence is free; lifting is not")
    print("=" * 78)

    sources = [Base(3, False, "wrap"), Base(3, False, "sat"),
               Base(4, True, "wrap"), Base(4, True, "sat")]

    # ---------------- obligation one, with a control that must fail
    print("OBLIGATION ONE: does EQUALITY transport through a construction?")
    print("  each base is compared against a RELABELLED twin (identical tables,")
    print("  different construction path) and against a SABOTAGED twin (one")
    print("  operation genuinely changed). the sabotaged arm must fail, or the")
    print("  relabelled arm's zero means nothing.")
    print()
    print(f"  {'construction':<12} {'relabelled: same?':>19} {'sabotaged: same?':>19}")
    cong_fail = 0
    ctrl_fail = 0
    for C in CONSTRUCTIONS:
        rel_same = sab_same = 0
        rel_n = sab_n = 0
        for src in sources:
            rel, sab = Relabelled(src), Sabotaged(src)
            assert base_tables(src) == base_tables(rel), "relabelled twin is not a twin"
            assert base_tables(src) != base_tables(sab), "sabotaged twin is not sabotaged"
            rel_n += 1
            if tables(C, src) == tables(C, rel):
                rel_same += 1
            sab_n += 1
            if tables(C, src) == tables(C, sab):
                sab_same += 1
        cong_fail += (rel_n - rel_same)
        ctrl_fail += sab_same
        print(f"  {C.name:<12} {f'{rel_same}/{rel_n}':>19} {f'{sab_same}/{sab_n}':>19}")
    print()
    print(f"  congruence failures on the relabelled twin: {cong_fail}")
    print(f"  sabotaged twins that slipped through as 'same': {ctrl_fail}")
    if ctrl_fail == 0 and cong_fail == 0:
        print("  the control fails as it must and the real arm passes, so the zero")
        print("  is a measurement rather than a default.")
    print()

    # ---------------- obligation two
    print("OBLIGATION TWO: does a PREDICATE transport through a construction?")
    print("  predicate: the base is EXACT on the extent {0..e}, no reduction fires.")
    print("  candidate rule: componentwise, which is the obvious rule and the one")
    print("  a design reaches for when it has already seen congruence hold.")
    print()
    print(f"  {'base':<14} {'construction':<12} {'extent':>7} {'unsound pairs':>15}")
    lift_fail = 0
    fired = 0
    for base in sources:
        for e in range(0, 6):
            if not base_exact_on(base, e):
                continue
            for C in CONSTRUCTIONS:
                fired += 1
                bad, total = componentwise_holds(C, base, e)
                if bad:
                    lift_fail += 1
                    print(f"  {base.label():<14} {C.name:<12} {f'<= {e}':>7} "
                          f"{f'{bad}/{total}':>15}")
    print(f"  rule fired {fired} times; unsound in {lift_fail} of them")
    print()

    # ---------------- the independence claim
    print("THE POINT:")
    print(f"  congruence failures:  {cong_fail}")
    print(f"  lifting failures:     {lift_fail}")
    if cong_fail == 0 and lift_fail > 0:
        print()
        print("  Every construction transports EQUALITY and some do not transport")
        print("  the PREDICATE, over the same bases and the same constructions.")
        print("  The two obligations are independent and the first is no evidence")
        print("  for the second.")
        print()
        print("  The reason is structural rather than incidental. A construction is")
        print("  a function of the base's operation tables, so equal tables in give")
        print("  equal tables out however much the construction mixes components.")
        print("  A predicate is a claim quantified over a region, and a construction")
        print("  that mixes components mixes the regions, so nothing carries it")
        print("  across for free.")
        print()
        print("  A design that reads 'the construction respects the base' off the")
        print("  congruence and lifts a predicate on that basis makes exactly the")
        print("  error `112` measured. The two look like one property and are not.")
    else:
        print("  The predicted separation did NOT appear; this probe's claim fails.")


if __name__ == "__main__":
    main()
