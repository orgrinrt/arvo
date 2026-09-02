#!/usr/bin/env python3
"""
t3. Two measurements of one cell disagree, and the dimension that explains it
    appears in neither predicate.

THE DISAGREEMENT
----------------
Unsigned, W = 4, F = 0, saturating, operation = addition, the homomorphism
identity `R(a op b) == R(R(a) op R(b))`:

  `116_probes/p4_output.txt`   add   0 / 2116
  `118_probes/q3_output.txt`   add 720 / 2304

Same primitive, same operation, same identity, opposite verdicts.  One of us
is not wrong: the two probes sweep different ambient domains, and neither
predicate says which.  My F116-7 lists no span at all.  `118` F118-7 lists
"ambient span = three times the container" without saying whether it straddles
zero.

WHY IT MATTERS BEYOND BOOKKEEPING
---------------------------------
t2 established that the domain's sign is the load-bearing half of F116-4's
hypothesis and the operation set is not.  This is the same dimension showing up
one layer down, in a routine measurement, as an unexplained numerical conflict
between two files that both believe they measured the same thing.

And it decides a clause.  `119` 4.4 states "A saturating map is a homomorphism
for no operation".  That is true on a domain containing negatives and false on
a non-negative one, and the clause carries no domain condition.

PREDICTIONS, RECORDED BEFORE THE RUN
------------------------------------
P1. Unsigned saturating on a NON-NEGATIVE span: zero failures for addition and
    for multiplication, nonzero for subtraction.  Reproducing my p4 row.
P2. Unsigned saturating on a span straddling zero: nonzero for all three.
    Reproducing `118`'s q3 row.
P3. Wrapping is unaffected by the span, zero on all three at `F = 0` either
    way.  That is the control: if wrapping moved too, the effect would be
    something other than the one I am naming.
P4. So `119` 4.4's saturating clause is true only under a domain condition it
    does not state, and both files' numbers are correct.
P5. The shipped `warm-clamp-shared` test asserting that eager and deferred
    clamping agree on NON-NEGATIVE addition is the same fact, which means the
    clause as written contradicts a test in this repository.

CONDITION-CAN-FIRE CHECK
------------------------
The wrapping control must stay at zero across both spans while saturating
moves.  If both move, the span is not what separates them.
"""

from itertools import product


def ex(op, a, b):
    return a + b if op == "add" else (a - b if op == "sub" else a * b)


def make(policy, lo, hi):
    n = hi - lo + 1

    def R(v):
        if policy == "sat":
            return min(max(v, lo), hi)
        return ((v - lo) % n) + lo

    return R


def hom_failures(R, span, op):
    s = set(span)
    bad = tot = 0
    for a in span:
        for b in span:
            la, rb = ex(op, R(a), R(b)), ex(op, a, b)
            if la not in s or rb not in s:
                continue
            tot += 1
            if R(la) != R(rb):
                bad += 1
    return bad, tot


def monotone(R, span):
    s = sorted(span)
    return all(R(s[i]) <= R(s[i + 1]) for i in range(len(s) - 1))


def main():
    print("=" * 96)
    print("t3. Reconciling two files' numbers for one cell")
    print("=" * 96)

    W, lo, hi = 4, 0, 15
    reach = 3
    spans = {
        "non-negative 0..45": list(range(0, hi * reach + 1)),
        "straddling -45..45": list(range(-hi * reach, hi * reach + 1)),
    }

    print()
    print("P1/P2/P3. The same primitive on two ambient spans")
    print()
    print(f"  {'policy':<8} {'span':<22} {'add':>12} {'sub':>12} {'mul':>12} {'monotone':>10}")
    for policy in ("sat", "wrap"):
        R = make(policy, lo, hi)
        for label, span in spans.items():
            cells = []
            for op in ("add", "sub", "mul"):
                b, t = hom_failures(R, span, op)
                cells.append(f"{b}/{t}")
            print(
                f"  {policy:<8} {label:<22} {cells[0]:>12} {cells[1]:>12} "
                f"{cells[2]:>12} {str(monotone(R, span)):>10}"
            )
    print()
    print("  The saturating rows move with the span and the wrapping rows do not,")
    print("  which is the control: the span is what separates them.")

    # ---- the witness that makes it concrete -------------------------------
    print()
    print("  The witness, on the straddling span:")
    R = make("sat", lo, hi)
    for a, b in ((-1, 1), (-5, 3)):
        print(
            f"    a={a}, b={b}: R(a+b) = {R(a + b)}, R(R(a)+R(b)) = {R(R(a) + R(b))}"
        )
    print("  On the non-negative span neither argument exists, so the identity holds")
    print("  there for a reason rather than by luck.")

    # ---- P4: what the clause says -----------------------------------------
    print()
    print("P4. `119` 4.4's clause, tested on both spans")
    print()
    for label, span in spans.items():
        R = make("sat", lo, hi)
        homs = [op for op in ("add", "sub", "mul") if hom_failures(R, span, op)[0] == 0]
        print(
            f"  saturating on {label:<22}: homomorphism for {homs if homs else 'no operation'}"
        )
    print()
    print("  The clause is true on one span and false on the other, and states no")
    print("  span. On the non-negative one it is a homomorphism for two of three.")

    # ---- P5: the shipped test ---------------------------------------------
    print()
    print("P5. The same fact as a shipped test, re-derived here")
    print()
    print("  `warm-clamp-shared/src/lib.rs` carries")
    print("  `clamping_is_a_retraction_on_non_negative_addition_at_every_swept_width`,")
    print("  which asserts that folding with a clamp at every step equals clamping")
    print("  once at the end, on non-negative addition. That is exactly the")
    print("  homomorphism identity for add on a non-negative domain. Re-derived:")
    print()
    for w in (3, 4, 5):
        limit = (1 << w) - 1
        R = make("sat", 0, limit)
        bad = tot = 0
        for arity in (2, 4, 8):
            for terms in product(range(0, limit + 1), repeat=min(arity, 3)):
                tot += 1
                eager = 0
                for t in terms:
                    eager = R(eager + t)
                once = R(sum(terms))
                if eager != once:
                    bad += 1
        print(f"    W={w}: eager and deferred clamping disagree on {bad}/{tot} folds")
    print()
    print("  Zero everywhere, which is the test's own claim and is the clause's")
    print("  counterexample.")


if __name__ == "__main__":
    main()
