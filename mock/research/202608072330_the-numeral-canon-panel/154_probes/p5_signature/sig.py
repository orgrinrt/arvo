#!/usr/bin/env python3
"""P5. Self-attack on P4. Was my collapse a property of the primitives, or of my
signature?

P4 measured the index-to-primitive map with a signature of ONE arity-1 operation
(mask/clamp on an already-in-range input) and reported 128 indices -> 127
primitives, collapsing at W = 64. P4b reported the degenerate set moving with the
container, [64] -> [8,16,32,64].

`110` (read in phase two) draws a distinction P4 does not: an axis vanishes
DEFINITIONALLY when the definition of the realisation map stops reading it, and
vanishes by REACHABILITY when the map still reads it but no term in the current
signature produces an argument on which it matters. Only the first is safe to
name away, and `110`'s own P4 was falsified by exactly this confusion.

This probe asks whether MY collapse is the unsafe kind. The test, per `110`:
probe the realisation map on arguments the current signature cannot produce.

NEGATIVE CONTROL, stated before the run. Under an arity-2 signature {add}, the
pair (W=64, wrap) vs (W=64, clamp) must SEPARATE, witnessed by a concrete pair of
operands and two different results. If it does not separate, P4's collapse is
real and this probe refutes my own suspicion rather than confirming it. And the
pair (W=13, wrap) vs (W=13, clamp) must also separate, since if nothing separates
the instrument is broken.
"""
import itertools, sys

def realise(exact, w, policy):
    """R : Z -> V. The whole point: this is defined on ALL integers, including
    ones the arity-1 signature can never produce."""
    m = (1 << w) - 1
    if policy == "wrap":
        return exact & m
    return 0 if exact < 0 else (m if exact > m else exact)

def sep_arity1(w, p, q):
    """The signature P4 actually used: x |-> R(x) for x already in [0, 2^ctr)."""
    # arity-1 mask over a container of `ctr` bits; the input is a container value.
    ctr = 64
    # exhaustive over a sample of container values plus the exact boundaries
    xs = [0, 1, (1 << w) - 1, 1 << w, (1 << ctr) - 1, (1 << ctr) - 2, 12345678901234]
    for x in xs:
        if x >= (1 << ctr):
            continue
        if realise(x, w, p) != realise(x, w, q):
            return x
    return None

def sep_arity2(w, p, q):
    """A signature with add: the exact result may leave the value set."""
    m = (1 << w) - 1
    for a, b in [(m, 1), (m, m), (1, m), (m - 1, 2), (0, 0), (m, 0)]:
        if realise(a + b, w, p) != realise(a + b, w, q):
            return (a, b, realise(a + b, w, p), realise(a + b, w, q))
    return None

def sep_whole_line(w, p, q):
    """`110`'s direct test: probe R on arguments no term need ever produce."""
    for x in [-1, -5, (1 << w), (1 << w) + 7, (1 << w) * 3, 1 << 70]:
        if realise(x, w, p) != realise(x, w, q):
            return x
    return None

WIDTHS = list(range(1, 65))

def main():
    c1 = sep_arity2(64, "wrap", "clamp")
    c2 = sep_arity2(13, "wrap", "clamp")
    print(f"CONTROL (64,wrap) vs (64,clamp) separates under add: {c1} -> {'ok' if c1 else 'FAIL'}")
    print(f"CONTROL (13,wrap) vs (13,clamp) separates under add: {c2} -> {'ok' if c2 else 'FAIL'}")
    if not (c1 and c2):
        print("NEGATIVE CONTROL FAILED -- result suppressed"); sys.exit(1)
    print()
    a1 = [w for w in WIDTHS if sep_arity1(w, "wrap", "clamp") is None]
    a2 = [w for w in WIDTHS if sep_arity2(w, "wrap", "clamp") is None]
    wl = [w for w in WIDTHS if sep_whole_line(w, "wrap", "clamp") is None]
    print(f"widths where wrap == clamp, signature = arity-1 mask (what P4 used) : {a1}")
    print(f"widths where wrap == clamp, signature = arity-2 add                 : {a2}")
    print(f"widths where R itself does not read the policy (whole-line test)    : {wl}")
    print()
    print(f"index points collapsing, arity-1 signature : {len(a1)} of {len(WIDTHS)*2}")
    print(f"index points collapsing, arity-2 signature : {len(a2)} of {len(WIDTHS)*2}")
    print(f"index points collapsing DEFINITIONALLY     : {len(wl)} of {len(WIDTHS)*2}")
    print()
    if a1 and not a2:
        print("VERDICT: P4's collapse is a REACHABILITY degeneracy, not a definitional one.")
        print("         It is an artifact of P4's one-operation, arity-1 signature. Adding")
        print("         `add` -- an operation any numeral must interpret -- destroys it.")
        print("         F11 and F12 as stated in P4 are WITHDRAWN. What survives is stated")
        print("         in p5_signature/FINDINGS.md.")
    elif a2:
        print("VERDICT: the collapse survives an arity-2 signature; P4 stands.")

if __name__ == "__main__":
    main()
