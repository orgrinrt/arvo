#!/usr/bin/env python3
"""Open every citation 167 makes and test its CONTENT, not merely that it resolves.

Run from the repo root:  python3 mock/research/.../167_probes/citecheck/check.py
"""
import os, sys, re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..', '..', '..', '..'))
D = 'mock/research/202608072330_the-numeral-canon-panel'
B = 'mock/benches'

# (label, path, line-or-None, substring that must be present at/near it)
CITES = [
    ("109:438 the deferred cost", f"{D}/109_bellard_the_primitive_derived_cold.md", 438,
     "The deferred route's accumulator grows with the"),
    ("109 sec8 header", f"{D}/109_bellard_the_primitive_derived_cold.md", 389,
     "Chain accuracy cannot live in a primitive"),
    ("109:424 Mul not endomorphism", f"{D}/109_bellard_the_primitive_derived_cold.md", 424,
     "not an endomorphism"),
    ("110:314 what composing means", f"{D}/110_willsey_the_primitive_derived_cold.md", 314,
     "What composing means, and why the word carries two jobs"),
    ("110:322 construction algebra to algebra", f"{D}/110_willsey_the_primitive_derived_cold.md", 322,
     "a construction taking an algebra to an algebra"),
    ("112:850 declared extent discharges", f"{D}/112_leijen_where_the_refinement_lives.md", 850,
     "A declared extent discharges a construction's base predicate"),
    ("AGREEMENTS:206 C9 chain concept", f"{D}/AGREEMENTS.md", 206,
     "a chain is exact operations plus a schedule of adaptation points"),
    ("AGREEMENTS:497 cross-topic chain", f"{D}/AGREEMENTS.md", 497,
     "chain / multiplicative-accuracy finding recurs"),
    ("AGREEMENTS:499 63 sec5 unstatable", f"{D}/AGREEMENTS.md", 499,
     "schedule of adaptation points, and closing operations over the format"),
    ("OPTIONS Q42 noise floor", f"{D}/OPTIONS.md", 2029,
     "noise floor on this host"),
    ("OPTIONS Q42 no strategy dimension", f"{D}/OPTIONS.md", 2027,
     "No dimension for strategy is listed"),
    ("OPTIONS Q12 70.1 pct", f"{D}/OPTIONS.md", 1095,
     "70.1%"),
    ("satfold lib cites panel 80/82", f"{B}/variants/satfold-shared/src/lib.rs", 6,
     "Panel file `80` section 5.3 and `82` section 9"),
    ("INTENTS I7", f"{D}/INTENTS.md", 135,
     "Precise is accurate across chains, not only per operation"),
    ("INTENTS I11 compose quote", f"{D}/INTENTS.md", 196,
     "contracts for things that compose to bigger units"),
    ("INTENTS I3 ergonomics", f"{D}/INTENTS.md", 91,
     "Neither, it's ergonomics"),
    ("INTENTS I15 never runtime checks", f"{D}/INTENTS.md", 306,
     "Never any runtime checks, ever"),
    ("RULES eighteen files unpriced", f"{D}/RULES.md", 407,
     "eighteen files reported the packed-storage trade as"),
    ("satfold assoc law test", f"{B}/variants/satfold-shared/src/lib.rs", None,
     "fn saturating_addition_is_associative_at_eight_bits"),
    ("60 sec3 five things", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "What a chain needs that a single op does not"),
    ("60 schedule is the index set", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "the schedule is also the index set of the error analysis"),
    ("60 sec5 the window", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "The resolution is the **window**"),
    ("60 D-C is D-B with ceremony", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "D-C is D-B with ceremony"),
    ("60 probe B saturating fold orders", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "gives 7767 or 15000 depending on order"),
    ("60 grade b counts", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "15,628 of 46,656 inputs wrong"),
    ("60 chain definition", f"{D}/60_stam_the_chain_derived_cold.md", None,
     "A chain is a composition of exact operations together with a schedule"),
    ("OPTIONS Q42 crossovers", f"{D}/OPTIONS.md", None,
     "the 16-lane arm first pays at `L = 32`"),
]

WINDOW = 3

def norm(t):
    """Collapse all whitespace. A quotation is a claim about words, not about where
    the source happened to wrap. Added after two of my own citations failed on a
    newline inside the quoted phrase and turned out to be accurate."""
    return " ".join(t.split())

fails = 0
for label, path, line, needle in CITES:
    full = os.path.join(ROOT, path)
    if not os.path.exists(full):
        print(f"FAIL  {label}: file does not exist -> {path}")
        fails += 1
        continue
    text = open(full, encoding='utf-8').read()
    if line is None:
        ok = norm(needle) in norm(text)
        where = "anywhere in file"
    else:
        lines = text.splitlines()
        lo, hi = max(0, line - 1 - WINDOW), min(len(lines), line + WINDOW)
        ok = norm(needle) in norm("\n".join(lines[lo:hi]))
        where = f"lines {lo+1}..{hi}"
    print(("ok    " if ok else "FAIL  ") + f"{label}  [{where}]")
    if not ok:
        fails += 1

# Negative control: a citation that must NOT resolve, proving the checker can fail.
bogus = [
    ("NEGATIVE CONTROL: nonexistent file", f"{D}/999_no_such_file.md", None, "anything"),
    ("NEGATIVE CONTROL: right file wrong text", f"{D}/INTENTS.md", 135,
     "the strategy set is closed at exactly four and always will be"),
]
print()
nc_caught = 0
for label, path, line, needle in bogus:
    full = os.path.join(ROOT, path)
    if not os.path.exists(full):
        caught = True
    else:
        lines = open(full, encoding='utf-8').read().splitlines()
        lo, hi = max(0, line - 1 - WINDOW), min(len(lines), line + WINDOW)
        caught = norm(needle) not in norm("\n".join(lines[lo:hi]))
    print(("caught" if caught else "MISSED") + f"  {label}")
    nc_caught += caught

print()
print(f"citations checked: {len(CITES)}, failures: {fails}")
print(f"negative controls caught: {nc_caught} of {len(bogus)} (must be {len(bogus)}, else the checker cannot fail)")
sys.exit(1 if (fails or nc_caught != len(bogus)) else 0)
