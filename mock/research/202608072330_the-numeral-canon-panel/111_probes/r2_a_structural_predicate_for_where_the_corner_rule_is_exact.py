#!/usr/bin/env python3
"""R2. The corner rule's exactness has a structural predicate, and the second
source of conservatism is an artifact of one-sided declarations.

Reply probe. `112` section 6 is right that `111` F111-9's zero-conservative
result is a property of the term shape swept, and I concede that in full. What
`112` states as the boundary does not hold, and its own probe output refutes it
three times.

`112` F112-6: "exact only on left-nested chains of one operation over
independent leaves". From `112_probes/p3b_output.txt`:

  (x + y) - z, SIGNED sat W=4     conservative 0     <- MIXED operations, exact
  (x + y) - x, SIGNED sat W=4     conservative 0     <- REPEATED leaf, exact
  x - x,       SIGNED sat W=4     conservative 0     <- REPEATED leaf, exact
  (x + y) * z, unsigned sat W=4   conservative 120   <- INDEPENDENT leaves, not

So "one operation" is not necessary, "independent leaves" is not sufficient, and
the word "only" makes F112-6 a necessary condition its own control row breaks.
`112` notices half of this at F112-8 and does not carry it back into F112-6.

This probe replaces the shape list with a predicate over the term's structure
and the declared extents, computable from what a type already carries:

  (a) every leaf occurs at most once, so at every node the two children have
      disjoint leaf sets and the corner rule is exact on the RANGE by induction;
  (b) no internal node has an ancestor multiplication whose other child's
      interval contains zero, so no node's overflow can be annihilated
      downstream.

(a) is the range condition and (b) is the licence condition, which is the
decomposition `112` section 6 describes in prose as "two distinct sources" and
does not turn into a test.

Predicted before running, recorded so it can be wrong:

  1. the predicate is SOUND as a sufficient condition: wherever it fires, the
     conservative count is zero. It is deliberately one-directional and will be
     over-conservative on `x - x`.
  2. the annihilation conservatism `112` F112-7 calls unreachable by any
     node-wise rule DISAPPEARS when the multiplier is declared away from zero.
     If it does, that conservatism is an artifact of every extent in both our
     files being one-sided with a lower bound of zero, which is `112` F112-23's
     own limitation applied to `112` F112-7.
"""

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


# ------------------------------------------------------------------- terms

def ev(P, t, env, general):
    if t[0] == "leaf":
        return env[t[1]]
    _, op, a, b = t
    x, y = ev(P, a, env, general), ev(P, b, env, general)
    v = x + y if op == "add" else (x - y if op == "sub" else x * y)
    return P.R(v) if general else v


def leaves(t):
    if t[0] == "leaf":
        return [t[1]]
    return leaves(t[2]) + leaves(t[3])


def internal(t):
    if t[0] == "leaf":
        return []
    return [t] + internal(t[2]) + internal(t[3])


def iv(t, ext):
    if t[0] == "leaf":
        return ext[t[1]]
    _, op, a, b = t
    la, ha = iv(a, ext)
    lb, hb = iv(b, ext)
    if op == "add":
        return (la + lb, ha + hb)
    if op == "sub":
        return (la - hb, ha - lb)
    c = [la * lb, la * hb, ha * lb, ha * hb]
    return (min(c), max(c))


def rule_licenses(P, t, ext):
    return all(P.lo <= iv(n, ext)[0] and iv(n, ext)[1] <= P.hi for n in internal(t))


def arms_agree(P, t, ext):
    for env in product(*[range(lo, hi + 1) for lo, hi in ext]):
        if ev(P, t, env, False) != ev(P, t, env, True):
            return False
    return True


# ------------------------------------------------- the structural predicate

def leaves_are_linear(t):
    """(a) every leaf occurs at most once."""
    ls = leaves(t)
    return len(ls) == len(set(ls))


def no_annihilating_ancestor(t, ext):
    """(b) no INTERNAL node sits under a multiplication whose sibling can be
    zero. The root is excluded because a root has no downstream to be masked
    by, which is why `x * y` is exact and `(x + y) * z` is not."""
    def walk(node, masked):
        if node[0] == "leaf":
            return True
        _, op, a, b = node
        # `masked` is True when this node's value can fail to reach the result.
        if node is not t and masked:
            return False
        for child, sibling in ((a, b), (b, a)):
            if child[0] == "leaf":
                continue
            m = masked
            if op == "mul":
                lo, hi = iv(sibling, ext)
                if lo <= 0 <= hi:
                    m = True
            if not walk(child, m):
                return False
        return True
    return walk(t, False)


def predicted_exact(t, ext):
    return leaves_are_linear(t) and no_annihilating_ancestor(t, ext)


# ------------------------------------------------------------------ sweeps

def sweep(P, t, decls):
    unsound = conservative = exact = 0
    pred_fired = pred_violations = 0
    for ext in decls:
        lic = rule_licenses(P, t, ext)
        agree = arms_agree(P, t, ext)
        if lic and not agree:
            unsound += 1
        elif agree and not lic:
            conservative += 1
        else:
            exact += 1
        if predicted_exact(t, ext):
            pred_fired += 1
            if agree and not lic:
                pred_violations += 1
    return unsound, conservative, exact, pred_fired, pred_violations


def one_sided(P, k):
    return [[(0, b) for b in bs]
            for bs in product(range(P.hi + 1), repeat=k)]


T = {
    "x + y": (None, "add", ("leaf", 0), ("leaf", 1)),
    "x * y": (None, "mul", ("leaf", 0), ("leaf", 1)),
    "(x + y) + z": (None, "add", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 2)),
    "(x + y) - z": (None, "sub", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 2)),
    "(x + y) * z": (None, "mul", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 2)),
    "(x + y) - y": (None, "sub", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 1)),
    "(x + y) - x": (None, "sub", (None, "add", ("leaf", 0), ("leaf", 1)), ("leaf", 0)),
    "x - x": (None, "sub", ("leaf", 0), ("leaf", 0)),
    "x * (y - y)": (None, "mul", ("leaf", 0), (None, "sub", ("leaf", 1), ("leaf", 1))),
    "(x * y) + z": (None, "add", (None, "mul", ("leaf", 0), ("leaf", 1)), ("leaf", 2)),
}


def main():
    print("R2. a structural predicate for where the corner rule is exact")
    print("=" * 78)

    rows = [
        ("x + y", Prim(4, False, "sat")),
        ("x * y", Prim(4, False, "sat")),
        ("(x + y) + z", Prim(4, False, "sat")),
        ("(x + y) - z", Prim(4, True, "sat")),
        ("(x + y) * z", Prim(4, False, "sat")),
        ("(x * y) + z", Prim(4, False, "sat")),
        ("(x + y) - y", Prim(4, False, "sat")),
        ("(x + y) - x", Prim(4, True, "sat")),
        ("x - x", Prim(4, True, "sat")),
        ("x * (y - y)", Prim(4, True, "sat")),
        ("x + y", Prim(4, False, "wrap")),
        ("(x + y) - y", Prim(4, False, "wrap")),
    ]

    print()
    print(f"  {'term':<14} {'primitive':<12} {'unsnd':>6} {'consv':>6} {'exact':>6} "
          f"{'pred fires':>11} {'pred wrong':>11}")
    tot_v = 0
    for name, P in rows:
        t = T[name]
        k = max(leaves(t)) + 1
        u, c, e, pf, pv = sweep(P, t, one_sided(P, k))
        tot_v += pv
        print(f"  {name:<14} {P.label():<12} {u:>6} {c:>6} {e:>6} {pf:>11} {pv:>11}")
    print()
    print(f"  predicate violations across every row: {tot_v}")
    print("  (a violation is the predicate firing on a conservative cell, which")
    print("   is what it would take for the predicate to be wrong)")

    # ------------------------------------------------------------------
    print()
    print("-" * 78)
    print("The annihilation case, with the multiplier declared away from zero.")
    print("112 F112-7 reports the corner rule and an enumerating oracle both")
    print("refusing 385 of 4096 while the arms agree on 120 more, and says no")
    print("node-wise rule reaches it. Every extent in that sweep is [0, b].")
    print()
    P = Prim(4, False, "sat")
    t = T["(x + y) * z"]
    print(f"  {'z declared':<20} {'extents':>9} {'consv':>7} {'exact':>7} "
          f"{'pred fires':>11}")
    for zlo in range(0, 4):
        decls = [[(0, a), (0, b), (zlo, zhi)]
                 for a, b in product(range(P.hi + 1), repeat=2)
                 for zhi in range(zlo, P.hi + 1)]
        u, c, e, pf, pv = sweep(P, t, decls)
        tag = f"[{zlo}, zhi]"
        print(f"  {tag:<20} {len(decls):>9} {c:>7} {e:>7} {pf:>11}")
    print()
    print("  the same term with z's lower bound raised off zero: the second")
    print("  source of conservatism goes with it, and the predicate fires exactly")
    print("  where it does.")

    # ------------------------------------------------------------------
    print()
    print("-" * 78)
    print("What each rule costs to carry, since the composition turns on it.")
    print("The corner rule carries two numbers per node whatever the term is.")
    print("An affine form carries one coefficient per leaf plus one per")
    print("non-constant multiplication, which 112 p8b states and which is a")
    print("property of the term rather than of the width.")
    print()
    print(f"  {'term':<26} {'corner state':>13} {'affine state':>13} {'leaf repeats':>13}")
    folds = [(f"fold of {k} adds", fold(k)) for k in (2, 4, 8, 16, 64)]
    for name, t in [(n, T[n]) for n in ("x + y", "(x + y) - y", "(x + y) * z",
                                        "x * (y - y)")] + folds:
        ls = leaves(t)
        nm = sum(1 for n in internal(t) if n[1] == "mul")
        print(f"  {name:<26} {2:>13} {len(set(ls)) + nm:>13} "
              f"{str(len(ls) != len(set(ls))):>13}")
    print()
    print("  On a fold, which is the shape satfold and warm-clamp are built")
    print("  around, no leaf repeats, the predicate fires, the corner rule is")
    print("  exact, and the affine form costs one coefficient per element for")
    print("  nothing. On a correlated term the corner rule loses licences and")
    print("  the affine form is what recovers them. That is two arms with one")
    print("  const-checkable predicate between them, not a better rule.")


def fold(k):
    t = ("leaf", 0)
    for i in range(1, k):
        t = (None, "add", t, ("leaf", i))
    return t


if __name__ == "__main__":
    main()
