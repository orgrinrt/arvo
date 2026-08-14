#!/usr/bin/env python3
"""
p1. Reproducing `114` F114-6 on my own instrument, before conceding it.

THE CLAIM AGAINST ME
--------------------
`112:928`, inside my offered canon statement, reads:

    Checking only the derivation's result rather than every node is unsound.

Unqualified.  `114` section 3.2 reports it false at `overflow policy = wrap`
with operations contained in the ring operations, where the root-only check is
unsound on 0 of 13032 and 0 of 2148 cells.  It also reports that my own
F112-21 at `112:1116` carries `overflow policy = sat` and is correct, so the
measurement was right and the predicate went missing when the finding was
compressed into a sentence.

A concession on someone else's numbers is not a concession, so this probe
builds the measurement independently: my own term enumeration, my own model,
my own propagation, no import from `114_probes`.

THE MECHANISM UNDER TEST
------------------------
`114` F114-1: a wrapping realisation map is a ring homomorphism and a
saturating one is not.  If that holds then reducing at every node and reducing
once at the root compute the same value, so the root's interval is the whole
condition and no intermediate node appears in it.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. The homomorphism identity R(R(a) op R(b)) == R(a op b) holds on every
    triple at `wrap` and fails on some at `sat`.  If it does not fail at
    `sat` the check cannot tell the two maps apart and proves nothing.
P2. The root-only check is unsound on zero cells at `wrap`.
P3. The root-only check is unsound on a nonzero count at `sat`, which is my
    F112-21 arriving from a systematic enumeration rather than from one hand
    witness.
P4. The cells where the root-only check licenses and the per-node check
    refuses are NONZERO at both policies.  If that count is zero the whole
    comparison is vacuous, because the two checks would never differ and a
    zero unsound count would say nothing.
P5. Adding a non-ring operation to the signature breaks P2, because the
    mechanism is the ring rather than the wrapping.

CONDITION-CAN-FIRE CHECK
------------------------
P4 is that check and it is reported first, before any verdict is read.
"""

from itertools import product


# ---------------------------------------------------------------------------
# The primitive.  Logical range only; F = 0 throughout, which is the region
# `114`'s arms are stated over and which p5 of this file leaves.
# ---------------------------------------------------------------------------


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
        if self.policy == "clampmin":  # a non-ring extra used only in C3
            return min(max(v, self.lo), self.hi)
        raise ValueError(self.policy)


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    if op == "mul":
        return a * b
    if op == "min":
        return a if a < b else b
    raise ValueError(op)


# ---------------------------------------------------------------------------
# Terms over n leaf slots, enumerated systematically rather than hand-picked.
# ---------------------------------------------------------------------------


def terms(nslots, ops):
    """Every binary tree over `nslots` leaf occurrences, with every operator
    assignment and every leaf-name assignment drawn from `nslots` names."""
    names = [chr(ord("x") + i) for i in range(nslots)]
    out = []

    def shapes(k):
        if k == 1:
            return [None]
        res = []
        for split in range(1, k):
            for l in shapes(split):
                for r in shapes(k - split):
                    res.append((l, r))
        return res

    def fill(shape, slots, op_assign, pos):
        if shape is None:
            return ("leaf", slots[pos[0]]), pos[0] + 1
        l, i = fill(shape[0], slots, op_assign, pos)
        r, j = fill(shape[1], slots, op_assign, [i])
        op = op_assign[fill.counter]
        fill.counter += 1
        return ("op", op, l, r), j

    for sh in shapes(nslots):
        nops = nslots - 1
        for op_assign in product(ops, repeat=nops):
            for slots in product(names, repeat=nslots):
                fill.counter = 0
                t, _ = fill(sh, list(slots), list(op_assign), [0])
                out.append(t)
    return out


def leaves_of(t):
    return {t[1]} if t[0] == "leaf" else leaves_of(t[2]) | leaves_of(t[3])


def eval_exact(t, env):
    return env[t[1]] if t[0] == "leaf" else ex(t[1], eval_exact(t[2], env), eval_exact(t[3], env))


def eval_pernode(P, t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return P.R(ex(t[1], eval_pernode(P, t[2], env), eval_pernode(P, t[3], env)))


def corner(t, g):
    if t[0] == "leaf":
        return g[t[1]]
    a, b = corner(t[2], g), corner(t[3], g)
    cs = [ex(t[1], x, y) for x in a for y in b]
    return (min(cs), max(cs))


def fits(P, iv):
    return P.lo <= iv[0] and iv[1] <= P.hi


def pernode_ok(P, t, g):
    if t[0] == "leaf":
        return fits(P, g[t[1]])
    if not pernode_ok(P, t[2], g) or not pernode_ok(P, t[3], g):
        return False
    return fits(P, corner(t, g))


def rootonly_ok(P, t, g):
    return fits(P, corner(t, g))


# ---------------------------------------------------------------------------


def sweep(P, ts, bounds_for, label):
    cells = agree = ro_lic = pn_lic = ro_unsound = pn_unsound = differ_checks = 0
    for t in ts:
        names = sorted(leaves_of(t))
        for bs in bounds_for(P, len(names)):
            g = {n: b for n, b in zip(names, bs)}
            doms = [list(range(g[n][0], g[n][1] + 1)) for n in names]
            if any(len(d) == 0 for d in doms):
                continue
            cells += 1
            ro = rootonly_ok(P, t, g)
            pn = pernode_ok(P, t, g)
            if ro != pn:
                differ_checks += 1
            ok = True
            for tup in product(*doms):
                env = dict(zip(names, tup))
                if eval_exact(t, env) != eval_pernode(P, t, env):
                    ok = False
                    break
            agree += int(ok)
            ro_lic += int(ro)
            pn_lic += int(pn)
            if ro and not ok:
                ro_unsound += 1
            if pn and not ok:
                pn_unsound += 1
    print(
        f"  {label:<40} cells {cells:>6}  agree {agree:>6}  "
        f"root-lic {ro_lic:>6}  pernode-lic {pn_lic:>6}  "
        f"ro-UNSOUND {ro_unsound:>5}  pn-unsound {pn_unsound:>4}  "
        f"checks-differ {differ_checks:>5}"
    )
    return dict(
        cells=cells,
        ro_unsound=ro_unsound,
        pn_unsound=pn_unsound,
        differ=differ_checks,
        ro_lic=ro_lic,
        pn_lic=pn_lic,
    )


def onesided(P, k):
    return [tuple((0, b) for b in bs) for bs in product(range(P.lo, P.hi + 1), repeat=k)]


def main():
    print("=" * 96)
    print("p1. Reproducing the root-only claim independently")
    print("=" * 96)

    # ---- P1: the homomorphism identity, which is the mechanism ------------
    print()
    print("P1. Is the realisation map a ring homomorphism")
    print()
    for P in [Prim(3, False, "wrap"), Prim(3, True, "wrap"), Prim(4, False, "wrap"),
              Prim(3, False, "sat"), Prim(3, True, "sat"), Prim(4, False, "sat")]:
        bad = n = 0
        span = range(P.lo * 3, P.hi * 3 + 1)
        for op in ("add", "sub", "mul"):
            for a in span:
                for b in span:
                    n += 1
                    if P.R(ex(op, P.R(a), P.R(b))) != P.R(ex(op, a, b)):
                        bad += 1
        print(f"  {str(P):<12} R(R(a) op R(b)) != R(a op b) on {bad:>6} of {n}")

    # ---- the sweeps -------------------------------------------------------
    ts2 = terms(2, ("add", "sub", "mul"))
    ts3 = terms(3, ("add", "sub", "mul"))
    print()
    print(f"Term enumeration: {len(ts2)} terms at 2 leaf slots, {len(ts3)} at 3")
    print()
    print("P2/P3/P4. The root-only check against the per-node check")
    print()
    res = {}
    for P in [Prim(3, False, "wrap"), Prim(3, True, "wrap"),
              Prim(3, False, "sat"), Prim(3, True, "sat")]:
        res[str(P)] = sweep(P, ts2 + ts3, onesided, f"{P}, one-sided, arity 2 and 3")

    print()
    print("THE CONDITION-CAN-FIRE CHECK, read before any verdict")
    print()
    for k, v in res.items():
        print(
            f"  {k:<14} the two checks disagree on {v['differ']:>6} cells "
            f"({'LIVE' if v['differ'] > 0 else 'VACUOUS, do not read the verdict'})"
        )

    print()
    print("VERDICTS")
    print()
    for k, v in res.items():
        print(
            f"  {k:<14} root-only unsound on {v['ro_unsound']:>5} of {v['cells']} cells, "
            f"per-node unsound on {v['pn_unsound']}"
        )

    # ---- P5: it is the ring, not the wrapping -----------------------------
    print()
    print("P5. Add a non-ring operation and the wrapping rows should break")
    print()
    tsm2 = terms(2, ("add", "sub", "mul", "min"))
    tsm3 = terms(3, ("add", "sub", "mul", "min"))
    for P in [Prim(3, False, "wrap"), Prim(3, True, "wrap")]:
        bad = n = 0
        span = range(P.lo * 3, P.hi * 3 + 1)
        for a in span:
            for b in span:
                n += 1
                if P.R(ex("min", P.R(a), P.R(b))) != P.R(ex("min", a, b)):
                    bad += 1
        print(f"  {str(P):<12} the identity fails for min on {bad} of {n}")
        sweep(P, tsm2 + tsm3, onesided, f"{P}, signature with min")

    print()
    print("INSTRUMENT CHECK")
    print()
    print("  MUTATION: halve the propagated interval, which must go unsound everywhere")

    def halved_corner(t, g):
        lo, hi = corner(t, g)
        mid = (lo + hi) // 2
        return (mid, mid)

    saved = globals()["corner"]
    P = Prim(3, False, "wrap")
    bad = n = 0
    for t in ts2:
        names = sorted(leaves_of(t))
        for bs in onesided(P, len(names)):
            g = {nm: b for nm, b in zip(names, bs)}
            doms = [list(range(g[nm][0], g[nm][1] + 1)) for nm in names]
            if any(len(d) == 0 for d in doms):
                continue
            n += 1
            if fits(P, halved_corner(t, g)):
                ok = all(
                    eval_exact(t, dict(zip(names, tp))) == eval_pernode(P, t, dict(zip(names, tp)))
                    for tp in product(*doms)
                )
                if not ok:
                    bad += 1
    print(f"    a halved-interval rule at uW3/wrap is unsound on {bad} of {n} cells")
    print(f"    so the soundness counter is live at wrap and its zero is a result: {bad > 0}")


if __name__ == "__main__":
    main()
