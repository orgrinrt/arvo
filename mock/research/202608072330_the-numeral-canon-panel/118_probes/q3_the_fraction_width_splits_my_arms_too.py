#!/usr/bin/env python3
"""q3. `116` F116-7 reproduced, and what it does to arm W0 and to the cut rule.

WHAT `116` FOUND
----------------
F116-7: at `F > 0` under wrap the realisation map is still a ring homomorphism
for addition and subtraction and is **not** one for multiplication, 608 of 2116
at `F = 1` and 1234 at `F = 2`. F116-8: so the root-only check goes unsound on
2079 to 16063 cells the moment a product appears. F116-9: the multiplicative
homomorphism is restored exactly when the operands are declared on the unit grid,
and not when their products merely avoid requantisation, which refuted `116`'s own
predicted mechanism.

`116` bounded `114`'s arm W1 with that and split it into three. It did not ask
what the same mechanism does to `114`'s **arm W0**, which is the licence to drop
intermediate reductions, nor to the **cut rule** that extends W0 across a
non-homomorphic operation. Both rest on the same homomorphism and both should
split the same way, and if they do the cut rule's statement gets strictly better:
the set of operations to cut at stops being "the non-ring operations" and becomes
"the operations for which the map is not a homomorphism here", which is a
function of the fraction width as well as the policy.

`116` also did not compose F116-7 with its own F116-4. If the trade between the
two licence families is forced by **addition alone**, which `118` q2 measures, then
a map that is a homomorphism only for addition still cannot be monotone, so the
trade holds at every fraction width rather than only at `F = 0`.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. F116-7 reproduces on my own model: zero homomorphism failures for add and
    sub at every `F`, nonzero for mul at `F > 0`, zero for mul at `F = 0`.
P2. The map is not monotone at any `F` under wrap. With q2's result that addition
    alone forces the trade, the two licence families stay disjoint at `F > 0`.
P3. Arm W0 splits along the same line: dropping every intermediate reduction is
    still unconditionally equal to reducing at every node on terms with no
    multiplication, at any `F`, and is not equal on terms with one.
P4. The cut rule generalises: reducing the operands of every **non-homomorphic**
    operation and the root equals reducing at every node, at `F > 0` with mul in
    the cut set.
P5. Reducing the non-homomorphic operation's **result** instead of its operands
    does NOT work, so the placement of the cut is load-bearing and P4 is not
    "reduce somewhere near the multiply".
P6. F116-9 reproduces: the unit-grid declaration restores the multiplicative
    homomorphism, and a grid on which products need no requantisation does not.

NEGATIVE CONTROLS
-----------------
C1. The `F = 0` rows must show zero failures on all three operations, or the
    instrument cannot tell the fraction widths apart.
C2. A saturating map must fail the homomorphism at every `F` including zero, or
    the measurement is about the fraction width rather than about the map.
C3. P5 is a control on P4: if both arrangements work, the cut placement is not
    what makes it work and P4 is measuring something else.
C4. Arm W0 must be measured on cells where the root leaves the range, or the
    "unconditional" half is being read off cells where nothing could differ.
"""

from fractions import Fraction
from itertools import product
import importlib.util
import random
import sys
from pathlib import Path

sys.setrecursionlimit(10000)
HERE = Path(__file__).parent
P114 = HERE.parent / "114_probes"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


p1 = load("p1_local", P114 / "p1_the_structural_predicate_on_a_systematic_term_enumeration.py")


class Fx:
    """A fixed-point primitive at width W and fraction width F."""

    def __init__(self, W, F, signed, policy):
        self.W, self.F, self.signed, self.policy = W, F, signed, policy
        self.step = Fraction(1, 2 ** F)
        n = 1 << W
        self.klo = -(n // 2) if signed else 0
        self.khi = (n // 2 - 1) if signed else n - 1
        self.lo, self.hi = self.klo * self.step, self.khi * self.step

    def label(self):
        return f"{'i' if self.signed else 'u'}W{self.W}F{self.F}/{self.policy}"

    def raws(self):
        return range(self.klo, self.khi + 1)

    def values(self):
        return [k * self.step for k in self.raws()]

    def R(self, q):
        k = int(q / self.step) if q >= 0 else -int(-q / self.step)  # trunc toward zero
        span = self.khi - self.klo + 1
        if self.policy == "sat":
            k = min(max(k, self.klo), self.khi)
        else:
            k = self.klo + (k - self.klo) % span
        return k * self.step


OPS = ("add", "sub", "mul")


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    return a * b


def hom_failures(P, op, ambient_mult=3):
    """R(R(a) op R(b)) against R(a op b), over an ambient range wider than the
    container so the reduction actually fires."""
    span = P.khi - P.klo + 1
    ks = range(P.klo - span, P.khi + span + 1)
    dom = [k * P.step for k in ks]
    bad = tot = 0
    for a in dom:
        for b in dom:
            tot += 1
            if P.R(ex(op, P.R(a), P.R(b))) != P.R(ex(op, a, b)):
                bad += 1
    return bad, tot


def is_monotone(P, ambient_mult=3):
    span = P.khi - P.klo + 1
    ks = list(range(P.klo - span, P.khi + span + 1))
    vals = [P.R(k * P.step) for k in ks]
    return all(vals[i] <= vals[i + 1] for i in range(len(vals) - 1))


# ------------------------------------------------------------------ the arms


def ev_every_node(P, t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return P.R(ex(t[0], ev_every_node(P, t[1], env), ev_every_node(P, t[2], env)))


def ev_exact(t, env):
    if t[0] == "leaf":
        return env[t[1]]
    return ex(t[0], ev_exact(t[1], env), ev_exact(t[2], env))


def ev_w0(P, t, env):
    """Arm W0: no intermediate reduction, one at the root."""
    return P.R(ev_exact(t, env))


def ev_cut(P, t, env, cutset, where="operands"):
    """Reduce at every node whose operation is in `cutset`, plus the root.
    `where` decides whether the cut reduces the operation's operands or its
    result, which P5 says is not the same thing."""

    def go(node):
        if node[0] == "leaf":
            return env[node[1]]
        x, y = go(node[1]), go(node[2])
        if node[0] in cutset and where in ("operands", "both"):
            x, y = P.R(x), P.R(y)
        v = ex(node[0], x, y)
        if node[0] in cutset and where in ("result", "both"):
            v = P.R(v)
        return v

    return P.R(go(t))


def has_mul(t):
    return any(nd[0] == "mul" for nd in p1.internal(t))


def sweep_arms(P, terms, decls_per_term, seed, cutset):
    rng = random.Random(seed)
    out = {k: 0 for k in ("cells", "w0", "cut_op", "cut_res", "cut_both", "root_out")}
    for t in terms:
        k = max(p1.leaves(t)) + 1
        for _ in range(decls_per_term):
            bs = [rng.choice(list(P.raws())) for _ in range(k)]
            los = [min(0, b) for b in bs]
            doms = [[v * P.step for v in range(min(lo, b), max(lo, b) + 1)]
                    for lo, b in zip(los, bs)]
            if any(len(d) == 0 for d in doms):
                continue
            out["cells"] += 1
            bad = {"w0": False, "cut_op": False, "cut_res": False, "cut_both": False}
            ro = False
            for tup in product(*doms):
                env = dict(enumerate(tup))
                base = ev_every_node(P, t, env)
                if ev_w0(P, t, env) != base:
                    bad["w0"] = True
                if ev_cut(P, t, env, cutset, "operands") != base:
                    bad["cut_op"] = True
                if ev_cut(P, t, env, cutset, "result") != base:
                    bad["cut_res"] = True
                if ev_cut(P, t, env, cutset, "both") != base:
                    bad["cut_both"] = True
                e = ev_exact(t, env)
                if not (P.lo <= e <= P.hi):
                    ro = True
            for kk in bad:
                out[kk] += bad[kk]
            out["root_out"] += ro
    return out


def main():
    print("=" * 106)
    print("q3. The fraction width splits arm W0 and the cut rule the same way")
    print("=" * 106)

    # --------------------------------------------------------------- P1, C1, C2
    print()
    print("P1, C1 and C2. The homomorphism, per operation, per fraction width.")
    print()
    print(f"  {'primitive':<14} {'add':>14} {'sub':>14} {'mul':>14}   {'monotone':>9}")
    for P in (
        Fx(4, 0, False, "wrap"), Fx(4, 1, False, "wrap"), Fx(4, 2, False, "wrap"),
        Fx(4, 2, True, "wrap"),
        Fx(4, 0, False, "sat"), Fx(4, 1, False, "sat"), Fx(4, 2, False, "sat"),
    ):
        cells = []
        for op in OPS:
            bad, tot = hom_failures(P, op)
            cells.append(f"{bad:>6}/{tot:<7}")
        print(f"  {P.label():<14} " + " ".join(cells) + f"   {str(is_monotone(P)):>9}")
    print()
    print("  C1 is the F0 wrap row: zero on all three. C2 is the sat rows: nonzero")
    print("  at every F including zero, so the failure is the map and not the grid.")
    print()
    print("P2. The monotone column is False on every wrap row. With q2's result that")
    print("  ADDITION ALONE forces the trade, a map that keeps the homomorphism only")
    print("  for add and sub still cannot be monotone, so the two licence families")
    print("  stay disjoint at every fraction width rather than only at F = 0.")

    # ------------------------------------------------------- P3, P4, P5, C3, C4
    print()
    print("-" * 106)
    print("P3, P4, P5. Arm W0 and the cut rule at a nonzero fraction width.")
    print("The cut set is the operations for which the map is NOT a homomorphism")
    print("here, which is empty at F = 0 over the ring and {mul} at F > 0.")
    print()
    terms = p1.all_terms(2) + p1.all_terms(3)
    nomul = [t for t in terms if not has_mul(t)]
    withmul = [t for t in terms if has_mul(t)]
    print(f"  terms: {len(terms)} total, {len(nomul)} without a multiplication, "
          f"{len(withmul)} with one")
    print()
    print(f"  {'primitive':<14} {'class':<10} {'cutset':<8} {'cells':>6} {'root out':>9} "
          f"{'W0 differs':>11} {'cut@operands':>13} {'cut@result':>11} {'cut@both':>9}")
    for P, cutset in (
        (Fx(4, 0, False, "wrap"), frozenset()),
        (Fx(4, 1, False, "wrap"), frozenset({"mul"})),
        (Fx(4, 2, False, "wrap"), frozenset({"mul"})),
        (Fx(3, 2, True, "wrap"), frozenset({"mul"})),
        (Fx(4, 1, False, "sat"), frozenset({"mul"})),
    ):
        for cname, tset in (("no-mul", nomul), ("with-mul", withmul)):
            r = sweep_arms(P, tset, 3, 20260814, cutset)
            cs = "{}" if not cutset else "{mul}"
            print(
                f"  {P.label():<14} {cname:<10} {cs:<8} {r['cells']:>6} "
                f"{r['root_out']:>9} {r['w0']:>11} {r['cut_op']:>13} "
                f"{r['cut_res']:>11} {r['cut_both']:>9}",
                flush=True,
            )
    print()
    print("  C4 is the 'root out' column: nonzero means the sweep contains cells")
    print("  where the exact result leaves the range, so a zero in 'W0 differs' is")
    print("  a result rather than a region where nothing could differ.")
    print()
    print("  C3 is 'cut@result' against 'cut@operands'. If both are zero the")
    print("  placement is not what makes the cut work.")

    # ------------------------------------------------------------------- P7
    # Added after the first run, which REFUTED P4: cutting at the operands is
    # not enough, and neither is cutting at the result, and cutting at both
    # works. That points at a sharper rule than "cut at the non-homomorphic
    # operation", because the reduction at a node does two jobs and only one of
    # them is failing.
    #
    # `112` F112-4 established that the map has two parts: a grid part and a
    # range part. A multiplication leaves the GRID, because a product of two
    # grid values lands on the finer grid s^2. The range part is modular and
    # stays a homomorphism at any F, which is exactly what the add and sub rows
    # above measure. So the prediction is:
    #
    #   the GRID part must be applied at every node that leaves the grid;
    #   the RANGE part may be deferred to the root.
    #
    # P7. Requantising at each multiplication, without any range reduction
    #     there, equals reducing at every node. If it does, "cut at both" was
    #     the coarse form and this is the statement.
    # C5. Deferring the grid part too, which is arm W0 unchanged, must differ,
    #     or the grid part is not what has to be local.
    print()
    print("-" * 106)
    print("P7. Which HALF of the map has to be applied at the node.")
    print("`112` F112-4 splits the map into a grid part and a range part. The")
    print("range part is modular and stays a homomorphism at any F, per the add")
    print("and sub rows above. So the prediction is that only the GRID part has")
    print("to be local, and the range part may be deferred to the root.")
    print()

    def quantise(P, q):
        k = int(q / P.step) if q >= 0 else -int(-q / P.step)
        return k * P.step

    def ev_quantise_at_mul(P, t, env):
        def go(node):
            if node[0] == "leaf":
                return env[node[1]]
            v = ex(node[0], go(node[1]), go(node[2]))
            return quantise(P, v) if node[0] == "mul" else v
        return P.R(go(t))

    print(f"  {'primitive':<14} {'class':<10} {'cells':>6} {'root out':>9} "
          f"{'W0 differs [C5]':>16} {'quantise-at-mul':>16} {'cut@both':>9}")
    for P in (Fx(4, 1, False, "wrap"), Fx(4, 2, False, "wrap"), Fx(3, 2, True, "wrap")):
        for cname, tset in (("with-mul", withmul), ("no-mul", nomul)):
            rng = random.Random(20260814)
            cells = w0 = qm = cb = ro = 0
            for t in tset:
                k = max(p1.leaves(t)) + 1
                for _ in range(3):
                    bs = [rng.choice(list(P.raws())) for _ in range(k)]
                    doms = [[v * P.step for v in range(min(0, b), max(0, b) + 1)]
                            for b in bs]
                    if any(len(d) == 0 for d in doms):
                        continue
                    cells += 1
                    b_w0 = b_qm = b_cb = b_ro = False
                    for tup in product(*doms):
                        env = dict(enumerate(tup))
                        base = ev_every_node(P, t, env)
                        if ev_w0(P, t, env) != base:
                            b_w0 = True
                        if ev_quantise_at_mul(P, t, env) != base:
                            b_qm = True
                        if ev_cut(P, t, env, frozenset({"mul"}), "both") != base:
                            b_cb = True
                        e = ev_exact(t, env)
                        if not (P.lo <= e <= P.hi):
                            b_ro = True
                    w0 += b_w0
                    qm += b_qm
                    cb += b_cb
                    ro += b_ro
            print(f"  {P.label():<14} {cname:<10} {cells:>6} {ro:>9} {w0:>16} "
                  f"{qm:>16} {cb:>9}", flush=True)
    print()
    print("  P7 holds when 'quantise-at-mul' is zero on the with-mul rows while")
    print("  'W0 differs' is nonzero on the same rows, which is C5: deferring the")
    print("  grid part too must break it, or the grid part is not what is local.")

    # ------------------------------------------------------------------- P8
    # Added after the second run, which REFUTED P7 as well: requantising at the
    # multiplication is not enough either, and only reducing both its operands
    # and its result works. Two refutations in a row point somewhere, and the
    # place they point is the ROUNDING MODE, which no sweep in this sitting has
    # moved and which `116` section 7 names as the next axis it would look at.
    #
    # `R` is a composition: quantise to the grid, then reduce into range. For
    # the range part to be deferrable past an addition, the grid part has to
    # commute with adding an on-grid value. Truncation TOWARD ZERO does not:
    #   trunc(-1/2 + 1) = 0  while  trunc(-1/2) + 1 = 1.
    # FLOOR does, because it is translation-invariant by an integer multiple of
    # the step. So the prediction is that the deferral licence is a fact about
    # the rounding mode and not only about the fraction width.
    #
    # P8. Under floor, requantising at each multiplication and deferring the
    #     range part to the root equals reducing at every node. Under truncation
    #     toward zero it does not, which is the row already measured.
    # C6. Terms containing a multiplication and NO subtraction must behave the
    #     same under both rounding modes on an unsigned base, because nothing
    #     reaches a negative value and the two modes coincide there. If they
    #     differ, the mechanism is not the one named.
    print()
    print("-" * 106)
    print("P8. The deferral licence is a fact about the ROUNDING MODE.")
    print("Two refutations in a row point here: R is quantise-then-reduce, and")
    print("the range part can only be deferred past an addition if the grid part")
    print("commutes with adding an on-grid value. Truncation toward zero does not")
    print("commute; floor does.")
    print()

    def mk(P, rounding):
        Q = Fx(P.W, P.F, P.signed, P.policy)
        Q.rounding = rounding
        return Q

    def q_of(P, q, rounding):
        if rounding == "floor":
            import math
            k = math.floor(q / P.step)
        else:
            k = int(q / P.step) if q >= 0 else -int(-q / P.step)
        return k * P.step

    def R_of(P, q, rounding):
        k = q_of(P, q, rounding) / P.step
        k = int(k)
        span = P.khi - P.klo + 1
        if P.policy == "sat":
            k = min(max(k, P.klo), P.khi)
        else:
            k = P.klo + (k - P.klo) % span
        return k * P.step

    def ev_every(P, t, env, rounding):
        if t[0] == "leaf":
            return env[t[1]]
        return R_of(P, ex(t[0], ev_every(P, t[1], env, rounding),
                          ev_every(P, t[2], env, rounding)), rounding)

    def ev_qmul(P, t, env, rounding):
        def go(node):
            if node[0] == "leaf":
                return env[node[1]]
            v = ex(node[0], go(node[1]), go(node[2]))
            return q_of(P, v, rounding) if node[0] == "mul" else v
        return R_of(P, go(t), rounding)

    def ev_w0r(P, t, env, rounding):
        return R_of(P, ev_exact(t, env), rounding)

    has_sub = lambda t: any(nd[0] == "sub" for nd in p1.internal(t))
    mul_nosub = [t for t in withmul if not has_sub(t)]
    print(f"  with-mul terms: {len(withmul)}, of which without a subtraction: "
          f"{len(mul_nosub)}")
    print()
    print(f"  {'primitive':<14} {'rounding':<9} {'class':<14} {'cells':>6} "
          f"{'root out':>9} {'W0 differs':>11} {'quantise-at-mul':>16}")
    for P in (Fx(4, 1, False, "wrap"), Fx(4, 2, False, "wrap"), Fx(3, 2, True, "wrap")):
        for rounding in ("trunc", "floor"):
            for cname, tset in (("with-mul", withmul), ("mul, no sub [C6]", mul_nosub)):
                rng = random.Random(20260814)
                cells = w0 = qm = ro = 0
                for t in tset:
                    k = max(p1.leaves(t)) + 1
                    for _ in range(3):
                        bs = [rng.choice(list(P.raws())) for _ in range(k)]
                        doms = [[v * P.step for v in range(min(0, b), max(0, b) + 1)]
                                for b in bs]
                        if any(len(d) == 0 for d in doms):
                            continue
                        cells += 1
                        b_w0 = b_qm = b_ro = False
                        for tup in product(*doms):
                            env = dict(enumerate(tup))
                            base = ev_every(P, t, env, rounding)
                            if ev_w0r(P, t, env, rounding) != base:
                                b_w0 = True
                            if ev_qmul(P, t, env, rounding) != base:
                                b_qm = True
                            e = ev_exact(t, env)
                            if not (P.lo <= e <= P.hi):
                                b_ro = True
                        w0 += b_w0
                        qm += b_qm
                        ro += b_ro
                print(f"  {P.label():<14} {rounding:<9} {cname:<14} {cells:>6} "
                      f"{ro:>9} {w0:>11} {qm:>16}", flush=True)
    print()
    print("  P8 holds when 'quantise-at-mul' is zero on the floor rows and nonzero")
    print("  on the trunc rows, with 'W0 differs' nonzero on both so the cells are")
    print("  live. C6 is the no-subtraction rows, where the two modes should agree")
    print("  on an unsigned base because nothing reaches a negative value.")

    # ------------------------------------------------------------------- P6
    print()
    print("-" * 106)
    print("P6. F116-9 reproduced: which grid restores the multiplicative")
    print("homomorphism, and which merely avoids requantisation.")
    print()
    P = Fx(4, 2, False, "wrap")
    span = P.khi - P.klo + 1
    ks = list(range(P.klo - span, P.khi + span + 1))
    print(f"  {'grid step':>10} {'hom failures':>14} {'products already on the fine grid':>36}")
    for gstep in (Fraction(1, 4), Fraction(1, 2), Fraction(1), Fraction(2)):
        dom = [k * P.step for k in ks if (k * P.step) % gstep == 0]
        bad = tot = 0
        exact_grid = True
        for a in dom:
            for b in dom:
                tot += 1
                if P.R(P.R(a) * P.R(b)) != P.R(a * b):
                    bad += 1
                if (a * b) % P.step != 0:
                    exact_grid = False
        print(f"  {str(gstep):>10} {bad:>6}/{tot:<7} {str(exact_grid):>36}")
    print()
    print("  `116` predicted the homomorphism would return once products needed no")
    print("  requantisation, and its own step-1/2 row refuted that. This reproduces")
    print("  the refutation: the step-1/2 row has every product on the fine grid and")
    print("  the homomorphism still fails. The unit grid is the condition.")

    print()
    print("=" * 106)
    print(
        """
  READING IT

  P3 holds when 'W0 differs' is zero on every no-mul row at every F and nonzero
  on the with-mul rows at F > 0. That is arm W0 splitting exactly as arm W1 does,
  which nobody had checked.

  P4 holds when 'cut@operands' is zero on the with-mul rows at F > 0, because
  that is the cut rule with the generalised cut set doing the job the ring
  version did at F = 0.

  P5 and C3 hold when 'cut@result' is nonzero there. If it is zero the cut could
  be placed anywhere near the multiply and the rule is weaker than stated.
"""
    )


if __name__ == "__main__":
    main()
