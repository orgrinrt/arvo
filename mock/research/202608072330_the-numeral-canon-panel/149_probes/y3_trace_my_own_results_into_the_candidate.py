#!/usr/bin/env python3
"""y3: trace 141's nine findings and twelve replacements into the candidate.

The dispatch asks whether anything of mine has been dropped, widened, or promoted
above the rung it was claimed at. A prose read of seven files answers that badly,
because the thing I am looking for is an ABSENCE and a reader does not notice one.
So this counts.

Three classes are traced, and they are traced differently because they are named
differently in the corpus:

  findings      `141` F1 .. F9, cited as "141 F<n>" or "F<n>" inside a 141 context
  replacements  A .. L, cited in prose as "141 replacement <X>" or "141's <X>"
  positions     the eleven I carried forward, which have no ids at all

Only the first is fully mechanical. The second is half mechanical: the corpus does
use the phrase "replacement" and I can count occurrences, but a replacement carried
without being named by letter will not be found and that is a limitation I state
rather than hide. The third is not mechanical at all and is hand-checked in the file.

CONTROLS:
  K1. A finding id that does not exist (`141` F99) must be found nowhere. If it is
      found, the matcher is matching on something other than the id.
  K2. A finding id that certainly IS carried (`141` F7, which `145` A2 and `146`
      section 5.5 both rest on) must be found. If it is not, the matcher is blind.
  K3. The possessive trap `146` reported against its own instrument: `F9's` must not
      be read as an id called `F9'`. Checked by asserting F9 is found in a file that
      only ever writes `F9's`.
"""

import re
import sys
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent

FILES = [
    "142_muratori_reply_the_repair_was_dead_on_arrival.md",
    "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
    "144_fog_the_weighting_half_measured.md",
    "145_leroy_formalising_the_strategy_object.md",
    "146_leroy_the_canon_candidate_for_the_strategy_object.md",
]

# 141's own finding list, from its section 7.
MY_FINDINGS = {
    "F1": "a lossless storage container contributes zero classes",
    "F2": "the accumulator is answer-visible exactly at signed saturating",
    "F3": "reduction mod 2^W absorbs a prior reduction",
    "F4": "the minimum slack admitting fusion, per cell",
    "F5": "the residual unit is a rounding relocation, characterised biconditionally",
    "F6": "the declared slack permits a mean of 41.74 answers of 64",
    "F7": "the fused lowering is the exact-intermediate policy, bit-identical",
    "F8": "the class count is monotone non-decreasing and not strict",
    "F9": "the count does not move with W for these axes, and an axis can break that",
}

MY_REPLACEMENTS = {
    "A": "keep the firewall, gate the fused arm on a const predicate",
    "B": "spell the fractional shift as an arithmetic shift right",
    "C": "select the exact-intermediate axis position rather than declaring slack",
    "D": "if a tolerance is wanted, make it an axis position not a modifier",
    "E": "compose the firewall with Q51's denotation repair",
    "F": "restate F2 as monotone non-decreasing",
    "G": "state the count once as a quotient over the observation set",
    "H": "state the count as a function rather than as an absence",
    "I": "carry the W result with its own predicate",
    "J": "predicate the invisibility: column versus accumulator",
    "K": "the storage question and the fusion question are one congruence question",
    "L": "a minimising choice is cost where the reduction is a congruence, policy where not",
}


def find_ids(text, fid):
    """Match `141` F<n> or a bare F<n>, refusing a possessive and refusing a longer id."""
    # not preceded by a digit or letter, not followed by a digit or by an apostrophe-s
    pat = re.compile(r"(?<![0-9A-Za-z-])" + fid + r"(?![0-9])(?!'s)")
    hits = []
    for i, line in enumerate(text.splitlines(), 1):
        if pat.search(line):
            hits.append(i)
    return hits


def find_possessive(text, fid):
    pat = re.compile(r"(?<![0-9A-Za-z-])" + fid + r"'s")
    return [i for i, line in enumerate(text.splitlines(), 1) if pat.search(line)]


print("y3: 141's results traced into the later files of its own topic\n")

texts = {}
for f in FILES:
    p = BASE / f
    if not p.exists():
        print(f"MISSING: {f}")
        sys.exit(1)
    texts[f] = p.read_text()

print("=== findings ===")
print(f"{'id':<5} {'142':>6} {'143':>6} {'144':>6} {'145':>6} {'146':>6}   carried?")
missing = []
for fid, gloss in MY_FINDINGS.items():
    counts = []
    total = 0
    in_candidate = 0
    for f in FILES:
        direct = len(find_ids(texts[f], fid))
        poss = len(find_possessive(texts[f], fid))
        n = direct + poss
        counts.append(n)
        total += n
        if f.startswith("146"):
            in_candidate = n
    tag = "in 146" if in_candidate else ("elsewhere only" if total else "NOWHERE")
    if not total:
        missing.append(fid)
    print(
        f"{fid:<5} {counts[0]:>6} {counts[1]:>6} {counts[2]:>6} {counts[3]:>6} {counts[4]:>6}   {tag}"
    )
print()
for fid in MY_FINDINGS:
    if not any(find_ids(texts[f], fid) or find_possessive(texts[f], fid) for f in FILES):
        print(f"  ! {fid} appears in no later file: {MY_FINDINGS[fid]}")

print("\n=== controls ===")
k1 = sum(len(find_ids(texts[f], "F99")) for f in FILES)
print(f"K1: nonexistent id F99 found {k1} times (must be 0)")
k2 = sum(len(find_ids(texts[f], "F7")) + len(find_possessive(texts[f], "F7")) for f in FILES)
print(f"K2: F7, which two files rest on, found {k2} times (must be > 0)")
f9_direct = sum(len(find_ids(texts[f], "F9")) for f in FILES)
f9_poss = sum(len(find_possessive(texts[f], "F9")) for f in FILES)
print(f"K3: F9 as a bare id {f9_direct}, as a possessive {f9_poss}; both counted, neither read as F9'")

print("\n=== replacements, by letter, in the candidate and in 145 ===")
print("(half mechanical: a replacement carried without being named by letter is invisible here)")
for f in ["145_leroy_formalising_the_strategy_object.md", "146_leroy_the_canon_candidate_for_the_strategy_object.md"]:
    t = texts[f]
    named = re.findall(r"replacement[s]?\s+([A-L])\b", t)
    named += re.findall(r"replacement\s+([A-L])-prime", t)
    print(f"  {f[:3]}: replacement letters named = {sorted(set(named))}")
    print(f"       occurrences of the word 'replacement' = {t.count('replacement')}")

print("\n=== which of my twelve replacements are named by letter anywhere ===")
allt = "\n".join(texts.values())
for k, gloss in MY_REPLACEMENTS.items():
    pat = re.compile(r"replacement[s]?\s+" + k + r"\b|replacement\s+" + k + r"-prime")
    n = len(pat.findall(allt))
    print(f"  {k}: {n:>3} named occurrences   {gloss}")

print("\nNOTE. A zero in the last block does NOT mean the replacement was dropped.")
print("It means it is not carried under my letter, which is the expected shape for the")
print("ones that became findings or clauses rather than staying labelled proposals.")
print("The file resolves each zero by hand and says which are carried unlabelled and")
print("which are genuinely absent.")
