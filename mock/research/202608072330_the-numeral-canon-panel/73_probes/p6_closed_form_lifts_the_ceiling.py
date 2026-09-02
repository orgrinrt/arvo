#!/usr/bin/env python3
"""p6. Is the membership test's model-width ceiling forced, or only usual?

This attacks an asymmetry this file's section 8 identified rather than leaving it
reported.

Section 8 says the hosting test is decidable at every width, because "is the
ingest predicate writable at arity (type parameters, bits)" is a typing question,
while the membership test's law verdicts are exhaustive checks and `68:196-211`
re-established that rustc refuses the 9-bit exhaustive const check under
`deny(long_running_const_eval)`.

HYPOTHESIS, written before the run.

    That asymmetry is not uniform across verdicts. `63` C6 (`63:659-664`) says
    the congruence half "is decided by the range's geometry per operation,
    mirror symmetry for multiplication and sign confinement for addition". A
    geometric condition on the range is O(1) in the width. So wherever C6 supplies
    a closed form, the verdict needs no sweep and the membership test is decidable
    at any width for that family.

    I predict: (a) for clamped addition, coherence holds exactly when the window
    is sign-confined, over EVERY window at widths 3 through 6, exhaustively, not
    sampled; and (b) the same is NOT automatic per operation, so the lift is
    per-family rather than general, which I test by trying the analogous
    geometric predicate on clamped multiplication and expecting it to fail.

If (a) holds, C6 is not only a law-layer result. It is the mechanism that lifts
the membership test off the model width, and a canon sentence about membership
can say which of its verdicts are swept and which are computed.

Exhaustive over all windows at each width listed. Not a bench. Counts only.
"""

import sys


def coherent_clamped(lo, hi, op):
    """exhaustive: is clamping a homomorphism onto its induced operation?"""
    Q = range(lo, hi + 1)
    reach = sorted({op(a, b) for a in Q for b in Q} | set(Q))

    def rho(e):
        return lo if e < lo else (hi if e > hi else e)

    f = lambda a, b: rho(op(a, b))
    for x in reach:
        rx = rho(x)
        for y in reach:
            if rho(op(x, y)) != f(rx, rho(y)):
                return False
    return True


def sign_confined(lo, hi):
    return lo >= 0 or hi <= 0


def mirror_symmetric(lo, hi):
    return lo == -hi


ADD = lambda a, b: a + b
MUL = lambda a, b: a * b


def sweep(width, op, predicate, label):
    n = 1 << width
    lo_min, hi_max = -(n // 2), n // 2 - 1
    agree = 0
    total = 0
    disagree = []
    for lo in range(lo_min, hi_max + 1):
        for hi in range(lo, hi_max + 1):
            total += 1
            measured = coherent_clamped(lo, hi, op)
            predicted = predicate(lo, hi)
            if measured == predicted:
                agree += 1
            elif len(disagree) < 6:
                disagree.append((lo, hi, measured, predicted))
    return total, agree, disagree


print("=" * 84)
print("p6. does a closed form replace the sweep, and for which operation")
print("=" * 84)
print()

print("--- clamped ADDITION against the sign-confinement predicate ---")
print("%-8s %-10s %-10s %s" % ("width", "windows", "agreeing", "first disagreements"))
add_ok = True
for w in (3, 4, 5, 6):
    total, agree, dis = sweep(w, ADD, sign_confined, "add")
    print("%-8d %-10d %-10d %s" % (w, total, agree, dis if dis else ""))
    if agree != total:
        add_ok = False
print()

print("--- clamped MULTIPLICATION against the mirror-symmetry predicate ---")
print("%-8s %-10s %-10s %s" % ("width", "windows", "agreeing", "first disagreements"))
mul_ok = True
mul_counts = []
for w in (3, 4, 5):
    total, agree, dis = sweep(w, MUL, mirror_symmetric, "mul")
    print("%-8d %-10d %-10d %s" % (w, total, agree, dis if dis else ""))
    mul_counts.append((total, agree))
    if agree != total:
        mul_ok = False
print()

print("-" * 84)
print("ASSERTIONS")
print("-" * 84)

assert add_ok, "the sign-confinement closed form did not predict the additive verdict"
print("  ok  for clamped addition the closed form predicts the verdict at EVERY")
print("      window at widths 3 through 6, exhaustively over windows, zero residue")

# the predicate must be non-trivial: both outcomes must occur.
seen = set()
for lo in range(-8, 8):
    for hi in range(lo, 8):
        seen.add(sign_confined(lo, hi))
assert seen == {True, False}, "the predicate is constant, so the agreement is vacuous"
print("  ok  and the predicate is not constant, so the agreement is not vacuous")

assert not mul_ok, (
    "the naive multiplicative predicate also held; the control does not separate"
)
print("  ok  the analogous predicate for multiplication does NOT hold, so the lift")
print("      is per-operation rather than a general property of closed forms:")
for (t, a), w in zip(mul_counts, (3, 4, 5)):
    print("        width %d: %d of %d windows agree" % (w, a, t))

print()
print("READING. The membership test's law verdicts are not uniformly stuck at the")
print("model width. Where the panel's own law frame supplies a geometric condition")
print("on the range, the verdict is O(1) in the width and needs no sweep at all, so")
print("membership for that family is decidable wherever hosting is. Where it does")
print("not, the sweep and its transfer proviso stand. So a canon sentence about")
print("membership owes one distinction it does not currently make: which of its")
print("verdicts are computed and which are assumed uniform above the model width.")
print()
print("BOUND, stated. This does not establish the closed form at real widths; it")
print("establishes that the closed form agrees with the sweep wherever the sweep")
print("can run, which is the same epistemic position `63` section 4.2 holds and no")
print("stronger. What is new is the consequence for the membership test.")
