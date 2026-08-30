"""q4: are my three headline results represented at the strength they support?

NOT A BENCHMARK. Exact arithmetic plus a parse of two committed panel files.

The coordinator asks me to check `146`'s representation of the exact rational LP,
the 90.2% against the 44.3% outlier, and the portability inversion. A cosignature
that reads the prose and nods is worth nothing, so this checks mechanically.

PART A. `146` section 4 says `144` F144-2 "decides that `139`'s compromise arm
loses by at least one unit at every point of the simplex, by exact vertex
enumeration over rationals rather than at 2001 sampled points". My F144-2's
predicate says `cost coordinates = 2`. `139`'s probe swept the arm embedded in
THREE coordinates with the third identically zero, and my own F144-4 is the
finding that such a coordinate makes every arm weakly selectable. So the two
readings of "the simplex" give different answers and the candidate's sentence
does not say which. Part A recomputes both.

PART B. Which of my findings does the candidate carry with a predicate, and which
bare? A canon candidate carrying a predicated finding without its predicate is
the failure I13's notation exists to prevent, so this is worth counting rather
than asserting.

PREDICTIONS, before running:
  GG1 the optimum is +1 in two coordinates and exactly 0 in the three-coordinate
      embedding `139` actually swept, so `146` section 4's sentence is true only
      under the reading my predicate names and false under the one its own prose
      invites.
  GG2 `146` carries every one of my nineteen finding ids somewhere in its text,
      and two of them, F144-10 and F144-17, appear ONLY in the paragraph
      accounting for their absence. That is `146` section 10's self-disabling
      guard firing, and it means the id count is not evidence of carriage.
  GG3 44.3% appears nowhere in `146` as a magnitude, so the candidate creates no
      new hazard; what it does is drop the correction, leaving `139`'s number
      uncorrected in the file the candidate compresses.
  GG4 at least one finding is carried with a predicate and at least one bare, so
      the classifier below is measuring something.

CONTROLS:
  HH1 THE CASE THAT MUST FAIL. A finding id that does not exist must be reported
      absent from both files. If the parse reports it present, it is matching
      something other than an id.
  HH2 the two-coordinate and three-coordinate computations must use the same
      solver, so a difference between them is the embedding rather than the code.
"""

import os
import re
import sys
from fractions import Fraction as F

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.normpath(os.path.join(HERE, "..", "144_probes")))
from exact_lp import min_regret  # noqa: E402

PANEL = os.path.normpath(os.path.join(HERE, ".."))
F144 = os.path.join(PANEL, "144_fog_the_weighting_half_measured.md")
F146 = os.path.join(PANEL, "146_leroy_the_canon_candidate_for_the_strategy_object.md")

fail = []

print("=" * 80)
print("PART A. which simplex does F144-2 hold on?")
print("=" * 80)
TWO = [(F(0), F(10)), (F(10), F(0)), (F(6), F(6))]
THREE = [(F(0), F(10), F(0)), (F(10), F(0), F(0)), (F(6), F(6), F(0))]
NAMES = ["endpoint-A", "endpoint-B", "compromise-C"]
for label, arms in (("two coordinates, my F144-2 predicate", TWO),
                    ("three coordinates, the embedding 139 swept", THREE)):
    print(f"  {label}:")
    for i, nm in enumerate(NAMES):
        print(f"    {nm:<14} min over the simplex of (its cost minus the best other) = "
              f"{min_regret(arms, i)}")
t2, t3 = min_regret(TWO, 2), min_regret(THREE, 2)
gg1 = t2 == 1 and t3 == 0
print(f"  GG1 two coordinates give {t2} and three give {t3}: "
      f"{'CONFIRMED' if gg1 else 'REFUTED'}")
if not gg1:
    fail.append("GG1")
print(f"  HH2 both computed by the same imported solver, so the difference is the embedding.")
print()
print("  reading: the claim is true and its truth depends on a dimension the candidate's")
print("  sentence does not carry. In the three-coordinate embedding the arm TIES rather than")
print("  losing, because every arm scores zero when all the weight goes on the constant")
print("  coordinate, which is my own F144-4 and which the candidate states three sections")
print("  later without connecting the two.")

print()
print("=" * 80)
print("PART B. carried with a predicate, carried bare, or only accounted for")
print("=" * 80)
with open(F146) as fh:
    body146 = fh.read()
with open(F144) as fh:
    body144 = fh.read()

ids = sorted({m for m in re.findall(r"F144-\d+", body144)},
             key=lambda s: int(s.split("-")[1]))
print(f"  finding ids declared in 144: {len(ids)}")

# split 146 into sections at any heading, and note which contain a predicate block
sections = re.split(r"\n(?=#{2,4} )", body146)
ACCOUNTING = "What the `not carried` sets are"

rows = []
for fid in ids:
    hits = [s for s in sections if re.search(rf"\b{fid}\b", s)]
    only_accounting = bool(hits) and all(ACCOUNTING in s or "not carried" in s for s in hits)
    with_pred = any("holds for:" in s for s in hits)
    rows.append((fid, len(hits), only_accounting, with_pred))

print(f"  {'id':<10}{'sections':>9}{'only in accounting':>20}{'a predicate nearby':>20}")
for fid, n, acct, pred in rows:
    print(f"  {fid:<10}{n:>9}{str(acct):>20}{str(pred):>20}")

accounted = [r[0] for r in rows if r[2]]
bare = [r[0] for r in rows if r[1] and not r[2] and not r[3]]
predicated = [r[0] for r in rows if r[3]]
print()
print(f"  only in the accounting paragraph: {accounted}")
print(f"  carried with a predicate in the same section: {predicated}")
print(f"  carried bare: {bare}")
gg2 = set(accounted) == {"F144-10", "F144-17"}
print(f"  GG2 -> {'CONFIRMED' if gg2 else 'REFUTED'}")
if not gg2:
    fail.append("GG2")
print()
print("  BOUND ON PART B, stated because the number above reads sharper than it is.")
print("  This is a proximity test: it asks whether a `holds for:` block sits in the same")
print("  heading-delimited section as the id. `146` deliberately separates its argument-kind")
print("  discussion (section 4) from its predicate blocks (section 5), so a finding discussed")
print("  in the first and predicated in the second reads as bare here and is not. The count is")
print("  a screen for where to look, not a verdict. The one case I verified by hand is F144-2,")
print("  in Part A, and it is bare in the strong sense: no section of 146 carries its")
print("  `cost coordinates = 2` dimension, and Part A shows the claim needs it.")
print()
gg4 = bool(predicated) and bool(bare)
print(f"  GG4 the classifier separates: {'PASS' if gg4 else 'FAIL, it is constant'}")
if not gg4:
    fail.append("GG4")

print()
print("=" * 80)
print("GG3. does the candidate reuse 44.3% as a magnitude?")
print("=" * 80)
occurrences = [line for line in body146.splitlines() if "44.3" in line]
print(f"  lines containing 44.3: {len(occurrences)}")
for line in occurrences:
    print(f"    {line.strip()[:100]}")
gg3 = all("not carried" in line or "F144-10" in line for line in occurrences)
print(f"  GG3 every occurrence is inside the accounting of its own absence: "
      f"{'CONFIRMED' if gg3 else 'REFUTED'}")
if not gg3:
    fail.append("GG3")
print("  so the candidate creates no new hazard. What it does is drop the correction, and")
print("  139's 44.3% stays in 139 with nothing beside it saying it is a second-percentile")
print("  outlier of a distribution whose median is 90.2%.")

print()
print("=" * 80)
print("HH1. the case that must fail")
print("=" * 80)
ghost = "F144-99"
present = bool(re.search(rf"\b{ghost}\b", body146)) or bool(re.search(rf"\b{ghost}\b", body144))
print(f"  {ghost} reported present: {present} -> {'PASS' if not present else 'FAIL'}")
if present:
    fail.append("HH1")

print()
print("=" * 80)
print(f"control failures: {len(set(fail))} {sorted(set(fail))}")
print("=" * 80)
raise SystemExit(1 if fail else 0)
