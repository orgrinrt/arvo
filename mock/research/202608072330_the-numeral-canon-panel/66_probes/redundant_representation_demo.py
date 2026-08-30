#!/usr/bin/env python3
"""
Probe: is one value having several encodings a defect, or a tool?

Demonstrates a genuinely redundant (non-canonical) numeral representation:
signed-digit binary, digit set {-1, 0, 1} instead of the canonical {0, 1}.
With a digit set wider than the base, the same integer has more than one
valid digit string, and this is not a bug in the representation. It is the
standard technique for breaking carry-propagation chains in adder/multiplier
hardware (carry-save arithmetic): addition of two signed-digit numbers of
width N can be done in O(1) depth instead of O(N), because no digit position
needs to wait on a carry from its neighbour. The cost is that comparison and
canonical-form recovery need an extra pass.

This probe does not claim arvo needs signed-digit representations. It
establishes, by construction, that "many bit patterns per value" is a known,
useful, named technique in the literature this question sits inside, so the
canon should not treat redundancy as automatically a defect to rule out.
"""

from itertools import product


def digit_string_value(digits, base=2):
    """digits is a tuple, most significant first, values in {-1, 0, 1}."""
    v = 0
    for d in digits:
        v = v * base + d
    return v


def all_digit_strings(width, digit_set=(-1, 0, 1)):
    return list(product(digit_set, repeat=width))


def canonical_binary(n, width):
    """Standard two's-complement-free unsigned canonical form for n >= 0."""
    bits = []
    x = n
    for _ in range(width):
        bits.append(x % 2)
        x //= 2
    return tuple(reversed(bits))


if __name__ == "__main__":
    WIDTH = 4
    strings = all_digit_strings(WIDTH)
    by_value = {}
    for s in strings:
        v = digit_string_value(s)
        by_value.setdefault(v, []).append(s)

    redundant_values = {v: reps for v, reps in by_value.items() if len(reps) > 1}
    total_strings = len(strings)
    distinct_values = len(by_value)

    print(f"signed-digit binary, width {WIDTH}, digit set {{-1,0,1}}")
    print(f"total digit strings: {total_strings}")
    print(f"distinct integer values represented: {distinct_values}")
    print(f"values with more than one valid encoding: {len(redundant_values)}")

    # show the smallest example concretely
    example_value = min(redundant_values, key=lambda v: abs(v))
    reps = redundant_values[example_value]
    print(f"\nexample: the value {example_value} has {len(reps)} distinct valid "
          f"digit-string encodings in width {WIDTH}, including:")
    for r in reps[:4]:
        print(f"  {r}  ->  {example_value}")

    # contrast: canonical unsigned binary, width WIDTH, digit set {0,1}, is a bijection
    canon_strings = all_digit_strings(WIDTH, digit_set=(0, 1))
    canon_by_value = {}
    for s in canon_strings:
        v = digit_string_value(s)
        canon_by_value.setdefault(v, []).append(s)
    canon_redundant = {v: reps for v, reps in canon_by_value.items() if len(reps) > 1}

    print(f"\ncontrast: canonical unsigned binary, width {WIDTH}, digit set {{0,1}}")
    print(f"total digit strings: {len(canon_strings)}")
    print(f"distinct values represented: {len(canon_by_value)}")
    print(f"values with more than one encoding: {len(canon_redundant)}")

    assert len(canon_redundant) == 0, "canonical binary must be a bijection value<->encoding"
    assert len(redundant_values) > 0, "signed-digit binary must exhibit redundancy in this width"
    assert len(canon_strings) == len(canon_by_value), "canonical binary: strings and values must be in 1-1 correspondence"

    print("\nCONCLUSION: canonical binary is a bijection (one encoding per value, by")
    print("construction, since the digit set exactly matches the base). Signed-digit")
    print("binary is deliberately non-bijective (digit set wider than the base) and this")
    print("redundancy is exactly what buys constant-depth carry-free addition in hardware.")
    print("Redundancy is a property some numeral representations have on purpose, not an")
    print("accident to be designed away; the tradeoff is representation width and a")
    print("canonicalisation step, paid for O(1)-depth combination instead of O(N).")
