#!/usr/bin/env python3
"""p2. What does a crossing between two numerals actually preserve?

The dispatch asks what two systems must expose to interoperate, and whether that
interoperation is conversion, embedding, or something with no good name yet. This
probe answers the prior question: WHICH crossings exist, and what each preserves.

Three crossings are distinguished by which component of the telescope moves:

  X1 restrategise.  Same ambient domain, same representable set, same encoding,
                    same container. Only the adaptation moves.
  X2 widen.         Same ambient domain, representable set grows. The value map
                    is the inclusion.
  X3 reinterpret.   Same container, same bit patterns, DIFFERENT ambient domain.
                    Nothing about the storage moves at all.

For each, two questions are asked separately and never conflated:

  (a) is the map a bijection / injection ON VALUES (or on patterns, for X3)?
  (b) does the map COMMUTE WITH THE OPERATIONS?

The panel's vocabulary has one word, "conversion", covering (a), and no word at
all for the failure of (b). That is the gap this probe measures.

Exhaustive at the 4-bit model width. No sampling anywhere.

Run: python3 p2_three_crossings.py
"""

LO_S, HI_S = -8, 7
LO_U, HI_U = 0, 15
Q_S = list(range(LO_S, HI_S + 1))
Q_U = list(range(LO_U, HI_U + 1))


def wrap(lo, hi):
    n = hi - lo + 1
    return lambda x: (x - lo) % n + lo


def sat(lo, hi):
    return lambda x: lo if x < lo else (hi if x > hi else x)


wrap_s, sat_s = wrap(LO_S, HI_S), sat(LO_S, HI_S)
wrap_u, sat_u = wrap(LO_U, HI_U), sat(LO_U, HI_U)

# 8-bit signed, the widening target for X2
LO_S8, HI_S8 = -128, 127
wrap_s8, sat_s8 = wrap(LO_S8, HI_S8), sat(LO_S8, HI_S8)

out = []


def say(s=""):
    out.append(s)
    print(s)


say("p2. three crossings, exhaustive at the 4-bit model width")
say("=" * 72)

# ---------------------------------------------------------------------------
say()
say("X1 RESTRATEGISE: same (ambient, representable set, encoding, container);")
say("   only the adaptation moves. wrap <-> saturate on Q = [-8, 7].")
say()

# (a) on values
val_pres = sum(1 for v in Q_S if v == v)
say(f"  (a) value map is the identity on Q: {val_pres}/{len(Q_S)} preserved, "
    f"bijective by construction.")

# (b) on operations
for opname, op in (("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)):
    agree = sum(1 for a in Q_S for b in Q_S if wrap_s(op(a, b)) == sat_s(op(a, b)))
    tot = len(Q_S) ** 2
    say(f"  (b) {opname}: the two adaptations agree on {agree}/{tot} operand pairs "
        f"({100.0 * agree / tot:.1f}%)")

# and the same on the unsigned window, where the panel's law cube is friendlier
for opname, op in (("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)):
    agree = sum(1 for a in Q_U for b in Q_U if wrap_u(op(a, b)) == sat_u(op(a, b)))
    tot = len(Q_U) ** 2
    say(f"  (b) {opname} on Q = [0, 15]: agree on {agree}/{tot} "
        f"({100.0 * agree / tot:.1f}%)")

say()
say("  So X1 is TOTAL and value-preserving and is not operation-preserving.")
say("  A crossing that is free on values and not free on operations has no name")
say("  in the panel's vocabulary, and it is the one that looks free.")

# ---------------------------------------------------------------------------
say()
say("X2 WIDEN: same ambient, Q grows from [-8, 7] to [-128, 127].")
say("   The value map is the inclusion.")
say()

inj = len(set(Q_S))
say(f"  (a) inclusion is injective on values: {inj}/{len(Q_S)} distinct images.")

for pol, small, big in (("wrap", wrap_s, wrap_s8), ("saturate", sat_s, sat_s8)):
    for opname, op in (("add", lambda a, b: a + b), ("mul", lambda a, b: a * b)):
        agree = sum(1 for a in Q_S for b in Q_S if small(op(a, b)) == big(op(a, b)))
        tot = len(Q_S) ** 2
        say(f"  (b) {pol:8s} {opname}: emb(op_small(a,b)) == op_big(emb a, emb b) "
            f"on {agree}/{tot} ({100.0 * agree / tot:.1f}%)")

say()
say("  So X2 is injective on values and is a homomorphism only where the small")
say("  numeral's adaptation did not fire. Widening is not free either; it is free")
say("  exactly on the sub-box where no adaptation occurs, which is a checkable")
say("  condition and is what a lossless-conversion predicate is really about.")

# ---------------------------------------------------------------------------
say()
say("X3 REINTERPRET: same container, same 4-bit patterns, different ambient.")
say("   (Z, +) with wrapping, against GF(2)^4 with xor, against (Z, min).")
say()

pats = list(range(16))
say(f"  (a) the pattern map is the identity: {len(pats)}/{len(pats)} patterns "
    f"carried, bijective by construction. Nothing in storage moves.")

agree_xor = sum(1 for a in pats for b in pats if wrap_u(a + b) == (a ^ b))
say(f"  (b) wrapping add vs xor agree on {agree_xor}/{len(pats) ** 2} "
    f"({100.0 * agree_xor / len(pats) ** 2:.1f}%)")

# A prediction written into this file before it was run, kept with its refutation
# per the panel's probe discipline: the agreement set is the carry-free pairs
# (a & b == 0). REFUTED by this probe's own output.
carryfree = sum(1 for a in pats for b in pats if a & b == 0)
say(f"      prediction: the agreement set is the carry-free pairs (a & b == 0), "
    f"which number {carryfree}.")
say(f"      REFUTED: {agree_xor} != {carryfree}. Wrapping discards the carry OUT of")
say(f"      the container, so a shared top bit also agrees. Corrected closed form:")
closed = sum(1 for a in pats for b in pats if (a & b) in (0, 8))
same = {(a, b) for a in pats for b in pats if wrap_u(a + b) == (a ^ b)} == \
       {(a, b) for a in pats for b in pats if (a & b) in (0, 8)}
say(f"      agreement iff (a & b) is 0 or the top bit alone: count {closed}, "
    f"set-equal to the measured agreement: {same}")

agree_min = sum(1 for a in pats for b in pats if wrap_u(a + b) == min(a, b))
say(f"  (b) wrapping add vs min agree on {agree_min}/{len(pats) ** 2} "
    f"({100.0 * agree_min / len(pats) ** 2:.1f}%)")

say()
say("  So X3 preserves every pattern and preserves nothing else. The container is")
say("  identical, the value map is the identity, and the arithmetic is unrelated.")

# ---------------------------------------------------------------------------
say()
say("=" * 72)
say("SUMMARY")
say("  every crossing above is total and value- or pattern-preserving at 100%.")
say("  no crossing above is operation-preserving at 100%.")
say("  the two properties are independent, and a canon that has one word for")
say("  'conversion' is naming (a) and silent about (b).")
say()
say("  What this does not establish: that these are the only three crossings, or")
say("  anything at all about magnitudes. One model width, exhaustive within it.")

with open("p2_three_crossings.out", "w") as f:
    f.write("\n".join(out) + "\n")
