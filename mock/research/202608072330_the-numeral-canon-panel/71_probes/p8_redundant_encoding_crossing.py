#!/usr/bin/env python3
"""
p8. Can a crossing be total on values and NOT A FUNCTION on patterns?

This is the item p1 through p7 left open, and it is the one gap in the "one relation or several"
answer that could be closed cheaply, so leaving it as a suggestion for someone else would have
been the wrong output.

Every crossing measured so far has a pattern map that is a function: an identity, a bijection, a
shift, a sign-extension. `63` section 3.5 records that "Redundant encodings are wholly
unexamined", and a redundant encoding is exactly the case where one value has several patterns.

HYPOTHESIS: at telescope index 4, when the target encoding is redundant, the crossing INTO it is
a relation rather than a function (one value, several patterns), the crossing BACK is a function
and is many-to-one, and the pair is a section and a retraction rather than a bijection. If that
holds, the panel's crossing vocabulary needs the section-retraction shape it already adopted once
for a different crossing (`DROPLIST.md:106-108`, where two round-trip identities were replaced by
"the section-retraction triple"), and the value relation and the pattern relation are independent
in a stronger sense than p1 showed: not merely different maps, but different KINDS of thing.

SECOND HYPOTHESIS, about the pattern-level operation: a redundant encoding's own container
operation is not closed in its digit set, which is why redundant arithmetic needs a wider
intermediate. Predicted to be a small minority of digit-string pairs.

Model: Q is the signed four-bit window [-8, 7]. Encoding A is two's complement, a bijection.
Encoding B is signed-digit binary over four digits from {-1, 0, 1}, value = sum d_i * 2^i.
Exhaustive over all 81 digit strings and all 6561 ordered pairs of them.
"""

from itertools import product

DIGITS = (-1, 0, 1)
N = 4
Q = list(range(-8, 8))


def value(s):
    return sum(d << i for i, d in enumerate(s))


STRINGS = list(product(DIGITS, repeat=N))
VALUES = sorted({value(s) for s in STRINGS})

by_value = {}
for s in STRINGS:
    by_value.setdefault(value(s), []).append(s)

print("=" * 88)
print("THE REDUNDANT ENCODING ITSELF")
print("=" * 88)
print(f"  digit strings                     {len(STRINGS)}")
print(f"  distinct values                   {len(VALUES)}  (range {VALUES[0]} to {VALUES[-1]})")
mult = {v: len(ss) for v, ss in by_value.items()}
print(f"  values with more than one string  {sum(1 for v in mult if mult[v] > 1)}")
print(f"  maximum multiplicity              {max(mult.values())}  at value "
      f"{sorted(v for v in mult if mult[v] == max(mult.values()))}")
print(f"  `66` reports 81 strings onto 31 values for this shape; reproduced: "
      f"{len(STRINGS)} onto {len(VALUES)}")

# ------------------------------------------------------------- the crossing into the encoding

print()
print("=" * 88)
print("THE CROSSING INTO IT, index 4, from a bijective encoding of Q")
print("=" * 88)
in_q = [v for v in Q]
images = {v: by_value[v] for v in in_q}
total_images = sum(len(images[v]) for v in in_q)
one_image = sum(1 for v in in_q if len(images[v]) == 1)
print(f"  source values (all of Q)              {len(in_q)}")
print(f"  target patterns they reach            {total_images}")
print(f"  source values with exactly one image  {one_image} of {len(in_q)}")
print(f"  so the pattern relation is a function: {one_image == len(in_q)}")
print(f"  multiplicity by value: "
      f"{ {v: len(images[v]) for v in in_q} }")

# ------------------------------------------------------------- the crossing back out of it

print()
print("=" * 88)
print("THE CROSSING BACK, index 4, into the bijective encoding")
print("=" * 88)
outside = [s for s in STRINGS if value(s) not in Q]
print(f"  digit strings whose value lies outside Q   {len(outside)} of {len(STRINGS)}")
print("  so the crossing back is PARTIAL: it is a function exactly on the preimage of Q,")
print("  and on the rest it needs a reduction, which is a coordinate this crossing does not move")

retraction_holds = all(value(s) == v for v in in_q for s in images[v])
print(f"  retraction property, every image of v decodes to v: {retraction_holds}")
print(f"    checked over {total_images} strings, exhaustive")


def canonical(v):
    """A section: fewest nonzero digits, ties broken lexicographically. One choice among many,
    and the point of the probe is that a choice has to be made at all."""
    return min(images[v], key=lambda s: (sum(1 for d in s if d != 0), s))


section = {v: canonical(v) for v in in_q}
fixed = sum(1 for s in STRINGS if value(s) in Q and section[value(s)] == s)
reachable = sum(1 for s in STRINGS if value(s) in Q)
print(f"  section then retraction is the identity on Q:   {all(value(section[v]) == v for v in in_q)}")
print(f"  retraction then section is the identity on the encoding: "
      f"{fixed} of {reachable} strings fixed")
print("  So the two crossings are a SECTION and a RETRACTION, not a bijection, which is the")
print("  shape `DROPLIST.md:106-108` already records the panel adopting for the encode and")
print("  decode crossing after two round-trip identities were refuted.")

# --------------------------------------------------- the pattern-level operation in the target

print()
print("=" * 88)
print("THE TARGET'S OWN PATTERN-LEVEL OPERATION")
print("=" * 88)
closed = 0
total_pairs = 0
for a in STRINGS:
    for b in STRINGS:
        total_pairs += 1
        s = tuple(x + y for x, y in zip(a, b))
        if all(d in DIGITS for d in s):
            closed += 1
print(f"  ordered pairs of digit strings                     {total_pairs}")
print(f"  pairs whose digitwise sum stays in the digit set   {closed}")
print(f"  fraction closed                                    {closed / total_pairs:.4f}")
print("  A redundant encoding's container operation is not closed in its own digit set, so it")
print("  has no pattern-level operation to preserve without a wider intermediate. That is the")
print("  mechanism behind carry-save rather than a defect of this model.")

# ------------------------------------------------------------------------------- the verdict

print()
print("=" * 88)
print("VERDICT")
print("=" * 88)
h1 = one_image != len(in_q) and retraction_holds and fixed < reachable
h2 = closed < total_pairs
print(f"  HYPOTHESIS ONE (into is a relation, back is a function, the pair is a "
      f"section-retraction): {'HELD' if h1 else 'REFUTED'}")
print(f"  HYPOTHESIS TWO (the target's container operation is not closed): "
      f"{'HELD' if h2 else 'REFUTED'}")
print()
print("  What this adds to the crossing question. p1 showed the value relation and the pattern")
print("  relation are different MAPS. This shows they can be different KINDS: the value relation")
print("  is a total function and the pattern relation is a relation with no canonical function")
print("  inside it, so a crossing into a redundant encoding is only a function once a section is")
print("  named, and naming one is a choice the two systems do not make between them. That is the")
print("  same shape as the choice p2 found for a lossy crossing, at a different coordinate:")
print("  the crossing needs something neither endpoint supplies.")
print()
print("  And it bounds a canon sentence. 'A crossing preserves values' is true here and says")
print("  nothing about whether the crossing is well defined, because the pattern relation is")
print("  where the ambiguity lives and the value relation cannot see it.")
