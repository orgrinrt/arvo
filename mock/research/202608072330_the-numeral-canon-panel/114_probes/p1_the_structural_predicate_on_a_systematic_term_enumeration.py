#!/usr/bin/env python3
"""p1. The structural predicate, measured over every term rather than over a
hand-picked list, and its INCOMPLETENESS measured for the first time.

WHAT IS ALREADY ESTABLISHED, AND WHAT IS NOT
--------------------------------------------
`111` r2 proposes a structural predicate for where the corner rule is exact:

  (a) every leaf occurs at most once;
  (b) no internal node has an ancestor multiplication whose sibling interval
      contains zero.

and measures ZERO violations over twelve hand-picked rows. That establishes
SUFFICIENCY on those twelve rows. Three things it does not establish, each of
which this probe measures:

  1. Sufficiency over the term space rather than over twelve chosen shapes.
     Twelve rows chosen by two authors who were arguing with each other is a
     sample, and `112` section 6 is exactly the argument that a hand-picked row
     set hides the boundary.
  2. NECESSITY, or, since it is plainly not necessary (`111` says so, naming
     `x - x` and `(x + y) - x`), HOW FAR from necessary. Nobody has counted the
     cells the predicate declines that are in fact exact. That count is the
     price the design pays for using it as a selector.
  3. Whether any of it survives TWO-ENDPOINT declarations. `112` F112-23 records
     that every extent in that file is one-sided from zero, and `111` r2's own
     annihilation sweep is the single exception in either file.

PREDICTIONS, RECORDED BEFORE THE RUN SO THEY CAN BE WRONG
----------------------------------------------------------
P1. Sufficiency survives the systematic enumeration: zero violations. The
    argument is a proof rather than a hope and is written out in `note` below,
    so a violation would mean the proof is wrong and I would want to know.
P2. Sufficiency survives two-endpoint declarations. Condition (a)'s induction
    never used the lower bound being zero, and condition (b) already reads the
    sibling INTERVAL rather than its upper bound.
P3. The predicate is substantially incomplete, and most of the loss is cells
    where the corner rule LICENSES, so its refusal was never in question. That
    matters because a cell where the rule licenses is a cell where nothing is
    lost by using the cheap rule, whatever the predicate says.
P4. Therefore the disjunction `corner licenses OR (a AND b)` is sound, is
    strictly more complete, and is the predicate a selector actually wants. I
    expect it to close most of the gap and not all of it.

WHY (a) AND (b) ARE SUFFICIENT, AS AN ARGUMENT RATHER THAN A COUNT
-------------------------------------------------------------------
A cell is CONSERVATIVE when the arms agree and the rule refuses. So sufficiency
is: (a) and (b) together forbid that pair.

  Step 1, from (a). With every leaf distinct, the two children of any node have
  disjoint leaf sets, so their values vary independently. For add, sub and mul,
  the extremes of the result over a box are attained at corners of the operand
  intervals. So by induction the propagated interval's endpoints are the exact
  extremes of the node's REACHABLE set. Hence "the rule refuses" is exactly
  "some node's reachable set leaves the container", which is exactly "some node
  overflows on some tuple in the declaration".

  Step 2, from (b). With no internal node under a multiplication whose sibling
  can be zero, an overflow at a node cannot be annihilated on the way to the
  root. Formally: the general arm applies R at every node, so it differs from
  the exact arm at the first node that overflows, and (b) says that difference
  is carried rather than masked. Hence "some node overflows on some tuple" is
  exactly "the arms disagree on some tuple", which is "the arms do not agree".

  Chaining: rule refuses <=> arms do not agree. So conservatism is impossible.

Step 2 is the weaker half. (b) as stated forbids the multiplicative annihilator
specifically, which is the only masking available in a signature of add, sub and
mul. A signature with an idempotent or an absorbing element elsewhere would need
(b) restated, and this probe cannot see that because it sweeps that signature.
That is a bound and it is listed in the finding.

NEGATIVE CONTROLS, EACH SHOWN FIRING BEFORE ANY NUMBER IS BELIEVED
------------------------------------------------------------------
C1. Drop condition (b): the weakened predicate must produce violations. If it
    does not, condition (b) is doing nothing and the finding is about (a) alone.
C2. Drop condition (a): likewise.
C3. A predicate that always fires: must produce violations, or the whole sweep
    contains no conservative cell and measures nothing.
C4. The soundness counter itself: the corner rule must be shown capable of being
    reported unsound, by mutating it into a rule that under-approximates.
"""

from itertools import product
import random
import sys

# --------------------------------------------------------------------- model


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


OPS = ("add", "sub", "mul")


def apply_op(op, x, y):
    if op == "add":
        return x + y
    if op == "sub":
        return x - y
    return x * y


# --------------------------------------------------------------------- terms
# A term is ("leaf", i) or (op, left, right).


def shapes(n):
    """Every binary tree with n leaf slots, as nested None placeholders."""
    if n == 1:
        return [None]
    out = []
    for k in range(1, n):
        for a in shapes(k):
            for b in shapes(n - k):
                out.append((a, b))
    return out


def set_partitions(n):
    """Every way of identifying leaf slots, canonically named 0,1,2,..."""
    if n == 0:
        yield []
        return
    for rest in set_partitions(n - 1):
        m = max(rest) + 1 if rest else 0
        for lab in range(m + 1):
            yield rest + [lab]


def fill(shape, ops, labels, oi, li):
    if shape is None:
        lab = labels[li[0]]
        li[0] += 1
        return ("leaf", lab)
    a = fill(shape[0], ops, labels, oi, li)
    b = fill(shape[1], ops, labels, oi, li)
    op = ops[oi[0]]
    oi[0] += 1
    return (op, a, b)


def all_terms(n):
    """Every term with n leaf slots over {add, sub, mul} and every leaf
    identification, canonically labelled."""
    out = []
    for sh in shapes(n):
        for ops in product(OPS, repeat=n - 1):
            for labels in set_partitions(n):
                out.append(fill(sh, list(ops), labels, [0], [0]))
    return out


def leaves(t):
    return [t[1]] if t[0] == "leaf" else leaves(t[1]) + leaves(t[2])


def internal(t):
    return [] if t[0] == "leaf" else [t] + internal(t[1]) + internal(t[2])


def show(t):
    if t[0] == "leaf":
        return "xyzw"[t[1]]
    sym = {"add": "+", "sub": "-", "mul": "*"}[t[0]]
    return f"({show(t[1])} {sym} {show(t[2])})"


# ----------------------------------------------------------- the corner rule


def iv(t, ext):
    if t[0] == "leaf":
        return ext[t[1]]
    la, ha = iv(t[1], ext)
    lb, hb = iv(t[2], ext)
    if t[0] == "add":
        return (la + lb, ha + hb)
    if t[0] == "sub":
        return (la - hb, ha - lb)
    c = [la * lb, la * hb, ha * lb, ha * hb]
    return (min(c), max(c))


def corner_licenses(P, t, ext):
    """Per node, including the root and the leaves. `112` p7c establishes that
    a root-only check is unsound, so this is the per-node form throughout."""
    for lo, hi in ext:
        if not (P.lo <= lo and hi <= P.hi):
            return False
    return all(P.lo <= iv(n, ext)[0] and iv(n, ext)[1] <= P.hi for n in internal(t))


def corner_licenses_ROOT_ONLY(P, t, ext):
    """C4's mutation: the unsound spelling `112` p7c refutes."""
    lo, hi = iv(t, ext)
    return P.lo <= lo and hi <= P.hi


# -------------------------------------------------------------- the ground truth


def ev(P, t, env, general):
    if t[0] == "leaf":
        return env[t[1]]
    x = ev(P, t[1], env, general)
    y = ev(P, t[2], env, general)
    v = apply_op(t[0], x, y)
    return P.R(v) if general else v


def tuples(ext, k):
    return product(*[range(ext[i][0], ext[i][1] + 1) for i in range(k)])


def arms_agree(P, t, ext, k):
    for env in tuples(ext, k):
        if ev(P, t, env, False) != ev(P, t, env, True):
            return False
    return True


def oracle_licenses(P, t, ext, k):
    """Every reachable intermediate inside the container. Strictly stronger than
    `arms_agree`, which tolerates an overflow that is masked downstream."""
    nodes = internal(t)
    for env in tuples(ext, k):
        for nd in nodes:
            v = ev(P, nd, env, False)
            if not (P.lo <= v <= P.hi):
                return False
    return True


# ------------------------------------------------------ the structural predicate


def cond_a(t):
    ls = leaves(t)
    return len(ls) == len(set(ls))


def cond_b(t, ext):
    def walk(node, masked):
        if node[0] == "leaf":
            return True
        if node is not t and masked:
            return False
        for child, sib in ((node[1], node[2]), (node[2], node[1])):
            if child[0] == "leaf":
                continue
            m = masked
            if node[0] == "mul":
                lo, hi = iv(sib, ext)
                if lo <= 0 <= hi:
                    m = True
            if not walk(child, m):
                return False
        return True

    return walk(t, False)


def pred_ab(P, t, ext):
    return cond_a(t) and cond_b(t, ext)


def pred_a_only(P, t, ext):
    return cond_a(t)


def pred_b_only(P, t, ext):
    return cond_b(t, ext)


def pred_always(P, t, ext):
    return True


def pred_disjunctive(P, t, ext):
    """P4's candidate: either the rule got its licence, in which case nothing was
    lost whatever the shape, or the refusal is provably honest."""
    return corner_licenses(P, t, ext) or (cond_a(t) and cond_b(t, ext))


PREDICATES = [
    ("(a) and (b)", pred_ab),
    ("(a) only  [C1]", pred_a_only),
    ("(b) only  [C2]", pred_b_only),
    ("always    [C3]", pred_always),
    ("licenses or (a and b)", pred_disjunctive),
]


# ----------------------------------------------------------------- declarations


def one_sided(P, k):
    return [tuple((0, b) for b in bs) for bs in product(range(0, P.hi + 1), repeat=k)]


def two_endpoint(P, k, rng=None):
    per = [(lo, hi) for lo in range(P.lo, P.hi + 1) for hi in range(lo, P.hi + 1)]
    if rng is None:
        return [tuple(c) for c in product(per, repeat=k)]
    return [tuple(rng.choice(per) for _ in range(k)) for _ in range(4000)]


# ------------------------------------------------------------------------ sweep


def sweep(P, terms, decls_for, want_oracle=False):
    tot = dict(cells=0, unsound=0, conservative=0, exact=0, licenses=0, agree=0)
    pred = {name: dict(fires=0, violations=0, missed=0) for name, _ in PREDICATES}
    honest = dict(checked=0, wrong=0)
    root_only_unsound = 0
    for t in terms:
        k = max(leaves(t)) + 1
        for ext in decls_for(k):
            lic = corner_licenses(P, t, ext)
            agree = arms_agree(P, t, ext, k)
            tot["cells"] += 1
            tot["licenses"] += int(lic)
            tot["agree"] += int(agree)
            if lic and not agree:
                tot["unsound"] += 1
            elif agree and not lic:
                tot["conservative"] += 1
            else:
                tot["exact"] += 1
            if corner_licenses_ROOT_ONLY(P, t, ext) and not agree:
                root_only_unsound += 1
            for name, fn in PREDICATES:
                fires = fn(P, t, ext)
                if fires:
                    pred[name]["fires"] += 1
                    if agree and not lic:
                        pred[name]["violations"] += 1
                else:
                    if not (agree and not lic):
                        pred[name]["missed"] += 1
            if want_oracle and cond_a(t) and cond_b(t, ext) and not lic:
                honest["checked"] += 1
                if oracle_licenses(P, t, ext, k):
                    honest["wrong"] += 1
    return tot, pred, honest, root_only_unsound


def report(title, P, tot, pred, honest, root_only_unsound):
    print()
    print("-" * 78)
    print(f"{title}   primitive {P.label()}")
    print(
        f"  cells {tot['cells']}   corner licenses {tot['licenses']}   "
        f"arms agree {tot['agree']}"
    )
    print(
        f"  corner rule: unsound {tot['unsound']}   conservative "
        f"{tot['conservative']}   exact {tot['exact']}"
    )
    print()
    print(f"  {'predicate':<24} {'fires':>8} {'violations':>11} {'declines an exact cell':>24}")
    for name, _ in PREDICATES:
        d = pred[name]
        print(f"  {name:<24} {d['fires']:>8} {d['violations']:>11} {d['missed']:>24}")
    if honest["checked"]:
        print()
        print(
            f"  honest-refusal check: {honest['checked']} cells where (a) and (b) hold "
            f"and the rule refuses;"
        )
        print(
            f"    an enumerating oracle also refuses on all but {honest['wrong']} of them"
        )
    print()
    print(
        f"  C4 mutation, root-only discharge: unsound on {root_only_unsound} cells "
        f"(0 would mean the check cannot fire here)"
    )


def main():
    print("=" * 78)
    print("p1. The structural predicate over every term, and its incompleteness")
    print("=" * 78)

    rng = random.Random(20260814)

    # ---- 1. Exhaustive, arity 2 and 3, one-sided, four primitives.
    for P in (
        Prim(3, False, "sat"),
        Prim(3, True, "sat"),
        Prim(3, False, "wrap"),
        Prim(3, True, "wrap"),
    ):
        terms = all_terms(2) + all_terms(3)
        tot, pred, honest, ro = sweep(P, terms, lambda k: one_sided(P, k), want_oracle=True)
        report(
            f"EXHAUSTIVE  {len(terms)} terms at 2 and 3 leaf slots, one-sided [0,b]",
            P,
            tot,
            pred,
            honest,
            ro,
        )

    # ---- 2. Exhaustive, arity 2, TWO-ENDPOINT declarations. P2's test.
    for P in (Prim(3, False, "sat"), Prim(3, True, "sat"), Prim(3, True, "wrap")):
        terms = all_terms(2)
        tot, pred, honest, ro = sweep(
            P, terms, lambda k: two_endpoint(P, k), want_oracle=True
        )
        report(
            f"EXHAUSTIVE  {len(terms)} terms at 2 leaf slots, TWO-ENDPOINT [lo,hi]",
            P,
            tot,
            pred,
            honest,
            ro,
        )

    # ---- 3. Sampled, arity 3, two-endpoint. Exhaustive is 46656 declarations
    #         per term times 512 tuples times 90 terms, which does not finish.
    for P in (Prim(3, False, "sat"), Prim(3, True, "sat")):
        terms = all_terms(3)
        tot, pred, honest, ro = sweep(
            P, terms, lambda k: two_endpoint(P, k, rng), want_oracle=True
        )
        report(
            f"SAMPLED  {len(terms)} terms at 3 leaf slots, TWO-ENDPOINT, "
            f"4000 declarations each",
            P,
            tot,
            pred,
            honest,
            ro,
        )

    # ---- 4. Sampled, arity 4, one-sided. 2025 terms is too many to pair with
    #         4096 declarations and 4096 tuples, so terms are sampled too.
    for P in (Prim(2, False, "sat"), Prim(3, False, "sat")):
        terms = rng.sample(all_terms(4), 120)
        tot, pred, honest, ro = sweep(
            P,
            terms,
            lambda k: [
                tuple((0, b) for b in bs)
                for bs in rng.sample(
                    list(product(range(0, P.hi + 1), repeat=k)),
                    min(300, (P.hi + 1) ** k),
                )
            ],
            want_oracle=False,
        )
        report(
            f"SAMPLED  120 of 2025 terms at 4 leaf slots, one-sided, "
            f"300 declarations each",
            P,
            tot,
            pred,
            honest,
            ro,
        )

    print()
    print("=" * 78)
    print("READING THE CONTROLS")
    print("=" * 78)
    print(
        """
  C1 and C2 fire when the '(a) only' and '(b) only' rows show violations. A
  violation is the predicate claiming exactness on a cell where the corner rule
  is in fact conservative, which is the only way the predicate can be wrong.
  If either control shows zero, that condition is carrying no weight in this
  region and the finding must say so.

  C3, the always-fires row, must show violations equal to the conservative count.
  If it shows zero the sweep contains no conservative cell at all and every
  other row's zero is a dead branch rather than a result.

  C4 fires when the root-only mutation shows a nonzero unsound count, which is
  `112` F112-21 reproduced on this instrument.

  The 'declines an exact cell' column is the predicate's INCOMPLETENESS: cells
  where the corner rule is not conservative and the predicate refuses to say so.
  Lower is better and zero is unreachable, because the predicate is structural
  and exactness is not.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
