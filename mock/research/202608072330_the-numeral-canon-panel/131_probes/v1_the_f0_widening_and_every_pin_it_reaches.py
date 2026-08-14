#!/usr/bin/env python3
"""v1. `125` F4's widening, checked, and then applied to every truncation pin in
the preceding topic.

WHY THIS ONE FIRST
------------------
`125` F4 states that for operations in `{+, -, x}` at `F = 0`, and for `{+, -}`
at every `F`, the realised operation is identical under every rounding mode, so
any finding predicated `rounding = trunc` over those operations widens to
`rounding any`. The dispatch adds `min` to the set and says this is the single
most consequential sentence in the topic, because it decides whether the
preceding topic's truncation pins were a real restriction.

`125` states it and its own probe checks it. It is carried by three later files
and re-derived by none of them. So it gets checked here, and then the half nobody
has done gets done: **applying it to the actual pins** rather than asserting that
it applies.

THE ARGUMENT, WHICH IS WHY THIS IS NOT ONLY A SWEEP
-----------------------------------------------------
A rounding mode is a retraction onto the grid: `Q(g) = g` for every `g` on the
grid. At `F = 0` the grid is the integers. Addition, subtraction, multiplication
and `min` of integers are integers. So the exact result is already on the grid,
`Q` is the identity on it, and every mode agrees. The same runs for `{+, -}` at
any `F`, because the grid is a group under addition at every quantum.

That argument mentions no width and no signedness, so it is one of the few claims
in either topic that carries past what a sweep covers. The sweep below is a
control on the argument rather than the evidence for it, and this probe says which
is which rather than letting a reader guess.

**Division is the residue and is the control.** It leaves the grid at `F = 0`, so
if the sweep reports division mode-invariant the instrument is broken.

PREDICTIONS, RECORDED BEFORE THE RUN
-------------------------------------
P1. At `F = 0`, ops in `{+, -, x, min}` give identical results under all six
    modes, exhaustively, at every width and signedness swept.
P2. At `F = 0`, division differs between `floor` and `toward_zero` on signed
    operands, and agrees on unsigned. That is the control, and it is also `125`
    F5 and F9 reproduced.
P3. At `F > 0`, multiplication differs across modes, which is the second control.
    Addition and subtraction still do not.
P4. `min` behaves exactly as `+` does here, so the dispatch's addition to the set
    is free rather than a widening that needs its own evidence.
P5. Of the preceding topic's truncation pins, the ones whose operations lie in
    the vacuous set at their own stated fraction width widen; the ones with
    multiplication at a nonzero fraction width do not. The counts are what this
    probe is for and I do not predict them.

NEGATIVE CONTROLS
-----------------
C1. P2 and P3 are the arithmetic controls.
C2. The pin extractor must find zero pins in a file with none and must not match
    a predicate that merely mentions rounding without pinning it.
C3. A deliberately broken mode, one that fails to fix the grid, must break P1, or
    the sweep is not testing the retraction property that the argument turns on.
"""

import re
import sys
from fractions import Fraction
from itertools import product
from pathlib import Path

PANEL = Path(__file__).parent.parent


# --------------------------------------------------------------- the six modes


def q_floor(x, F):
    s = Fraction(1, 2 ** F)
    import math
    return Fraction(math.floor(x / s)) * s


def q_ceil(x, F):
    s = Fraction(1, 2 ** F)
    import math
    return Fraction(math.ceil(x / s)) * s


def q_toward_zero(x, F):
    return q_floor(x, F) if x >= 0 else q_ceil(x, F)


def q_half_up(x, F):
    s = Fraction(1, 2 ** F)
    return q_floor(x + s / 2, F)


def q_half_even(x, F):
    s = Fraction(1, 2 ** F)
    lo = q_floor(x, F)
    d = x - lo
    if d < s / 2:
        return lo
    if d > s / 2:
        return lo + s
    k = lo / s
    return lo if int(k) % 2 == 0 else lo + s


def q_stochastic_up(x, F):
    """A realisation of the stochastic kernel: the draw that always rounds up
    off-grid. On-grid it must still be the identity, which is what makes it a
    member rather than an arbitrary map."""
    lo = q_floor(x, F)
    return lo if lo == x else q_ceil(x, F)


def q_broken(x, F):
    """C3: not a retraction. Adds a quantum to on-grid values."""
    s = Fraction(1, 2 ** F)
    lo = q_floor(x, F)
    return lo + s if lo == x else lo


MODES = {
    "floor": q_floor, "ceil": q_ceil, "toward_zero": q_toward_zero,
    "half_up": q_half_up, "half_even": q_half_even,
    "stochastic(up-draw)": q_stochastic_up,
}


def grid_values(W, signed, F):
    s = Fraction(1, 2 ** F)
    lo = -(1 << (W - 1)) if signed else 0
    hi = (1 << (W - 1)) - 1 if signed else (1 << W) - 1
    return [Fraction(k) * s for k in range(lo, hi + 1)]


OPS = {
    "add": lambda a, b: a + b,
    "sub": lambda a, b: a - b,
    "mul": lambda a, b: a * b,
    "min": lambda a, b: a if a <= b else b,
    "div": lambda a, b: a / b if b != 0 else None,
}


def sweep(W, signed, F, op, modes):
    vals = grid_values(W, signed, F)
    disagree = tot = 0
    witness = None
    for a, b in product(vals, repeat=2):
        e = OPS[op](a, b)
        if e is None:
            continue
        tot += 1
        outs = {name: fn(e, F) for name, fn in modes.items()}
        if len(set(outs.values())) > 1:
            disagree += 1
            if witness is None:
                # Name a PAIR that actually differs. The first version printed
                # floor and toward_zero unconditionally, which agree on unsigned
                # domains, so the witness column read as though nothing differed
                # on the very rows where the count said something did.
                pair = next((n1, n2) for n1 in outs for n2 in outs
                            if outs[n1] != outs[n2])
                witness = (a, b, e, pair, outs[pair[0]], outs[pair[1]])
    return disagree, tot, witness


# ------------------------------------------------------- the pin classification

PIN = re.compile(r"rounding\s*=\s*(trunc|truncation)\b")
VACUOUS_OPS = {"add", "sub", "mul", "min"}
ALWAYS_VACUOUS_OPS = {"add", "sub", "min"}


def predicates_in(text):
    """Every predicate block, however it is delimited in that file: a `holds
    for:` span or a backticked predicate inside a finding."""
    out = []
    for m in re.finditer(r"holds for:(.{0,900}?)(?:\*|```|\n\n)", text, re.S):
        out.append(re.sub(r"\s+", " ", m.group(1)).strip())
    for m in re.finditer(r"`(W\s*(?:in|=)[^`]{0,700}?)`", text, re.S):
        out.append(re.sub(r"\s+", " ", m.group(1)).strip())
    return out


def parse_F(p):
    m = re.search(r"\bF\s*(?:=|in)\s*(\{[^}]*\}|[0-9]+|any)", p)
    if not m:
        return None
    v = m.group(1)
    if v == "any":
        return "any"
    if v.startswith("{"):
        return {int(x) for x in re.findall(r"\d+", v)}
    return {int(v)}


def parse_ops(p):
    m = re.search(r"operations?\s*(?:in|=)\s*(\{[^}]*\}(?:\s*and\s*\{[^}]*\})?)", p)
    if not m:
        m2 = re.search(r"\boperation\s*=\s*([a-z_]+)", p)
        return {m2.group(1)} if m2 else None
    # A predicate listing two operation sets joins them with "and", and the
    # first version of this read that conjunction as an operation named `and`,
    # which pushed three pins into "does not widen" that in fact widen. The
    # defect is recorded here rather than silently repaired.
    return set(re.findall(r"[a-z_]+", m.group(1))) - {"and"}


def classify(p):
    F = parse_F(p)
    ops = parse_ops(p)
    if ops is None:
        return "unclassifiable: no operation set named", F, ops
    if ops <= ALWAYS_VACUOUS_OPS:
        return "WIDENS at any F", F, ops
    if F is None:
        return "unclassifiable: no fraction width named", F, ops
    if F == "any":
        return "does not widen: F any with a rounding operation present", F, ops
    if ops <= VACUOUS_OPS and F == {0}:
        return "WIDENS at F = 0", F, ops
    return "does not widen", F, ops


def main():
    print("=" * 96)
    print("v1. The F = 0 widening, checked, and every pin it reaches")
    print("=" * 96)

    # ------------------------------------------------------------- P1, P4, C3
    print()
    print("P1 and P4. The vacuous set at F = 0, all six modes, exhaustive.")
    print()
    print(f"  {'W':>3} {'signedness':<10} {'F':>2} {'op':<5} {'pairs':>7} "
          f"{'modes disagree on':>18}")
    for W, signed in ((3, False), (3, True), (4, False), (4, True), (5, True)):
        for op in ("add", "sub", "mul", "min"):
            d, t, _ = sweep(W, signed, 0, op, MODES)
            print(f"  {W:>3} {'signed' if signed else 'unsigned':<10} {0:>2} "
                  f"{op:<5} {t:>7} {d:>18}")

    print()
    print("C3. The same sweep with a mode that is not a retraction, which must")
    print("    break it or the sweep is not testing the property the argument uses.")
    print()
    broken = dict(MODES)
    broken["broken(not a retraction)"] = q_broken
    for W, signed in ((3, True), (4, False)):
        for op in ("add", "mul"):
            d, t, w = sweep(W, signed, 0, op, broken)
            print(f"  {W:>3} {'signed' if signed else 'unsigned':<10} {0:>2} "
                  f"{op:<5} {t:>7} {d:>18}")

    # ------------------------------------------------------------- P2, P3, C1
    print()
    print("-" * 96)
    print("P2 and P3. The controls: where rounding does fire.")
    print()
    print(f"  {'W':>3} {'signedness':<10} {'F':>2} {'op':<5} {'pairs':>7} "
          f"{'modes disagree on':>18}   witness")
    for W, signed, F, op in ((4, True, 0, "div"), (4, False, 0, "div"),
                             (3, True, 2, "mul"), (3, True, 2, "add"),
                             (3, True, 2, "sub"), (3, True, 2, "min")):
        d, t, w = sweep(W, signed, F, op, MODES)
        wit = ""
        if w:
            a, b, e, (n1, n2), v1, v2 = w
            wit = f"{a} {op} {b} = {e}: {n1} {v1} vs {n2} {v2}"
        print(f"  {W:>3} {'signed' if signed else 'unsigned':<10} {F:>2} "
              f"{op:<5} {t:>7} {d:>18}   {wit}")

    # ------------------------------------------------------------- P5 and C2
    print()
    print("-" * 96)
    print("P5. Every truncation pin in the preceding topic, classified.")
    print()
    files = sorted(
        [p for p in PANEL.glob("1[0-2][0-9]_*.md")] + [p for p in PANEL.glob("9*_*.md")]
    )
    totals = {}
    rows = []
    for f in files:
        text = f.read_text()
        for p in predicates_in(text):
            if not PIN.search(p):
                continue
            verdict, F, ops = classify(p)
            totals[verdict] = totals.get(verdict, 0) + 1
            rows.append((f.name.split("_")[0], verdict, F, sorted(ops) if ops else None,
                         p[:70]))
    print(f"  {'file':<5} {'verdict':<52} {'F':<12} {'ops'}")
    for name, verdict, F, ops, _ in rows:
        fs = ("any" if F == "any" else (str(sorted(F)) if F else "-"))
        print(f"  {name:<5} {verdict:<52} {fs:<12} {ops}")
    print()
    print(f"  pins found: {len(rows)}")
    for k in sorted(totals):
        print(f"    {k:<52} {totals[k]}")

    print()
    print("C2. The extractor on text with no pin, and on a predicate that mentions")
    print("    rounding without pinning it.")
    print()
    for probe, label in (
        ("W = 3, F = 0, signedness any, threads = 1", "no rounding at all"),
        ("W = 3, F = 0, rounding any, operations in {add}", "rounding any"),
        ("W = 3, F = 0, rounding = trunc, operations in {add, sub}", "a real pin"),
    ):
        print(f"    {label:<28} -> {'pin' if PIN.search(probe) else 'no pin'}"
              f"   {classify(probe)[0] if PIN.search(probe) else ''}")

    print()
    print("=" * 96)
    print(
        """
  READING IT

  P1 holds when every F = 0 row in the first table reads zero disagreements, and
  C3's rows must not, or the sweep is insensitive to the retraction property the
  argument turns on.

  The first table is a CONTROL ON AN ARGUMENT, not the evidence for it. The
  argument is that the grid is closed under those operations so the mode never
  fires, and it mentions no width and no signedness, which is why the claim
  carries past the widths swept while nothing else in this topic does.

  P5's counts decide whether the preceding topic's truncation pins were a real
  restriction. A pin classified 'does not widen' is one where rounding genuinely
  acts and the predicate was right to name it.
"""
    )


if __name__ == "__main__":
    sys.setrecursionlimit(10000)
    main()
