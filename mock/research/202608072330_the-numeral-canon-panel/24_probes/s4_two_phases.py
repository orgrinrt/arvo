#!/usr/bin/env python3
"""
s4: the one word both vocabularies already share, and it means two different things.

The concept vocabulary uses "phase" for the offset of the arithmetic progression of
denotable magnitudes within a binade. 08:222-224 puts it in the membership predicate
("all at one phase") and 08:306 uses it for half-unit-biased, "progression, phase half a
step".

The derivation vocabulary uses "phase" for the bit offset of a packed element within a
byte. 16:178-179: "Its phase within a byte is 13k mod 8, and since thirteen and eight are
coprime the phase cycles through all eight residues."

These are the only word the two vocabularies share (see the per-file counts in the
deliverable). They are independent: this probe exhibits all four combinations, so neither
determines the other and a canon may not use one word for both.

Run:  python3 s4_two_phases.py
"""

from fractions import Fraction as Q


def value_phase(W, F, bias_steps):
    """The offset of the progression within a binade, in units of the step.
    bias_steps is the affine map's B expressed in steps: 0 for an anchored grid,
    1/2 for half-unit-biased."""
    return Q(bias_steps)


def storage_phases(W, count):
    """The bit offset of element k within a byte, for a packed run of W-bit elements.
    16:178-179. Returns the set of residues the run visits."""
    return {(W * k) % 8 for k in range(count)}


CASES = [
    ("W=13 F=0 anchored",   13, 0, Q(0)),
    ("W=8  F=0 anchored",    8, 0, Q(0)),
    ("W=13 F=0 HUB",        13, 0, Q(1, 2)),
    ("W=8  F=0 HUB",         8, 0, Q(1, 2)),
    ("W=16 F=4 anchored",   16, 4, Q(0)),
    ("W=3  F=1 anchored",    3, 1, Q(0)),
]


def main():
    print("s4: the value-space phase and the storage-space phase are independent")
    print()
    print(f"{'numeral':<22} {'value phase':<14} {'storage phases over 64 elements':<36} same word?")
    print("-" * 92)
    rows = []
    for name, W, F, bias in CASES:
        vp = value_phase(W, F, bias)
        sp = sorted(storage_phases(W, 64))
        rows.append((name, vp, sp))
        sp_s = "{" + ",".join(str(x) for x in sp) + "}"
        print(f"{name:<22} {str(vp):<14} {sp_s:<36} yes, and it is a hazard")
    print("-" * 92)
    print()

    # the four combinations, to establish independence rather than assert it
    def cls(vp, sp):
        return ("zero" if vp == 0 else "non-zero", "single" if len(sp) == 1 else "cycling")

    seen = {}
    for name, vp, sp in rows:
        seen.setdefault(cls(vp, sp), []).append(name)

    print("all four combinations are realised, so neither phase determines the other:")
    for key in [("zero", "single"), ("zero", "cycling"), ("non-zero", "single"), ("non-zero", "cycling")]:
        got = seen.get(key, [])
        mark = "PRESENT" if got else "ABSENT"
        print(f"  value phase {key[0]:<9} + storage phase {key[1]:<8}: {mark:<8} {got[0] if got else ''}")
    print()
    print("The two are orthogonal by construction: the value phase is a property of the")
    print("grid, fixed by the numeral's type. The storage phase is a property of the")
    print("element's index in a run, and 16:186 gives it as a function of W alone. A")
    print("numeral has one value phase and a run of it visits many storage phases.")


if __name__ == "__main__":
    main()
