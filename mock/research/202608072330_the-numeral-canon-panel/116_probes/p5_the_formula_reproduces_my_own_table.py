#!/usr/bin/env python3
"""
p5. Checking `114` F114-12's formula against my own F112-14, by computing it
    rather than by reading their table against mine.

THE CLAIM
---------
`114` section 6.3 reports that `112` F112-14's three constructions are three
instances of one rule over the structure constants: the L1 norm of each output
component's row, plus one bit for whether that row has a negative entry, read
against the base's signedness.

My F112-14 was a table of three measured rows.  If the formula reproduces them
without measuring them, the table is superseded by a sentence, which is
strictly better for a canon: a table is an obligation that grows with every
construction anyone adds.

This probe derives the three constructions' structure constants, applies the
formula, and compares against the rows my own `112_probes/p5b` MEASURED.  The
comparison is against my committed output, not against `114`'s account of it.

PREDICTION, RECORDED BEFORE THE RUN
-----------------------------------
The formula reproduces all six cells of my table: componentwise for product2
at both signednesses, twice-componentwise for dual at both, twice-componentwise
for complex over a signed base, and nothing at all for complex over an unsigned
one.

CONDITION-CAN-FIRE CHECK
------------------------
A formula that returned the same verdict for every construction would match
nothing.  The verdicts it produces must differ across the three, and the run
reports that before the comparison.
"""

# c[i][j][k]: coefficient of a_j * b_k in output component i
CONSTR = {
    "product2": [
        {(0, 0): 1},
        {(1, 1): 1},
    ],
    "dual": [
        {(0, 0): 1},
        {(0, 1): 1, (1, 0): 1},
    ],
    "complex": [
        {(0, 0): 1, (1, 1): -1},
        {(0, 1): 1, (1, 0): 1},
    ],
    "quaternion_row0": [
        {(0, 0): 1, (1, 1): -1, (2, 2): -1, (3, 3): -1},
    ],
}


def formula(rows, signed):
    """Returns (max L1 norm, any-negative-row) and the verdict."""
    norms = [sum(abs(v) for v in r.values()) for r in rows]
    negs = [any(v < 0 for v in r.values()) for r in rows]
    if not signed and any(negs):
        return max(norms), True, "NONE fires soundly"
    n = max(norms)
    return n, any(negs), f"{n} * m^2 must fit  ->  " + (
        "componentwise" if n == 1 else f"{n}x componentwise"
    )


# what `112_probes/p5b_output.txt` MEASURED, transcribed from my own committed run
MEASURED = {
    ("uW3sat", "product2"): "componentwise",
    ("uW3sat", "dual"): "twice-componentwise",
    ("uW3sat", "complex"): "NONE of the three fires soundly",
    ("sW4sat", "product2"): "componentwise",
    ("sW4sat", "dual"): "twice-componentwise",
    ("sW4sat", "complex"): "twice-componentwise",
}


def norm(v):
    v = v.replace("2x componentwise", "twice-componentwise")
    if "NONE" in v:
        return "NONE"
    if "twice" in v:
        return "twice"
    if "componentwise" in v:
        return "once"
    return v


def main():
    print("=" * 84)
    print("p5. The formula against my own measured table")
    print("=" * 84)
    print()
    print(f"  {'construction':<14} {'signed?':<9} {'L1 norms':<12} {'neg row':<9} verdict")
    verdicts = set()
    rowsout = []
    for name in ("product2", "dual", "complex"):
        rows = CONSTR[name]
        norms = [sum(abs(v) for v in r.values()) for r in rows]
        for signed, tag in ((False, "unsigned"), (True, "signed")):
            n, neg, verdict = formula(rows, signed)
            verdicts.add(verdict)
            rowsout.append((name, tag, verdict))
            print(
                f"  {name:<14} {tag:<9} {str(norms):<12} {str(neg):<9} {verdict}"
            )

    print()
    print("CONDITION-CAN-FIRE CHECK")
    print(f"  distinct verdicts the formula produces: {len(verdicts)}")
    print(f"  ({'live' if len(verdicts) > 1 else 'VACUOUS, it says the same thing everywhere'})")

    print()
    print("AGAINST MY OWN MEASURED ROWS (from 112_probes/p5b_output.txt)")
    print()
    agree = disagree = 0
    for (base, name), measured in MEASURED.items():
        signed = base.startswith("s")
        _, _, verdict = formula(CONSTR[name], signed)
        ok = norm(verdict) == norm(measured)
        agree += int(ok)
        disagree += int(not ok)
        print(
            f"  {base:<8} {name:<10} formula: {norm(verdict):<6} "
            f"measured: {norm(measured):<6} {'agree' if ok else 'DISAGREE'}"
        )
    print()
    print(f"  {agree} agree, {disagree} disagree")

    print()
    print("AT DIMENSION FOUR, where nothing in the derivation mentions the dimension")
    print()
    q = CONSTR["quaternion_row0"]
    for signed, tag in ((True, "signed"), (False, "unsigned")):
        n, neg, verdict = formula(q, signed)
        print(f"  quaternion real part, {tag:<9}: L1 norm {n}, negative entry {neg} -> {verdict}")
    print()
    print("  which is `114` F114-13's row, arrived at from the constants rather")
    print("  than from a sweep, and it says the componentwise rule is unsound there.")


if __name__ == "__main__":
    main()
