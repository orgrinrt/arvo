#!/usr/bin/env python3
"""169 P6. Check every quotation and file reference in 169 by opening it, with
whitespace normalised.

168's own fifth instrument defect was a citation checker whose `grep -F`
returned four false negatives out of fifteen on quotations spanning a line
break. So this collapses all runs of whitespace to one space on BOTH sides
before comparing.

NEGATIVE CONTROLS, stated before the run.
  C1. A quotation that IS in the source must be found. Planted: a sentence
      taken verbatim from 168.
  C2. A quotation that is NOT in any source must be reported missing.
      Planted: a sentence that exists nowhere.
  C3. A naive substring check WITHOUT normalisation must fail on at least one
      real quotation, or normalisation is not doing anything and the defect
      168 found does not apply here.
  C4. Every non-control quotation must be found. A missing one is either a
      wrong citation of mine or a defect in this normaliser, and the two are
      told apart by opening the source by hand, which is what happened for
      the two that failed the first run.
"""
import os, re, sys

D = os.path.dirname(os.path.abspath(__file__)) + "/.."
def norm(s):
    """Collapse whitespace AND strip the markup that survives it.

    The first version of this normaliser handled line breaks only, and it
    reported two of my own true quotations as missing: one sits inside a
    blockquote, so the wrap inserted `> ` mid-sentence, and one carries `**`
    around the numbers. 168's fifth defect was a checker that missed
    quotations spanning a line break; this is the same defect one layer in,
    found by this checker's own missing-count rather than by a control, and
    recorded rather than quietly fixed."""
    s = re.sub(r'\s+', ' ', s)
    s = s.replace('> ', ' ')          # blockquote continuation markers
    s = s.replace('**', '').replace('`', '')
    return re.sub(r'\s+', ' ', s).strip()

SOURCES = {}
for n in os.listdir(D):
    if n.endswith('.md') and re.match(r'^\d+_', n):
        SOURCES[n] = open(os.path.join(D, n), errors='replace').read()
for sub in ("168_probes", "167_probes", "157_probes"):
    p = os.path.join(D, sub)
    if os.path.isdir(p):
        for n in os.listdir(p):
            fp = os.path.join(p, n)
            if os.path.isfile(fp):
                SOURCES[f"{sub}/{n}"] = open(fp, errors='replace').read()

NORMED = {k: norm(v) for k, v in SOURCES.items()}
RAW = SOURCES

QUOTES = [
    ("168 heading claim",
     "Two of our section headings are word-for-word the same"),
    ("168 pointwise claim",
     "There is no input, and no chain, on which any other placement is strictly closer"),
    ("168 T1 bound",
     "carrier widths 16 through 19 on this construction"),
    ("168 T1 measured wording",
     "And the contest is bounded, which I measured after reading"),
    ("168 summary sentence",
     "checked over 3000 chains with a control that finds 1330"),
    ("167 4.1 conclusion",
     "There is no `M` strictly between `F` and `2F` with zero disagreements, at any `F` tested."),
    ("167 definition",
     "a maximal stretch of a computation in which no intermediate is named by anyone outside it"),
    ("168 definition",
     "A chain is a maximal run of operations whose intermediates are not observable."),
    ("full_mask line",
     "let fm = full_mask(steps.len());"),
    ("p6 criterion",
     "if !a_exact_fits && a_resolved_fits {"),
    ("the rule's thesis",
     "A guarantee about a type holds only over the operations through which the type can be observed."),
    ("CONTROL C1 planted-present",
     "arvo's world has two levels and everything below turns on keeping them apart."),
    ("CONTROL C2 planted-absent",
     "the chain is a lemon and the carrier is a hat"),
]
RULE = "/Users/orgrinrt/Dev/clause-dev/.claude/rules/what-you-can-observe-is-what-you-guaranteed.md"
NORMED["<rule>"] = norm(open(RULE).read()); RAW["<rule>"] = open(RULE).read()

def find(q, table):
    return [k for k, v in table.items() if q in v]

ok = miss = 0
naive_failures = 0
print(f"{'quotation':34} {'found in':>10}  where")
for label, q in QUOTES:
    hits = find(norm(q), NORMED)
    raw_hits = find(q, RAW)
    if not hits and not raw_hits:
        naive = ""
    if len(raw_hits) < len(hits):
        naive_failures += 1
    where = hits[0] if hits else "<none>"
    if len(hits) > 1:
        where = f"{hits[0]} (+{len(hits)-1})"
    print(f"{label:34} {len(hits):>10}  {where}")
    if label.startswith("CONTROL C2"):
        if hits: print("  C2 FAILED: a planted-absent quotation was found"); sys.exit(1)
        continue
    if hits: ok += 1
    else: miss += 1; print(f"  MISSING: {q[:70]}")

print()
print(f"C1 planted-present quotation found : {'ok' if find(norm(QUOTES[-2][1]), NORMED) else 'FAILED'}")
print(f"C2 planted-absent quotation missing: ok")
print(f"C3 normalisation mattered on {naive_failures} quotation(s) (want >= 1)")
if naive_failures < 1:
    print("  C3 FAILED: normalisation changed nothing, so this checker is not")
    print("  testing the defect 168 found. Reported rather than suppressed, because")
    print("  the citation results above are still valid.")
print()
print(f"resolved={ok} missing={miss}")
sys.exit(1 if miss else 0)
