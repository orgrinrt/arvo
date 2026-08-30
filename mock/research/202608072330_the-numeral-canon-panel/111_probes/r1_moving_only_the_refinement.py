#!/usr/bin/env python3
"""R1. Move only the refinement, and apply the criterion `108` actually states.

Reply probe, written after reading `112`. Two things are wrong upstream of it and
both are cheap to fix.

**`108` states its membership criterion in a sentence neither `111` nor `112`
used.** The clause at `108:822-827` is three sentences:

  1. "The declared semantics is an assignment on the axes a consumer can
     observe: those where moving the assignment changes what the program
     denotes, or whether it denotes at all."          <- the DEFINITION
  2. "It is supplied and never derived, because a consumer of a value cannot
     recover it from the bits, so every consumer of that value must agree about
     it."                                             <- a property, its reason,
                                                          and its consequence
  3. "An axis belongs here if there is any reachable chain on which moving it is
     observable."                                     <- the MEMBERSHIP CRITERION

`111` section 12 alternative E used the reason from sentence two. `112` section
3.2 corrected that to the consequence from sentence two. **Both of us reached
into the middle sentence while the criterion sits in the third**, stated in the
words "an axis belongs here if".

**And the criterion has never been applied to a refinement, by anyone.** The
measurement `112` cites for its answer (`112_probes/p2b`, the Q2-Q6 table) moves
the overflow policy, the total width and the signedness with the grade held
fixed. That is a real result and it is `112` F112-3. It is not a measurement of
what happens when the refinement moves, which is what the criterion asks.

So this probe does the missing thing: hold the primitive and every axis fixed,
move ONLY the declared refinement, and ask whether any reachable term's answer
changes. Under sentence three, a yes puts the refinement in the declared
semantics and a no keeps it out.

Predicted before running, and recorded so it can be wrong: zero, on every term
and every pair of refinements, because a refinement selects which arm is
available rather than what the arms compute. The control moves an observable
axis over the same terms so the instrument is shown able to fire.
"""

from fractions import Fraction
from itertools import product


class Prim:
    def __init__(self, W, signed, policy):
        self.W, self.signed, self.policy = W, signed, policy
        n = 1 << W
        self.lo = -(n // 2) if signed else 0
        self.hi = (n // 2 - 1) if signed else n - 1

    def label(self):
        return f"{'i' if self.signed else 'u'}W{self.W}/{self.policy}"

    def R(self, v):
        if self.lo <= v <= self.hi:
            return v
        if self.policy == "sat":
            return self.hi if v > self.hi else self.lo
        span = self.hi - self.lo + 1
        return ((v - self.lo) % span) + self.lo


# --------------------------------------------------------------- terms
# A term is (op, children) or ("leaf", index). Evaluated two ways: the general
# arm applies R at every node, the cheap arm never does.

def ev(P, t, env, general):
    if t[0] == "leaf":
        return env[t[1]]
    _, op, a, b = t
    x = ev(P, a, env, general)
    y = ev(P, b, env, general)
    v = x + y if op == "add" else (x - y if op == "sub" else x * y)
    return P.R(v) if general else v


def leaf_count(t, n=0):
    if t[0] == "leaf":
        return max(n, t[1] + 1)
    return max(leaf_count(t[2], n), leaf_count(t[3], n))


def nodes(t):
    if t[0] == "leaf":
        return []
    return [t] + nodes(t[2]) + nodes(t[3])


def corner_interval(t, ext):
    """The propagated interval, corners, leaves treated as independent."""
    if t[0] == "leaf":
        return ext[t[1]]
    _, op, a, b = t
    la, ha = corner_interval(a, ext)
    lb, hb = corner_interval(b, ext)
    if op == "add":
        return (la + lb, ha + hb)
    if op == "sub":
        return (la - hb, ha - lb)
    c = [la * lb, la * hb, ha * lb, ha * hb]
    return (min(c), max(c))


def discharges(P, t, ext):
    """Per-node check, which `112` p7c establishes is the load-bearing form."""
    for nd in nodes(t):
        lo, hi = corner_interval(nd, ext)
        if lo < P.lo or hi > P.hi:
            return False
    return True


def tuples(ext):
    return product(*[range(lo, hi + 1) for lo, hi in ext])


def selected(P, t, ext, env):
    """What the program computes under this declaration: the cheap arm where the
    declaration discharges, the general arm otherwise."""
    return ev(P, t, env, not discharges(P, t, ext))


TERMS = [
    ("x + y", (None, "add", ("leaf", 0), ("leaf", 1))),
    ("(x + y) + z", (None, "add", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 2))),
    ("x * y", (None, "mul", ("leaf", 0), ("leaf", 1))),
    ("(x + y) - y", (None, "sub", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 1))),
    ("(x + y) * z", (None, "mul", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 2))),
    ("(x + y) - z", (None, "sub", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 2))),
]


def declarations(P, k, uniform):
    """One-sided declarations over k leaves. Uniform for k >= 3 so the pair loop
    stays enumerable; every tuple at k = 2, which is the exhaustive case."""
    if uniform:
        return [tuple([b] * k) for b in range(P.hi + 1)]
    return list(product(range(P.hi + 1), repeat=k))


def move_only_the_refinement(P, name, t):
    """Every pair of one-sided declarations, compared on the values the tighter
    one admits. Only weakenings are compared, since a tightening below the truth
    is a false declaration rather than a move of the axis."""
    k = leaf_count(t)
    decls = declarations(P, k, uniform=(k >= 3))
    diffs = 0
    pairs = 0
    crossing = 0
    disagreed_pairs = 0
    for bs_t in decls:
        ext_t = [(0, b) for b in bs_t]
        dt = discharges(P, t, ext_t)
        for bs_w in decls:
            if any(w < x for w, x in zip(bs_w, bs_t)):
                continue  # only weakenings
            ext_w = [(0, b) for b in bs_w]
            pairs += 1
            if dt != discharges(P, t, ext_w):
                crossing += 1
            bad = 0
            for env in tuples(ext_t):
                if selected(P, t, ext_t, env) != selected(P, t, ext_w, env):
                    bad += 1
            diffs += bad
            if bad:
                disagreed_pairs += 1
    return pairs, diffs, disagreed_pairs, crossing


def move_an_observable_axis(PA, PB, name, t):
    """The control. Same terms, same declarations, one observable axis moved."""
    k = leaf_count(t)
    diffs = 0
    cells = 0
    for bs in product(range(PA.hi + 1), repeat=k):
        ext = [(0, b) for b in bs]
        for env in tuples(ext):
            cells += 1
            if selected(PA, t, ext, env) != selected(PB, t, ext, env):
                diffs += 1
    return cells, diffs


def main():
    print("R1. does moving ONLY the refinement change what the program denotes?")
    print("=" * 78)
    print("criterion applied: 108:826, 'an axis belongs here if there is any")
    print("reachable chain on which moving it is observable'")
    print()

    P = Prim(3, False, "sat")
    print(f"primitive held fixed at {P.label()}, declarations one-sided [0, b],")
    print("every declaration pair at two leaves, uniform declarations at three")
    print()
    print(f"  {'term':<16} {'decl pairs':>11} {'arm changed':>12} "
          f"{'pairs disagreeing':>18} {'value disagreements':>20}")
    total_diffs = 0
    total_cross = 0
    for name, t in TERMS:
        pairs, diffs, dpairs, cross = move_only_the_refinement(P, name, t)
        total_diffs += diffs
        total_cross += cross
        print(f"  {name:<16} {pairs:>11} {cross:>12} {dpairs:>18} {diffs:>20}")
    print()
    print(f"  declaration pairs where the SELECTED ARM changed : {total_cross}")
    print(f"  chains where the ANSWER changed                  : {total_diffs}")
    print("  the first being large is what makes the second's zero a result")

    print()
    print("CONTROL: the same terms with an observable axis moved instead,")
    print("so the instrument is shown able to report a difference.")
    print()
    sat = Prim(3, False, "sat")
    wrp = Prim(3, False, "wrap")
    sgn = Prim(3, True, "sat")
    print(f"  {'term':<16} {'axis moved':<22} {'cells':>9} {'disagreements':>15}")
    for name, t in TERMS:
        c, d = move_an_observable_axis(sat, wrp, name, t)
        print(f"  {name:<16} {'overflow policy':<22} {c:>9} {d:>15}")
    for name, t in TERMS[:3]:
        c, d = move_an_observable_axis(sat, sgn, name, t)
        print(f"  {name:<16} {'signedness':<22} {c:>9} {d:>15}")

    print()
    print("-" * 78)
    print("reading:")
    print("  Under 108's own stated criterion, a refinement is not a member of")
    print("  the declared semantics: there is no reachable chain on which moving")
    print("  it is observable. 112 reaches the same verdict and 111 leaned the")
    print("  other way; both of us argued from the middle sentence of the clause")
    print("  rather than from the criterion in the sentence after it.")
    print()
    print("  What moving a refinement changes is which arm is AVAILABLE. The")
    print("  control shows the instrument reports a difference when an axis that")
    print("  does change the denotation is moved instead.")


if __name__ == "__main__":
    main()
