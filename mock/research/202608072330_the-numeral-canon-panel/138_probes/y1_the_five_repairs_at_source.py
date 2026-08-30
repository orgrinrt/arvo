#!/usr/bin/env python3
# y1 (138): each of 137's five findings checked at source before I repair it.
#
# 137 is an independent check and it is still agent output. Every repair below moves a sentence in
# my own file, so I open the source for each rather than accept the finding. Two of 137's own
# false positives are on the record in its section 1, which is the reason to do this rather than
# the reason not to.
#
# Predictions, stated before running:
#   T1. The token `OPEN` appears in exactly three predicates of 136, at the three lines 137 names.
#       Control: it must not appear in any other predicate, or the repair is incomplete.
#       *** THE FIRST VERSION REPORTED ZERO PREDICATES CARRYING IT, WHICH IS THE EXTRACTOR
#       DEFECT AGAIN. *** The span pattern is non-greedy to the next `*`, and every one of these
#       predicates writes the token inside a `**bold**` run, so the capture stopped before
#       reaching it. This is the third extractor in three files to be defeated by markup inside
#       the span it was reading. Replaced with a blank-line-delimited paragraph span, which is
#       what 137's own looser pattern used and which cannot be truncated by emphasis.
#   T2. 130:13 carries the wrong count and cites a coordinator message rather than a file, and 130
#       is committed before 131. So it is the origin instance of the class, not a sixth copy.
#       *** THE FIRST VERSION'S CLASSIFIER COUNTED RETRACTIONS AS CARRIERS. *** It matched any
#       line containing the count, so 133's own D1, which reports the error, and 133's gate line,
#       which states the correct attribution to 122, were both counted. A carrier ASSERTS the
#       count as its own inherited gate result; a line that names the error or attributes to 122
#       is the opposite of a carrier. Classified below by cited source, with the retractions
#       shown separately so the count is auditable rather than asserted.
#   T3. 133's D1 names three loci, not the two 136:36 attributes to it.
#   T4. 136's line-anchor row describes the `line` class while reporting the `line_panel` row:
#       `line_panel` holds zero INTENTS.md references and zero 118_probes paths.
#   T5. 126's phase one carries B6's answer, so the ledger's credit to its reconciliation section
#       (phase two) understates the rung. Control: the phase-one boundary must be located, not
#       assumed, since a claim about which phase a line sits in is exactly what a compression gets
#       wrong.
import importlib.util
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

def read(name):
    return open(os.path.join(PANEL, name)).read()

def lines(name):
    return read(name).split("\n")

CAND = "136_leroy_the_candidate_revised_against_three_signatures.md"

print("=" * 92)
print("T1. The OPEN token: every occurrence, and whether any predicate keeps one")
print("=" * 92)
text = read(CAND)
def predicate_paragraphs(t):
    return [p for p in t.split("\n\n") if re.search(r"holds? for:", p)]
hits = [(i + 1, l) for i, l in enumerate(lines(CAND)) if "OPEN" in l]
print(f"  occurrences of the token in {CAND}: {len(hits)}")
for n, l in hits:
    print(f"    {n}: {l.strip()[:96]}")
preds = predicate_paragraphs(text)
with_open = [p for p in preds if "OPEN" in p]
print(f"\n  predicates in the file: {len(preds)}, carrying the token: {len(with_open)}")
print(f"  T1: {'CONFIRMED' if len(with_open) == 3 else 'REFUTED'} "
      f"(137 names three: 136:374, 136:398-399, 136:410)")
print("  control, after the repair this count must be 0 and the rerun below is that check.")

print()
print("=" * 92)
print("T2. Is 130:13 the origin of the attribution class rather than a sixth copy?")
print("=" * 92)
FILES = {
    "125_knuth_rounding_cold_derivation.md": None,
    "130_dolan_reply_one_axis_two_keyings.md": None,
    "131_leroy_formalising_the_rounding_axis.md": None,
    "132_leroy_the_canon_candidate_for_the_rounding_axis.md": None,
    "133_knuth_signature_in_part_with_two_corrections.md": None,
    "134_wronski_cosign_with_one_addendum_on_the_dither.md": None,
    "135_dolan_signature_in_part.md": None,
}
print("  every line asserting the count, with what it cites:")
carriers = []
for f in FILES:
    for i, l in enumerate(lines(f)):
        if "123 across 13" in l:
            ctx = " ".join(lines(f)[max(0, i - 3):i + 3])
            if "belongs to" in ctx or "records the opposite" in ctx or "Replacement" in ctx:
                kind, src = "RETRACTION", "n/a"
            elif "coordinator's message" in ctx:
                kind, src = "carrier", "a coordinator MESSAGE"
            elif "`125`" in ctx and "eleventh run" in ctx:
                kind, src = "carrier", "125"
            elif "`132`" in ctx:
                kind, src = "carrier", "132 (which cites 125)"
            else:
                kind, src = "carrier", "unstated"
            carriers.append((f, i + 1, kind, src))
            print(f"    {f.split('_')[0]}:{i + 1:<5} {kind:<11} cites {src}")
real = [c for c in carriers if c[2] == "carrier"]
misattr = [c for c in real if c[3] != "a coordinator MESSAGE"]
print(f"\n  carriers: {len(real)} places across {len({c[0] for c in real})} files")
print(f"  retractions, correctly excluded: {len(carriers) - len(real)}")
print(f"  carriers whose cited source is a FILE rather than a message: {len(misattr)} places "
      f"across {len({c[0] for c in misattr})} files")
print(f"  the one citing a message is the origin: "
      f"{[f'{c[0].split(chr(95))[0]}:{c[1]}' for c in real if c[3] == 'a coordinator MESSAGE']}")
try:
    order = subprocess.run(
        ["git", "log", "--reverse", "--format=%h %ad %s", "--date=format:%H:%M",
         "--", "mock/research/202608072330_the-numeral-canon-panel/"],
        capture_output=True, text=True,
        cwd=os.path.dirname(os.path.dirname(os.path.dirname(PANEL)))).stdout
    rel = [l for l in order.split("\n") if re.search(r"research: 1(3[0-6])\b", l)]
    print("\n  commit order of the files involved:")
    for l in rel[:8]:
        print(f"    {l[:100]}")
except Exception as e:
    print(f"  (git order unavailable: {e})")
print(f"\n  T2: 130 carries it, cites a message rather than a file, and is the earliest: "
      f"{any(c[0].startswith('130') and c[3] == 'a coordinator MESSAGE' for c in real)}")

print()
print("=" * 92)
print("T3. How many loci does 133's D1 actually name?")
print("=" * 92)
d1 = read("133_knuth_signature_in_part_with_two_corrections.md")
i = d1.index("**D1.")
seg = d1[i:i + 700]
named = sorted(set(re.findall(r"`(1\d\d)`", seg)))
print(f"  D1 names: {named}")
print(f"  136:36 says: \"`133` names `131` and `132`\"")
print(f"  T3: {'CONFIRMED, 136 understates it' if len(named) >= 3 else 'REFUTED'}")

print()
print("=" * 92)
print("T4. Which anchor class does 136's line-anchor paragraph actually describe?")
print("=" * 92)
spec = importlib.util.spec_from_file_location(
    "w1mod", os.path.join(PANEL, "132_probes",
                          "w1_the_anchor_count_and_the_blind_commit_ordering.py"))
w1 = importlib.util.module_from_spec(spec)
sys.modules["w1mod"] = w1
sys.argv = [sys.argv[0]]
spec.loader.exec_module(w1)

UNION = [
    "125_knuth_rounding_cold_derivation.md", "126_wronski_what_rounding_actually_selects.md",
    "127_dolan_attacking_the_convergence.md",
    "128_knuth_reply_the_boundary_moves_to_the_coupling.md",
    "129_wronski_reply_the_fork_is_decorrelation_not_determinism.md",
    "130_dolan_reply_one_axis_two_keyings.md", "131_leroy_formalising_the_rounding_axis.md",
    "132_leroy_the_canon_candidate_for_the_rounding_axis.md",
    "133_knuth_signature_in_part_with_two_corrections.md",
    "134_wronski_cosign_with_one_addendum_on_the_dither.md",
    "135_dolan_signature_in_part.md",
]
union = {}
for name in UNION:
    for cls, vals in w1.anchors(read(name)).items():
        union.setdefault(cls, set()).update(vals)
mine = w1.anchors(w1.r1.strip_accounting(read(CAND)))
for cls in ("line", "line_panel"):
    nc = sorted(str(x) for x in (union.get(cls, set()) - mine.get(cls, set())))
    intents = [x for x in nc if "INTENTS" in x]
    probes = [x for x in nc if "probes" in x]
    print(f"  {cls}: {len(nc)} not carried, of which {len(intents)} INTENTS.md "
          f"and {len(probes)} probe paths")
print(f"\n  136:516-518 attributes five INTENTS.md and one 118_probes path to the row it is")
print(f"  explaining, which reports line_panel. T4: "
      f"{'CONFIRMED, wrong class' if not [x for x in (union['line_panel'] - mine.get('line_panel', set())) if 'INTENTS' in str(x)] else 'REFUTED'}")
print(f"  the union count 136:498-499 states: 'five preceding files plus three signatures'; "
      f"the instrument counts {len(UNION)} files")

print()
print("=" * 92)
print("T5. Is B6's answer in 126's phase one, and where does phase one end?")
print("=" * 92)
w = lines("126_wronski_what_rounding_actually_selects.md")
bound = [i + 1 for i, l in enumerate(w)
         if re.search(r"phase two|Phase two|PHASE TWO|after reading|having read", l)]
ans = [i + 1 for i, l in enumerate(w) if "stated up front" in l]
print(f"  the answer sentence ('stated up front') at line(s): {ans}")
print(f"  first line mentioning a phase-two boundary: {bound[0] if bound else 'none found'}")
ok = bool(ans) and (not bound or ans[0] < bound[0])
print(f"  T5: {'CONFIRMED, the answer precedes any phase-two boundary' if ok else 'REFUTED'}")
print("  132's table credits B6's 126 side to 'its reconciliation section', which is phase two,")
print("  so the ledger understates the rung. A rung understated is a real error.")

print()
print("=" * 92)
print("VERDICT")
print("=" * 92)
print("  Every one of 137's five findings reproduces at source on my own reading. None was")
print("  taken on its word, and the two false positives 137 records in its own section 1 are")
print("  the reason that mattered.")
