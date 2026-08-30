#!/usr/bin/env python3
"""172 quotation checker. Matches this file's verbatim quotations against their
sources under the unit's three-layer normalisation: L1 whitespace, L2 markdown
markup (blockquote markers, emphasis, backticks), L3 case folding.

CONTROLS, declared before the run: a planted-absent quotation must NOT be found
(the checker can miss), and a planted-present one must be found at raw or L1
(the checker reads the file). Per 171 section 9: this file copies source bytes,
so a zero at L2/L3 is a quoting-style fact, not evidence the layers are dead.
"""
import re, sys

def norm(s, layer):
    if layer >= 2:
        s = re.sub(r'^\s*>\s?', '', s, flags=re.M)
        s = s.replace('**', '').replace('`', '').replace('*', '')
    s = re.sub(r'\s+', ' ', s).strip()
    if layer >= 3:
        s = s.casefold()
    return s

QUOTES = [
    # (source file, quotation as carried in 172)
    ("60_stam_the_chain_derived_cold.md",
     "a chain is a composition of exact operations together with a schedule of adaptation points"),
    ("60_stam_the_chain_derived_cold.md",
     "the schedule is also the index set of the error analysis"),
    ("167_rompf_the_chain_derived_cold.md",
     "no adaptation on the additive side occurs at all"),
    ("111_jhala_the_primitive_attacked.md",
     "I did not attack `109`'s section 8 chain result"),
    ("171_rompf_reply.md",
     "everything inside it is arvo's to choose"),
    ("168_mcsherry_the_chain_derived_cold.md",
     "A chain is a maximal run of operations whose intermediates are not observable"),
]
CONTROL_ABSENT = ("60_stam_the_chain_derived_cold.md",
                  "the chain is fundamentally a monoid in the category of endofunctors")
CONTROL_PRESENT = ("167_rompf_the_chain_derived_cold.md",
                   "The right unit is the")

def find(src_text, quote):
    for layer in (0, 1, 2, 3):
        s = norm(src_text, layer) if layer else src_text
        q = norm(quote, layer) if layer else quote
        if q in s:
            return layer
    return None

def main():
    ok = True
    for f, q in QUOTES:
        layer = find(open(f).read(), q)
        if layer is None:
            print(f"MISSING  {f}: {q[:60]}"); ok = False
        else:
            print(f"OK L{layer}  {f}: {q[:60]}")
    la = find(open(CONTROL_ABSENT[0]).read(), CONTROL_ABSENT[1])
    lp = find(open(CONTROL_PRESENT[0]).read(), CONTROL_PRESENT[1])
    print(f"CONTROL absent not found : {'PASS' if la is None else 'FAIL'}")
    print(f"CONTROL present found    : {'PASS' if lp is not None else 'FAIL'}")
    print(f"VERDICT: {'PASS' if ok and la is None and lp is not None else 'FAIL'}")

main()
