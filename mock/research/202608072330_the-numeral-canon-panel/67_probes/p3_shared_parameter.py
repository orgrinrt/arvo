#!/usr/bin/env python3
"""p3. Two shapes of shared parameter over a run, separated by a decidable test.

The panel files stored-pair rationals, intervals and error-carrying pairs as
"compositions over formats" (`63` section 3.6). Every one of those is a POINT
composition: one datum is a tuple of numerals, and the tuple's parts travel
together.

There is a second shape nobody has named: many data sharing ONE parameter that
is stored once for the whole run. Block floating point is the textbook case. So,
this probe argues, is the panel's own `stride`, which is a parameter of a run
rather than of a value (`OPTIONS.md`, the derivation-outputs section: "`Cold` is
not a container choice with a field attached; it is a statement about how a run
of values composes ... a lone `Cold` value has the identical carrier to `Warm`").

The two are not the same kind of thing, and this probe supplies the test that
separates them, taken verbatim from the panel's own identity condition:

    is the representable set a CONSTANT of the element type?

`63`'s C2: "The representable set is a constant of the type: a value set that
depends on other data is not a format but storage."

Applied mechanically:

  - A block-floating-point element FAILS it. The value set moves with the block
    exponent, so the element type has no Q.
  - A packed-run element PASSES it. The value set is identical at every stride;
    only where the bits sit moves.

So the shared parameter sits at a different depth in the two cases: at the
identity layer for BFP, at the realisation layer for stride. That is one test,
one answer, and it makes "shared parameter over a run" a two-member category
rather than an exotic special case.

Exhaustive over the enumerated model parameters. No sampling.

Run: python3 p3_shared_parameter.py
"""

from fractions import Fraction

out = []


def say(s=""):
    out.append(s)
    print(s)


say("p3. shared parameters over a run: two shapes, one decidable test")
say("=" * 72)
say()
say('TEST (from `63` C2): "the representable set is a constant of the type: a value')
say(' set that depends on other data is not a format but storage."')

# ---------------------------------------------------------------------------
# Case A: block floating point. 4-bit unsigned mantissa, one 3-bit exponent
# shared by the whole block. Element value = m * 2^(e - 3).
say()
say("CASE A: block floating point. 4-bit mantissa per element, one 3-bit")
say("        exponent shared by the block. value(m) = m * 2^(e-3).")
say()

mantissas = list(range(16))
exponents = list(range(8))
sets_a = {}
for e in exponents:
    q = frozenset(Fraction(m) * Fraction(2) ** (e - 3) for m in mantissas)
    sets_a[e] = q

distinct_a = len(set(sets_a.values()))
say(f"  block exponents enumerated: {len(exponents)}")
say(f"  distinct representable sets over the element type: {distinct_a}")
for e in exponents:
    lo, hi = min(sets_a[e]), max(sets_a[e])
    say(f"    e={e}: |Q|={len(sets_a[e]):2d}  range [{lo}, {hi}]  step {Fraction(2) ** (e - 3)}")
say()
say(f"  Q is a constant of the element type: {distinct_a == 1}")
say("  VERDICT: FAILS the test. A BFP element has no Q of its own, so it is not")
say("  a format under the panel's own identity condition. It is not 'storage'")
say("  either, in any sense the panel has developed: the parameter it depends on")
say("  is arithmetic-bearing, not a layout fact.")

# ---------------------------------------------------------------------------
# Case B: a packed run. 4-bit unsigned elements, stride varying.
say()
say("CASE B: a packed run. 4-bit unsigned elements, stride varying over")
say("        {4, 5, 8, 16} bits (the shared parameter is where the next element")
say("        starts, not what any element means).")
say()

strides = [4, 5, 8, 16]
sets_b = {}
for s in strides:
    # the element's value set does not consult the stride at all; the stride only
    # decides the bit offset of element i, which is modelled here explicitly so
    # the independence is exhibited rather than asserted.
    q = frozenset(Fraction(m) for m in range(16))
    offsets = [(i * s) % 8 for i in range(8)]
    sets_b[s] = (q, offsets)

distinct_b = len({q for q, _ in sets_b.values()})
say(f"  strides enumerated: {len(strides)}")
say(f"  distinct representable sets over the element type: {distinct_b}")
for s in strides:
    q, offs = sets_b[s]
    say(f"    stride={s:2d}: |Q|={len(q):2d}  range [{min(q)}, {max(q)}]  "
        f"bit offsets of the first eight elements {offs}")
say()
say(f"  Q is a constant of the element type: {distinct_b == 1}")
say("  VERDICT: PASSES the test. The shared parameter moves the bit offsets and")
say("  moves no value. It is a realisation-layer shared parameter.")

# ---------------------------------------------------------------------------
# Case C: the control. A self-contained float of the same total budget, where
# the exponent travels WITH each element. It must pass, or the test is measuring
# 'has an exponent' rather than 'the parameter is shared'.
say()
say("CASE C (control): a self-contained float, 4-bit mantissa AND 3-bit exponent")
say("        per element. Same arithmetic as case A, exponent not shared.")
say()

q_c = frozenset(Fraction(m) * Fraction(2) ** (e - 3)
                for m in mantissas for e in exponents)
say(f"  distinct representable sets over the element type: 1 (by construction, "
    f"the type has no free parameter left)")
say(f"  |Q| = {len(q_c)}  range [{min(q_c)}, {max(q_c)}]")
say("  VERDICT: PASSES. So the test separates SHARED from PER-ELEMENT, not")
say("  'has a scale factor' from 'does not'. Cases A and C encode the same")
say("  arithmetic and land on opposite sides.")

# ---------------------------------------------------------------------------
say()
say("=" * 72)
say("SUMMARY")
say(f"  A (block floating point):  Q constant = {distinct_a == 1}  -> not a format")
say(f"  B (packed run, stride):    Q constant = {distinct_b == 1}   -> a format, shared")
say("                                                     parameter at realisation")
say("  C (self-contained float):  Q constant = True   -> a format")
say()
say("  So arvo already ships one shared-parameter aggregate (`Cold`'s run) and")
say("  excludes another (block floating point) with the same sentence, and the")
say("  sentence does not distinguish them: it distinguishes WHICH LAYER the")
say("  shared parameter sits at. A concept with one word for 'composition' cannot")
say("  say that, because point compositions and shared-parameter aggregates differ")
say("  in exactly this.")
say()
say("  What this does not establish: whether BFP should be admitted, what it")
say("  would cost, or anything about magnitudes. It establishes only that the")
say("  panel's own test files it outside `format` and that `Cold` is inside, and")
say("  that the two are the same structural shape at two different depths.")

with open("p3_shared_parameter.out", "w") as f:
    f.write("\n".join(out) + "\n")
