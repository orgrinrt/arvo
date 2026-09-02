#!/usr/bin/env python3
"""
p7. Attacking p3b's blocker: can a correlation-tracking grade be written, does
    it recover the licences the corner rule loses, and is it const-shaped?

THE BLOCKER
-----------
p3b measures that the corner rule loses 120 of the 136 available licences on
`(x + y) - y`, and that a reachable-set oracle recovers all of them and cannot
be a const predicate because its domain is `2^(W*k)`.  I named the middle
ground and did not build it, which is a blocker reported and left.  This is the
attack.

THE CANDIDATE
-------------
An AFFINE grade.  Instead of an interval, carry a linear form

    c0 + sum_i ci * e_i,     e_i in [-1, 1]

with one noise symbol e_i per declared leaf.  Addition and subtraction are
exact on this representation because they are linear, so the two occurrences
of `y` in `(x + y) - y` carry the same symbol and cancel.  Multiplication of
two non-constant forms is not linear, so it contributes a fresh symbol
carrying the product of the two radii, which is the standard affine-arithmetic
treatment and is sound rather than exact.

The interval is recovered from a form by summing the absolute coefficients, so
the discharge test is unchanged: does the form's interval fit the container.

WHY IT COULD BE CONST-SHAPED, WHICH IS THE POINT
------------------------------------------------
A term is known statically, so the number of leaves is known statically, so
the coefficient vector has a const length and every coefficient is a const
expression.  Nothing here needs arithmetic in type position: the same
associated-const carrier `109` P5 and `111` p8 use for a scalar bound carries a
fixed-length array just as well, since the array length is a property of the
term rather than of a generic parameter.  That is a claim about expressibility
and `p8` compiles it rather than leaving it asserted here.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. On the control shapes, the affine rule agrees with the corner rule
    exactly.  It cannot be better where the corner rule is already exact.
P2. On `(x + y) - y`, the affine rule recovers all 136 licences the oracle
    licenses, against the corner rule's 16.
P3. On `x * (y - y)`, the affine rule recovers all 64, because the y symbols
    cancel before the multiply.
P4. On `(x + y) * z` with z declared zero, the affine rule ALSO recovers the
    120, because a form with a zero radius is a constant and multiplying by it
    is linear.  So it reaches the annihilation case too, which I said in p3b no
    node-wise rule reaches.  If that holds, p3b's second source of conservatism
    is narrower than I stated it.
P5. Unsound stays 0 everywhere.
P6. Multiplication of two non-constant forms is strictly worse than the
    reachable set, so the affine rule is not exact in general, and I should be
    able to name a term where it is conservative.
"""

from fractions import Fraction
from itertools import product


class Prim:
    def __init__(self, W, F, signed, policy):
        self.W, self.F, self.signed, self.policy = W, F, signed, policy
        self.step = Fraction(1, 2**F)
        if signed:
            self.klo, self.khi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.klo, self.khi = 0, 2**W - 1
        self.lo, self.hi = self.klo * self.step, self.khi * self.step

    def values(self):
        return [k * self.step for k in range(self.klo, self.khi + 1)]

    def R(self, q):
        k = int(q / self.step) if q >= 0 else -int(-q / self.step)
        span = self.khi - self.klo + 1
        if self.policy == "sat":
            k = min(max(k, self.klo), self.khi)
        else:
            k = self.klo + (k - self.klo) % span
        return k * self.step

    def op(self, name, a, b):
        return self.R(exact(name, a, b))


def exact(name, a, b):
    if name == "add":
        return a + b
    if name == "sub":
        return a - b
    if name == "mul":
        return a * b
    raise ValueError(name)


# ---------------------------------------------------------------------------
# Terms
# ---------------------------------------------------------------------------

L = lambda n: ("leaf", n)
O = lambda op, a, b: ("op", op, a, b)


def leaves_of(t):
    return {t[1]} if t[0] == "leaf" else leaves_of(t[2]) | leaves_of(t[3])


def eval_exact(t, env):
    return env[t[1]] if t[0] == "leaf" else exact(t[1], eval_exact(t[2], env), eval_exact(t[3], env))


def eval_general(P, t, env):
    return (
        env[t[1]]
        if t[0] == "leaf"
        else P.op(t[1], eval_general(P, t[2], env), eval_general(P, t[3], env))
    )


def node_ok_exact(P, t, env):
    """Every reachable intermediate inside the container.  The oracle."""
    if t[0] == "leaf":
        return True
    if not node_ok_exact(P, t[2], env) or not node_ok_exact(P, t[3], env):
        return False
    v = eval_exact(t, env)
    return P.lo <= v <= P.hi


# ---------------------------------------------------------------------------
# Grade A: the interval corner rule (p3b's, and `111`'s)
# ---------------------------------------------------------------------------


def corner(t, g):
    if t[0] == "leaf":
        return g[t[1]]
    a, b = corner(t[2], g), corner(t[3], g)
    cs = [exact(t[1], x, y) for x in a for y in b]
    return (min(cs), max(cs))


def corner_ok(P, t, g):
    if t[0] == "leaf":
        lo, hi = g[t[1]]
        return P.lo <= lo and hi <= P.hi
    if not corner_ok(P, t[2], g) or not corner_ok(P, t[3], g):
        return False
    lo, hi = corner(t, g)
    return P.lo <= lo and hi <= P.hi


# ---------------------------------------------------------------------------
# Grade B: the affine form.  (centre, {symbol: coefficient})
# ---------------------------------------------------------------------------


class Aff:
    __slots__ = ("c0", "c")

    def __init__(self, c0, c=None):
        self.c0 = c0
        self.c = dict(c or {})

    def interval(self):
        r = sum(abs(v) for v in self.c.values())
        return (self.c0 - r, self.c0 + r)

    def radius(self):
        return sum(abs(v) for v in self.c.values())

    def is_const(self):
        return all(v == 0 for v in self.c.values())


_fresh = [0]


def aff_from_interval(lo, hi, sym):
    return Aff((lo + hi) / 2, {sym: (hi - lo) / 2})


def aff_add(a, b, sign=1):
    out = Aff(a.c0 + sign * b.c0, a.c)
    for k, v in b.c.items():
        out.c[k] = out.c.get(k, Fraction(0)) + sign * v
    return out


def aff_mul(a, b):
    """Exact when one side is a constant; otherwise a fresh symbol."""
    if a.is_const():
        return Aff(a.c0 * b.c0, {k: a.c0 * v for k, v in b.c.items()})
    if b.is_const():
        return Aff(a.c0 * b.c0, {k: b.c0 * v for k, v in a.c.items()})
    out = Aff(a.c0 * b.c0, {})
    for k, v in a.c.items():
        out.c[k] = out.c.get(k, Fraction(0)) + v * b.c0
    for k, v in b.c.items():
        out.c[k] = out.c.get(k, Fraction(0)) + v * a.c0
    _fresh[0] += 1
    out.c[f"_e{_fresh[0]}"] = a.radius() * b.radius()
    return out


def affine(t, g):
    if t[0] == "leaf":
        return g[t[1]]
    a, b = affine(t[2], g), affine(t[3], g)
    if t[1] == "add":
        return aff_add(a, b, 1)
    if t[1] == "sub":
        return aff_add(a, b, -1)
    return aff_mul(a, b)


def affine_ok(P, t, g):
    if t[0] == "leaf":
        lo, hi = g[t[1]].interval()
        return P.lo <= lo and hi <= P.hi
    if not affine_ok(P, t[2], g) or not affine_ok(P, t[3], g):
        return False
    lo, hi = affine(t, g).interval()
    return P.lo <= lo and hi <= P.hi


# ---------------------------------------------------------------------------
# The sweep
# ---------------------------------------------------------------------------


def sweep(P, t, bound_range, label):
    names = sorted(leaves_of(t))
    values = P.values()
    rows = dict(
        corner_yes=0,
        affine_yes=0,
        oracle_yes=0,
        corner_unsound=0,
        affine_unsound=0,
        extents=0,
    )
    for bs in product(bound_range, repeat=len(names)):
        gi = {n: (Fraction(0), Fraction(b)) for n, b in zip(names, bs)}
        ga = {
            n: aff_from_interval(Fraction(0), Fraction(b), n)
            for n, b in zip(names, bs)
        }
        doms = [[v for v in values if 0 <= v <= b] for b in bs]
        if any(len(d) == 0 for d in doms):
            continue
        rows["extents"] += 1
        c = corner_ok(P, t, gi)
        a = affine_ok(P, t, ga)
        oracle = all(
            node_ok_exact(P, t, dict(zip(names, tup))) for tup in product(*doms)
        )
        agrees = all(
            eval_exact(t, dict(zip(names, tup))) == eval_general(P, t, dict(zip(names, tup)))
            for tup in product(*doms)
        )
        rows["corner_yes"] += int(c)
        rows["affine_yes"] += int(a)
        rows["oracle_yes"] += int(oracle)
        if c and not agrees:
            rows["corner_unsound"] += 1
        if a and not agrees:
            rows["affine_unsound"] += 1
    print(
        f"  {label:<34} corner {rows['corner_yes']:>5}/{rows['extents']:<5} "
        f"affine {rows['affine_yes']:>5}/{rows['extents']:<5} "
        f"oracle {rows['oracle_yes']:>5}/{rows['extents']:<5} "
        f"unsound c={rows['corner_unsound']} a={rows['affine_unsound']}"
    )
    return rows


def show(t):
    return t[1] if t[0] == "leaf" else f"({show(t[2])} {t[1]} {show(t[3])})"


def main():
    print("=" * 78)
    print("p7. An affine grade against the corner grade and the oracle")
    print("=" * 78)

    P4 = Prim(4, 0, False, "sat")
    S4 = Prim(4, 0, True, "sat")

    print()
    print("CONTROL: shapes where the corner rule is already exact")
    print()
    sweep(P4, O("add", L("x"), L("y")), range(0, 16), "x + y, unsigned sat W=4")
    sweep(
        P4,
        O("add", O("add", L("x"), L("y")), L("z")),
        range(0, 16),
        "(x + y) + z, unsigned sat W=4",
    )
    sweep(P4, O("mul", L("x"), L("y")), range(0, 16), "x * y, unsigned sat W=4")

    print()
    print("THE CASES THE CORNER RULE LOSES")
    print()
    sweep(
        P4,
        O("sub", O("add", L("x"), L("y")), L("y")),
        range(0, 16),
        "(x + y) - y, unsigned sat W=4",
    )
    sweep(
        S4,
        O("mul", L("x"), O("sub", L("y"), L("y"))),
        range(0, 8),
        "x * (y - y), SIGNED sat W=4",
    )
    sweep(
        P4,
        O("mul", O("add", L("x"), L("y")), L("z")),
        range(0, 16),
        "(x + y) * z, unsigned sat W=4",
    )

    print()
    print("WHERE THE AFFINE RULE IS ITSELF CONSERVATIVE")
    print()
    sweep(
        P4,
        O("mul", O("add", L("x"), L("y")), O("add", L("z"), L("w"))),
        range(0, 4),
        "(x+y) * (z+w), unsigned sat W=4",
    )
    sweep(
        S4,
        O("sub", O("mul", L("x"), L("y")), O("mul", L("x"), L("y"))),
        range(0, 4),
        "x*y - x*y, SIGNED sat W=4",
    )

    print()
    print("THE COEFFICIENT VECTOR'S LENGTH, which decides whether it is const-shaped")
    print()
    for t, rng, P in [
        (O("sub", O("add", L("x"), L("y")), L("y")), range(0, 16), P4),
        (O("mul", O("add", L("x"), L("y")), O("add", L("z"), L("w"))), range(0, 4), P4),
        (
            O(
                "mul",
                O("mul", O("add", L("a"), L("b")), O("add", L("c"), L("d"))),
                O("add", L("e"), L("f")),
            ),
            range(0, 2),
            P4,
        ),
    ]:
        names = sorted(leaves_of(t))
        ga = {n: aff_from_interval(Fraction(0), Fraction(1), n) for n in names}
        before = _fresh[0]
        form = affine(t, ga)
        print(
            f"  {show(t):<44} leaves {len(names)}, "
            f"symbols in the result {len(form.c)}, "
            f"fresh symbols added {_fresh[0] - before}"
        )
    print()
    print(
        "  So the vector's length is (leaves + non-constant multiplications), both"
    )
    print(
        "  of which are properties of the TERM and are therefore statically known."
    )

    print()
    print("INSTRUMENT CHECK")
    print()
    r = sweep(
        P4,
        O("sub", O("add", L("x"), L("y")), L("y")),
        range(0, 16),
        "the headline case, repeated",
    )
    print(
        f"  affine beats corner here: "
        f"{r['affine_yes'] > r['corner_yes']} "
        f"({r['affine_yes']} against {r['corner_yes']})"
    )
    r2 = sweep(P4, O("add", L("x"), L("y")), range(0, 16), "the control, repeated")
    print(
        f"  and does NOT beat it where the corner rule is exact: "
        f"{r2['affine_yes'] == r2['corner_yes']}"
    )
    print()
    print("  MUTATION: give the two occurrences of y different symbols")
    t = O("sub", O("add", L("x"), L("y")), L("y2"))
    names = sorted(leaves_of(t))
    yes = 0
    tot = 0
    for bx, by in product(range(0, 16), repeat=2):
        ga = {
            "x": aff_from_interval(Fraction(0), Fraction(bx), "x"),
            "y": aff_from_interval(Fraction(0), Fraction(by), "y"),
            "y2": aff_from_interval(Fraction(0), Fraction(by), "y2"),
        }
        tot += 1
        if affine_ok(P4, t, ga):
            yes += 1
    print(
        f"    with the correlation broken, the affine rule licenses {yes}/{tot}, "
        f"which should match the corner rule's 16"
    )


if __name__ == "__main__":
    main()
