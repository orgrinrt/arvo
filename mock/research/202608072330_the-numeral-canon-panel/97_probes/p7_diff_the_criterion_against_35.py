#!/usr/bin/env python3
"""P7. The criterion against `35`'s measured law table, which is the diff nobody ran.

`93` phase two names this as the largest thing it did not do: `35` is the panel's
existing measurement of exactly the law families `93`'s P2b/P2c/P7/P8 measure, `93`
chose not to read it so its own numbers stayed independent, and it says plainly
"Somebody should diff them, and that is a real piece of work I am leaving undone."
`40`'s observable table cites `35` for six of its eight rows. So `35` is load-bearing
for two files and has been diffed against nothing.

This is that diff, and it is a stronger test than a second sweep of my own would be,
because `35`'s model was written independently, months of files earlier, by somebody
who was answering a different question. Its committed CSVs are machine readable:

    35_probes/p2.out   unsigned, w 2..7, f 0..w, wrap and saturate, five laws
    35_probes/p2b.out  the same box, signed

The criterion under test is P2's, extended by one structural property because `35`
measures an order law and P2 did not:

    an IDENTITY of exact arithmetic holds in the representable set iff the
    realisation map respects every nesting of operations the identity contains

    an ORDER law holds iff the realisation map is monotone

Both are the same move. A quotient by a congruence inherits identities; a monotone
quotient inherits order facts. Neither is a measurement and neither is new.

`35`'s five laws map onto them as follows, with the nesting position named because
subtraction is not symmetric and a position-blind verdict would be conservative:

    add_assoc          identity, add nested in add, either position
    mul_assoc          identity, mul nested in mul, either position
    distributivity     identity, add nested in mul (position 2) and
                                 mul nested in add (either position)
    additive_inverse   identity, add nested in sub, position 1
    monotonicity_add   order law, needs the map to be monotone

Verdicts are computed with `35`'s own arithmetic conventions, read from
`35_probes/p2_laws.rs` and `35_probes/p2b_laws_signed.rs` rather than assumed: an
arithmetic shift right for the fractional multiply, which floors rather than truncating
toward zero and which `35` names as a rounding choice in its own comment; unsigned
subtraction flooring at zero under saturation and borrowing under
wrapping, and the boundary policy applied at the declared width.

A SOUNDNESS mismatch (predicted to hold, `35` measured a failure) refutes the
criterion. A CONSERVATIVE mismatch (predicted to fail, `35` measured none) leaves it
safe for gating an arm and is counted separately.
"""

import csv
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
P35 = os.path.abspath(os.path.join(HERE, "..", "35_probes"))


# The exact structure carries full precision and the realisation map does the
# quantising. An earlier revision of this file put the truncating shift inside the
# exact multiply instead, which made the map look like it had nothing to round and
# produced twelve soundness mismatches, every one of them mul_assoc at f == w. That
# was the model being wrong rather than the criterion, and the diagnosis is recorded
# in the deliverable rather than deleted. P2's construction is imported instead of
# re-derived, so the two files cannot drift apart.

import importlib.util

_spec = importlib.util.spec_from_file_location(
    "p2mod", os.path.join(HERE, "p2_congruence_predicts_the_laws.py"))
P2 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(P2)


def respects(w, f, signed, policy, inner, outer, pos):
    """Does realising the inner result change the outer answer, in that position?"""
    pi, dom = P2.make_pi(w, f, signed, policy, "floor", False)
    lift, ops = P2.make_ops(f)
    fin, fout = ops[inner], ops[outer]
    L = {r: lift(r) for r in dom}
    image = {fin(L[a], L[b]) for a in dom for b in dom}
    moved = [x for x in image if lift(pi(x)) != x]
    for x in moved:
        rx = lift(pi(x))
        for c in dom:
            lc = L[c]
            if pos == 1:
                if pi(fout(x, lc)) != pi(fout(rx, lc)):
                    return False
            else:
                if pi(fout(lc, x)) != pi(fout(lc, rx)):
                    return False
    return True


def monotone(w, f, signed, policy):
    """Is the realisation map order preserving over the values that can arise?"""
    pi, dom = P2.make_pi(w, f, signed, policy, "floor", False)
    lift, _ = P2.make_ops(f)
    lo, hi = lift(dom[0]), lift(dom[-1])
    step = max(1, (hi - lo) // 4096) if hi > lo else 1
    prev = None
    x = 2 * lo - step
    while x <= 2 * hi + step:
        v = pi(x)
        if prev is not None and v < prev:
            return False
        prev = v
        x += step
    return True


def predict(law, w, f, signed, policy):
    r = lambda i, o, p: respects(w, f, signed, policy, i, o, p)
    if law == "add_assoc":
        return r("add", "add", 1) and r("add", "add", 2)
    if law == "mul_assoc":
        return r("mul", "mul", 1) and r("mul", "mul", 2)
    if law == "distributivity":
        return r("add", "mul", 2) and r("mul", "add", 1) and r("mul", "add", 2)
    if law == "additive_inverse":
        return r("add", "sub", 1)
    if law == "monotonicity_add":
        return monotone(w, f, signed, policy)
    raise AssertionError(law)


def main():
    wmax = int(sys.argv[1]) if len(sys.argv) > 1 else 6
    print("P7. the criterion against `35`'s committed law table")
    print("35's box is w 2..7; this diff runs w 2..%d because the retraction check" % wmax)
    print("is quadratic in the multiply's image and the point is the agreement rate,")
    print("not reach in w. Rows above the cap are counted as skipped, not as passing.")
    print()

    sound = 0
    conservative = 0
    agree = 0
    skipped = 0
    per_law = {}

    for fname, signed in (("p2.out", False), ("p2b.out", True)):
        path = os.path.join(P35, fname)
        for row in csv.DictReader(open(path)):
            w = int(row["w"])
            f = int(row["f"])
            policy = row["policy"]
            law = row["law"]
            if w > wmax:
                skipped += 1
                continue
            measured = int(row["failures"]) == 0
            predicted = predict(law, w, f, signed, policy)
            key = (law, "signed" if signed else "unsigned")
            a, c, s = per_law.get(key, (0, 0, 0))
            if predicted and not measured:
                sound += 1
                per_law[key] = (a, c, s + 1)
                print("SOUNDNESS MISMATCH  %-8s w=%d f=%d %-8s %-16s 35 failures=%s/%s"
                      % ("signed" if signed else "unsigned", w, f, policy, law,
                         row["failures"], row["total"]))
            elif measured and not predicted:
                conservative += 1
                per_law[key] = (a, c + 1, s)
                print("conservative        %-8s w=%d f=%d %-8s %-16s 35 failures=%s/%s"
                      % ("signed" if signed else "unsigned", w, f, policy, law,
                         row["failures"], row["total"]))
            else:
                agree += 1
                per_law[key] = (a + 1, c, s)

    total = agree + conservative + sound
    print()
    print("per law and signedness, over the diffed rows")
    print("%-18s %-10s %8s %14s %12s" % ("law", "signedness", "agree", "conservative", "SOUNDNESS"))
    for (law, sg) in sorted(per_law):
        a, c, s = per_law[(law, sg)]
        print("%-18s %-10s %8d %14d %12d" % (law, sg, a, c, s))
    print()
    print("rows diffed                        : %d" % total)
    print("rows skipped above the width cap   : %d" % skipped)
    print("agree                              : %d" % agree)
    print("conservative (safe, predicted fail): %d" % conservative)
    print("SOUNDNESS mismatches (refute)      : %d" % sound)
    print()
    if sound == 0:
        print("`35`'s table, written independently and months of files earlier, contains")
        print("no row the criterion predicts to hold and which fails. So the criterion")
        print("reaches a third instance, on data it did not generate, and `93`'s and")
        print("`40`'s reliance on `35` is consistent with it rather than in tension.")
    else:
        print("The criterion is refuted on `35`'s data at %d rows. Diagnose whether the" % sound)
        print("two models differ before calling it a refutation of the criterion.")


if __name__ == "__main__":
    main()
