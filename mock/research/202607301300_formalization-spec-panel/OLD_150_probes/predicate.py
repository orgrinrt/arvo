"""Closed-form inclusion predicates, cross-validated, then used at real widths.

Method two of three. The brute-force probe (poset.py) enumerates value sets and
so cannot reach widths anyone would ship. This one derives inclusion from the
shape parameters alone, checks the derivation against brute force everywhere the
two overlap, and then applies it where brute force cannot go.

Shapes:
  Fix(i, f, signed)   uniform grid, step 2^-f, anchored so that 0 is a member.
  Flt(p, a, b)        binary float, p significand digits, exponents [a, b],
                      with subnormals, so the finest step is 2^(a-p+1).
"""

from fractions import Fraction as Q
from itertools import combinations, product
from poset import fixed, flt


class Fix:
    kind = 'fix'

    def __init__(self, i, f, signed=False):
        self.i, self.f, self.signed = i, f, signed

    # magnitude reached, as an exponent: the set spans up to just under 2^top
    @property
    def top(self):
        return self.i - 1 if self.signed else self.i

    @property
    def step(self):
        return -self.f                       # step is 2^step

    @property
    def bits(self):
        return self.i + self.f               # significant digits available

    def __repr__(self):
        return f"{'I' if self.signed else 'U'}{self.i}.{self.f}"


class Flt:
    kind = 'flt'

    def __init__(self, p, a, b, signed=True):
        self.p, self.a, self.b, self.signed = p, a, b, signed

    @property
    def top(self):
        return self.b + 1

    @property
    def step(self):
        return self.a - self.p + 1           # finest subnormal step

    def __repr__(self):
        return f"F(p={self.p},e=[{self.a},{self.b}])"


def subset(x, y):
    """Is every value of x a value of y?"""
    if x.signed and not y.signed:
        return False
    if x.kind == 'fix' and y.kind == 'fix':
        # uniform in uniform: finer or equal step, and no further reach
        return x.step >= y.step and x.top <= y.top
    if x.kind == 'fix' and y.kind == 'flt':
        # the largest odd multiple carries x.bits significant digits, and the
        # float can only hold p of them anywhere in its normal range
        return (x.bits <= y.p) and (x.top <= y.top) and (x.step >= y.step)
    if x.kind == 'flt' and y.kind == 'fix':
        # a uniform grid must be at least as fine as the float's finest step
        # and reach at least as far as its largest value
        return (x.step >= y.step) and (x.top <= y.top)
    if x.kind == 'flt' and y.kind == 'flt':
        return (x.p <= y.p) and (x.top <= y.top) and (x.step >= y.step)
    raise AssertionError


# ------------------------------------------------------------ cross-validate

def valueset(s):
    if s.kind == 'fix':
        return fixed(s.i, s.f, s.signed)
    return flt(s.p, s.a, s.b, signed=s.signed)


def crossvalidate():
    shapes = []
    for i in range(0, 5):
        for f in range(0, 5):
            if i + f > 0:
                shapes.append(Fix(i, f, False))
    for p in (2, 3, 4, 5):
        for a in (-2, -1, 0, 1):
            for b in (a, a + 1, a + 2):
                shapes.append(Flt(p, a, b, signed=False))

    bad = []
    for x, y in product(shapes, shapes):
        want = valueset(x) <= valueset(y)
        got = subset(x, y)
        if want != got:
            bad.append((x, y, want, got))
    print(f"cross-validation over {len(shapes)**2} ordered shape pairs: "
          f"{len(bad)} disagreements with brute force")
    for b in bad[:10]:
        print("   MISMATCH", b[0], "in", b[1], " brute:", b[2], " predicate:", b[3])
    return not bad


# ------------------------------------------------------------------ analysis

def structure(shapes, label):
    n = len(shapes)
    le = [[subset(shapes[a], shapes[b]) for b in range(n)] for a in range(n)]
    meet_fail = join_fail = 0
    ex_meet = ex_join = None
    for a, b in combinations(range(n), 2):
        lows = [c for c in range(n) if le[c][a] and le[c][b]]
        tops = [c for c in lows if not any(c != d and le[c][d] for d in lows)]
        if len(tops) != 1:
            meet_fail += 1
            if ex_meet is None:
                ex_meet = (shapes[a], shapes[b], [shapes[t] for t in tops][:6], len(tops))
        ups = [c for c in range(n) if le[a][c] and le[b][c]]
        bots = [c for c in ups if not any(c != d and le[d][c] for d in ups)]
        if len(bots) != 1:
            join_fail += 1
            if ex_join is None:
                ex_join = (shapes[a], shapes[b], [shapes[t] for t in bots][:6], len(bots))
    print(f"\n--- {label}: {n} shapes, {n*(n-1)//2} pairs")
    print(f"    meet not unique: {meet_fail}    join not unique: {join_fail}")
    if ex_meet:
        print(f"    meet witness: {ex_meet[0]} ^ {ex_meet[1]} has {ex_meet[3]} "
              f"maximal lower bounds, e.g. {ex_meet[2]}")
    if ex_join:
        print(f"    join witness: {ex_join[0]} v {ex_join[1]} has {ex_join[3]} "
              f"minimal upper bounds, e.g. {ex_join[2]}")
    return meet_fail, join_fail


if __name__ == "__main__":
    ok = crossvalidate()
    assert ok, "predicate disagrees with brute force; do not trust what follows"

    IEEE = [Flt(11, -14, 15, False),    # binary16
            Flt(8, -126, 127, False),   # bfloat16
            Flt(24, -126, 127, False),  # binary32
            Flt(53, -1022, 1023, False)]  # binary64

    fixes_small = [Fix(i, f) for i in range(0, 9) for f in range(0, 9) if i + f > 0]
    fixes_wide = [Fix(i, f) for i in range(0, 33, 4) for f in range(0, 33, 4) if i + f > 0]

    structure(fixes_small, "fixed only, I,F <= 8")
    structure(IEEE, "the four shipped float formats only")
    structure(fixes_small + IEEE, "fixed I,F <= 8 together with the four float formats")
    structure(fixes_wide + IEEE, "fixed I,F <= 32 step 4 together with the four float formats")

    print("\n--- the hand-checkable witness, at widths anyone would ship")
    a, b = Fix(20, 0), Fix(0, 20)
    cands = [Fix(20, 20)] + IEEE
    for c in cands:
        print(f"    {a} subset {c}: {subset(a,c)}   {b} subset {c}: {subset(b,c)}")
    print(f"    Fix(20,20) subset binary32: {subset(Fix(20,20), IEEE[2])}")
    print(f"    binary32 subset Fix(20,20): {subset(IEEE[2], Fix(20,20))}")
