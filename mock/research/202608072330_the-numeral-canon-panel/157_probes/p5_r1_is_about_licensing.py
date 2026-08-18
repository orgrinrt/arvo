#!/usr/bin/env python3
"""
157 P5. What 111 section 18.2's zero measures.

CLAIM UNDER TEST
  111:1155-1178 answers OPTIONS.md Q52's open item by applying 108:825's criterion ("an
  axis belongs to the declared semantics if there is any reachable chain on which moving
  it is observable"). Its instrument r1 holds the primitive at uW3/sat, moves only the
  declared bound, and reports 1753 declaration pairs changing the selected arm and 0
  chains changing the answer, over six terms.

  The conclusion is a UNIVERSAL NEGATIVE (no reachable chain observes it). The instrument
  is a sweep over six terms at one primitive at one declaration shape. This probe tests
  whether the zero is an empirical fact about those six terms or is ENTAILED by the arm
  licence condition, in which case no sweep at any size establishes the universal and the
  sweep is measuring something else.

NEGATIVE CONTROLS, stated before the run
  E1. An UNSOUND licence must produce disagreements. If the harness reports zero under a
      licence that is wrong by construction, the harness cannot detect a difference and
      its zero under the sound licence proves nothing.
  E2. Moving an OBSERVABLE axis (the overflow policy) with declarations fixed must produce
      disagreements, reproducing 111's own control.
  E3. The zero under the sound licence must hold at widths beyond the model width. If it
      is a model-width artifact it will break at W = 16 or 32.
"""
import random

def V(W):           return (0, (1 << W) - 1)
def sat(x, W):
    lo, hi = V(W)
    return max(lo, min(hi, x))
def wrap(x, W):
    lo, hi = V(W)
    return (x - lo) % (hi - lo + 1) + lo

def general_arm(a, b, W, policy, op):
    e = a + b if op == "add" else (a * b if op == "mul" else a - b)
    return sat(e, W) if policy == "sat" else wrap(e, W)

def cheap_arm(a, b, W, op):
    """No projection at all. Licensed only where the declaration proves it cannot fire."""
    return a + b if op == "add" else (a * b if op == "mul" else a - b)

def licence_sound(bound, W, op):
    """The declaration [0, bound] proves the cheap arm agrees: the worst exact result
    over the declared box lands inside V. This is the arm's own licence condition."""
    lo, hi = V(W)
    worst = bound + bound if op == "add" else (bound * bound if op == "mul" else 0 - bound)
    return lo <= worst <= hi

def licence_unsound(bound, W, op):
    """CONTROL E1: an off-by-one licence. It admits exactly one bound beyond the sound
    frontier, so it fires where the cheap arm does NOT agree with the general one.

    The FIRST version of this control returned `lo <= bound <= hi`, which is true for
    every bound in the value set, so it selected the cheap arm for every declaration,
    both sides of every pair agreed trivially and the control reported zero. That is the
    same defect this panel has recorded four times (110 P4, 110 P8 first run, 111 s9.4,
    154 P4): an instrument fed only the configuration where it cannot disagree. It is
    kept in the file rather than edited away, and its output is committed at
    p5_output_E1_control_failed.txt."""
    lo, hi = V(W)
    worst = bound + bound if op == "add" else (bound * bound if op == "mul" else 0 - bound)
    return lo <= worst <= hi + 1 if op == "add" else lo <= worst <= hi + bound

def select(bound, W, policy, op, licence):
    return "cheap" if licence(bound, W, op) else "general"

def run(W, policy, ops, licence, sample=None, label="", bound_cap=None):
    lo, hi = V(W)
    if bound_cap is not None and (hi - lo + 1) > bound_cap:
        bounds = sorted({random.randint(lo, hi) for _ in range(bound_cap * 2)})[:bound_cap]
        bounds = sorted(set(bounds) | {lo, hi})
    else:
        bounds = list(range(lo, hi + 1))
    arm_changes = 0
    answer_changes = 0
    pairs = 0
    for i, b1 in enumerate(bounds):
        for b2 in bounds[i + 1:]:
            tight, loose = (b1, b2) if b1 <= b2 else (b2, b1)
            a1 = select(tight, W, policy, ops, licence)
            a2 = select(loose, W, policy, ops, licence)
            pairs += 1
            if a1 != a2:
                arm_changes += 1
            # values admitted by the TIGHTER declaration, so both declarations are
            # discharged by every value tested. this is the weakening direction.
            dom = range(lo, tight + 1)
            if sample is not None and (tight - lo + 1) ** 2 > sample:
                for _ in range(sample):
                    a = random.randint(lo, tight); b = random.randint(lo, tight)
                    r1 = cheap_arm(a, b, W, ops) if a1 == "cheap" else general_arm(a, b, W, policy, ops)
                    r2 = cheap_arm(a, b, W, ops) if a2 == "cheap" else general_arm(a, b, W, policy, ops)
                    if r1 != r2:
                        answer_changes += 1
                        break
            else:
                bad = False
                for a in dom:
                    for b in dom:
                        r1 = cheap_arm(a, b, W, ops) if a1 == "cheap" else general_arm(a, b, W, policy, ops)
                        r2 = cheap_arm(a, b, W, ops) if a2 == "cheap" else general_arm(a, b, W, policy, ops)
                        if r1 != r2:
                            bad = True
                            break
                    if bad:
                        break
                if bad:
                    answer_changes += 1
    print(f"  {label:<46} pairs={pairs:<8} arm changed={arm_changes:<8} answer changed={answer_changes}")
    return answer_changes

random.seed(20260818)
print("SOUND LICENCE. Only the declared bound moves; the primitive is fixed.")
tot = 0
for W in (3, 4, 5):
    for op in ("add", "mul"):
        tot += run(W, "sat", op, licence_sound, None, f"W={W} sat {op} exhaustive")
for W in (8, 12, 16, 24, 32):
    for op in ("add", "mul"):
        tot += run(W, "sat", op, licence_sound, 400,
                   f"W={W} sat {op} 64 bounds x 400 values/pair", bound_cap=64)
print(f"  E3 (zero survives past the model width) : {'PASS' if tot == 0 else 'FAIL'}   total={tot}")
print()

print("CONTROL E1. The same sweep under a licence that is unsound by construction.")
bad = 0
for W in (3, 4, 5):
    for op in ("add", "mul"):
        bad += run(W, "sat", op, licence_unsound, None, f"W={W} sat {op} exhaustive")
print(f"  E1 (an unsound licence is detected)     : {'PASS' if bad > 0 else 'FAIL'}   total={bad}")
print()

print("CONTROL E2. Move an observable axis instead, declarations held fixed.")
diff = 0
tried = 0
for W in (3, 4, 5):
    lo, hi = V(W)
    for a in range(lo, hi + 1):
        for b in range(lo, hi + 1):
            tried += 1
            if general_arm(a, b, W, "sat", "add") != general_arm(a, b, W, "wrap", "add"):
                diff += 1
print(f"  overflow policy moved: {diff} of {tried} argument pairs disagree")
print(f"  E2 (the instrument can report a difference) : {'PASS' if diff > 0 else 'FAIL'}")
print()
print("READING. The sound-licence zero is not an empirical fact about six terms. The arm")
print("is substituted only where its licence proves it agrees with the general arm on the")
print("declared box, and every value tested lies in that box, so agreement is the licence")
print("condition restated. The sweep therefore measures LICENCE SOUNDNESS, which E1 shows")
print("it can fail, and the axis classification follows from the definition of a")
print("refinement rather than from the count.")
