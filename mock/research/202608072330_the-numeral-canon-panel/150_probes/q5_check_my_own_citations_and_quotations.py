"""q5: open every citation and every quotation in 150 and test its content.

NOT A BENCHMARK. It reads files.

`144_probes/p11` checked line-anchored citations. This file's load-bearing
references are mostly VERBATIM QUOTATIONS without line anchors, because my
dissents rest on what `145` and `146` say and on what my own `144` says. A
quotation is a citation with a stricter obligation, so both kinds are checked
here and the quotations are checked exactly.

Markdown is stripped from both sides before matching. `145` z7 had two verbatim
citations fail on the source's own backticks and emphasis, and `146` section 10
records that as the fifth instance of the class in this panel.

**This probe made it the sixth, with a token none of the five used.** My first run
reported a mismatch on a quotation from `144` that is verbatim correct. The
passage is a BLOCKQUOTE, so every source line begins `> `, and collapsing
whitespace leaves a stray `>` in the middle of the sentence. Stripping backticks
and emphasis, which is what the five earlier instances needed, does not touch it.
So the class is not "backticks defeat a substring check", it is "any markdown the
normaliser does not know about does", and each instance has arrived with a
different token. The normaliser now strips leading blockquote markers as well, and
the three mutation controls confirm the wider stripping cannot manufacture a hit.

A second mismatch in the same run was mine and simpler: I quoted an `INTENTS.md`
heading with the wrong capitalisation. Case is left significant on purpose,
because a case-insensitive check would stop catching a real misquote, so the
expectation was corrected rather than the comparison loosened.

THREE CASES THAT MUST FAIL:
  a phrase nobody wrote;
  a real phrase attributed to the wrong file;
  a real phrase attributed to the wrong span of the right file.
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.normpath(os.path.join(HERE, ".."))

FILES = {
    "139": "139_muratori_the_strategy_set_derived_cold.md",
    "144": "144_fog_the_weighting_half_measured.md",
    "145": "145_leroy_formalising_the_strategy_object.md",
    "146": "146_leroy_the_canon_candidate_for_the_strategy_object.md",
    "INTENTS.md": "INTENTS.md",
    "RULES.md": "RULES.md",
    "z3out": os.path.join("145_probes", "z3_output.txt"),
}


def norm(s):
    s = re.sub(r"(?m)^\s*>+\s?", "", s)          # blockquote markers, the sixth token
    s = s.replace("`", "").replace("*", "").replace("_", "")
    return " ".join(s.split())


def whole(key):
    p = os.path.join(PANEL, FILES[key])
    if not os.path.exists(p):
        return None
    with open(p) as fh:
        return norm(fh.read())


def span(key, a, b):
    p = os.path.join(PANEL, FILES[key])
    if not os.path.exists(p):
        return None
    with open(p) as fh:
        lines = fh.readlines()
    if b > len(lines) or a < 1:
        return None
    return norm("".join(lines[a - 1:b]))


fails = 0

print("=" * 78)
print("PART A. line-anchored citations, parsed out of 150 itself")
print("=" * 78)
with open(os.path.join(PANEL, "150_fog_signature_in_part_on_the_strategy_object.md")) as fh:
    body = fh.read()
found = set()
for m in re.finditer(r"`(139|140|141|142|143|144|145|146|INTENTS\.md|RULES\.md)"
                     r":(\d+)(?:-(\d+))?`", body):
    found.add((m.group(1), int(m.group(2)), int(m.group(3) or m.group(2))))

EXPECT = {
    ("139", 396, 396): ["838 of 1891 grid points, 44.3%"],
    ("INTENTS.md", 51, 61): ["DEMOTED TO OPEN"],
    ("INTENTS.md", 214, 235): ["RATIFIED", "predicated arms composed"],
    ("INTENTS.md", 317, 331): ["The canon does not police what shape a law takes"],
    ("INTENTS.md", 363, 383): ["cold isn't to be depriorised"],
    ("RULES.md", 486, 541): ["Every finding carries its predicate"],
}
print(f"  parsed {len(found)} distinct line-anchored citations")
for key in sorted(found | set(EXPECT)):
    wants = EXPECT.get(key)
    label = f"{key[0]}:{key[1]}-{key[2]}"
    if wants is None:
        print(f"  {label:<22} NO-EXPECT  cited with nothing declared for it")
        fails += 1
        continue
    text = span(*key)
    if text is None:
        print(f"  {label:<22} NOSPAN")
        fails += 1
        continue
    for w in wants:
        ok = norm(w) in text
        print(f"  {label:<22} {('ok' if ok else 'MISMATCH'):<9} {w!r}")
        if not ok:
            fails += 1

print()
print("=" * 78)
print("PART B. verbatim quotations, which is what the dissents rest on")
print("=" * 78)
QUOTES = [
    ("145", "at `overflow = wrap` all four lowering arms in the topic's own arm set conform "
            "to every intermediate position"),
    ("145", "at `overflow = sat` the four split two and two"),
    ("145", "conflict and the file does not say so"),
    ("145", "z4's verdict block asserted D3's predicted outcome while the run reported it not "
            "established"),
    ("z3out", "overflow = wrap   conforming-arm counts observed: [2, 4]"),
    ("z3out", "toward_zero/wrap/stepwise          2 conforming arms"),
    ("144", "Every strategy's cost claim is stated against the same named arm, and the "
            "comparison is made after the weighting, on the weighted scalar."),
    ("144", "the arm a consumer asked for is not the arm they get"),
    ("144", "And section 4.3 is the other face of the same coin"),
    ("144", "It is not wrong, it is a different operation with a different purpose, and using "
            "one where the other is meant is the failure."),
    ("144", "whether the design wants one baseline for reporting or one for normalising. They "
            "are different objects and the measurement says they cannot be the same one."),
    ("146", "loses by at least one unit **at every point of the simplex**, by exact vertex "
            "enumeration over rationals rather than at 2001 sampled points"),
    ("146", "over 29 askable families under the conservative gate reading and 20 under the "
            "symmetric one"),
    ("146", "closure for the unsigned half and equivariance for the signed wrapping half"),
    ("146", "roughly one non-dominated arm in nine"),
    ("146", "the median 90.2% cross-target switch rate"),
]
for key, q in QUOTES:
    text = whole(key)
    ok = text is not None and norm(q) in text
    print(f"  {key:<8}{('ok' if ok else 'MISMATCH'):<10}{q[:82]!r}")
    if not ok:
        fails += 1

print()
print("=" * 78)
print("PART C. the negatives 150 asserts, checked as negatives")
print("=" * 78)
ABSENT = [
    ("146", "all four lowering arms",
     "150 D1 claims 146 does not carry 145's wrapping clause"),
    ("146", "split two and two",
     "150 D1 claims 146 does not carry 145's wrapping clause"),
    ("146", "44.3",
     "150 D4 claims 146 never reuses 139's figure as a magnitude"),
]
for key, q, why in ABSENT:
    text = whole(key)
    ok = text is not None and norm(q) not in text
    print(f"  {key:<8}{('absent, ok' if ok else 'PRESENT, MISMATCH'):<20}{q!r}")
    print(f"          {why}")
    if not ok:
        fails += 1
print("  NOTE: a negative is only as good as its phrasing. These test the exact wording 150")
print("  attributes to 145 and to 139; a paraphrase of the same claim in 146 would not be")
print("  caught, and I say so rather than presenting an absence as a proof.")

print()
print("=" * 78)
print("the three cases that must fail")
print("=" * 78)
CONTROLS = [
    ("nobody wrote this", "145", "the four arms conform under every rounding position whatsoever"),
    ("right phrase, wrong file", "139", "conflict and the file does not say so"),
    ("right phrase, wrong span", None, None),
]
for label, key, q in CONTROLS[:2]:
    text = whole(key)
    caught = text is not None and norm(q) not in text
    print(f"  {label:<26} -> {'PASS, reported' if caught else 'FAIL, not caught'}")
    if not caught:
        fails += 1
# wrong span: a real 139 phrase looked for at the top of 139
t = span("139", 1, 5)
caught = t is not None and norm("838 of 1891 grid points, 44.3%") not in t
print(f"  {'right phrase, wrong span':<26} -> {'PASS, reported' if caught else 'FAIL, not caught'}")
if not caught:
    fails += 1

print()
print(f"citations, quotations and negatives checked: "
      f"{len(found | set(EXPECT)) + len(QUOTES) + len(ABSENT)}, failures: {fails}")
sys.exit(1 if fails else 0)
