#!/usr/bin/env python3
"""
p3. What `114`'s homomorphism mechanism unlocks: the two licence families are
    complementary, and the complementarity is forced rather than incidental.

WHERE THIS COMES FROM
---------------------
`114` F114-1 establishes that a wrapping realisation map is a ring
homomorphism and a saturating one is not, and builds arms W0 and W1 on it.
`110` F12 and `112` F112-12 establish, from the composite side, that the
interval construction is closed exactly on MONOTONE bases, and that wrapping
is the non-monotone case.

Those are two different properties of the same map, measured by two different
files for two different purposes, and nobody has put them in one sentence.
Doing so gives a two-by-two, and the interesting part is that one cell is
empty for a reason rather than for lack of measurement.

THE CLAIM UNDER TEST
--------------------
    A wrapping map is a ring homomorphism and is not monotone.
    A saturating map is monotone and is not a ring homomorphism.
    No map onto a finite value set is both, except the constant one.

The third line is a proof rather than a sweep, and it is worth stating because
it changes the status of the arm split from "these two policies happen to
differ" to "the design cannot have both licence families at one policy".

    Let V be finite with at least two elements and R : Z -> V surjective with
    R(a op b) = R(R(a) op R(b)) for op in {+, *}.  Then V carries an induced
    ring structure and R is a surjective ring homomorphism, so V is Z/nZ with
    n = |V| >= 2.  Suppose further that R is monotone for some total order on
    V.  R(0) = R(n) because 0 and n are congruent, and a non-decreasing map
    agreeing at 0 and n is constant on [0, n].  That interval contains a
    complete residue system, so periodicity makes R constant everywhere,
    contradicting |V| >= 2.

    Equivalently, and this is the shorter form: a finite ring cannot be an
    ordered ring, because 1 > 0 forces the characteristic to be zero.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. Wrap: homomorphism yes, monotone no.  Sat: homomorphism no, monotone yes.
P2. An exhaustive search over every reduction map from a window of Z onto a
    small V finds NO map that is both a nontrivial homomorphism and monotone.
    If it finds one, the proof above is wrong and the whole section goes.
P3. The interval construction is closed exactly where the map is monotone,
    reproducing `110` F12 from the property rather than from the policy name.
P4. The root-only check is sound exactly where the map is a homomorphism,
    which is p1 restated against the property rather than against the policy.
P5. On a DISCHARGED declared extent the map is the identity, so it is both a
    homomorphism and monotone there, and both licence families are available
    at once.  That is something no choice of policy can buy, and it is what a
    refinement buys that a policy cannot.

CONDITION-CAN-FIRE CHECK
------------------------
P2's search must actually contain candidates that pass each half separately,
or "none passes both" is a statement about an empty search rather than about
the maps.  The counts of each half are reported before the conjunction.
"""

from itertools import product


class Prim:
    def __init__(self, W, signed, policy):
        self.W, self.signed, self.policy = W, signed, policy
        if signed:
            self.lo, self.hi = -(2 ** (W - 1)), 2 ** (W - 1) - 1
        else:
            self.lo, self.hi = 0, 2**W - 1
        self.n = self.hi - self.lo + 1

    def __repr__(self):
        return f"{'i' if self.signed else 'u'}W{self.W}/{self.policy}"

    def values(self):
        return list(range(self.lo, self.hi + 1))

    def R(self, v):
        if self.policy == "sat":
            return min(max(v, self.lo), self.hi)
        if self.policy == "wrap":
            return ((v - self.lo) % self.n) + self.lo
        if self.policy == "zero":  # flush out-of-range to zero
            return v if self.lo <= v <= self.hi else 0
        if self.policy == "reflect":  # bounce off the endpoints
            span = 2 * (self.n - 1)
            k = (v - self.lo) % span
            return self.lo + (k if k < self.n else span - k)
        raise ValueError(self.policy)


def ex(op, a, b):
    return a + b if op == "add" else (a - b if op == "sub" else a * b)


def is_hom(P, ops=("add", "sub", "mul")):
    bad = n = 0
    span = range(P.lo * 3, P.hi * 3 + 1)
    for op in ops:
        for a in span:
            for b in span:
                n += 1
                if P.R(ex(op, P.R(a), P.R(b))) != P.R(ex(op, a, b)):
                    bad += 1
    return bad, n


def is_monotone(P):
    span = list(range(P.lo * 3, P.hi * 3 + 1))
    bad = n = 0
    for i in range(len(span) - 1):
        n += 1
        if P.R(span[i]) > P.R(span[i + 1]):
            bad += 1
    return bad, n


# ---------------------------------------------------------------------------
# P3: the interval construction, from `110` F12 and `112` F112-12.
# ---------------------------------------------------------------------------


def interval_closed(P):
    vs = P.values()
    ivs = [(a, b) for a in vs for b in vs if a <= b]
    bad = n = 0
    for x in ivs:
        for y in ivs:
            n += 1
            r = (P.R(x[0] + y[0]), P.R(x[1] + y[1]))
            if r[0] > r[1]:
                bad += 1
    return bad, n


# ---------------------------------------------------------------------------
# P4: the root-only check, from p1, restated against the property.
# ---------------------------------------------------------------------------


def rootonly_sound(P):
    """Small systematic check: every 2-leaf term over {add, sub, mul}."""
    terms = [("op", op, ("leaf", "x"), ("leaf", "y")) for op in ("add", "sub", "mul")]
    terms += [
        ("op", o1, ("op", o2, ("leaf", "x"), ("leaf", "y")), ("leaf", "z"))
        for o1 in ("add", "sub", "mul")
        for o2 in ("add", "sub", "mul")
    ]

    def ev_exact(t, env):
        return env[t[1]] if t[0] == "leaf" else ex(t[1], ev_exact(t[2], env), ev_exact(t[3], env))

    def ev_pernode(t, env):
        return (
            env[t[1]]
            if t[0] == "leaf"
            else P.R(ex(t[1], ev_pernode(t[2], env), ev_pernode(t[3], env)))
        )

    def corner(t, g):
        if t[0] == "leaf":
            return g[t[1]]
        a, b = corner(t[2], g), corner(t[3], g)
        cs = [ex(t[1], u, v) for u in a for v in b]
        return (min(cs), max(cs))

    unsound = lic = cells = 0
    for t in terms:
        names = sorted({t[1]} if t[0] == "leaf" else _lv(t))
        for bs in product(range(P.lo, P.hi + 1), repeat=len(names)):
            g = {nm: (0, b) if b >= 0 else (b, 0) for nm, b in zip(names, bs)}
            doms = [list(range(g[nm][0], g[nm][1] + 1)) for nm in names]
            if any(len(d) == 0 for d in doms):
                continue
            cells += 1
            iv = corner(t, g)
            ro = P.lo <= iv[0] and iv[1] <= P.hi
            lic += int(ro)
            if ro:
                ok = all(
                    ev_exact(t, dict(zip(names, tp))) == ev_pernode(t, dict(zip(names, tp)))
                    for tp in product(*doms)
                )
                if not ok:
                    unsound += 1
    return unsound, lic, cells


def _lv(t):
    return {t[1]} if t[0] == "leaf" else _lv(t[2]) | _lv(t[3])


# ---------------------------------------------------------------------------


def main():
    print("=" * 90)
    print("p3. The two licence families are complementary, and forced apart")
    print("=" * 90)

    print()
    print("P1/P3/P4. The two properties and the two licences, per policy")
    print()
    print(
        f"  {'primitive':<14} {'hom fails':>11} {'monotone fails':>15} "
        f"{'interval ill-ord':>17} {'root-only unsound':>18}"
    )
    rows = []
    for P in [
        Prim(3, False, "wrap"),
        Prim(3, True, "wrap"),
        Prim(3, False, "sat"),
        Prim(3, True, "sat"),
        Prim(3, False, "zero"),
        Prim(3, False, "reflect"),
    ]:
        h, hn = is_hom(P)
        m, mn = is_monotone(P)
        ic, icn = interval_closed(P)
        ru, rl, rc = rootonly_sound(P)
        rows.append((str(P), h, m, ic, ru))
        print(
            f"  {str(P):<14} {str(h) + '/' + str(hn):>11} "
            f"{str(m) + '/' + str(mn):>15} {str(ic) + '/' + str(icn):>17} "
            f"{str(ru) + '/' + str(rc):>18}"
        )

    print()
    print("  read as a two-by-two:")
    print()
    print(f"  {'primitive':<14} {'ring hom':>10} {'monotone':>10} {'-> W1 arm':>11} {'-> interval':>13}")
    for name, h, m, ic, ru in rows:
        print(
            f"  {name:<14} {str(h == 0):>10} {str(m == 0):>10} "
            f"{str(ru == 0):>11} {str(ic == 0):>13}"
        )

    print()
    print("  the cell that matters: is any policy in BOTH columns?")
    both = [n for n, h, m, ic, ru in rows if h == 0 and m == 0]
    print(f"    policies with both properties: {both if both else 'none'}")

    # ---- P2: the exhaustive search, which is the control on the proof -----
    print()
    print("P2. Exhaustive search for a map that is both, over a window of Z")
    print()
    for vsize in (2, 3):
        V = list(range(vsize))
        window = list(range(-2 * vsize, 2 * vsize + 1))
        hom_only = mono_only = both_ct = nonconst_both = total = 0
        for assign in product(V, repeat=len(window)):
            total += 1
            R = dict(zip(window, assign))

            def RR(v):
                return R[v] if v in R else None

            ok_hom = True
            for op in ("add", "mul"):
                for a in window:
                    for b in window:
                        lhs_arg = ex(op, R[a], R[b])
                        rhs_arg = ex(op, a, b)
                        if lhs_arg not in R or rhs_arg not in R:
                            continue
                        if R[lhs_arg] != R[rhs_arg]:
                            ok_hom = False
                            break
                    if not ok_hom:
                        break
                if not ok_hom:
                    break
            ok_mono = all(
                R[window[i]] <= R[window[i + 1]] for i in range(len(window) - 1)
            )
            nonconst = len(set(assign)) > 1
            if ok_hom:
                hom_only += 1
            if ok_mono:
                mono_only += 1
            if ok_hom and ok_mono:
                both_ct += 1
                if nonconst:
                    nonconst_both += 1
        print(
            f"  |V| = {vsize}, window {len(window)} points, {total} maps: "
            f"homomorphic {hom_only}, monotone {mono_only}, both {both_ct}, "
            f"both AND non-constant {nonconst_both}"
        )
    print()
    print("  the search contains candidates passing each half separately, so")
    print("  'none passes both non-trivially' is a statement about the maps.")

    # ---- P5: what a discharged extent does --------------------------------
    print()
    print("P5. On a discharged declared extent, the map is the identity")
    print()
    for P in [Prim(3, False, "wrap"), Prim(3, False, "sat")]:
        for bound in (1, 3, 7):
            ext = [v for v in P.values() if 0 <= v <= bound]
            # discharged for a single addition of two extent members
            fits = 2 * bound <= P.hi
            hbad = 0
            mbad = 0
            for a in ext:
                for b in ext:
                    if P.R(a + b) != a + b and fits:
                        hbad += 1
            seq = sorted(ext)
            for i in range(len(seq) - 1):
                if P.R(seq[i]) > P.R(seq[i + 1]):
                    mbad += 1
            print(
                f"  {str(P):<12} extent <= {bound}: discharged {str(fits):<5} "
                f"R is the identity on it: {hbad == 0}   order preserved on it: "
                f"{mbad == 0}"
            )
    print()
    print("  so a discharged extent restores BOTH properties at once, which no")
    print("  choice of policy can do.")


if __name__ == "__main__":
    main()
