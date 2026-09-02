#!/usr/bin/env python3
"""g9: every file:line in 137, opened and its CONTENT tested, not merely
resolved. Whitespace normalised and blockquote markers stripped on both sides;
neither can make an absent phrase appear. Mutation-tested three ways at the end,
because a checker that has never failed has not been tested either."""
import re, pathlib, sys
H = pathlib.Path(__file__).resolve().parent.parent
A = H.parents[2]

def norm(s):
    s = re.sub(r"^\s*(>|///|//!|\*)\s?", "", s, flags=re.M)
    return re.sub(r"\s+", " ", s).strip()

def f(pat): return next(H.glob(pat))

CITES = [
 ("op's instruction forbidding a hedge token in a predicate",
  H/"INTENTS.md", (237,247), 'No adding "unsure" into the predicate'),
 ("INTENTS.md's gloss: it should write nothing there",
  H/"INTENTS.md", (244,247), "It should write nothing there"),
 ("I13 scopes its own elaboration out",
  H/"INTENTS.md", (263,267), "is **not** part of what was ratified"),
 ("136 refuses to fill a predicate with an unmeasured value",
  f("136_*.md"), (265,269), "Filling a predicate with a value nobody measured"),
 ("136 writes domain: OPEN into the non-commutation predicate",
  f("136_*.md"), (372,376), "domain: OPEN, and the clause claims nothing until it is stated"),
 ("136 writes W, F and signedness: OPEN into the variance predicate",
  f("136_*.md"), (395,400), "W, F and\nsignedness: OPEN"),
 ("136 writes domain, W, F and signedness: OPEN into the entropy predicate",
  f("136_*.md"), (408,411), "domain, W, F and signedness: OPEN"),
 ("136 exempts the operation dimension on a principled test",
  f("136_*.md"), (144,148), "a characterisation of a map is not a claim about an operation"),
 ("136 says signedness does not stand in for domain",
  f("136_*.md"), (249,252), "`signedness` is present and does not stand in for it"),
 ("136 gives the class one mechanism",
  f("136_*.md"), (269,275), "a predicate's dimensions read off the clause above rather than off the argument underneath"),
 ("131 R3's compound predicate with a shared leading clause",
  f("131_*.md"), (169,174), "domain closed under negation, threads any, stated on the equivariance argument. For the\nnon-commutation"),
 ("131 R5's compound predicate, same shape",
  f("131_*.md"), (203,210), "For the\nkeying divergence, one input shape"),
 ("131 R6 carries no domain dimension",
  f("131_*.md"), (221,224), "toolchain = the pinned nightly, edition 2021, crate type = library"),
 ("136 names the five attribution locations",
  f("136_*.md"), (36,38), "five places across four files"),
 ("136 says 133 names 131 and 132",
  f("136_*.md"), (36,37), "`133` names `131` and `132`"),
 ("133 D1 in fact names three, including 130",
  f("133_*.md"), (197,200), "and `130`'s gate note citing the coordinator's message"),
 ("130 carries the count from a coordinator message",
  f("130_*.md"), (11,15), "per the coordinator's\nmessage the eleventh run of the gate already stands at 123 across 13"),
 ("136 section 10's six-files count",
  f("136_*.md"), (489,492), "Six files have now inherited a gate count through it"),
 ("125 section 10 records twelve of thirteen at 108",
  f("125_*.md"), (462,467), "Twelve of the thirteen `*-shared`\ncrates completed green: 108 tests"),
 ("136 misreads its own line-anchor row",
  f("136_*.md"), (514,519), "five are `INTENTS.md` line references from other members' gate sections"),
 ("136's bolded no-drop claim and the eight signature files",
  f("136_*.md"), (509,513), "including the eight signature files' own anchors"),
 ("136 describes the union as five preceding plus three",
  f("136_*.md"), (497,500), "the five preceding files of this topic plus the three signatures"),
 ("136's misdirected F131-6 citation",
  f("136_*.md"), (63,67), "`131` F131-6's vocabulary count is unchallenged by any signature"),
 ("F131-6 is the staged-narrowing finding",
  f("131_*.md"), (553,556), "Staged narrowing equals direct narrowing for the directed modes"),
 ("125 T8 holds on one-signed domains too",
  f("125_*.md"), (283,291), "This holds on\none-signed domains too"),
 ("132 5.4's body says one-signed domains included",
  f("132_*.md"), (346,354), "one-signed domains included"),
 ("132 5.4's predicate excludes it",
  f("132_*.md"), (355,358), "domain closed under negation"),
 ("132 5.3's headline contradicts its own second bullet",
  f("132_*.md"), (320,332), "no member carries more than one of the first three"),
 ("128's five convergences, verbatim",
  f("128_*.md"), (235,248), "Five convergences\nbetween `126` and my phase one were reached blind by both of us"),
 ("129's two, verbatim",
  f("129_*.md"), (180,196), "the general shape of the answer to the brief's\nquestion"),
 ("126 states the shape in phase one",
  f("126_*.md"), (24,27), "My answer, stated up front and argued below: neither"),
 ("133 confirms B1..B6 as his phase-one content",
  f("133_*.md"), (162,166), "B1 through B6 are all phase-one content of\nmine"),
 ("134 confirms B6 and the six count",
  f("134_*.md"), (91,94), "so the union is six"),
 ("x87 is the Intel FPU, named in 126",
  f("126_*.md"), (222,225), "x87's 80-bit intermediate rounding"),
]

ok = bad = 0
for label, path, (a,b), phrase in CITES:
    if not path.exists():
        print(f"FAIL  {label}: no such path {path}"); bad += 1; continue
    seg = norm("\n".join(path.read_text(errors="replace").splitlines()[a-1:b]))
    if norm(phrase) in seg: ok += 1
    else:
        print(f"FAIL  {label}\n      {path.name}:{a}-{b}\n      wanted: {norm(phrase)[:88]}"); bad += 1
print(f"\ncitations checked: {ok+bad}   ok: {ok}   failed: {bad}")

print("\n--- mutation test, three ways ---")
for lbl, path, span, phrase in [
  ("a phrase op did not say", H/"INTENTS.md", (237,247), 'Always add "unsure" to the predicate'),
  ("a real phrase at the wrong span", f("131_*.md"), (10,20), "toolchain = the pinned nightly"),
  ("a real phrase in the wrong file", f("125_*.md"), (283,291), "Six files have now inherited"),
]:
    seg = norm("\n".join(path.read_text(errors="replace").splitlines()[span[0]-1:span[1]]))
    print(f"  {'CAUGHT ' if norm(phrase) not in seg else 'MISSED '} {lbl}")
sys.exit(1 if bad else 0)
