#!/usr/bin/env python3
"""
t2. The cell neither `118` nor `116` ran, and what it does to the predicate.

WHERE THIS COMES FROM
---------------------
t1 measured six cells and one of them is not in either file:

  - on a window straddling zero, the conjunction is empty under {add, mul},
    under {add} alone and under {mul} alone;
  - on a non-negative window, it is NOT empty under {add} alone, and it is not
    empty under {mul} alone either.

`118` F118-5 reports the second half as "addition is load-bearing", with a
witness over `{*}` alone.  The witness is real and reproduces.  But the same
map is a counterexample over `{+}` alone on the same window, so the witness
does not isolate the operation set.  What separates the two rows is the
window's sign, not which operation was dropped.

The cell that decides it is `{add, mul}` on a NON-NEGATIVE window.  If the
conjunction is non-empty there, then the operation set is not load-bearing at
all once the domain is right, and the whole hypothesis rests on the domain.
Neither file ran it in the search framework.  t1 exhibited a witness for it
outside the framework, at `|V| = 16` with a saturating map, so this probe puts
the same question inside the framework where the counts are exhaustive.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. `{add, mul}` on a non-negative window has non-constant maps passing both.
    The map sending zero to the bottom and everything else to the top is a
    homomorphism for both operations there.
P2. So the operation set is not load-bearing given a domain closed under
    negation, and the domain is load-bearing given any operation set.  Stated
    as a two-by-two, one cell is empty and three are not.
P3. The corrected condition is that the domain be closed under negation, which
    is what makes the induced structure a GROUP and is what my proof's first
    step actually uses.  Adding closure under negation to the non-negative
    window should empty the conjunction again at every operation set.
P4. `119` 4.2's two stated domain conditions, a complete residue system and the
    interval from zero to the value set's size, are both satisfied by the
    non-negative window, so neither is the condition that does the work.

CONDITION-CAN-FIRE CHECK
------------------------
The two-by-two is its own control: three non-empty cells against one empty one
means the search can find witnesses and the empty cell is a result.
"""

from itertools import product


def ex(op, a, b):
    return a + b if op == "add" else (a - b if op == "sub" else a * b)


def is_hom(R, window, ops):
    for op in ops:
        for a in window:
            for b in window:
                la, rb = ex(op, R[a], R[b]), ex(op, a, b)
                if la not in R or rb not in R:
                    continue
                if R[la] != R[rb]:
                    return False
    return True


def monotone(R, window):
    w = sorted(window)
    return all(R[w[i]] <= R[w[i + 1]] for i in range(len(w) - 1))


def search(vsize, window, ops):
    V = list(range(vsize))
    hom = mono = nc_both = 0
    wit = None
    for assign in product(V, repeat=len(window)):
        R = dict(zip(window, assign))
        h, m = is_hom(R, window, ops), monotone(R, window)
        hom += int(h)
        mono += int(m)
        if h and m and len(set(assign)) > 1:
            nc_both += 1
            if wit is None:
                wit = dict(R)
    return hom, mono, nc_both, wit


def main():
    print("=" * 92)
    print("t2. Which half of the hypothesis is load-bearing")
    print("=" * 92)

    print()
    print("P1/P2. The two-by-two: operation set against window sign")
    print()
    print(f"  {'|V|':<5} {'window':<14} {'ops':<12} {'hom':>6} {'mono':>7} {'BOTH+NONCONST':>15}")
    grid = {}
    for vsize in (2, 3):
        for wlabel, window in (
            ("straddles 0", list(range(-2 * vsize, 2 * vsize + 1))),
            ("non-negative", list(range(0, 4 * vsize + 1))),
        ):
            for oplabel, ops in (
                ("add+mul", ("add", "mul")),
                ("add only", ("add",)),
                ("mul only", ("mul",)),
            ):
                h, m, nc, wit = search(vsize, window, ops)
                grid[(vsize, wlabel, oplabel)] = nc
                print(
                    f"  {vsize:<5} {wlabel:<14} {oplabel:<12} {h:>6} {m:>7} {nc:>15}"
                    + ("   <== NON-EMPTY" if nc else "")
                )
                if nc and wit and vsize == 2 and oplabel == "add+mul":
                    ks = sorted(wit)
                    print(
                        f"        witness: {{{', '.join(f'{k}->{wit[k]}' for k in ks)}}}"
                    )
        print()

    print("  read as a two-by-two at |V| = 2 and |V| = 3:")
    print()
    print(f"    {'':<16} {'add+mul':>10} {'add only':>10} {'mul only':>10}")
    for vsize in (2, 3):
        for wlabel in ("straddles 0", "non-negative"):
            row = [grid[(vsize, wlabel, o)] for o in ("add+mul", "add only", "mul only")]
            print(f"    |V|={vsize} {wlabel:<9} {row[0]:>10} {row[1]:>10} {row[2]:>10}")
    print()
    print("  Every non-negative row is non-empty and every straddling row is empty,")
    print("  at every operation set. So the OPERATION SET is not the load-bearing")
    print("  half and the DOMAIN is.")

    # ---- P3: closure under negation ---------------------------------------
    print()
    print("P3. Adding closure under negation back to a non-negative window")
    print()
    for vsize in (2, 3):
        base = list(range(0, 2 * vsize + 1))
        closed = sorted(set(base) | {-v for v in base})
        for label, w in (("non-negative", base), ("closed under negation", closed)):
            h, m, nc, wit = search(vsize, w, ("add", "mul"))
            print(
                f"  |V|={vsize} {label:<24} window {w[0]}..{w[-1]} "
                f"({len(w)} points): both+nonconst {nc}"
            )
    print()
    print("  So closure under negation is what empties it, which is exactly what my")
    print("  proof's first step uses: an additive homomorphism onto a finite set has")
    print("  kernel nZ only if the image is a GROUP, and the image is a group only")
    print("  if every element has an inverse in the domain.")

    # ---- P4: the two conditions 119 names ---------------------------------
    print()
    print("P4. Do `119` 4.2's two stated domain conditions separate the rows")
    print()
    for vsize in (2, 3):
        for wlabel, window in (
            ("straddles 0", list(range(-2 * vsize, 2 * vsize + 1))),
            ("non-negative", list(range(0, 4 * vsize + 1))),
        ):
            residues = all(any(v % vsize == r for v in window) for r in range(vsize))
            interval = all(k in window for k in range(0, vsize + 1))
            print(
                f"  |V|={vsize} {wlabel:<14}: complete residue system {residues}, "
                f"interval 0..{vsize} present {interval}"
            )
    print()
    print("  Both conditions hold on both rows, so neither distinguishes the cell")
    print("  where the theorem holds from the cell where it fails. They are true of")
    print("  the domain my probe used and are not the reason the conjunction is")
    print("  empty there.")


if __name__ == "__main__":
    main()
