#!/usr/bin/env python3
"""r3. Check every quotation in 174 by opening it, under all three layers.

The unit has found three defects in citation checkers and this folds in all of
them, reporting which layer each quotation needed so the layers are measured
rather than assumed:
  L0 raw            (168's fifth defect: a bare grep cannot cross a line break)
  L1 whitespace     (169's seventh: markup survives whitespace normalisation)
  L2 markup         (blockquote markers, emphasis, inline code)
  L3 case           (170's eighth: a mid-sentence quotation lowercases its
                     leading capital, which is English and not a misquotation)

THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
  C1. A planted quotation known present must be found, or the checker is not
      reading the files.
  C2. A planted quotation known absent must not be found, or it matches
      anything.
  C3. The layer report must be produced for every quotation, so a layer doing
      nothing in this file is visible as a zero rather than assumed dead.
"""
import re

PANEL = ".."

def lay(t, ws=False, mk=False, cs=False):
    if mk:
        t = re.sub(r'^\s*>\s?', '', t, flags=re.M)
        t = t.replace('**', '').replace('`', '')
    if ws:
        t = re.sub(r'\s+', ' ', t).strip()
    if cs:
        t = t.casefold()
    return t

LAYERS = [("L0 raw", dict()),
          ("L1 ws", dict(ws=True)),
          ("L2 markup", dict(ws=True, mk=True)),
          ("L3 case", dict(ws=True, mk=True, cs=True))]

CITES = [
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "A concept whose operations are closed over the format, adaptation fused invisibly into each one, can state the stepwise grade and nothing above it"),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "theorem, with one measured premise and an enumeration bound"),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "deferring every interior resolution to the boundary is pointwise optimal"),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "End state: (P) at two instances"),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "Cite the theorem, not the original sweep."),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "printing dropped anchors inside the file being diffed makes them present and disables the check"),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "an intermediate format, a schedule, an association and order statement, a count bound, and an error bound composed per adaptation point"),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "63/60 via the adaptation schedule"),
    ("60_stam_the_chain_derived_cold.md",
     "That is a statability argument, not a benchmark"),
    ("60_stam_the_chain_derived_cold.md",
     "An error bound composed per adaptation point"),
    ("172_leroy_formalising_the_chain.md",
     "A chain of total steps"),
    ("172_leroy_formalising_the_chain.md",
     "(P) carries two instances, one with a rule-free derivation"),
]
CONTROLS = [
    ("173_leroy_the_canon_candidate_for_the_chain.md", "The statement", True),
    ("173_leroy_the_canon_candidate_for_the_chain.md",
     "the deferral theorem is hereby withdrawn at every carrier", False),
]

cache = {}
def body(f):
    if f not in cache:
        cache[f] = open(f"{PANEL}/{f}").read()
    return cache[f]

print(f"{'file':<44} {'layer needed':<12} quotation")
need = {n: 0 for n, _ in LAYERS}
missing = 0
for f, q in CITES:
    raw = body(f)
    hit = None
    for name, kw in LAYERS:
        if lay(q, **kw) in lay(raw, **kw):
            hit = name
            break
    if hit is None:
        missing += 1
        print(f"{f[:44]:<44} {'MISSING':<12} {q[:52]}")
    else:
        need[hit] += 1
        print(f"{f[:44]:<44} {hit:<12} {q[:52]}")

print()
print("layer report:", ", ".join(f"{n} {c}" for n, c in need.items()), f", missing {missing}")
print()
print("=== CONTROLS ===")
ok = True
for f, q, want in CONTROLS:
    got = lay(q, ws=True, mk=True, cs=True) in lay(body(f), ws=True, mk=True, cs=True)
    print(f"C{'1' if want else '2'} planted-{'present' if want else 'absent '} {'found' if got else 'not found'} : {got == want}")
    ok &= got == want
assert ok, "a planted control behaved wrongly, so this checker's verdicts mean nothing"
print("C3 a layer count is printed for every quotation, zeros included : True")
assert missing == 0, f"{missing} quotations do not resolve; fix the file, not the checker"
print()
print("RESULT: every quotation in 174 opens to the text it claims.")
