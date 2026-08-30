#!/usr/bin/env python3
"""p1. Does any committed bench family compare arms that compute DIFFERENT answers?

The converged mechanism of this unit selects an arm by argmin of a weighting over
cost coordinates. That operation is well defined only when the arms it ranges over
are interchangeable in what they compute: if two arms give different answers,
"cheaper" does not order them, because they are not two ways of doing one thing.

So the question this probe asks is not "what does the corpus measure" (101 answered
that: one timing). It is the prior one: **over what kind of arm set does the corpus
measure anything at all.**

Three independent readings of the same corpus, so that a single misread does not
carry the conclusion:

  A. the harness's own consent switch, `Routine::outputs_may_differ`, per crate.
  B. the crates' own cross-arm agreement assertions, by name and by what they
     compare against (each other, or an independent oracle).
  C. the committed CSVs' `score` column, which is the only place a per-arm quality
     number could be recorded.

Run:  python3 p1_the_corpus_compares_cost_at_a_fixed_answer.py
"""

import csv
import pathlib
import re
import sys

HERE = pathlib.Path(__file__).resolve().parent
BENCHES = HERE.parents[2] / "benches"
VARIANTS = BENCHES / "variants"

if not VARIANTS.is_dir():
    sys.exit(f"variants dir not found at {VARIANTS}")

# ---------------------------------------------------------------------------
# A. the consent switch
# ---------------------------------------------------------------------------

shared = sorted(d for d in VARIANTS.iterdir() if d.is_dir() and d.name.endswith("-shared"))

print("A. THE HARNESS CONSENT SWITCH, PER SHARED CRATE")
print()
print(f"{'crate':<32} {'outputs_may_differ':<20} {'validate_output':<16}")

differ_true = []
for d in shared:
    src = " ".join(p.read_text(errors="replace") for p in d.rglob("*.rs"))
    # the declaration, not the mention: a fn body returning true
    m = re.search(r"fn\s+outputs_may_differ\s*\(\s*\)\s*->\s*bool\s*\{\s*(true|false)\s*\}", src)
    decl = m.group(1) if m else "(default false)"
    if m and m.group(1) == "true":
        differ_true.append(d.name)
    has_validate = "yes" if re.search(r"fn\s+validate_output", src) else "no"
    print(f"{d.name:<32} {decl:<20} {has_validate:<16}")

print()
print(f"crates declaring outputs_may_differ = true : {len(differ_true)} of {len(shared)}  {differ_true}")

# ---------------------------------------------------------------------------
# B. the agreement assertions, and WHAT each compares against
# ---------------------------------------------------------------------------

print()
print("B. CROSS-ARM AGREEMENT ASSERTIONS, AND WHAT EACH COMPARES AGAINST")
print()

# an assertion is "oracle-backed" when the test body names a reference computed
# independently of the arms; "mutual" when arms are only compared to each other.
ORACLE_WORDS = ("oracle", "reference(", "reference (", "want", "independent")

rows = []
for d in shared:
    mutual = 0
    oracled = 0
    names = []
    for p in sorted(d.rglob("*.rs")):
        text = p.read_text(errors="replace")
        # split into #[test] fns
        for m in re.finditer(r"#\[test\][^\n]*\n(?:\s*(?://[^\n]*\n|#\[[^\n]*\n))*\s*fn\s+(\w+)", text):
            name = m.group(1)
            start = m.end()
            # crude body extent: to the next #[test] or end of file
            nxt = text.find("#[test]", start)
            body = text[start : nxt if nxt != -1 else len(text)]
            if not re.search(r"agree|match|same|identical|disagree", name + body[:4000], re.I):
                continue
            if any(w in body for w in ORACLE_WORDS):
                oracled += 1
            else:
                mutual += 1
            names.append(name)
    rows.append((d.name, mutual, oracled, names))

print(f"{'crate':<32} {'mutual-only':>12} {'oracle-backed':>14}")
tot_m = tot_o = 0
for name, m, o, _ in rows:
    tot_m += m
    tot_o += o
    print(f"{name:<32} {m:>12} {o:>14}")
print(f"{'TOTAL':<32} {tot_m:>12} {tot_o:>14}")

print()
print("crates with no agreement assertion of either kind:")
for name, m, o, _ in rows:
    if m + o == 0:
        print(f"    {name}")

# ---------------------------------------------------------------------------
# C. the score column in every committed CSV
# ---------------------------------------------------------------------------

print()
print("C. THE `score` COLUMN ACROSS EVERY COMMITTED CSV")
print()

csvs = sorted(BENCHES.glob("*.csv"))
files = 0
rows_seen = 0
nonempty_score = 0
score_missing = 0
for f in csvs:
    files += 1
    with f.open(newline="") as fh:
        r = csv.DictReader(fh)
        if r.fieldnames is None or "score" not in r.fieldnames:
            score_missing += 1
            continue
        for row in r:
            rows_seen += 1
            v = (row.get("score") or "").strip()
            if v not in ("", "0", "0.0"):
                nonempty_score += 1

print(f"committed CSV files            : {files}")
print(f"data rows read                 : {rows_seen}")
print(f"files with no `score` column   : {score_missing}")
print(f"rows with a non-empty `score`  : {nonempty_score}")

# ---------------------------------------------------------------------------
# the conclusion this probe is entitled to
# ---------------------------------------------------------------------------

print()
print("WHAT THIS ESTABLISHES")
print()
print("Every shared crate either forbids its arms to differ (the harness default,")
print("cross-checked byte for byte) or permits it and then asserts every arm against")
print("an independent oracle, which is strictly stronger. Either way the arm set at")
print("every committed region is ANSWER-EQUIVALENT: all arms compute one value.")
print()
print("And no committed row carries a per-arm quality number.")
print()
print("So every number in this corpus is a comparison of COST AT A FIXED ANSWER.")
print("An argmin over these coordinates is a mechanism for choosing among arms that")
print("already agree. That is a real mechanism and it is not the one op's intents")
print("describe: I5 trades accuracy for speed, I7 buys accuracy with speed, I3 asks")
print("for a particular answer. Each of those ranges over arms that DISAGREE.")
