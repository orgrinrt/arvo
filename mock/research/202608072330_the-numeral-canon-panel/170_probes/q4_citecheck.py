#!/usr/bin/env python3
"""q4. Check every quotation in 170 by opening its source.

Two defects are already on record in this unit's checkers and both are folded in
here:
  168's fifth: `grep -F` fails on a quotation spanning a line break, because the
    source carries a newline where the quotation carries a space.
  169's seventh: whitespace normalisation is NECESSARY AND NOT SUFFICIENT,
    because a markdown corpus wraps quotations inside blockquotes and bolds the
    numbers inside them, and `> ` and `**` survive the whitespace fix.
  and an eighth, found by this probe's first run: CASE. A quotation lifted into
    the middle of a sentence legitimately lowercases its leading capital, which
    is ordinary English and not a misquotation, and a case-sensitive checker
    reports it as missing. One of this file's thirteen failed for exactly that
    and the quotation was correct. Comparison is casefolded below.

So normalisation here strips blockquote markers, emphasis, and inline-code
backticks, then collapses whitespace.

THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
  C1. A planted quotation known to be present must be FOUND, or the checker is
      not reading the files.
  C2. A planted quotation known to be absent must NOT be found, or the checker
      matches anything.
  C3. At least one real quotation must require the markup strip to resolve, or
      169's seventh defect does not bite in this file and the extra machinery is
      unexercised here (reported either way, not asserted).
"""
import re, sys

PANEL = ".."
WS = "/Users/orgrinrt/Dev/clause-dev/.claude/rules"

def norm(t, strip_markup=True):
    if strip_markup:
        t = re.sub(r'^\s*>\s?', '', t, flags=re.M)   # blockquote markers
        t = t.replace('**', '').replace('`', '')      # emphasis, inline code
        t = re.sub(r'(?<!\w)\*(?!\s)(.+?)(?<!\s)\*(?!\w)', r'\1', t)  # italics
    return re.sub(r'\s+', ' ', t).strip().casefold()

CITES = [
    ("169_kiselyov_the_chain_attacked.md", "let fm = full_mask(steps.len());"),
    ("169_kiselyov_the_chain_attacked.md", "the extractor finds exactly one exact match"),
    ("169_kiselyov_the_chain_attacked.md", "for 60 I have only that the rule existed"),
    ("169_kiselyov_the_chain_attacked.md", "a non-nearest projection onto the same representable set"),
    ("169_kiselyov_the_chain_attacked.md", "nearest-point onto a coarser set than the operations can land on"),
    ("169_kiselyov_the_chain_attacked.md", "which is where a placement might buy something"),
    ("169_kiselyov_the_chain_attacked.md", "semantically self-contained: it runs on pi . g . pi . f against pi . g . f and needs no rule to get there"),
    ("169_kiselyov_the_chain_attacked.md", "a corrected count that reads as a refutation is how a true finding gets retired"),
    ("169_kiselyov_the_chain_attacked.md", "whitespace normalisation is necessary and not sufficient"),
    ("168_mcsherry_the_chain_derived_cold.md", "There is no input, and no chain, on which any other placement is strictly closer"),
    ("168_mcsherry_the_chain_derived_cold.md", "chains with a control that finds 1330 counterexamples once the resolution stops being nearest-point"),
    ("how-to-run-a-panel.md", "what you settled, what you moved, and what you could not"),
    ("what-you-can-observe-is-what-you-guaranteed.md", "A guarantee about a type holds only over the operations through which the type can be observed."),
]

CONTROLS = [
    ("169_kiselyov_the_chain_attacked.md", "the sixth instrument defect", True),
    ("169_kiselyov_the_chain_attacked.md", "the pointwise claim is hereby refuted at every width", False),
]

def load(f):
    path = f"{WS}/{f}" if f in ("what-you-can-observe-is-what-you-guaranteed.md",
                                "how-to-run-a-panel.md") else f"{PANEL}/{f}"
    return open(path).read()

bodies = {}
def body(f):
    if f not in bodies:
        bodies[f] = load(f)
    return bodies[f]

print(f"{'source':<46} {'ws-only':>8} {'ws+markup':>10}  quotation")
resolved = markup_needed = missing = 0
for f, q in CITES:
    raw = body(f)
    a = norm(q, False) in norm(raw, False)
    b = norm(q, True) in norm(raw, True)
    if b:
        resolved += 1
        if not a:
            markup_needed += 1
    else:
        missing += 1
    print(f"{f[:46]:<46} {str(a):>8} {str(b):>10}  {q[:58]}")

print()
print(f"resolved {resolved}/{len(CITES)}, missing {missing}, "
      f"needing the markup strip {markup_needed}")
print()
print("=== CONTROLS ===")
ok = True
for f, q, want in CONTROLS:
    got = norm(q, True) in norm(body(f), True)
    label = "planted-present" if want else "planted-absent "
    print(f"C{'1' if want else '2'} {label} {'found' if got else 'not found'} : {got == want}")
    ok &= (got == want)
assert ok, "a planted control behaved wrongly, so this checker's verdicts mean nothing"
print(f"C3 the markup strip was load-bearing on {markup_needed} of {len(CITES)} quotations "
      f": {'yes' if markup_needed else 'no, unexercised in this file'}")
assert missing == 0, f"{missing} quotations do not resolve; fix the file, not the checker"
print()
print("RESULT: every quotation in 170 opens to the text it claims.")
