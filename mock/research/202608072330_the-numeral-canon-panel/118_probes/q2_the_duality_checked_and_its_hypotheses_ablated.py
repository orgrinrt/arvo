#!/usr/bin/env python3
"""q2. `116` F116-4 checked on a different search, and its hypotheses ablated.

THE CLAIM
---------
`116` F116-4: no realisation map onto a finite value set is both a ring
homomorphism and monotone, except a constant one. It is offered as structural,
with the argument at `116` section 5.2 and an exhaustive search over 512 and
1594323 maps presented as a control on the argument rather than as its evidence.
`116` says it is the only result in that file it would carry to a real width
without further work, because it quantifies over finiteness rather than over size.

That is a strong claim and it is the root of what three files now agree on, so it
gets checked rather than accepted. Two things to check and one to establish.

CHECK ONE: the argument. Re-derived here rather than quoted.

    Let V be finite, |V| >= 2, and R : Z -> V surjective with
    R(a op b) = R(R(a) op R(b)) for op in {+, *}. The relation R(a) = R(b) is
    then a congruence for both operations, so V carries an induced ring
    structure and R is a surjective ring homomorphism from Z. Every such
    quotient is Z/nZ with n = |V|. Now suppose some total order on V makes R
    non-decreasing. R(0) = R(n), since 0 and n are congruent. A non-decreasing
    map agreeing at the endpoints of [0, n] is constant on [0, n]. That interval
    contains a complete residue system mod n, and R factors through the
    residues, so R is constant on Z, contradicting |V| >= 2.

The derivation goes through. Two hypotheses in it are worth attacking, because
neither file says whether they are load-bearing.

CHECK TWO: a different search. `116`'s search enumerates every map from a window
of Z onto V and tests monotonicity against the natural order on V. Two things
differ here, both deliberately:

  - The maps searched are **realisation-shaped**: R fixes V pointwise, which is
    what a realisation map does and what `116`'s search does not require. That is
    a smaller and more design-relevant space, and a theorem that survives a
    smaller space is not thereby confirmed, so the wider ablations below are what
    carry the weight.
  - Monotonicity is tested against **every total order on V**, not only the
    natural one. `116`'s section 5.2 states the hypothesis as "some total order
    on V" and its probe tests one. This closes that gap.

ESTABLISH: which hypotheses are load-bearing.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. With `{+, *}`: no non-constant map passes both, at every size and window
    swept. `116` F116-4 reproduced on a different space.
P2. With `{+}` alone: still none. The proof only ever used addition to get the
    quotient and the periodicity, so multiplication is **not** load-bearing and
    the theorem is really about the additive group.
P3. With `{*}` alone: some non-constant map passes both, so addition **is**
    load-bearing. The witness I expect is `R(0) = 0`, `R(v) = 1` otherwise, over
    a non-negative window: zero is absorbing under multiplication so the identity
    holds, and the map is non-decreasing on that window.
P4. With a window narrower than `|V|`: some non-constant map passes both, because
    the periodicity the proof turns on is not visible. That is not a defect in
    the theorem; it says the hypothesis is a map defined on **enough** of Z, which
    a realisation map is, and it is worth pinning because a probe with a narrow
    window would report the theorem false.
P5. Quantifying monotonicity over every total order rather than the natural one
    finds strictly more monotone maps and still no non-constant map passing both.

NEGATIVE CONTROLS
-----------------
C1. Each half must have non-constant candidates on its own at every setting, or
    "none passes both" is a statement about an empty search.
C2. The constant maps must be found passing both, or the search cannot see the
    exception the theorem names and is not looking where the theorem points.
C3. P3 and P4 are themselves controls: if neither ablation produces a witness,
    the search cannot produce a witness at all and P1's zero means nothing.
"""

from itertools import permutations, product
import sys


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    return a * b


def monotone_some_order(vals, V):
    """Is there a total order on V making the sequence non-decreasing?

    Equivalent, and cheaper than enumerating orders for large V: the sequence is
    non-decreasing under some order exactly when each value's occurrences form
    one contiguous run. Both forms are computed and compared, so the equivalence
    is checked rather than assumed."""
    runs = []
    for v in vals:
        if not runs or runs[-1] != v:
            runs.append(v)
    contiguous = len(runs) == len(set(runs))

    by_order = False
    if len(V) <= 6:
        for order in permutations(V):
            rank = {v: i for i, v in enumerate(order)}
            if all(rank[vals[i]] <= rank[vals[i + 1]] for i in range(len(vals) - 1)):
                by_order = True
                break
        assert by_order == contiguous, (vals, by_order, contiguous)
    return contiguous


def monotone_natural(vals):
    return all(vals[i] <= vals[i + 1] for i in range(len(vals) - 1))


def search(vsize, window, ops, fix_pointwise=True, order_mode="some"):
    V = list(range(vsize))
    free = [w for w in window if not (fix_pointwise and w in V)]
    total = vsize ** len(free)
    pairs = [(a, b, op) for a in window for b in window for op in ops]
    hom = mono = both = both_nonconst = 0
    witnesses = []
    for assign in product(V, repeat=len(free)):
        R = {w: w for w in window if fix_pointwise and w in V}
        R.update(dict(zip(free, assign)))
        ok = True
        for a, b, op in pairs:
            lhs = ex(op, R[a], R[b])
            rhs = ex(op, a, b)
            if lhs not in R or rhs not in R:
                continue
            if R[lhs] != R[rhs]:
                ok = False
                break
        vals = [R[w] for w in window]
        mo = monotone_some_order(vals, V) if order_mode == "some" else monotone_natural(vals)
        const = len(set(vals)) == 1
        hom += ok
        mono += mo
        if ok and mo:
            both += 1
            if not const:
                both_nonconst += 1
                if len(witnesses) < 3:
                    witnesses.append(dict(R))
    return total, hom, mono, both, both_nonconst, witnesses


def row(label, vsize, window, ops, **kw):
    total, hom, mono, both, bn, wit = search(vsize, window, ops, **kw)
    print(
        f"  {label:<40} |V|={vsize} win={len(window):>3} maps {total:>8}  "
        f"hom {hom:>6}  monotone {mono:>7}  both {both:>5}  "
        f"both AND non-constant {bn:>4}",
        flush=True,
    )
    if wit:
        for w in wit[:2]:
            seq = ", ".join(f"{k}->{w[k]}" for k in sorted(w))
            print(f"      witness: {seq}")
    return bn, hom, mono


def main():
    print("=" * 118)
    print("q2. The duality checked on a different search, and its hypotheses ablated")
    print("=" * 118)

    print()
    print("P1 and P5. Realisation-shaped maps (R fixes V pointwise), monotone")
    print("under SOME total order on V rather than only the natural one.")
    print()
    for vsize, span in ((2, 3), (2, 4), (2, 5), (3, 2)):
        window = list(range(-span * vsize, span * vsize + 1))
        row("{+, *}", vsize, window, ("add", "mul"))

    print()
    print("C2 cannot fire in the space above: a map fixing V pointwise cannot be")
    print("constant when |V| >= 2, so the exception the theorem names is excluded")
    print("by construction. These rows drop that restriction, which is `116`'s own")
    print("space, so the constants appear and its figures can be compared directly.")
    print()
    for vsize, span in ((2, 2), (3, 2)):
        window = list(range(-span * vsize, span * vsize + 1))
        row("{+, *}, free maps, SOME order", vsize, window, ("add", "mul"),
            fix_pointwise=False)
    for vsize, span in ((2, 2), (3, 2)):
        window = list(range(-span * vsize, span * vsize + 1))
        row("{+, *}, free maps, natural order", vsize, window, ("add", "mul"),
            fix_pointwise=False, order_mode="natural")

    print()
    print("The same rows with monotonicity under the NATURAL order only, which is")
    print("`116`'s test. P5 holds if the SOME-order rows find at least as many")
    print("monotone maps and still no non-constant map passing both.")
    print()
    for vsize, span in ((2, 3), (3, 2)):
        window = list(range(-span * vsize, span * vsize + 1))
        row("{+, *}, natural order", vsize, window, ("add", "mul"),
            order_mode="natural")

    print()
    print("-" * 118)
    print("P2. Is multiplication load-bearing? Drop it.")
    print()
    for vsize, span in ((2, 3), (2, 5), (3, 2)):
        window = list(range(-span * vsize, span * vsize + 1))
        row("{+} alone", vsize, window, ("add",))

    print()
    print("-" * 118)
    print("P3. Is addition load-bearing? Drop it instead. This one is a control:")
    print("if nothing passes both here either, the search cannot produce a")
    print("witness at all and every zero above is a dead branch.")
    print()
    for vsize, span in ((2, 3), (3, 2)):
        window = list(range(-span * vsize, span * vsize + 1))
        row("{*} alone, window straddles zero", vsize, window, ("mul",))
    for vsize, hi in ((2, 8), (3, 10)):
        window = list(range(0, hi + 1))
        row("{*} alone, window non-negative", vsize, window, ("mul",))

    print()
    print("-" * 118)
    print("P4. Is the window's width load-bearing? Narrow it below |V|.")
    print()
    for vsize in (3, 4):
        window = list(range(0, vsize))
        row("{+, *}, window = V exactly", vsize, window, ("add", "mul"))
    for vsize in (3, 4):
        window = list(range(0, vsize + 1))
        row("{+, *}, window = V plus one", vsize, window, ("add", "mul"))

    print()
    print("=" * 118)
    print(
        """
  READING IT

  P1 holds when 'both AND non-constant' is zero on every {+, *} row with a
  window wide enough to contain [0, |V|], and 'both' is nonzero, which is C2:
  the constants are found, so the search is looking where the theorem points.

  C1 is the 'hom' and 'monotone' columns: each must be nonzero on its own.

  P2 holding means the theorem is really about the additive group and
  multiplication is decoration in the hypothesis. That is a widening of
  `116` F116-4 rather than a correction to it.

  P3 is the control that makes every zero meaningful. A witness there is a map
  that is a multiplicative homomorphism and monotone and not constant, which
  the theorem permits precisely because it assumes addition.

  P4 pins the scope. A window narrower than |V| cannot see the periodicity, so a
  probe built that way would report the theorem false. A realisation map is
  defined on all of Z, so the hypothesis holds in the design, but a later probe
  measuring on a narrow window would be measuring nothing.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
