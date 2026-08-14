#!/usr/bin/env python3
"""P1. One criterion decides which axes are part of a primitive's identity.

The working assumption this panel carries is that a primitive is a NAMED
COMPOSITION of an enumerated list of things (format, number system, law set,
strategy). An enumerated list has to be relitigated every time somebody invents
a new axis, and it cannot answer "is this new thing part of the primitive or
part of how the primitive is written down".

This probe tests a replacement: model a primitive as a Sigma-algebra (a carrier
set with a total interpretation of every operation symbol in a fixed signature),
and define

    two primitives are THE SAME primitive
      iff there is a denotation-preserving isomorphism between their algebras.

Then run six candidate axes through that one criterion and see whether it
reproduces the answers a hand-written table would have to state, WITHOUT being
told any of them.

Axes swept:
  1. total width W                     (expected: identity-bearing)
  2. fraction width F at fixed W       (expected: identity-bearing)
  3. signedness                        (expected: identity-bearing)
  4. overflow policy (sat vs wrap)     (expected: identity-bearing)
  5. radix (2 vs 10 vs 3)              (expected: identity-bearing)
  6. code assignment / encoding        (expected: PRESENTATION, i.e. same
     (offset, gray, reversed, shuffled) primitive under a different spelling)

And a seventh nobody put on the list:
  7. rounding mode (nearest-even vs toward-zero) at F > 0

The second half re-runs the whole sweep under a SMALLER signature to test
whether identity is absolute or relative to which operations exist.

Outcome is printed as a table. Nothing here is a benchmark; it is exact
arithmetic over small exhaustive domains.
"""

from fractions import Fraction
from itertools import product

# ---------------------------------------------------------------- the model


class Prim:
    """A primitive: a carrier of integer codes, a denotation, and operations.

    `codes` are the raw machine-side patterns. `den` maps a code to the
    rational value it stands for. The operations are defined denotationally:
    compute exactly over the rationals, then apply the realisation map R to
    land back in the representable set. R is where overflow policy and
    rounding both live, which is one of the claims P2 checks separately.
    """

    def __init__(self, name, W, F, signed, radix, policy, rounding="near", enc="ident"):
        self.name = name
        self.W, self.F, self.signed, self.radix = W, F, signed, radix
        self.policy, self.rounding, self.enc = policy, rounding, enc

        n = 1 << W
        if signed:
            ints = list(range(-(n // 2), n // 2))
        else:
            ints = list(range(0, n))
        self.ints = ints
        self.scale = Fraction(1, radix ** F)
        # value set, in denotation order
        self.values = [Fraction(k) * self.scale for k in ints]
        self.lo, self.hi = min(self.values), max(self.values)

        # the encoding: a bijection value-index -> code. Pure presentation.
        m = len(ints)
        if enc == "ident":
            perm = list(range(m))
        elif enc == "offset":  # offset binary: shift the code origin
            perm = [(i + m // 2) % m for i in range(m)]
        elif enc == "gray":
            perm = [i ^ (i >> 1) for i in range(m)]
        elif enc == "reversed":
            perm = list(range(m))[::-1]
        elif enc == "shuffled":  # an arbitrary bijection, no structure at all
            perm = [(7 * i + 3) % m for i in range(m)]
            assert len(set(perm)) == m
        else:
            raise ValueError(enc)
        self.code_of_index = perm
        self.index_of_code = {c: i for i, c in enumerate(perm)}
        self.codes = sorted(perm)

    # denotation of a code
    def den(self, code):
        return self.values[self.index_of_code[code]]

    def code(self, value):
        return self.code_of_index[self.values.index(value)]

    def _R(self, q):
        """The realisation map: exact rational -> representable value."""
        step = self.scale
        if self.policy == "sat":
            if q <= self.lo:
                return self.lo
            if q >= self.hi:
                return self.hi
            return self._round(q, step)
        elif self.policy == "wrap":
            # modular on the integer code, after rounding to the grid
            k = self._round_k(q, step)
            n = 1 << self.W
            if self.signed:
                k = ((k + n // 2) % n) - n // 2
            else:
                k = k % n
            return Fraction(k) * step
        raise ValueError(self.policy)

    def _round_k(self, q, step):
        r = q / step  # exact Fraction
        if self.rounding == "trunc":
            return int(r)  # toward zero
        # nearest, ties to even
        fl = r.numerator // r.denominator
        frac = r - fl
        if frac > Fraction(1, 2):
            return fl + 1
        if frac < Fraction(1, 2):
            return fl
        return fl if fl % 2 == 0 else fl + 1

    def _round(self, q, step):
        return Fraction(self._round_k(q, step)) * step

    # operations, denotationally
    def add(self, a, b):
        return self._R(self.den(a) + self.den(b))

    def sub(self, a, b):
        return self._R(self.den(a) - self.den(b))

    def mul(self, a, b):
        return self._R(self.den(a) * self.den(b))

    def neg(self, a):
        return self._R(-self.den(a))

    def le(self, a, b):
        return self.den(a) <= self.den(b)


SIG_FULL = ["add", "sub", "mul", "neg", "le"]
SIG_ADD = ["add"]


def same_primitive(p, q, sig):
    """Is there a denotation-preserving isomorphism p -> q over signature `sig`?

    Denotation-preserving forces the map: a code in p must go to the code in q
    standing for the same rational. That map exists iff the two value sets are
    equal. Given it, the algebras are isomorphic iff every operation commutes
    with it. So the criterion is decidable by construction and needs no search.
    """
    if set(p.values) != set(q.values):
        return False, "value sets differ"

    def phi(code):
        return q.code(p.den(code))

    for op in sig:
        if op in ("add", "sub", "mul"):
            for a, b in product(p.codes, repeat=2):
                lhs = getattr(p, op)(a, b)
                rhs = getattr(q, op)(phi(a), phi(b))
                if lhs != rhs:
                    return False, f"{op} separates them at ({p.den(a)}, {p.den(b)}): {lhs} vs {rhs}"
        elif op == "neg":
            for a in p.codes:
                if p.neg(a) != q.neg(phi(a)):
                    return False, f"neg separates them at {p.den(a)}"
        elif op == "le":
            for a, b in product(p.codes, repeat=2):
                if p.le(a, b) != q.le(phi(a), phi(b)):
                    return False, f"le separates them at ({p.den(a)}, {p.den(b)})"
    return True, "isomorphic"


# ---------------------------------------------------------------- the sweep

def base(**kw):
    d = dict(name="base", W=4, F=1, signed=False, radix=2, policy="sat")
    d.update(kw)
    d["name"] = ",".join(f"{k}={v}" for k, v in d.items() if k != "name")
    return Prim(**d)


CASES = [
    ("1. total width W", base(W=4), base(W=5)),
    ("2. fraction width F at fixed W", base(W=4, F=1), base(W=4, F=2)),
    ("3. signedness", base(signed=False), base(signed=True)),
    ("4. overflow policy", base(policy="sat"), base(policy="wrap")),
    ("5. radix 2 vs 10", base(radix=2), base(radix=10)),
    ("5b. radix 2 vs 3", base(radix=2), base(radix=3)),
    ("6a. encoding: offset", base(enc="ident"), base(enc="offset")),
    ("6b. encoding: gray", base(enc="ident"), base(enc="gray")),
    ("6c. encoding: reversed", base(enc="ident"), base(enc="reversed")),
    ("6d. encoding: arbitrary bijection", base(enc="ident"), base(enc="shuffled")),
    ("7. rounding mode at F>0", base(F=1, rounding="near"), base(F=1, rounding="trunc")),
    ("7b. rounding mode at F=0", base(F=0, rounding="near"), base(F=0, rounding="trunc")),
]


def main():
    print("P1. identity as denotation-preserving isomorphism")
    print("=" * 78)
    print(f"{'axis varied':<38} {'full signature':<16} {'{add} only':<14}")
    print("-" * 78)
    rows = []
    for label, p, q in CASES:
        full_ok, full_why = same_primitive(p, q, SIG_FULL)
        add_ok, add_why = same_primitive(p, q, SIG_ADD)
        v_full = "SAME" if full_ok else "different"
        v_add = "SAME" if add_ok else "different"
        print(f"{label:<38} {v_full:<16} {v_add:<14}")
        rows.append((label, full_ok, add_ok, full_why, add_why))

    print()
    print("why each separation happened, under the full signature:")
    for label, full_ok, add_ok, full_why, add_why in rows:
        if not full_ok:
            print(f"  {label:<38} {full_why}")

    print()
    print("axes where the SMALLER signature merges what the full one separates:")
    flips = [r for r in rows if (not r[1]) and r[2]]
    if not flips:
        print("  none")
    for label, _, _, full_why, _ in flips:
        print(f"  {label:<38} merged under {{add}}, separated by: {full_why}")

    print()
    ident = [r[0] for r in rows if not r[1]]
    pres = [r[0] for r in rows if r[1]]
    print(f"identity-bearing under the full signature ({len(ident)}): ")
    for a in ident:
        print(f"    {a}")
    print(f"presentation under the full signature ({len(pres)}): ")
    for a in pres:
        print(f"    {a}")


if __name__ == "__main__":
    main()
