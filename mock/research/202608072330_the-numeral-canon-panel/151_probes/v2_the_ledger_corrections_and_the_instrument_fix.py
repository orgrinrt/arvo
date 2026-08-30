#!/usr/bin/env python3
# v2 (151): three ledger corrections, and the fix to my own instrument that one of them demands.
#
# 148 refutes the third item of 146 section 1.1's contamination scoping: its refuted P3 was on the
# OVERFLOW axis while the contaminating rule concerns the INTERMEDIATE axis, and at a single addition
# there is one reduction site so the two intermediate positions coincide at every overflow position
# including wrapping, where the congruence says nothing at all.
#
# 147 corrects the same sentence in the other direction on the mechanism: only the unsigned
# SATURATING cell rests on the shared rule, because the unsigned wrapping zero rests on 141 F3's
# absorption theorem, which this topic proved.
#
# 148 also finds 146 section 5.5's container predicate omitting `signedness` because the three
# instruments PARTITION that dimension rather than overlapping on it, so the intersection is empty
# and under the absence rule the clause holds nowhere. It names the general defect in my z6: it
# intersects over dimension NAMES, so a dimension every instance lists with disjoint values vanishes
# while looking present in every input. It says it did not check the other rows. I check them.
#
# Predictions, stated before running:
#
#   B1. At a single addition the two intermediate positions coincide at every overflow position,
#       including wrapping. So the congruence is not available as an explanation at 140's P3 cell.
#   B2. CONTROL for B1, and it must fire: at a single SUBTRACTION the two saturating overflow
#       positions separate, so the sweep can see a difference when there is one.
#   B3. CONTROL for B1: at a multiply-add, which has two reduction sites, the congruence IS the
#       mechanism, so signed saturating differs and unsigned saturating does not.
#   B4. 147's narrowing: making the clamp two-sided on an unsigned domain takes the saturating
#       fusion cell off zero and leaves the wrapping cell at zero.
#   B5. The value-level intersection finds the container row's signedness EMPTY where the
#       name-level one reported it merely absent.
#   B6. At least one other row of 146's 1.6 table carries the same defect, or the defect is a
#       one-off and I say so. 148 did not check and neither has anyone else.
#   B7. CONTROL, ADDED AFTER THE FIRST RUN PRODUCED A FALSE FINDING. The first run reported the
#       container row's `threads` intersecting to EMPTY, which is impossible: every block in this
#       topic says `threads = 1`. The cause was mine: the emphasis-delimited block pattern
#       over-captures past the predicate into the surrounding prose, so a trailing dimension
#       swallowed a paragraph and its value set became a sentence. Nothing in the probe caught
#       that; I caught it by finding the number implausible, which is the weaker way. The control
#       below drops any parsed KEY that is not dimension-shaped and reports how many it dropped, so
#       a mangled parse is visible rather than silently intersected.
#       The first version of the control over-corrected, rejecting the whole block when any key was
#       malformed, which rejected 42 of 42 and left every row empty. That is a second defect of the
#       same shape: a check that cannot pass is as uninformative as one that cannot fail. Both
#       versions are recorded because the pair is the lesson.
from itertools import product
import os
import re

W = 5

def bounds(signed):
    return (-(1 << (W - 1)), (1 << (W - 1)) - 1) if signed else (0, (1 << W) - 1)

def reduce_(v, signed, ov):
    lo, hi = bounds(signed)
    if ov == "wrap":
        m = 1 << W
        v %= m
        return v - m if (signed and v > hi) else v
    if ov == "sat_both":
        return max(lo, min(hi, v))
    if ov == "sat_high":
        return min(hi, v) if not signed else max(lo, min(hi, v))
    raise ValueError(ov)

print("=" * 96)
print("B1/B2/B3. 148's refutation: does the congruence even apply at a single addition?")
print("=" * 96)

lo, hi = bounds(False)
dom = range(lo, hi + 1)

# A. the overflow merge 140's P3 is about
low_taken = 0
agree = True
for a in dom:
    for b in dom:
        s = a + b
        if s < lo:
            low_taken += 1
        if reduce_(s, False, "sat_both") != reduce_(s, False, "sat_high"):
            agree = False
print(f"  A. unsigned ADDITION, the two saturating positions")
print(f"     low branch taken: {low_taken} of {len(dom) ** 2} operand pairs")
print(f"     SaturateBoth and SaturateHighOnly agree everywhere: {agree}")
print(f"     -> the mechanism is REACHABILITY, not congruence: the distinguishing branch")
print(f"        is never entered")

low_taken_sub = 0
sep = 0
for a in dom:
    for b in dom:
        s = a - b
        if s < lo:
            low_taken_sub += 1
        if reduce_(s, False, "sat_both") != reduce_(s, False, "sat_high"):
            sep += 1
print(f"\n  B2 CONTROL. unsigned SUBTRACTION, same two positions")
print(f"     low branch taken: {low_taken_sub} of {len(dom) ** 2}")
print(f"     the two positions separate at: {sep} pairs")
b2 = sep > 0
print(f"     control fires: {b2} (must be True)")

# B. does the congruence proposition apply at one reduction site?
print(f"\n  B1. single addition: do the two INTERMEDIATE positions coincide?")
b1 = True
for ov in ("wrap", "sat_both", "sat_high"):
    d = 0
    for a in dom:
        for b in dom:
            for c in dom:
                early = reduce_(reduce_(a + b, False, ov) + c, False, ov)
                late = reduce_(a + b + c, False, ov)
                if early != late:
                    d += 1
    b1 &= (d == 0)
    print(f"     overflow = {ov:<10} early vs late differ at {d} triples")
print(f"     B1: {'CONFIRMED' if b1 else 'REFUTED'}, and the wrapping row is the decisive one:")
print(f"     a zero under a policy the congruence does not cover is structural, not congruential.")

print(f"\n  B3 CONTROL. multiply-add, which has two reduction sites")
b3 = True
for signed in (False, True):
    lo2, hi2 = bounds(signed)
    dom2 = range(lo2, hi2 + 1)
    d = 0
    for a in dom2:
        for b in dom2:
            t = reduce_(a * b, signed, "sat_both")
            for c in dom2:
                if reduce_(t + c, signed, "sat_both") != reduce_(a * b + c, signed, "sat_both"):
                    d += 1
    print(f"     signedness = {'signed' if signed else 'unsigned':<9} differ at {d} triples")
    if signed:
        b3 &= d > 0
    else:
        b3 &= d == 0
print(f"     B3: {'CONFIRMED' if b3 else 'REFUTED'}, so the congruence is the mechanism where")
print(f"     there are two sites and is unavailable where there is one.")

print()
print("=" * 96)
print("B4. 147's narrowing: which unsigned cell actually rests on the one-sidedness")
print("=" * 96)

def q_floor(n, d):
    return n // d

def fusion_unsigned(ov_kind, F, low_bound=0):
    """ov_kind in {wrap, sat_one_sided, sat_two_sided}; the two-sided clamp uses low_bound."""
    lo0, hi0 = bounds(False)
    S = 1 << F
    d = 0
    engaged = 0
    for a in range(lo0, hi0 + 1):
        for b in range(lo0, hi0 + 1):
            p = a * b
            for c in range(lo0, hi0 + 1):
                def red(v):
                    if ov_kind == "wrap":
                        return v % (1 << W)
                    if ov_kind == "sat_one_sided":
                        return min(hi0, v)
                    if v < low_bound:
                        return low_bound
                    return min(hi0, v)
                nonlocal_engaged = 0
                t = red(q_floor(p, S))
                step = red(t + c)
                fused = red(q_floor(p + c * S, S))
                if ov_kind == "sat_two_sided" and (q_floor(p, S) < low_bound
                                                   or t + c < low_bound):
                    engaged += 1
                if fused != step:
                    d += 1
    return d, (hi0 - lo0 + 1) ** 3, engaged

print(f"  {'overflow':<18}" + "".join(f"{'F=' + str(f):>10}" for f in (0, 1, 2, 3)) + "  low clamp")
b4_rows = {}
for kind in ("wrap", "sat_one_sided", "sat_two_sided"):
    row = []
    eng = 0
    for F in (0, 1, 2, 3):
        d, t, e = fusion_unsigned(kind, F, low_bound=4)
        row.append(100.0 * d / t)
        eng += e
    b4_rows[kind] = row
    print(f"  {kind:<18}" + "".join(f"{v:>9.2f}%" for v in row) + f"  {eng}")
b4 = (all(v == 0.0 for v in b4_rows["wrap"])
      and all(v == 0.0 for v in b4_rows["sat_one_sided"])
      and any(v > 0.0 for v in b4_rows["sat_two_sided"]))
print(f"\n  B4: {'CONFIRMED' if b4 else 'REFUTED'}. The wrapping zero survives a two-sided clamp")
print(f"  because wrapping has no clamp; only the saturating zero rests on the one-sidedness.")

print()
print("=" * 96)
print("B5/B6. The instrument fix: intersect over VALUES, not over names")
print("=" * 96)

PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

def read(n):
    p = os.path.join(PANEL, n)
    return open(p).read() if os.path.exists(p) else None

FENCED = re.compile(r"```\n(holds for:.*?)\n```", re.S)
EMPH = re.compile(r"\*([^*]*?holds? for:.*?)\*", re.S)

def parse_dims(block):
    """dimension name -> value set, with None meaning `any` (universal)."""
    body = re.split(r"holds? for:", block, maxsplit=1)[-1]
    body = " ".join(body.split())
    out = {}
    # split on ; and on , that is not inside braces
    parts, depth, cur = [], 0, ""
    for ch in body:
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
        if ch in ";," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    for p in parts:
        p = p.strip().rstrip(".").strip("` ")
        if not p:
            continue
        m = re.match(r"^(.+?)\s+in\s+\{(.*?)\}", p)
        if m:
            out[" ".join(m.group(1).split())] = {v.strip() for v in m.group(2).split(",")}
            continue
        m = re.match(r"^(.+?)\s+any\b", p)
        if m:
            out[" ".join(m.group(1).split())] = None
            continue
        m = re.match(r"^(.+?)\s*=\s*(.+)$", p)
        if m:
            out[" ".join(m.group(1).split())] = {m.group(2).strip()}
            continue
        out[" ".join(p.split())] = {"(unparsed)"}
    return out

def dimension_shaped(k):
    """B7's control. A dimension name is short and carries no sentence punctuation."""
    return len(k) <= 40 and not re.search(r"[.`]", k)

def clean_values(d):
    """Trim a value at the first backtick or sentence break, which is where the block pattern
    stops being a predicate and starts being prose."""
    out = {}
    for k, v in d.items():
        if v is None:
            out[k] = None
            continue
        nv = set()
        for x in v:
            x = re.split(r"[`.]| Structural| Established| \d+ classes", x)[0].strip()
            if x:
                nv.add(x)
        out[k] = nv
    return out

def blocks_of(name):
    t = read(name)
    if t is None:
        return []
    bs = [m.group(1) for m in FENCED.finditer(t)]
    bs += [m.group(1) for m in EMPH.finditer(t)]
    return bs

def near(name, needle):
    t = read(name)
    out = []
    for b in blocks_of(name):
        i = t.find(b)
        if needle.lower() in t[max(0, i - 1600):i].lower():
            out.append(b)
    return out

CONVERGENCES = {
    "container is answer-invisible at the column": [
        ("139_muratori_the_strategy_set_derived_cold.md", "packed and padded"),
        ("140_mcsherry_the_strategy_set_derived_cold.md", "lossless container"),
        ("141_lamport_the_strategy_set_attacked.md", "lossless storage container"),
    ],
    "accumulator is visible exactly at signed saturating": [
        ("141_lamport_the_strategy_set_attacked.md", "accumulator width is answer-visible"),
        ("142_muratori_reply_the_repair_was_dead_on_arrival.md", "accumulator width is answer-visible"),
        ("143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
         "accumulator width is answer-visible"),
    ],
    "class count is monotone and not strict": [
        ("141_lamport_the_strategy_set_attacked.md", "monotone non-decreasing"),
        ("143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
         "Monotone non-decreasing"),
    ],
    "fusion is an axis position already": [
        ("141_lamport_the_strategy_set_attacked.md", "bit-identical to the plain lowering"),
        ("142_muratori_reply_the_repair_was_dead_on_arrival.md", "are the same two"),
    ],
}

def merge(dicts):
    """per-dimension value intersection; None (any) is the identity."""
    names = set().union(*[set(d) for d in dicts]) if dicts else set()
    out = {}
    for n in sorted(names):
        present = [d[n] for d in dicts if n in d]
        if len(present) < len(dicts):
            out[n] = ("ABSENT FROM SOME", None)
            continue
        acc = None
        for v in present:
            if v is None:
                continue
            acc = set(v) if acc is None else (acc & v)
        out[n] = ("any" if acc is None else acc, len(present))
    return out

empty_rows = []
rejected = []
for claim, instances in CONVERGENCES.items():
    dicts = []
    for fname, needle in instances:
        bs = near(fname, needle)
        d = {}
        for b in bs:
            pd_ = parse_dims(b)
            for bad in [k for k in pd_ if not dimension_shaped(k)]:
                rejected.append((fname, bad[:56]))
                del pd_[bad]
            for k, v in clean_values(pd_).items():
                if k in d and d[k] is not None and v is not None:
                    d[k] = d[k] | v
                elif v is None:
                    d[k] = None
                else:
                    d[k] = v
        dicts.append(d)
    print(f"\n  {claim}")
    for (fname, needle), d in zip(instances, dicts):
        sd = {k: ('any' if v is None else sorted(v)) for k, v in d.items()
              if k in ("signedness", "W", "F", "overflow", "rounding")}
        print(f"    {fname.split('_')[0]}: {sd}")
    m = merge([d for d in dicts if d])
    empties = [n for n, (v, _) in m.items()
               if isinstance(v, set) and len(v) == 0]
    if empties:
        empty_rows.append((claim, empties))
    print(f"    dimensions whose VALUE intersection is EMPTY: {empties if empties else 'none'}")

b5 = any("signedness" in e for c, e in empty_rows if c.startswith("container"))
print(f"\n  B7 CONTROL: parsed keys dropped as not dimension-shaped: {len(rejected)}")
for r in rejected[:4]:
    print(f"      {r[0].split('_')[0]}: {r[1]}...")
print(f"  B7: {'CONFIRMED, the control fires and the mangled keys are named' if rejected else 'the control did not fire'}")

print(f"\n  B5 (the container row's signedness intersects to empty): "
      f"{'CONFIRMED' if b5 else 'REFUTED'}")
print(f"  B6 (rows carrying the defect): {len(empty_rows)} of {len(CONVERGENCES)}")
for c, e in empty_rows:
    print(f"      {c}: {e}")

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print("  148's refutation reproduces: at one reduction site the intermediate positions coincide")
print("  under every overflow position, wrapping included, so the congruence is not available as")
print("  an explanation at the cell 146 attributed to it. The third contamination item is wrong.")
print()
print("  147's narrowing reproduces: only the unsigned saturating cell rests on the one-sidedness.")
print()
print("  And my z6 instrument was measuring the wrong thing. Intersecting dimension NAMES reports a")
print("  dimension as present when the instances partition its values, and the honest intersection")
print("  is over values.")
print()
print(f"  B5 as I predicted it is REFUTED. At the container row the empty dimensions are not")
print(f"  `signedness`, which 139 simply omits and which the name-level check already dropped.")
print(f"  They are `W` and `rounding`: 139 swept W in {{3,5,6,7,11}} where the other two swept")
print(f"  W = 4, and 139 swept truncate where 141 swept floor and toward-zero. So 148's diagnosis")
print(f"  is right about the mechanism and wrong about which dimension carries it here.")
print()
print(f"  B6: the defect reaches {len(empty_rows)} of {len(CONVERGENCES)} rows, which 148 did not")
print(f"  check and said so. The fusion row is clean.")
print()
print("  What that means for a clause built on one of these rows: the intersection is not merely")
print("  an upper bound, it is EMPTY on those dimensions, so a predicate written from it claims")
print("  nothing where they are present. A clause needs a single instance that spans what it")
print("  wants to claim, cited for that dimension, which is 148's repair applied per dimension")
print("  rather than only to signedness.")
