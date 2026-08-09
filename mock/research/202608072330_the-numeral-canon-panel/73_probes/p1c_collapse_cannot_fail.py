#!/usr/bin/env python3
"""p1c. The collapsed row cannot fail, demonstrated by mutation rather than asserted.

`72` section 2 established the standard this probe answers to: a row that cannot
fail is not a measurement, and the way to show it cannot fail is to corrupt what
it is nominally about and watch it not notice (`72_probes/p1`, the BROKEN_constant
encoding). p1 reported that every collapsed declaration returns zero coherence and
zero monotonicity failures. That number is worth nothing until someone tries to
make it nonzero.

MUTATION SET. Four reductions, three of them not reductions anyone would ship:

    honest saturate        the control, clamps to the window
    constant zero          every ambient value adapts to 0
    opposite bound         `56`'s mutant: clamps to the WRONG bound
    parity scramble        adapts to a value chosen by the input's parity

For each, the honest declaration's verdicts are measured, and the collapsed
declaration's verdicts are measured. If the collapsed column is zero for all four,
including for reductions that destroy every law in sight, then the collapsed
verdict is reporting on the shape of the declaration and not on the arithmetic,
and no membership test may read it.

Exhaustive at the 4-bit model width. Not a bench. Counts only.
"""

W = 4
Q = list(range(-(1 << (W - 1)), 1 << (W - 1)))
LO, HI = min(Q), max(Q)
AMB = lambda a, b: a + b
REACH = sorted({AMB(a, b) for a in Q for b in Q} | set(Q))


def honest_saturate(e):
    return LO if e < LO else (HI if e > HI else e)


def constant_zero(_e):
    return 0


def opposite_bound(e):
    return HI if e < LO else (LO if e > HI else e)


def parity_scramble(e):
    return honest_saturate(e) if e % 2 == 0 else honest_saturate(-e)


MUTANTS = [
    ("honest saturate", honest_saturate),
    ("constant zero", constant_zero),
    ("opposite bound", opposite_bound),
    ("parity scramble", parity_scramble),
]


def verdicts(amb, rho, reach, carrier):
    f = lambda a, b: rho(amb(a, b))
    coh = sum(
        1 for x in reach for y in reach if rho(amb(x, y)) != f(rho(x), rho(y))
    )
    mono = sum(1 for x in reach for y in reach if x <= y and rho(x) > rho(y))
    assoc = sum(
        1
        for a in carrier
        for b in carrier
        for c in carrier
        if f(f(a, b), c) != f(a, f(b, c))
    )
    return coh, mono, assoc, len(reach) ** 2, len(carrier) ** 3


print("=" * 78)
print("p1c. mutation: can the collapsed verdict be made to fail?")
print("Q = [%d, %d], ambient = integer addition, reachable set = %d values"
      % (LO, HI, len(REACH)))
print("=" * 78)
print()
hdr = "%-18s %-22s %-22s" % ("reduction", "HONEST coh/mono/assoc", "COLLAPSED coh/mono/assoc")
print(hdr)
print("-" * len(hdr))

collapsed_nonzero = 0
honest_nonzero = 0
for name, rho in MUTANTS:
    hc, hm, ha, hden, aden = verdicts(AMB, rho, REACH, Q)
    f = lambda a, b, r=rho: r(AMB(a, b))
    cc, cm, ca, cden, caden = verdicts(f, lambda e: e, sorted(set(Q)), Q)
    print(
        "%-18s %-22s %-22s"
        % (
            name,
            "%d/%d  %d/%d  %d/%d" % (hc, hden, hm, hden, ha, aden),
            "%d/%d  %d/%d  %d/%d" % (cc, cden, cm, cden, ca, caden),
        )
    )
    if hc or hm:
        honest_nonzero += 1
    if cc or cm:
        collapsed_nonzero += 1

print()
print("-" * 78)
print("ASSERTIONS")
print("-" * 78)
assert honest_nonzero >= 3, (
    "the mutants did not break the honest verdicts, so the mutation set is too weak"
)
print("  ok  the mutation set breaks the HONEST verdicts in %d of 4 cases"
      % honest_nonzero)
assert collapsed_nonzero == 0, "a mutant reached the collapsed verdict"
print("  ok  no mutant reaches the COLLAPSED verdict: 0 of 4, both law families")
print()
print("READING. The collapsed coherence and monotonicity numbers in p1 are not")
print("measurements and must not be cited as any. They are the shape of the")
print("declaration, and the shape is available to every candidate for free.")
print("A membership test that reads a candidate's own law verdicts off the pair")
print("the candidate named is passed by a reduction that adapts every value to")
print("zero. What p1c adds to p1 is that this is not a 4-bit fact.")
