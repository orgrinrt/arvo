#!/usr/bin/env python3
"""Citation check for 171, with the seventh and eighth defects adopted.

Three normalisation layers, applied cumulatively, so each one's contribution is
counted rather than assumed:
  L1 whitespace collapse                    (168's fifth defect, 167's fix)
  L2 + strip blockquote markers and emphasis (169's seventh defect)
  L3 + fold case                             (170's eighth defect)

CASES THAT MUST FAIL
  C-1 a citation naming a file that does not exist must be caught
  C-2 a citation naming a real file with text that is not there must be caught
  C-3 at least one layer must be load-bearing, else this is 167's checker again
"""
import os, re, sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', '..', '..', '..'))
D = 'mock/research/202608072330_the-numeral-canon-panel'
WS = os.path.abspath(os.path.join(ROOT, '..'))  # the workspace root, one above the arvo clone
R = '.claude/rules'

def L1(t): return " ".join(t.split())
def L2(t):
    t = re.sub(r'^\s*>\s?', ' ', t, flags=re.M)
    t = t.replace('**', '').replace('*', '').replace('`', '')
    return L1(t)
def L3(t): return L2(t).casefold()

CITES = [
    ("169 1.2 the rule is line 4", f"{D}/169_kiselyov_the_chain_attacked.md",
     "is **line 4** of the auto-loaded rule set"),
    ("169 1.2 none of the three names it", f"{D}/169_kiselyov_the_chain_attacked.md",
     "**None of `167`, `168` or `60` names the rule**"),
    ("169 1.2 168's derivation self-contained", f"{D}/169_kiselyov_the_chain_attacked.md",
     "derivation is semantically self-contained"),
    ("169 4 the widening to F any", f"{D}/169_kiselyov_the_chain_attacked.md",
     "4.1's conclusion holds for **`F any`** on the argument"),
    ("169 4 the closed form", f"{D}/169_kiselyov_the_chain_attacked.md",
     "The `M = 2F-1` column is exactly `2^(F-1)` at every `F` from 4 to 10"),
    ("169 R-9", f"{D}/169_kiselyov_the_chain_attacked.md",
     "State the widening in your own next file, in your own voice"),
    ("169 R-10", f"{D}/169_kiselyov_the_chain_attacked.md",
     "Add the closed form to the table"),
    ("169 8 did not attack two survivors", f"{D}/169_kiselyov_the_chain_attacked.md",
     "I could not attack `167`'s other two survivors, the backward-narrowing licence and the correlation finding"),
    ("169 9 seventh defect", f"{D}/169_kiselyov_the_chain_attacked.md",
     "whitespace normalisation is necessary and not sufficient"),
    ("170 8 the dependence", f"{D}/170_mcsherry_reply.md",
     "So I report dependence rather than independence"),
    ("170 8 one instance three hats", f"{D}/170_mcsherry_reply.md",
     "closer to **one instance wearing three hats** than to three"),
    ("170 8 right about the inequality", f"{D}/170_mcsherry_reply.md",
     "That is right about the inequality and wrong about the definition"),
    ("170 8 tried and could not", f"{D}/170_mcsherry_reply.md",
     "I tried to construct a route to the delimiter that does not pass through it, and I could not"),
    ("the rule's thesis", f"{R}/what-you-can-observe-is-what-you-guaranteed.md",
     "A guarantee about a type holds only over the operations through which the type can be observed"),
    ("167 sec1 the sentence I split", f"{D}/167_rompf_the_chain_derived_cold.md",
     "everything inside it is arvo's to choose"),
    ("167 R7 contains against bounds", f"{D}/167_rompf_the_chain_derived_cold.md",
     "C9 says what a chain **contains**. Mine says what **bounds** one"),
    ("167 4.1 the slack argument", f"{D}/167_rompf_the_chain_derived_cold.md",
     "so the theorem's slack has nowhere to live"),
    ("60 the defining sentence", f"{D}/60_stam_the_chain_derived_cold.md",
     "A chain is a composition of exact operations together with a schedule of adaptation points"),
    ("168 the defining sentence", f"{D}/168_mcsherry_the_chain_derived_cold.md",
     "A chain is a maximal run of operations whose intermediates are not observable"),
    ("I18 the build bound", f"{D}/INTENTS.md",
     "Dev and debug only. It does not survive into a release artifact"),
    ("I14 monomorphisation is the dispatch", f"{D}/INTENTS.md",
     "Monomorphisation is the dispatch. No `dyn`, no `TypeId`, no `std::any`"),
    ("I15 never any runtime checks", f"{D}/INTENTS.md",
     "Never any runtime checks, ever"),
]

BOGUS = [
    ("C-1 nonexistent file", f"{D}/999_no_such_file.md", "anything at all"),
    ("C-2 real file, absent text", f"{D}/INTENTS.md",
     "the strategy set is closed at exactly four and always will be"),
]

layers = {"L1": L1, "L2": L2, "L3": L3}
moved = {"L1": 0, "L2": 0, "L3": 0}
fails = 0
print(f"{'citation':<46} {'raw':>5} {'L1':>5} {'L2':>5} {'L3':>5}  first layer that finds it")
for label, path, needle in CITES:
    full = os.path.join(ROOT, path)
    if not os.path.exists(full):
        full = os.path.join(WS, path)
    if not os.path.exists(full):
        print(f"{label:<46}  FILE MISSING -> {path}")
        fails += 1
        continue
    text = open(full, encoding='utf-8').read()
    raw = needle in text
    res = {k: layers[k](needle) in layers[k](text) for k in ("L1", "L2", "L3")}
    first = "raw" if raw else next((k for k in ("L1", "L2", "L3") if res[k]), None)
    if first is None:
        fails += 1
    elif first != "raw":
        moved[first] += 1
    print(f"{label:<46} {str(raw):>5} {str(res['L1']):>5} {str(res['L2']):>5} {str(res['L3']):>5}  {first or 'NOT FOUND'}")

print()
print("--- would L2 matter if I had quoted the CONTENT rather than the markup? ---")
would = 0
for label, path, needle in CITES:
    full = os.path.join(ROOT, path)
    if not os.path.exists(full):
        full = os.path.join(WS, path)
    if not os.path.exists(full):
        continue
    text = open(full, encoding='utf-8').read()
    stripped = needle.replace('**','').replace('`','')
    if stripped != needle:
        found_l1 = L1(stripped) in L1(text)
        found_l2 = L2(stripped) in L2(text)
        if found_l2 and not found_l1:
            would += 1
print(f"  quotations carrying markup: {sum(1 for _,_,n in CITES if '**' in n or '`' in n)}")
print(f"  of those, ones L2 would rescue had I dropped the markup: {would}")

print()
caught = 0
for label, path, needle in BOGUS:
    full = os.path.join(ROOT, path)
    if not os.path.exists(full):
        ok = True
    else:
        ok = L3(needle) not in L3(open(full, encoding='utf-8').read())
    print(f"{'caught' if ok else 'MISSED'}  {label}")
    caught += ok

print()
print(f"citations: {len(CITES)}, not found: {fails}")
print(f"first found at L1 (whitespace, 168's fifth defect):        {moved['L1']}")
print(f"first found at L2 (markup, 169's seventh defect):          {moved['L2']}")
print(f"first found at L3 (case, 170's eighth defect):             {moved['L3']}")
print(f"C-3 at least one layer load-bearing: {any(moved.values())}")
print(f"negative controls caught: {caught} of {len(BOGUS)} (must be {len(BOGUS)})")
sys.exit(1 if (fails or caught != len(BOGUS)) else 0)
