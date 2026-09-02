#!/usr/bin/env python3
"""
t1. Checking `118`'s ablation of my F116-4, and checking the predicate `119`
    section 4.2 gives that theorem.

WHY
---
F116-4 is the one claim in this topic that quantifies over finiteness rather
than size, which is why it needs no width transfer argument.  `119` carries it
as clause 4.2 with a predicate its author wrote, and `118` F118-4 reports that
multiplication is not load-bearing in its hypothesis, which would make the
theorem wider than I stated it.  Both need checking before I sign, and a
signature on someone else's ablation of my own claim is not a signature.

WHAT IS CHECKED
---------------
A. The three ablations `118` reports, on my own search rather than on theirs.
B. The gap `118` names in my own probe: my statement quantifies monotonicity
   over SOME total order and my probe tested only the natural one.
C. Whether the predicate `119` 4.2 states is the predicate the theorem needs.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. Dropping multiplication leaves zero non-constant maps passing both.  My
    proof does not use multiplication either: an additive homomorphism onto a
    finite set has kernel `nZ`, which is all step two needs.  So F118-4 is
    right and my statement was wider than its own hypothesis.
P2. Over multiplication alone on a NON-NEGATIVE window a non-constant monotone
    homomorphism exists, so addition is load-bearing.  The witness I expect is
    the map sending zero to the bottom and everything else to the top.
P3. On a window no wider than the value set the identity passes both, so the
    domain's width is load-bearing.
P4. Quantifying monotonicity over every total order finds strictly more
    monotone maps and still zero non-constant passing both, so my probe was
    testing a weaker theorem than my statement claimed while my proof covered
    the stronger one.

P5. **The predicate `119` 4.2 states admits a counterexample.**  It reads
    "domain containing a complete residue system and the interval from zero to
    the value set's size", and a NON-NEGATIVE window satisfies that.  On such a
    window a saturating map is an additive homomorphism in the reduce-early-
    equals-reduce-late sense, is order-preserving, and is not constant.  What
    my proof actually needs is that the induced structure be a GROUP, which a
    non-negative window does not give, because saturation has no inverses
    there.  My own statement says `R : Z -> V`, which is safe; my own
    PREDICATE says "9 and 13 consecutive integers" without saying they
    straddle zero, which is the same gap one file earlier.

P6. The same map refutes `119` 4.4's clause that a saturating map is "a
    homomorphism for no operation".  My own `116_probes/p4` table already
    shows saturating unsigned at 0 failures for add, 885 for sub and 0 for mul
    at `F = 0`, which nobody read as a finding at the time, mine included.

CONDITION-CAN-FIRE CHECK
------------------------
Every ablation reports the count of maps passing each half separately before
the conjunction, so a zero on the conjunction is a result rather than an empty
search.  P5 and P6 are existence claims and are shown by exhibiting the map.
"""

from itertools import product, permutations


def ex(op, a, b):
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    if op == "mul":
        return a * b
    raise ValueError(op)


def is_hom(R, window, ops):
    """R(a op b) == R(R(a) op R(b)) wherever both arguments stay in the window."""
    for op in ops:
        for a in window:
            for b in window:
                lhs_arg = ex(op, R[a], R[b])
                rhs_arg = ex(op, a, b)
                if lhs_arg not in R or rhs_arg not in R:
                    continue
                if R[lhs_arg] != R[rhs_arg]:
                    return False
    return True


def monotone_natural(R, window):
    w = sorted(window)
    return all(R[w[i]] <= R[w[i + 1]] for i in range(len(w) - 1))


def monotone_some_order(R, window, V):
    """Is R non-decreasing under SOME total order on V?"""
    w = sorted(window)
    for perm in permutations(V):
        rank = {v: i for i, v in enumerate(perm)}
        if all(rank[R[w[i]]] <= rank[R[w[i + 1]]] for i in range(len(w) - 1)):
            return True
    return False


def search(vsize, window, ops, order="natural"):
    V = list(range(vsize))
    hom = mono = both = nonconst_both = total = 0
    witness = None
    for assign in product(V, repeat=len(window)):
        total += 1
        R = dict(zip(window, assign))
        h = is_hom(R, window, ops)
        m = (
            monotone_natural(R, window)
            if order == "natural"
            else monotone_some_order(R, window, V)
        )
        nc = len(set(assign)) > 1
        hom += int(h)
        mono += int(m)
        if h and m:
            both += 1
            if nc:
                nonconst_both += 1
                if witness is None:
                    witness = dict(R)
    return dict(
        total=total, hom=hom, mono=mono, both=both, nc_both=nonconst_both, witness=witness
    )


def show(res, label):
    print(
        f"  {label:<54} maps {res['total']:>8}  hom {res['hom']:>5}  "
        f"mono {res['mono']:>6}  both {res['both']:>4}  "
        f"BOTH+NONCONST {res['nc_both']:>4}"
    )
    if res["witness"]:
        w = res["witness"]
        keys = sorted(w)
        print(f"      witness: {{{', '.join(f'{k}->{w[k]}' for k in keys)}}}")


def main():
    print("=" * 100)
    print("t1. The ablation of F116-4, and the predicate 119 gives it")
    print("=" * 100)

    # ---- A: the three ablations, on a symmetric window ---------------------
    print()
    print("A. The ablations, on a window straddling zero (which is what my probe used)")
    print()
    for vsize in (2, 3):
        w = list(range(-2 * vsize, 2 * vsize + 1))
        show(search(vsize, w, ("add", "mul")), f"|V|={vsize}, window {w[0]}..{w[-1]}, ops add+mul")
        show(search(vsize, w, ("add",)), f"|V|={vsize}, window {w[0]}..{w[-1]}, ops ADD ONLY")
        show(search(vsize, w, ("mul",)), f"|V|={vsize}, window {w[0]}..{w[-1]}, ops MUL ONLY")
        print()

    print("  P1: dropping multiplication leaves the conjunction empty, so it is not")
    print("      load-bearing. P2: multiplication alone does not, on a window with")
    print("      negatives; the non-negative case is below.")

    # ---- P2 proper: a non-negative window ---------------------------------
    print()
    print("B. Multiplication alone on a NON-NEGATIVE window, where addition's absence shows")
    print()
    for vsize in (2, 3):
        w = list(range(0, 4 * vsize + 1))
        show(search(vsize, w, ("mul",)), f"|V|={vsize}, window 0..{w[-1]}, ops MUL ONLY")
        show(search(vsize, w, ("add",)), f"|V|={vsize}, window 0..{w[-1]}, ops ADD ONLY")
        print()

    # ---- P3: the narrow window --------------------------------------------
    print()
    print("C. A window no wider than the value set")
    print()
    for vsize in (3, 4):
        for width in (vsize, vsize + 1):
            w = list(range(0, width))
            show(search(vsize, w, ("add", "mul")), f"|V|={vsize}, window 0..{w[-1]} (width {width})")
    print()

    # ---- P4: monotonicity over every total order --------------------------
    print()
    print("D. Monotonicity under the natural order against under SOME total order")
    print()
    for vsize in (2, 3):
        w = list(range(-2 * vsize, 2 * vsize + 1))
        a = search(vsize, w, ("add", "mul"), order="natural")
        b = search(vsize, w, ("add", "mul"), order="some")
        print(
            f"  |V|={vsize}: natural-order monotone {a['mono']}, "
            f"some-order monotone {b['mono']}, "
            f"both+nonconst {a['nc_both']} and {b['nc_both']}"
        )
    print()
    print("  My statement quantifies over SOME total order and my probe tested the")
    print("  natural one only, so my probe verified a weaker theorem than I stated.")
    print("  My proof covers the stronger one: non-decreasing under any order with")
    print("  R(0) = R(n) forces constancy on [0, n] by antisymmetry alone.")

    # ---- P5: the counterexample to 119's predicate ------------------------
    print()
    print("E. THE PREDICATE `119` 4.2 STATES, tested against a saturating map")
    print()
    n = 16
    hi = n - 1
    for span in (3, 4):
        window = list(range(0, span * n))
        R = {v: min(v, hi) for v in window}
        h_add = is_hom(R, window, ("add",))
        h_mul = is_hom(R, window, ("mul",))
        h_sub = is_hom(R, window, ("sub",))
        m = monotone_natural(R, window)
        nc = len(set(R.values())) > 1
        has_residues = all(any(v % n == r for v in window) for r in range(n))
        has_interval = all(k in R for k in range(0, n + 1))
        print(
            f"  saturating R(v)=min(v,{hi}) on 0..{window[-1]}: "
            f"hom(add) {h_add}, hom(mul) {h_mul}, hom(sub) {h_sub}, "
            f"monotone {m}, non-constant {nc}"
        )
        print(
            f"    the two conditions `119` 4.2 names: complete residue system "
            f"{has_residues}, interval 0..{n} present {has_interval}"
        )
        print(
            f"    so it satisfies the stated predicate and is a counterexample: "
            f"{has_residues and has_interval and h_add and m and nc}"
        )
    print()
    print("  On a window that straddles zero the same map is NOT an additive hom:")
    window = list(range(-2 * n, 2 * n + 1))
    R = {v: min(max(v, 0), hi) for v in window}
    print(
        f"    R(v)=clamp(v,0,{hi}) on {window[0]}..{window[-1]}: "
        f"hom(add) {is_hom(R, window, ('add',))}, monotone {monotone_natural(R, window)}"
    )
    a, b = -1, 1
    print(
        f"    witness: a={a}, b={b}: R(a+b)={R[a + b]}, R(R(a)+R(b))={R[R[a] + R[b]]}"
    )
    print()
    print("  So the missing dimension is that the domain must contain additive")
    print("  inverses, equivalently that the induced structure is a GROUP. My own")
    print("  statement says `R : Z -> V`, which is safe. My own PREDICATE said")
    print("  'consecutive integers' without saying they straddle zero, and `119`")
    print("  inherited that gap and made it falsifiable by naming two weaker")
    print("  conditions in its place.")

    # ---- P6: what that does to 119 clause 4.4 -----------------------------
    print()
    print("F. `119` 4.4's clause that a saturating map is a homomorphism for NO operation")
    print()
    for W, signed in ((4, False), (4, True)):
        lo, hi2 = (0, 2**W - 1) if not signed else (-(2 ** (W - 1)), 2 ** (W - 1) - 1)
        window = list(range(lo * 3 if signed else 0, hi2 * 3 + 1))
        R = {v: min(max(v, lo), hi2) for v in window}
        row = []
        for op in ("add", "sub", "mul"):
            bad = 0
            tot = 0
            for a in window:
                for b in window:
                    la, rb = ex(op, R[a], R[b]), ex(op, a, b)
                    if la not in R or rb not in R:
                        continue
                    tot += 1
                    if R[la] != R[rb]:
                        bad += 1
            row.append(f"{op} {bad}/{tot}")
        print(
            f"  saturating {'signed' if signed else 'unsigned'} W={W}, "
            f"window {window[0]}..{window[-1]}: " + ", ".join(row)
        )
    print()
    print("  The unsigned row is the point: saturation IS a homomorphism for")
    print("  addition and for multiplication on a non-negative domain, and is not")
    print("  one for subtraction. `116_probes/p4` printed exactly this at")
    print("  0/2116, 885/2116 and 0/2116 and nobody read it as a finding, me")
    print("  included.")


if __name__ == "__main__":
    main()
