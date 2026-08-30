#!/usr/bin/env python3
"""u2. `121` section 2 reproduced, including the confound in my own `118` q2.

THE DISSENT
-----------
`119` 4.2 states the mutual-exclusion theorem with the predicate

    value set finite with at least two elements; operations including addition;
    domain containing a complete residue system and the interval from zero to
    the value set's size.

`121` says that predicate admits a counterexample: a saturating map onto sixteen
values on the domain `0..47` is an additive homomorphism, is order-preserving, is
non-constant, and satisfies both stated domain conditions. And it says the
condition that actually does the work is **closure under negation**, which no
predicate in this sitting names.

**This is a dissent against a predicate I wrote, resting on an ablation I ran.**
So it gets reproduced rather than accepted, and the reproduction has to include
the thing `121` accuses my own instrument of: that `118` q2's ablation rows
confounded the operation set with the window's sign.

WHAT MY OWN q2 ACTUALLY SWEPT, WHICH IS THE ACCUSATION
--------------------------------------------------------
Reading `118_probes/q2` at its `main`:

  the `{+, *}` rows use windows `range(-span * vsize, span * vsize + 1)`, which
  straddle zero;
  the `{+}`-alone rows use the same straddling windows;
  the `{*}`-alone rows use straddling windows AND, separately, non-negative
  windows `range(0, hi + 1)`.

The witnesses my F118-5 rests on came from the **non-negative** `{*}` rows. The
zeros my F118-4 rests on came from **straddling** `{+}` rows. So the two
conclusions were drawn from cells that differ in two dimensions at once, and the
cell that separates them, `{+}` alone on a **non-negative** window, was never run.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. `121`'s counterexample reproduces: the saturating map is an additive
    homomorphism, order-preserving, non-constant, and satisfies both conditions
    `119` 4.2 states.
P2. The missing cell has a witness: `{+}` alone on a non-negative window admits a
    non-constant map passing both. If it does, my F118-5 attributes to the
    operation what belongs to the domain, and I concede it.
P3. The two-by-two separates on the domain and not on the operation set: every
    straddling row empty at every operation set, every non-negative row
    non-empty at every operation set.
P4. Restoring closure under negation to a non-negative window empties it again,
    which is the direct test that the condition is the one doing the work.
P5. My F118-4 survives as stated, because dropping multiplication on a
    straddling window still gives zero. What does not survive is the framing that
    the operation set is what the theorem turns on.

NEGATIVE CONTROLS
-----------------
C1. Each half of the conjunction must be non-empty on its own on every row, or a
    zero in the "both" column is a statement about an empty search.
C2. The two conditions `119` 4.2 names must hold on BOTH rows of the two-by-two.
    If one of them separated the rows, it would be doing work and `121` would be
    wrong about which condition matters.
C3. `{mul}` alone on a domain closed under negation is measured rather than
    argued. My theorem's proof covers the additive case only, so a zero there is
    an observation with a predicate and not a consequence of the proof, and this
    probe reports it as such.
"""

from itertools import permutations, product
import sys


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    return a * b


_ASSERT = [0]


def monotone_some_order(vals, V):
    runs = []
    for v in vals:
        if not runs or runs[-1] != v:
            runs.append(v)
    contiguous = len(runs) == len(set(runs))
    if len(V) <= 5 and _ASSERT[0] < 200:
        _ASSERT[0] += 1
        by_order = any(
            all(
                {v: i for i, v in enumerate(o)}[vals[k]]
                <= {v: i for i, v in enumerate(o)}[vals[k + 1]]
                for k in range(len(vals) - 1)
            )
            for o in permutations(V)
        )
        assert by_order == contiguous, (vals, by_order, contiguous)
    return contiguous


def search(vsize, window, ops):
    """Every map from `window` onto V, with the homomorphism identity checked on
    pairs whose operands and results both land inside the window."""
    V = list(range(vsize))
    hom = mono = both = both_nonconst = 0
    witnesses = []
    pairs = [(a, b, op) for a in window for b in window for op in ops]
    for assign in product(V, repeat=len(window)):
        R = dict(zip(window, assign))
        ok = True
        for a, b, op in pairs:
            lhs, rhs = ex(op, R[a], R[b]), ex(op, a, b)
            if lhs not in R or rhs not in R:
                continue
            if R[lhs] != R[rhs]:
                ok = False
                break
        vals = [R[w] for w in window]
        mo = monotone_some_order(vals, V)
        const = len(set(vals)) == 1
        hom += ok
        mono += mo
        if ok and mo:
            both += 1
            if not const:
                both_nonconst += 1
                if len(witnesses) < 2:
                    witnesses.append(dict(R))
    return dict(total=vsize ** len(window), hom=hom, mono=mono, both=both,
                nonconst=both_nonconst, witnesses=witnesses)


# ------------------------------------------------------ `119` 4.2's conditions


def has_complete_residue_system(window, n):
    return len({w % n for w in window}) == n


def contains_zero_to_n(window, n):
    return all(k in window for k in range(0, n + 1))


def closed_under_negation(window):
    return all(-w in window for w in window)


def main():
    print("=" * 100)
    print("u2. The domain was confounded with the operation set, in my own ablation")
    print("=" * 100)

    # ------------------------------------------------------------------- P1
    print()
    print("P1. `121`'s counterexample to `119` 4.2's predicate, recomputed.")
    print()
    n = 16
    dom = list(range(0, 48))

    def R_sat(v):
        return 0 if v < 0 else (n - 1 if v > n - 1 else v)

    for ops, label in ((("add",), "add"), (("mul",), "mul"), (("sub",), "sub")):
        bad = tot = 0
        for a in dom:
            for b in dom:
                tot += 1
                if R_sat(ex(ops[0], R_sat(a), R_sat(b))) != R_sat(ex(ops[0], a, b)):
                    bad += 1
        print(f"    R(v) = clamp(v, 0, 15) on 0..47, homomorphism for {label:<3}: "
              f"{'YES' if bad == 0 else 'no'}  ({bad} failures of {tot})")
    vals = [R_sat(v) for v in dom]
    print(f"    order-preserving : {all(vals[i] <= vals[i+1] for i in range(len(vals)-1))}")
    print(f"    non-constant     : {len(set(vals)) > 1}")
    print()
    print(f"    `119` 4.2's condition 1, a complete residue system : "
          f"{has_complete_residue_system(dom, n)}")
    print(f"    `119` 4.2's condition 2, the interval 0..{n} present : "
          f"{contains_zero_to_n(dom, n)}")
    print(f"    closed under negation                              : "
          f"{closed_under_negation(dom)}")
    print()
    print("    So it satisfies both stated conditions, fails the one `121` names,")
    print("    and is a counterexample to the predicate as `119` states it:",
          all([has_complete_residue_system(dom, n), contains_zero_to_n(dom, n)])
          and not closed_under_negation(dom))

    # ----------------------------------------------------------- the same on Z
    print()
    print("    The same map on a domain that straddles zero, which is the control:")
    dom2 = list(range(-24, 25))
    bad = [(a, b) for a in dom2 for b in dom2
           if R_sat(ex("add", R_sat(a), R_sat(b))) != R_sat(ex("add", a, b))]
    print(f"      homomorphism for add on -24..24: {'YES' if not bad else 'no'}"
          f"  ({len(bad)} failures)")
    if bad:
        a, b = bad[0]
        print(f"      witness: a={a}, b={b}: R(a+b)={R_sat(a + b)}, "
              f"R(R(a)+R(b))={R_sat(R_sat(a) + R_sat(b))}")

    # --------------------------------------------------------- P2, P3, C1, C2
    print()
    print("-" * 100)
    print("P2 and P3. The two-by-two my own q2 never completed.")
    print()
    print("  My q2's rows, read from its `main`:")
    print("    {+, *}      straddling only")
    print("    {+} alone   straddling only          <- the missing cell is this one,")
    print("    {*} alone   straddling AND non-negative   non-negative")
    print()
    print(f"  {'|V|':>4} {'window':<26} {'ops':<10} {'maps':>8} {'hom':>6} "
          f"{'mono':>6} {'both':>6} {'NON-CONSTANT BOTH':>18}")
    grid = {}
    for vsize in (2, 3):
        for wlabel, window in (
            ("straddling zero", list(range(-vsize, vsize + 1))),
            ("non-negative, same size", list(range(0, 2 * vsize + 1))),
        ):
            for ops, olabel in ((("add", "mul"), "{add, mul}"),
                                (("add",), "{add}"),
                                (("mul",), "{mul}")):
                r = search(vsize, window, ops)
                grid[(vsize, wlabel, olabel)] = r["nonconst"]
                print(f"  {vsize:>4} {wlabel:<26} {olabel:<10} {r['total']:>8} "
                      f"{r['hom']:>6} {r['mono']:>6} {r['both']:>6} {r['nonconst']:>18}")
                if r["witnesses"] and r["nonconst"]:
                    w = r["witnesses"][0]
                    seq = ", ".join(f"{k}->{w[k]}" for k in sorted(w))
                    print(f"       witness: {seq}")

    print()
    print("  P3 holds when every straddling row is 0 and every non-negative row is")
    print("  nonzero, at every operation set. C1 is the 'hom' and 'mono' columns,")
    print("  each nonzero on its own everywhere.")

    # ------------------------------------------------------------------- C2
    print()
    print("C2. Do `119` 4.2's two conditions separate the rows? They must not, or")
    print("    `121` is wrong about which condition is load-bearing.")
    print()
    for vsize in (2, 3):
        for wlabel, window in (
            ("straddling zero", list(range(-vsize, vsize + 1))),
            ("non-negative, same size", list(range(0, 2 * vsize + 1))),
        ):
            print(f"    |V|={vsize} {wlabel:<18} residue system "
                  f"{str(has_complete_residue_system(window, vsize)):<6} "
                  f"interval 0..{vsize} present "
                  f"{str(contains_zero_to_n(window, vsize)):<6} "
                  f"closed under negation "
                  f"{closed_under_negation(window)}")

    # ------------------------------------------------------------------- P4
    print()
    print("-" * 100)
    print("P4. Restoring closure under negation to a non-negative window.")
    print()
    for vsize in (2, 3):
        base = list(range(0, vsize + 1))
        closed = sorted(set(base) | {-w for w in base})
        for ops, olabel in ((("add", "mul"), "{add, mul}"), (("add",), "{add}"),
                            (("mul",), "{mul}")):
            r = search(vsize, closed, ops)
            print(f"    |V|={vsize} non-negative window closed under negation "
                  f"({len(closed)} points) {olabel:<10} "
                  f"non-constant both {r['nonconst']}")

    print()
    print("=" * 100)
    print(
        """
  READING IT

  P1 decides the dissent on its own: a map satisfying every condition `119` 4.2
  states, and failing the theorem, means the predicate is wrong and is replaced
  rather than widened.

  P2 and P3 decide what replaces it. If the missing cell has a witness, my
  F118-5's attribution is wrong and the separating dimension is the domain.

  C3 is the `{mul}` rows on the closed windows. A zero there is a MEASUREMENT
  with the predicate this probe carries, not a consequence of the theorem's
  proof, which covers the additive case only. That distinction is kept in the
  finding rather than collapsed into `121`'s wider phrasing.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
