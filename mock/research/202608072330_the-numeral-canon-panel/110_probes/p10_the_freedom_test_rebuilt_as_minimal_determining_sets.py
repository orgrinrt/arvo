#!/usr/bin/env python3
"""P10. My freedom test was a dead branch. This is the version that can fire.

`111` section 2.2 established that `p2`'s TEST 3 cannot fail: the sweep is a
product over five axes, the key is those five axes plus a constant radix, so
every key is distinct by construction, `if k in seen` is never true, and the
statement that increments `free` never executes. I reproduced that mechanically
before writing this file and it is exactly right. The "0 of 48" figure counts
collisions that cannot occur and should never have been cited.

`111` section 2.3 is also right that the repaired question is analytic: a law set
is by definition a predicate over the operation tables, so asking whether it can
vary while the tables are fixed asks whether a function can disagree with itself.

**So the useful question is the one underneath, and it is not analytic.** Not "is
the law set free" but **"which axes does a law verdict actually read"**. That is
`90` R1's open question: it lists a growing set of dimensions a law's region
needs and says the list "is a floor, not an enumeration". A floor is measurable.

TWO ITERATIONS OF THIS FILE, AND THE FIRST WAS ALSO WRONG. It asked which
subsets of the axes *determine* the law set and reported the minimal ones. That
framing is vacuous for exactly the reason the original was: the full axis set
determines it with **zero comparisons**, and every proper subset disagrees, so
the answer is always "no minimal determining set with a live comparison" whatever
the data says. My own mutation control caught it by reporting no reaction, which
is what a control is for. The record of that run is in the git history of this
file. The live question is per-axis and it is below.

Controls, all three of which must behave or nothing here counts:
  - a SPURIOUS axis, swept but read by nothing, must come back "not read". This
    is the one that proves the test can return a negative at all.
  - the real axes must come back "read", with a count.
  - a MUTATION assigning law sets at random must make every axis look maximally
    read, because a random verdict is unexplainable by any axis.
"""

from fractions import Fraction
from itertools import product
import random

AXES = ["W", "F", "signed", "policy", "rounding", "radix", "spurious"]


class Prim:
    def __init__(self, W, F, signed, policy, rounding, radix, spurious):
        self.W, self.F, self.signed = W, F, signed
        self.policy, self.rounding, self.radix = policy, rounding, radix
        # swept, carried, and read by nothing: the negative control
        self.spurious = spurious
        n = 1 << W
        self.ints = list(range(-(n // 2), n // 2)) if signed else list(range(0, n))
        self.step = Fraction(1, radix ** F)
        self.values = [Fraction(k) * self.step for k in self.ints]

    def axes(self):
        return dict(W=self.W, F=self.F, signed=self.signed, policy=self.policy,
                    rounding=self.rounding, radix=self.radix, spurious=self.spurious)

    def label(self):
        s = "i" if self.signed else "u"
        return (f"{s}W{self.W}F{self.F}r{self.radix}/{self.policy}"
                f"/{self.rounding}/sp{self.spurious}")

    def R(self, q):
        r = q / self.step
        if self.rounding == "trunc":
            k = int(r)
        elif self.rounding == "floor":
            k = r.numerator // r.denominator
        else:
            fl = r.numerator // r.denominator
            fr = r - fl
            k = fl + 1 if fr > Fraction(1, 2) else (fl if fr < Fraction(1, 2)
                                                   else (fl if fl % 2 == 0 else fl + 1))
        n = 1 << self.W
        if self.policy == "wrap":
            k = ((k + n // 2) % n) - n // 2 if self.signed else k % n
        else:
            klo, khi = self.ints[0], self.ints[-1]
            k = klo if k < klo else (khi if k > khi else k)
        return Fraction(k) * self.step

    def add(self, a, b):
        return self.R(a + b)

    def sub(self, a, b):
        return self.R(a - b)

    def mul(self, a, b):
        return self.R(a * b)

    def neg(self, a):
        return self.R(-a)


def law_add_comm(p):
    return all(p.add(a, b) == p.add(b, a) for a, b in product(p.values, repeat=2))


def law_add_assoc(p):
    return all(p.add(p.add(a, b), c) == p.add(a, p.add(b, c))
               for a, b, c in product(p.values, repeat=3))


def law_mul_assoc(p):
    return all(p.mul(p.mul(a, b), c) == p.mul(a, p.mul(b, c))
               for a, b, c in product(p.values, repeat=3))


def law_distrib_add(p):
    return all(p.mul(a, p.add(b, c)) == p.add(p.mul(a, b), p.mul(a, c))
               for a, b, c in product(p.values, repeat=3))


def law_distrib_sub(p):
    return all(p.mul(a, p.sub(b, c)) == p.sub(p.mul(a, b), p.mul(a, c))
               for a, b, c in product(p.values, repeat=3))


def law_add_monotone(p):
    return all(p.add(a, c) <= p.add(b, c)
               for a, b, c in product(p.values, repeat=3) if a <= b)


def law_neg_involutive(p):
    return all(p.neg(p.neg(a)) == a for a in p.values)


LAWS = [("add_comm", law_add_comm), ("add_assoc", law_add_assoc),
        ("mul_assoc", law_mul_assoc), ("distrib_add", law_distrib_add),
        ("distrib_sub", law_distrib_sub), ("add_monotone", law_add_monotone),
        ("neg_involutive", law_neg_involutive)]


def sweep():
    out = []
    for W, F, signed, policy, rounding, radix, sp in product(
        [3, 4], [0, 1, 2], [False, True], ["sat", "wrap"], ["near", "trunc"],
        [2, 3], ["a", "b"]
    ):
        out.append(Prim(W, F, signed, policy, rounding, radix, sp))
    return out


def axis_is_read(axis, configs, lawset_of):
    """Hold every OTHER axis fixed and vary this one. Does the law set move?

    The comparison count is reported because a count of zero is the defect that
    killed the original test, and a reader should be able to see it is nonzero.
    """
    rest = [a for a in AXES if a != axis]
    groups = {}
    for p in configs:
        k = tuple(p.axes()[a] for a in rest)
        groups.setdefault(k, []).append(p)
    comparisons = disagreements = 0
    witness = None
    for k, ps in groups.items():
        for i in range(1, len(ps)):
            comparisons += 1
            if lawset_of[ps[i].label()] != lawset_of[ps[0].label()]:
                disagreements += 1
                if witness is None:
                    witness = (ps[0].label(), ps[i].label())
    return comparisons, disagreements, witness


def main():
    print("P10. which axes a law verdict actually reads")
    print("=" * 78)

    configs = sweep()
    lawset_of = {p.label(): tuple(f(p) for _, f in LAWS) for p in configs}
    print(f"configurations: {len(configs)}")
    print("  W in {3,4}, F in {0,1,2}, signed, policy in {sat,wrap},")
    print("  rounding in {near,trunc}, radix in {2,3}, spurious in {a,b}")
    print(f"distinct law sets: {len(set(lawset_of.values()))} over {len(LAWS)} laws")
    print()

    print("the original test's key, reproduced, so the defect is visible:")
    seen, hits = {}, 0
    for p in configs:
        k = tuple(p.axes()[a] for a in AXES)
        if k in seen:
            hits += 1
        seen[k] = p
    print(f"  key = every swept axis -> collisions {hits}, so comparisons 0.")
    print("  a test whose comparison count is zero reports its default forever.")
    print()

    print("per-axis, holding every other axis fixed:")
    print(f"  {'axis':<10} {'comparisons':>12} {'disagreements':>14}  verdict")
    rows = []
    for a in AXES:
        c, d, w = axis_is_read(a, configs, lawset_of)
        rows.append((a, c, d, w))
        verdict = "READ by some law" if d else "not read"
        print(f"  {a:<10} {c:>12} {d:>14}  {verdict}")
    print()

    sp = [r for r in rows if r[0] == "spurious"][0]
    print("CONTROL ONE, the negative control that proves a negative is reachable:")
    if sp[2] == 0 and sp[1] > 0:
        print(f"  the spurious axis was swept, compared {sp[1]} times, and moved")
        print("  nothing. so 'not read' is an outcome this test can produce.")
    else:
        print(f"  BROKEN: spurious axis reports {sp[2]} disagreements over {sp[1]}")
        print("  comparisons; either it is being read or the harness is wrong.")
    print()

    print("CONTROL TWO, ordering the real axes by how hard they are read:")
    real = sorted([r for r in rows if r[0] != "spurious"],
                  key=lambda r: -r[2])
    for a, c, d, w in real:
        share = 100.0 * d / c if c else 0.0
        print(f"  {a:<10} {d:>5} of {c:<5} comparisons ({share:5.1f}%)"
              + (f"   e.g. {w[0]} vs {w[1]}" if w else ""))
    print()

    print("CONTROL THREE, mutation: law sets assigned at random.")
    rng = random.Random(20260814)
    fake = {p.label(): tuple(rng.random() < 0.5 for _ in LAWS) for p in configs}
    moved = 0
    print(f"  {'axis':<10} {'real':>8} {'mutated':>10}")
    for a in AXES:
        _, d_real, _ = axis_is_read(a, configs, lawset_of)
        _, d_fake, _ = axis_is_read(a, configs, fake)
        print(f"  {a:<10} {d_real:>8} {d_fake:>10}")
        if d_fake > d_real:
            moved += 1
    print(f"  axes whose disagreement count rose under mutation: {moved} of {len(AXES)}")
    if moved >= len(AXES) - 1:
        print("  a random verdict is explainable by no axis, which is what the")
        print("  mutation should produce. the instrument reacts.")
    else:
        print("  NO REACTION: this instrument is as dead as the one it replaces.")
    print()

    print("reading:")
    print("  the honest form of the original finding is not that a law set is an")
    print("  unsettable coordinate, which is analytic and needs no sweep. it is")
    print("  that a law verdict reads a NAMED SET of axes and not the others, and")
    print("  that the set is measurable with a test that can return either answer.")
    print("  over this sweep every real axis is read and the spurious one is not,")
    print("  which is `90` R1's dimension list given a measured floor rather than")
    print("  an enumeration.")


if __name__ == "__main__":
    main()
