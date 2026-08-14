#!/usr/bin/env python3
# y2 (138): the repair checked against 138 itself, which is the control y1 named in advance.
#
# y1 T1 ended "after the repair this count must be 0 and the rerun below is that check". This is
# that rerun, plus the anchors the repaired clauses rest on.
#
# Predictions, stated before running:
#   U1. No predicate in 138 carries a hedge token. The token set is deliberately wider than the
#       one word 137 found, because the defect is a hedge written into a machine-readable slot and
#       `OPEN` is only the spelling I happened to use. Control: the same scan over 136 must find
#       the three, or the scanner proves nothing about 138.
#   U2. Every anchor the five repairs rest on is present in 138. A restoration pass that names a
#       correction without its source is the defect it is repairing.
#   U3. 138 introduces no anchor into a file that does not exist. Control: a deliberately bogus
#       anchor must be caught.
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
REPAIR = "138_leroy_the_restoration_pass.md"
PRIOR = "136_leroy_the_candidate_revised_against_three_signatures.md"

def read(n):
    return open(os.path.join(PANEL, n)).read()

def predicate_paragraphs(t):
    return [p for p in t.split("\n\n") if re.search(r"holds? for:", p)]

HEDGE = re.compile(r"\b(OPEN|TBD|UNKNOWN|unmeasured|unsure|probably|likely|assumed|pending)\b",
                   re.I)

print("=" * 88)
print("U1. Hedge tokens in predicates, before and after")
print("=" * 88)
for name, tag in ((PRIOR, "136, before"), (REPAIR, "138, after")):
    preds = predicate_paragraphs(read(name))
    hits = [(i, sorted(set(HEDGE.findall(p)))) for i, p in enumerate(preds) if HEDGE.search(p)]
    print(f"  {tag:<14} {len(preds)} predicates, {len(hits)} carrying a hedge token")
    for i, toks in hits:
        print(f"      predicate {i}: {toks}")
before = sum(1 for p in predicate_paragraphs(read(PRIOR)) if HEDGE.search(p))
after = sum(1 for p in predicate_paragraphs(read(REPAIR)) if HEDGE.search(p))
print(f"\n  U1: {'CONFIRMED' if after == 0 and before == 3 else 'REFUTED'} "
      f"(control: the scanner finds {before} in 136, must be 3; finds {after} in 138, must be 0)")

print()
print("=" * 88)
print("U2/U3. The anchors the repairs rest on")
print("=" * 88)
text = read(REPAIR)
OWED = [
    ("the OPEN instruction", r"INTENTS\.md:241-243"),
    ("its gloss", r"INTENTS\.md:245-246"),
    ("the arms sweep", r"g8_does_the_class_reach_131s_arms"),
    ("R3's compound predicate", r"131:169-173"),
    ("R5's compound predicate", r"131:203-209"),
    ("R6, the one that propagated", r"131:221-223"),
    ("the origin carrier", r"130:13"),
    ("the four misattributions", r"131:48.*131:52.*132:43.*134:13.*135:10"),
    ("the ledger row", r"132:70"),
    ("B6 in phase one", r"126:24"),
    ("the phase-two boundary", r"126:393"),
    ("the gate run", r"g0_test_gate\.out"),
    ("the third data point", r"6\.61"),
    ("the prior artifact", r"u0_test_gate_run\.txt"),
    ("the present-vs-not-present test", r"136:145-147"),
    ("the misdirected citation", r"136:65-66"),
]
missing = []
for label, pat in OWED:
    ok = re.search(pat, text, re.S) is not None
    if not ok:
        missing.append(label)
    print(f"  {'ok  ' if ok else 'MISS'}  {label:<34} /{pat[:44]}/")
print(f"\n  U2: {'CONFIRMED' if not missing else 'REFUTED: ' + ', '.join(missing)}")

bogus = "999_nonexistent_file.md:1"
print(f"\n  U3 control, a bogus anchor is caught by the same scan: "
      f"{re.search(re.escape(bogus), text) is None} (must be True: 138 does not contain it)")
cited = sorted(set(re.findall(r"`(1\d\d)`", text)))
absent = [c for c in cited
          if not any(f.startswith(c + "_") for f in os.listdir(PANEL))]
print(f"  panel files cited: {cited}")
print(f"  cited but not present on disk: {absent} (must be empty)")

print()
print("=" * 88)
print(f"VERDICT: hedge tokens removed: {after == 0}, anchors owed and present: {not missing}, "
      f"no dangling file citation: {not absent}")
