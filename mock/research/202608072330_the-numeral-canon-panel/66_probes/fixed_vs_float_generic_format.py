#!/usr/bin/env python3
"""
Probe: are fixed-point and floating-point genuinely different number systems,
or two parametrisations of one general one?

Both encode a value as (mantissa) * (base ** exponent), mantissa a bounded
integer. The difference: fixed-point fixes the exponent as a per-TYPE
constant (known at compile time, not stored in the bits); floating-point
lets the exponent vary per VALUE and stores it in the encoding. This is the
same shape Flocq's generic_format (the standard formalisation of both fixed
and float arithmetic, used in CompCert and other verified numerical code)
gives: a format is `beta`, a mantissa bound, and an exponent function
`fexp(e)`; `fexp(e) = c` (a constant) gives fixed-point, `fexp(e) = e - p`
(tracking the value's own exponent minus a fixed precision `p`) gives
floating-point.

This probe encodes a fixed target value in each of the two schemes at
matched total bit budgets and shows: same underlying "mantissa times base to
an exponent" shape, same rounding step (round mantissa to nearest
representable integer), different placement of where the exponent lives
(compile-time constant vs per-value field), and consequently different error
behaviour (fixed: constant absolute step; floating: step scales with
magnitude).
"""

import math


def encode_fixed(x, frac_bits, mantissa_bits):
    """Q(mantissa_bits - frac_bits).frac_bits, exponent is the compile-time
    constant -frac_bits, never stored."""
    scale = 2 ** frac_bits
    m = round(x * scale)
    lo, hi = -(2 ** (mantissa_bits - 1)), 2 ** (mantissa_bits - 1) - 1
    clipped = max(lo, min(hi, m))
    reconstructed = clipped / scale
    return {
        "scheme": "fixed",
        "mantissa": clipped,
        "exponent": -frac_bits,   # compile-time constant, not in the bits
        "exponent_stored_in_bits": False,
        "reconstructed": reconstructed,
        "abs_error": abs(x - reconstructed),
    }


def encode_float(x, exponent_bits, mantissa_bits):
    """Toy IEEE-shaped binary float: sign implicit in mantissa's sign,
    exponent is per-value and IS stored in the bits."""
    if x == 0:
        return {
            "scheme": "float", "mantissa": 0, "exponent": 0,
            "exponent_stored_in_bits": True, "reconstructed": 0.0, "abs_error": 0.0,
        }
    sign = -1 if x < 0 else 1
    ax = abs(x)
    e = math.floor(math.log2(ax))
    e_bias = 2 ** (exponent_bits - 1) - 1
    e_min, e_max = -e_bias, e_bias
    e = max(e_min, min(e_max, e))
    scale = 2 ** (mantissa_bits - 1 - e)
    m = round(ax * scale)
    m_max = 2 ** mantissa_bits - 1
    if m > m_max:
        m = m_max
    reconstructed = sign * (m / scale)
    return {
        "scheme": "float",
        "mantissa": sign * m,
        "exponent": e,          # per-value, stored alongside the mantissa
        "exponent_stored_in_bits": True,
        "reconstructed": reconstructed,
        "abs_error": abs(x - reconstructed),
    }


if __name__ == "__main__":
    # matched total width: 12 bits either way, split differently per scheme
    targets = [0.1, 3.75, 1000.25, 0.0009765625]  # last is 2^-10, tests small magnitude

    print("fixed: 12-bit mantissa, frac_bits=8 (Q3.8-shaped, exponent = -8 always)")
    print("float: 4-bit exponent, 8-bit mantissa (toy IEEE-shaped, exponent per value)\n")

    for t in targets:
        f = encode_fixed(t, frac_bits=8, mantissa_bits=12)
        g = encode_float(t, exponent_bits=4, mantissa_bits=8)
        print(f"target = {t}")
        print(f"  fixed: mantissa={f['mantissa']:>6} exponent={f['exponent']:>4} "
              f"(compile-time const) -> {f['reconstructed']:.10f}  abs_err={f['abs_error']:.2e}")
        print(f"  float: mantissa={g['mantissa']:>6} exponent={g['exponent']:>4} "
              f"(stored per value)    -> {g['reconstructed']:.10f}  abs_err={g['abs_error']:.2e}")
        print()

    # the structural claim to verify: for a value near the bottom of the fixed
    # format's precision floor, float's relative-error model should track the
    # target far more tightly in absolute terms, because its exponent adapts.
    small = 0.0009765625  # 2^-10, below Q3.8's step size of 2^-8
    f_small = encode_fixed(small, frac_bits=8, mantissa_bits=12)
    g_small = encode_float(small, exponent_bits=4, mantissa_bits=8)
    print(f"small-magnitude check at {small} (= 2^-10, below fixed's 2^-8 step):")
    print(f"  fixed abs_error = {f_small['abs_error']:.2e}")
    print(f"  float abs_error = {g_small['abs_error']:.2e}")

    assert f_small["abs_error"] > g_small["abs_error"], (
        "expected float's per-value exponent to track a small magnitude tighter "
        "than fixed's fixed step size; if this fails the toy model is wrong"
    )
    assert not f_small["exponent_stored_in_bits"]
    assert g_small["exponent_stored_in_bits"]

    print("\nCONCLUSION: both schemes compute reconstructed = mantissa * 2^exponent.")
    print("The only structural difference exercised here is WHERE the exponent lives:")
    print("a compile-time constant (fixed) versus a per-value field inside the encoding")
    print("(float). That difference is exactly why float tracks small magnitudes tighter")
    print("(exponent adapts) while fixed holds a constant absolute step regardless of")
    print("magnitude. Same general shape (Flocq's generic_format: mantissa * base^fexp(e)),")
    print("two placements of one parameter (the exponent function). Whether that makes them")
    print("the same 'number system' or two different ones is a naming question the shared")
    print("shape alone does not settle, since the operations' error/rounding behaviour still")
    print("differs in a way downstream reasoning (associativity, error accumulation) can see.")
