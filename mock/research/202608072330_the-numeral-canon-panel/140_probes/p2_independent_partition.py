"""p2: the same distinguishability count, from an independent model.

p1 models a numeral as a raw integer and implements rounding as integer
quotient arithmetic with div_euclid, and the range policy as arithmetic on the
raw integer. If that model is wrong, p1's numbers are wrong in a way p1 cannot
see, and running p1 again would reproduce the same error.

So this model shares nothing with it that I could avoid sharing:

  * values are exact rationals (fractions.Fraction), never raw integers;
  * rounding is "pick the nearest grid point under this mode", expressed by
    comparing the value against the two neighbouring multiples of 2**-F, not by
    a quotient and remainder;
  * the range policy is applied to the LOGICAL value against the logical bounds
    [0, (2**W - 1) / 2**F], not to a raw integer against a modulus;
  * wrapping is expressed as a rational modulo of the logical span, not as
    rem_euclid on an integer.

It sweeps the witness set p1 pins as COMPARABLE: W = 4, F in {0,1,2}, and the
five operations add, sub, mul, a*b+c, a*b-c. p1 reports 24 classes there.

THE CASE THAT MUST FAIL. Same two controls as p1: a duplicate assignment must
collapse into its twin, and a corrupted twin (every result nudged by one grid
step) must not. If the corrupted twin collapses, this model cannot separate
anything and its agreement with p1 would be meaningless agreement.

Prediction, written before running: this model reports 24, agreeing with p1.
If it reports something else, at least one of the two models is wrong and the
disagreement is the finding rather than the count.
"""

from fractions import Fraction
from itertools import product

ROUNDS = ["toward_zero", "ties_even", "ties_away", "toward_neg_inf", "toward_pos_inf"]
OVERFLOWS = ["wrap", "saturate_both", "saturate_high_only"]
INTERMEDIATES = ["round_each_step", "exact_then_round_once"]


def grid(f):
    """The spacing between representable values."""
    return Fraction(1, 2 ** f)


def quantise(v, f, mode):
    """Snap the exact rational v onto the grid of multiples of 2**-f.

    Expressed as a choice between the two neighbouring grid points, so that the
    mode is a statement about the real line rather than about a quotient.
    """
    g = grid(f)
    if v % g == 0:
        return v
    lo = (v // g) * g          # the grid point at or below v
    hi = lo + g
    dlo = v - lo
    dhi = hi - v
    if mode == "toward_neg_inf":
        return lo
    if mode == "toward_pos_inf":
        return hi
    if mode == "toward_zero":
        return lo if v > 0 else hi
    if dlo < dhi:
        return lo
    if dhi < dlo:
        return hi
    # a tie
    if mode == "ties_away":
        return hi if v > 0 else lo
    if mode == "ties_even":
        # "even" means the grid index is even
        idx_lo = lo / g
        return lo if idx_lo % 2 == 0 else hi
    raise ValueError(mode)


def apply_range(v, w, f, mode):
    """Bring the logical value back inside [0, max] under the range policy."""
    g = grid(f)
    top = Fraction(2 ** w - 1, 2 ** f)
    span = top + g  # the logical width of the value set
    if mode == "wrap":
        return v % span
    if mode == "saturate_both":
        if v < 0:
            return Fraction(0)
        if v > top:
            return top
        return v
    if mode == "saturate_high_only":
        if v > top:
            return top
        return v % span
    raise ValueError(mode)


def values(w, f):
    g = grid(f)
    return [g * i for i in range(2 ** w)]


def evaluate(op, a, b, c, w, f, asg):
    rnd, ovf, inter, control = asg
    if op == "add":
        r = a + b
    elif op == "sub":
        r = a - b
    elif op == "mul":
        r = quantise(a * b, f, rnd)
    elif op in ("chain_add", "chain_sub"):
        sign = 1 if op == "chain_add" else -1
        if inter == "round_each_step":
            p = quantise(a * b, f, rnd)
            p = apply_range(p, w, f, ovf)
            r = p + sign * c
        else:
            r = quantise(a * b + sign * c, f, rnd)
    else:
        raise ValueError(op)
    r = apply_range(r, w, f, ovf)
    if control == 2:
        # the corrupted twin: nudge one grid step, wrapped
        g = grid(f)
        top = Fraction(2 ** w - 1, 2 ** f)
        r = (r + g) % (top + g)
    return r


def assignments():
    out = []
    for rnd, ovf, inter in product(ROUNDS, OVERFLOWS, INTERMEDIATES):
        out.append((rnd, ovf, inter, 0))
    base = out[0]
    out.append((base[0], base[1], base[2], 1))  # duplicate control
    out.append((base[0], base[1], base[2], 2))  # corrupted control
    return out


def answer_vector(op, w, f, asg):
    vs = values(w, f)
    out = []
    if op in ("chain_add", "chain_sub"):
        for a in vs:
            for b in vs:
                for c in vs:
                    out.append(evaluate(op, a, b, c, w, f, asg))
    else:
        for a in vs:
            for b in vs:
                out.append(evaluate(op, a, b, None, w, f, asg))
    return tuple(out)


def partition(asgs, vectors):
    classes = []
    for i in range(len(asgs)):
        for cls in classes:
            if vectors[cls[0]] == vectors[i]:
                cls.append(i)
                break
        else:
            classes.append([i])
    return classes


def label(asg):
    tag = {0: "", 1: " [dup-control]", 2: " [corrupt-control]"}[asg[3]]
    return f"{asg[0]}/{asg[1]}/{asg[2]}{tag}"


def main():
    asgs = assignments()
    real = [i for i, a in enumerate(asgs) if a[3] == 0]
    dup = next(i for i, a in enumerate(asgs) if a[3] == 1)
    cor = next(i for i, a in enumerate(asgs) if a[3] == 2)

    print("p2: independent model, exact rationals, no raw-integer shifts")
    print(f"{len(real)} real assignments, plus a duplicate and a corrupted control")
    print("witness set: W=4, F in {0,1,2}, ops {add,sub,mul,a*b+c,a*b-c}\n")

    joint = [[] for _ in asgs]
    for f in (0, 1, 2):
        w = 4
        for op in ("add", "sub", "mul", "chain_add", "chain_sub"):
            vecs = [answer_vector(op, w, f, a) for a in asgs]
            classes = partition(asgs, vecs)
            nreal = sum(1 for c in classes if any(asgs[i][3] == 0 for i in c))
            print(f"W={w} F={f} {op:<10} -> {nreal} distinguishable classes")
            check_controls(classes, dup, cor, f"W={w} F={f} {op}")
            for i, a in enumerate(asgs):
                joint[i].extend(vecs[i])

    jclasses = partition(asgs, [tuple(v) for v in joint])
    jreal = sum(1 for c in jclasses if any(asgs[i][3] == 0 for i in c))
    print(f"\nCOMPARABLE_JOINT_CLASSES={jreal}")
    for cls in jclasses:
        names = [label(asgs[i]) for i in cls if asgs[i][3] == 0]
        if names:
            print("      { " + " | ".join(names) + " }")
    check_controls(jclasses, dup, cor, "JOINT")

    print(f"\np1 reports 24 for this witness set. p2 reports {jreal}.")
    print("AGREE" if jreal == 24 else "DISAGREE, and the disagreement is the finding")


FAILURES = []


def check_controls(classes, dup, cor, where):
    def find(x):
        return next(k for k, c in enumerate(classes) if x in c)
    base, d, c = find(0), find(dup), find(cor)
    if d != base:
        FAILURES.append(f"{where}: duplicate did not collapse")
        print(f"  !! CONTROL FAIL {where}: duplicate did not collapse into its twin")
    if c == base:
        FAILURES.append(f"{where}: corrupted collapsed")
        print(f"  !! CONTROL FAIL {where}: corrupted twin collapsed into its twin")


if __name__ == "__main__":
    main()
    print("\n=== controls ===")
    if FAILURES:
        print(f"CONTROL FAILURES: {len(FAILURES)}. every number above is void.")
        raise SystemExit(1)
    print("both controls behaved everywhere: the partitioner separates.")
