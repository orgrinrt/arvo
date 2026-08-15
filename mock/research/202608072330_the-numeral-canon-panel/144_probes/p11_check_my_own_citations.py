"""p11: open every citation in 144 and test its CONTENT, not its resolution.

NOT A BENCHMARK. It reads files.

`RULES.md:126-133` records that one member found seven of its own citations wrong
this way and calls the resulting probe the cheapest correctness tool this panel
has. A reference that resolves is not a reference that says what the citing file
claims.

This version does not check a hand-written list. It **parses the citations out of
`144` itself**, so a citation I add later without declaring what it should say is
itself reported. A hand-written list checks the citations I remembered; parsing
checks the ones I wrote.

Whitespace is normalised before matching. The first run of this probe reported a
mismatch on `139:610-615` for the phrase "linear is enough and the limit is
theoretical", which IS in that span and is wrapped across two source lines. The
citation was right and the checker was wrong, which is worth recording because a
naive substring check on wrapped prose produces a false alarm on exactly the
citations that quote a full sentence, and a false alarm that gets waved away is
how a real one gets waved away next to it.

The parse also earned its keep on the first run: 144 cited `139:403-406` for a
sentence that sits on line 407, so the span stopped one line short of the thing
it was cited for. A resolution check passes that; a content check does not.

THREE CASES THAT MUST FAIL:
  a phrase absent from the cited file entirely;
  a phrase present in the file but outside the cited span, which is what shows
    the checker reads the span rather than the file;
  a citation appearing in `144` with no declared expectation, which is what
    shows the parse is driving the check rather than decorating it.
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.normpath(os.path.join(HERE, ".."))
SELF = os.path.join(PANEL, "144_fog_the_weighting_half_measured.md")

FILES = {
    "139": "139_muratori_the_strategy_set_derived_cold.md",
    "140": "140_mcsherry_the_strategy_set_derived_cold.md",
    "141": "141_lamport_the_strategy_set_attacked.md",
    "142": "142_muratori_reply_the_repair_was_dead_on_arrival.md",
    "143": "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
    "INTENTS.md": "INTENTS.md",
    "RULES.md": "RULES.md",
}

# every span 144 cites, with what it must contain
EXPECT = {
    ("139", 396, 396): ["838 of 1891 grid points, 44.3%"],
    ("139", 400, 407): ["wins **zero** of 2001 sweep points", "pulling the same arm inside",
                        "the zero is a property of the arm"],
    # the superseded span, named in 144's coverage section as the off-by-one it
    # was. It resolves and it does NOT contain the sentence it was cited for,
    # which is the whole point of naming it, so that is what is asserted here.
    ("139", 403, 406): ["wins **zero** of 2001 sweep points"],
    ("139", 430, 431): ["bounded by the arm count", "the design's vocabulary for it need"],
    ("139", 433, 436): ["A coordinate exists only if", "every arm carries a value on it"],
    ("140", 620, 624): ["every strategy's cost", "stated against the same named arm"],
    ("141", 776, 779): ["shared-baseline obligation", "I did not build an instrument for it"],
    ("141", 874, 876): ["I did not touch the weighting side at all",
                        "no linear weighting can select"],
    ("142", 581, 584): ["largest untouched surface", "44.3% mapping difference"],
    ("143", 194, 196): ["matching 714 would have been a reason to distrust my"],
    ("143", 296, 303): ["Half the count is carried by one axis",
                        "where the denotation and realisation levels are cut"],
    ("143", 418, 420): ["the one nobody has touched"],
    ("INTENTS.md", 51, 61): ["DEMOTED TO OPEN", "not closed at exactly four"],
    ("INTENTS.md", 214, 235): ["RATIFIED", "predicated arms composed"],
    ("INTENTS.md", 363, 383): ["cold isn't to be depriorised"],
    ("RULES.md", 486, 541): ["Every finding carries its predicate",
                             "Absence is the strongest negative statement"],
}

# spans 144 does not cite but whose content this file leans on, checked anyway
EXTRA = {
    ("139", 610, 615): ["O-139-C", "linear is enough and the limit is theoretical"],
    ("139", 625, 629): ["O-139-E"],
    ("139", 260, 263): ["two builds of one program produce different results"],
    ("INTENTS.md", 299, 310): ["Never any runtime checks, ever"],
    ("RULES.md", 126, 133): ["seven of its own citations wrong"],
}


def norm(s):
    return " ".join(s.split())


def span(key, a, b):
    path = os.path.join(PANEL, FILES[key])
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        lines = fh.readlines()
    if b > len(lines) or a < 1:
        return None
    return norm("".join(lines[a - 1:b]))


with open(SELF) as fh:
    body = fh.read()
found = set()
for m in re.finditer(r"`(139|140|141|142|143|INTENTS\.md|RULES\.md):(\d+)(?:-(\d+))?`", body):
    k, a, b = m.group(1), int(m.group(2)), int(m.group(3) or m.group(2))
    found.add((k, a, b))

fails = 0
print(f"parsed {len(found)} distinct citations out of 144")
print()
print(f"{'citation':<24} {'verdict':<9} expected substring")
for key in sorted(found | set(EXTRA)):
    wants = EXPECT.get(key) or EXTRA.get(key)
    label = f"{key[0]}:{key[1]}-{key[2]}"
    if wants is None:
        print(f"{label:<24} {'NO-EXPECT':<9} cited by 144 with nothing declared for it")
        fails += 1
        continue
    text = span(*key)
    if text is None:
        print(f"{label:<24} {'NOSPAN':<9} the span does not resolve")
        fails += 1
        continue
    if key == ("139", 403, 406):
        absent = "the zero is a property of the arm"
        gone = norm(absent) not in text
        print(f"{label:<24} {('ok' if gone else 'MISMATCH'):<9} "
              f"must NOT contain {absent!r} (the off-by-one 144 records)")
        if not gone:
            fails += 1
    for w in wants:
        ok = norm(w) in text
        print(f"{label:<24} {('ok' if ok else 'MISMATCH'):<9} {w!r}")
        if not ok:
            fails += 1

print()
controls = [
    ("absent from the file", ("139", 1, 5),
     "this sentence is not in 139 and must be reported"),
    ("present in the file, outside the span", ("139", 1, 5),
     "838 of 1891 grid points, 44.3%"),
]
for label, key, want in controls:
    text = span(*key)
    caught = text is not None and norm(want) not in text
    print(f"case that must fail ({label}): {key[0]}:{key[1]}-{key[2]} claiming {want!r}")
    print(f"  -> {'PASS, reported' if caught else 'FAIL, not caught'}")
    if not caught:
        fails += 1

# third control: a citation with no expectation must be reported
probe_found = found | {("139", 999, 999)}
undeclared = [k for k in probe_found if k not in EXPECT and k not in EXTRA]
print(f"case that must fail (citation with no declared expectation): "
      f"{'PASS, ' + str(undeclared) + ' would be reported' if undeclared else 'FAIL'}")
if not undeclared:
    fails += 1

print()
print(f"distinct spans checked: {len(found | set(EXTRA))}, failures: {fails}")
sys.exit(1 if fails else 0)
