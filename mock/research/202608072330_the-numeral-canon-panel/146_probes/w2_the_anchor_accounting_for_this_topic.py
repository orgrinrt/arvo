#!/usr/bin/env python3
# w2 (146): the anchor accounting, reusing 119_probes/r1's stripper and 132_probes/w1's classes,
# extended for this topic's own anchor shapes. Neither of those is edited; both belong to other
# files.
#
# Three things this topic needs that the earlier instruments do not have:
#
#   1. MARKDOWN INSIDE THE ANCHOR. 145_probes/z7 had two citations fail that were verbatim
#      correct, defeated by the source's own backticks and emphasis inside the quoted phrase.
#      That is the fifth instance of the class in this panel, so this extractor strips markup
#      before matching rather than after, and carries a control proving the stripping cannot
#      manufacture a hit.
#   2. FILE-QUALIFIED FINDINGS. This topic reuses bare `F1` in three files for three different
#      claims, so a bare finding id is ambiguous. The extractor records both the bare form, for
#      counting, and the file-qualified form where the text supplies one.
#   3. OPTION IDS. `O-139-C`, `O-142-B`, `O-144-A` are anchors a clause rests on and no earlier
#      instrument matches them.
#
# Predictions, stated before running:
#   J1. The stripper fires on 146's accounting section, because that section names the anchors it
#       drops in order to account for them.
#   J2. Every anchor in the `not carried` set belongs to a clause 146 does not state, and none is
#       from this topic's own seven files unless 146 deliberately leaves that clause to its source.
#   J3. CONTROL: markup stripping must not manufacture a hit. A finding id that appears nowhere
#       must stay absent after stripping.
#   J4. CONTROL: the option-id class must be non-empty in the union, or the extension is untested.
#   J5. CONTROL: a possessive must not be read as an id. `F9's` must extract F9 and not F9'.
import importlib.util
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

def load(rel, name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(PANEL, rel))
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod

sys.argv = [sys.argv[0]]
r1 = load("119_probes/r1_the_anchor_inventory_and_what_the_candidate_carries.py", "r1mod")

# the trailing apostrophe is a real part of an id (F2', F3') but must not swallow a POSSESSIVE:
# `F9's` is a reference to F9, not to an id called F9'. Caught when `141 F9's width-sensitive half`
# extracted as F9' and left F9 reading as not carried.
_RE_FINDING = re.compile(r"\bF\d{1,3}(?:-\d{1,2})?(?:'(?!s))?\b")
_RE_QUALIFIED = re.compile(
    r"\b(1[34]\d)\b[^.\n]{0,24}?\b(F\d{1,3}(?:-\d{1,2})?(?:'(?!s))?)\b")
_RE_OPTION = re.compile(r"\bO-1\d\d-[A-Z]\b")
_RE_THM = re.compile(r"\bT\d{1,2}\b")
_RE_PROBE = re.compile(
    r"\b\d+_probes/[a-z]\d+[a-z]?(?:_[A-Za-z0-9_]*)?(?:\.(?:py|rs|sh|txt|s))?\b"
    r"|\b[a-z]\d+[a-z]?_[A-Za-z0-9_]+\.(?:py|rs|sh|txt|s)\b"
)
_RE_LINE = re.compile(r"\b(1[0-4]\d):(\d+)(?:-\d+)?\b")

def demarkup(text):
    """Strip the markup that defeated z7, before matching. Backticks, emphasis and blockquote
    markers are presentation; an anchor wrapped in them is still that anchor."""
    text = re.sub(r"^\s*>\s?", "", text, flags=re.M)
    return text.replace("`", "").replace("**", "").replace("*", "")

def _stem(ref):
    ref = ref.split("/")[-1]
    m = re.match(r"([a-z]\d+[a-z]?)", ref)
    return m.group(1) if m else ref

def anchors(text):
    t = demarkup(text)
    return {
        "finding": set(_RE_FINDING.findall(t)),
        "finding_qualified": {f"{a} {b}" for a, b in _RE_QUALIFIED.findall(t)},
        "option": set(_RE_OPTION.findall(t)),
        "theorem": set(_RE_THM.findall(t)),
        "probe_stem": {_stem(x) for x in _RE_PROBE.findall(t)},
        "line_panel": {f"{a}:{b}" for a, b in _RE_LINE.findall(t)},
    }

UNION = [
    "139_muratori_the_strategy_set_derived_cold.md",
    "140_mcsherry_the_strategy_set_derived_cold.md",
    "141_lamport_the_strategy_set_attacked.md",
    "142_muratori_reply_the_repair_was_dead_on_arrival.md",
    "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
    "144_fog_the_weighting_half_measured.md",
    "145_leroy_formalising_the_strategy_object.md",
]
CAND = "146_leroy_the_canon_candidate_for_the_strategy_object.md"

def read(name):
    p = os.path.join(PANEL, name)
    return open(p).read() if os.path.exists(p) else None

union = {}
missing = []
for name in UNION:
    t = read(name)
    if t is None:
        missing.append(name)
        continue
    for cls, vals in anchors(t).items():
        union.setdefault(cls, set()).update(vals)

raw = read(CAND)
if raw is None:
    print(f"  {CAND} not present yet; union only.")
    for cls in sorted(union):
        print(f"  {cls:<20}{len(union[cls]):>6}")
    sys.exit(0)

full = anchors(raw)
stripped = anchors(r1.strip_accounting(raw))

print("=" * 100)
print("Anchor accounting for 146, over this topic's seven files")
print("=" * 100)
if missing:
    print(f"  union files absent and therefore not counted: {missing}")
print(f"  union built from {len(UNION) - len(missing)} files")
print()
print(f"  {'class':<20}{'in the union':>14}{'in 146':>9}{'incl. accounting':>18}{'not carried':>13}")
fired = []
for cls in sorted(union):
    u, s, f = union[cls], stripped.get(cls, set()), full.get(cls, set())
    if len(f) > len(s):
        fired.append(f"{cls} +{len(f) - len(s)}")
    print(f"  {cls:<20}{len(u):>14}{len(s):>9}{len(f):>18}{len(u - s):>13}")

print()
print(f"  J1, the stripper: {'FIRED  (' + ', '.join(fired) + ')' if fired else 'did not fire'}")

print()
for cls in sorted(union):
    nc = sorted(union[cls] - stripped.get(cls, set()))
    print(f"  {cls}: {len(union[cls]) - len(nc)} carried, {len(nc)} not")
    for i in range(0, len(nc), 5):
        print("      " + "  ".join(f"{str(x):<22}" for x in nc[i:i + 5]))

print()
print("=" * 100)
print("J3/J4. The controls")
print("=" * 100)
ghost = "F999-9"
j3 = ghost not in anchors("a sentence with **no** such `finding` in it")["finding"]
j3b = "F144-15" in anchors("the claim at `**F144-15**` holds")["finding"]
print(f"  J3 stripping does not manufacture an absent id ({ghost}): {j3} (must be True)")
print(f"     and it DOES recover one wrapped in markup: {j3b} (must be True)")
j5 = anchors("141 F9's width-sensitive half")["finding"] == {"F9"}
print(f"  J5 a possessive extracts the id and not a primed variant: {j5} (must be True)")
print(f"  J4 option ids present in the union: {len(union.get('option', set()))} "
      f"(must be > 0), examples {sorted(union.get('option', set()))[:4]}")
print(f"\n  controls: {'CONFIRMED' if j3 and j3b and j5 and union.get('option') else 'REFUTED'}")
