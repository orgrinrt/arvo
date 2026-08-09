"""Is the anchored fixed-point family a lattice, and what does that depend on?

Hypothesis under test: the inclusion order on anchored fixed-point value sets
is the product of three chains, on (fraction digits, lower end, upper end).
A product of chains is a distributive lattice. So any failure of meet or join
is a failure of the SHAPE SPACE to be closed under the ambient operation, and
not a failure of the order.

Two closure conditions are separated here:
  Z  admit the numeral whose value set is exactly {0} (zero total width),
  N  admit negative integer width, so that the grid may be finer than one
     whole unit while still starting at zero.

The probe reports the verdict for each of the four combinations.
"""

from fractions import Fraction as Q
from itertools import combinations
from poset import fixed, analyse


def uni(i_lo, i_hi, f_lo, f_hi, signs, zero):
    u = {}
    if zero:
        u[frozenset({Q(0)})] = "ZERO"
    for signed in signs:
        for i in range(i_lo, i_hi + 1):
            for f in range(f_lo, f_hi + 1):
                if i + f <= 0:
                    continue
                v = fixed(i, f, signed)
                if v:
                    u.setdefault(v, f"{'I' if signed else 'U'}{i}.{f}")
    return u


if __name__ == "__main__":
    for zero in (False, True):
        for neg in (False, True):
            lo = -3 if neg else 0
            for signs, tag in (((False,), "unsigned"), ((False, True), "both signs")):
                analyse(uni(lo, 4, lo, 4, signs, zero),
                        f"{tag}: zero-width={'Y' if zero else 'N'} "
                        f"negative-width={'Y' if neg else 'N'}",
                        verbose_examples=2)
