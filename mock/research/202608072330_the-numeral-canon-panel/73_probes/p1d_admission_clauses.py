#!/usr/bin/env python3
"""p1d. Which clauses does an admission contract need, derived from what breaks it.

HYPOTHESES, both written before the run.

  H1. `71` X3 (`71:675-682`) lists what a system exposes for admission: its
      ambient domain, its representable set, and "its selected reduction with
      that reduction's two law verdicts". p1c found a reduction that adapts every
      ambient value to zero and passes BOTH verdicts under its honest declaration
      while computing nothing. I predict the clause that excludes it is the
      retraction clause, that the reduction is the identity on values already in
      the representable set, and that this clause is independent of the two
      verdicts: some shape passes the verdicts and fails it, and some shape fails
      the verdicts and passes it.

  H2. p1 found that collapsing a declaration voids the ambient half of every law
      argument while leaving the verdicts clean. I predict the repair is not a
      new clause on the reduction but the observation that the exposure list
      omits the ambient domain's OWN law inventory, and specifically that

          induced operation is associative
              <==>  ambient operation is associative on the reachable set
                    AND the reduction is coherent

      holds in every cell, with both sides observed true and false, so that the
      conjunction is what carries the information and neither conjunct alone
      does. That is `63` C6's frame (`63:659-664`) tested at the admission
      contract rather than at the law layer, by a third instrument.

If H2 holds, the collapse stops being dangerous, because a collapsed declaration
reports its own ambient's laws honestly, and the conjunction then returns the
right answer. What has to change is the exposure list, not the reduction.

Exhaustive at the 4-bit model width. Not a bench. Counts only.
"""

from itertools import product

W = 4
SQ = list(range(-(1 << (W - 1)), 1 << (W - 1)))   # signed window
UQ = list(range(1 << W))                          # unsigned window


def clamp(Q):
    lo, hi = min(Q), max(Q)
    return lambda e: lo if e < lo else (hi if e > hi else e)


def wrap(Q):
    lo, hi = min(Q), max(Q)
    span = hi - lo + 1
    return lambda e: (e - lo) % span + lo


def const_zero(_Q):
    return lambda _e: 0


def opposite(Q):
    lo, hi = min(Q), max(Q)
    return lambda e: hi if e < lo else (lo if e > hi else e)


def parity(Q):
    c = clamp(Q)
    return lambda e: c(e) if e % 2 == 0 else c(-e)


def ident(_Q):
    return lambda e: e


ADD = lambda a, b: a + b
XOR = lambda a, b: a ^ b


def build(name, Q, amb, rho_factory):
    rho = rho_factory(Q)
    reach = sorted({amb(a, b) for a in Q for b in Q} | set(Q))
    return dict(name=name, Q=Q, amb=amb, rho=rho, reach=reach)


def collapsed(d):
    f = lambda a, b: d["rho"](d["amb"](a, b))
    return dict(
        name=d["name"] + " [collapsed]",
        Q=d["Q"],
        amb=f,
        rho=lambda e: e,
        reach=sorted(set(d["Q"])),
    )


# ---- the candidate clauses -------------------------------------------------

def c_total(d):
    """every ambient reachable value adapts INTO the representable set."""
    S = set(d["Q"])
    return all(d["rho"](x) in S for x in d["reach"])


def c_retract(d):
    """the reduction moves no value that is already representable."""
    return all(d["rho"](q) == q for q in d["Q"])


def c_monotone(d):
    return all(
        d["rho"](x) <= d["rho"](y) for x in d["reach"] for y in d["reach"] if x <= y
    )


def c_coherent(d):
    f = lambda a, b: d["rho"](d["amb"](a, b))
    return all(
        d["rho"](d["amb"](x, y)) == f(d["rho"](x), d["rho"](y))
        for x in d["reach"]
        for y in d["reach"]
    )


def c_ambient_assoc(d):
    R = d["reach"]
    return all(
        d["amb"](d["amb"](a, b), c) == d["amb"](a, d["amb"](b, c))
        for a, b, c in product(R, R, R)
    )


def induced_assoc(d):
    f = lambda a, b: d["rho"](d["amb"](a, b))
    return all(
        f(f(a, b), c) == f(a, f(b, c)) for a, b, c in product(d["Q"], d["Q"], d["Q"])
    )


SHAPES = [
    build("i4 saturate", SQ, ADD, clamp),
    build("i4 wrap", SQ, ADD, wrap),
    build("u4 saturate", UQ, ADD, clamp),
    build("u4 wrap", UQ, ADD, wrap),
    build("gf(2)^4 xor", UQ, XOR, ident),
    build("u4 constant zero", UQ, ADD, const_zero),
    build("i4 opposite bound", SQ, ADD, opposite),
    build("i4 parity scramble", SQ, ADD, parity),
]
ALL = SHAPES + [collapsed(d) for d in SHAPES]

print("=" * 96)
print("p1d. admission clauses against every shape, honest and collapsed")
print("=" * 96)
print()
hdr = "%-28s %-6s %-9s %-9s %-9s %-9s | %-9s" % (
    "declaration", "total", "retract", "monotone", "coherent", "amb-assoc", "IND-ASSOC"
)
print(hdr)
print("-" * len(hdr))

rows = []
for d in ALL:
    r = (
        d["name"],
        c_total(d),
        c_retract(d),
        c_monotone(d),
        c_coherent(d),
        c_ambient_assoc(d),
        induced_assoc(d),
    )
    rows.append(r)
    print("%-28s %-6s %-9s %-9s %-9s %-9s | %-9s" % r)

print()
print("-" * 96)
print("ASSERTIONS")
print("-" * 96)

by = {r[0]: r for r in rows}

# H1. the retraction clause is independent of the two verdicts.
cz = by["u4 constant zero"]
assert cz[3] and cz[4], "constant zero was expected to pass monotone and coherent"
assert not cz[2], "constant zero was expected to FAIL the retraction clause"
print("  ok  H1a: a reduction adapting every value to zero passes BOTH proposed")
print("           verdicts (monotone, coherent) and fails the retraction clause")

ob = by["i4 opposite bound"]
assert ob[2] and not (ob[3] and ob[4]), "opposite bound did not invert the pair"
print("  ok  H1b: and a shape exists that passes retraction and fails the verdicts")
print("           (`56`'s opposite-bound mutant), so the clause is independent")

# H2. the biconditional.
mismatch = [
    r for r in rows if r[6] != (r[5] and r[4])
]
both_sides = {
    "lhs_true": any(r[6] for r in rows),
    "lhs_false": any(not r[6] for r in rows),
    "rhs_true": any(r[5] and r[4] for r in rows),
    "rhs_false": any(not (r[5] and r[4]) for r in rows),
}
print()
if mismatch:
    print("  H2 REFUTED, rows where induced-assoc != (amb-assoc AND coherent):")
    for r in mismatch:
        print("     %-28s ind=%s amb=%s coh=%s" % (r[0], r[6], r[5], r[4]))
else:
    assert all(both_sides.values()), "the biconditional held vacuously on one side"
    print("  ok  H2: induced associativity == (ambient associativity AND coherence)")
    print("          in %d of %d cells, with both sides observed true and false"
          % (len(rows), len(rows)))

# neither conjunct alone predicts it.
amb_only = [r for r in rows if r[6] != r[5]]
coh_only = [r for r in rows if r[6] != r[4]]
assert amb_only, "ambient associativity alone predicted the induced law everywhere"
assert coh_only, "coherence alone predicted the induced law everywhere"
print("  ok  neither conjunct alone predicts it: ambient-only disagrees on %d rows,"
      % len(amb_only))
print("      coherence-only disagrees on %d rows" % len(coh_only))

# the collapse specifically.
col = [r for r in rows if "[collapsed]" in r[0]]
assert all(r[4] for r in col), "a collapsed row failed coherence"
assert any(not r[5] for r in col), "no collapsed row lost ambient associativity"
kept = [r[0] for r in col if r[5]]
lost = [r[0] for r in col if not r[5]]
print("  ok  every collapsed row is coherent; %d keep ambient associativity and %d"
      % (len(kept), len(lost)))
print("      lose it, so the collapse is a laundering only where the induced")
print("      algebra was already broken. Kept: %s" % ", ".join(x.split(" [")[0] for x in kept))

print()
print("READING. The exposure list needs one item it does not have: the ambient")
print("domain's OWN law inventory. With it, a collapsed declaration is harmless,")
print("because it reports its ambient's laws honestly and the conjunction returns")
print("the right answer. Without it, a reduction's verdicts are a fact about which")
print("ambient domain was named. And the list needs the retraction clause, which")
print("neither verdict implies and which is what excludes a numeral that computes")
print("nothing while satisfying every law asked of it.")
