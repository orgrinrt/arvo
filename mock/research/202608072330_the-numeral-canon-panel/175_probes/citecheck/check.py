#!/usr/bin/env python3
"""Citation check for 175, all four normalisation layers, each counted.

  L0 raw
  L1 + whitespace collapse                     (168's fifth defect)
  L2 + strip blockquote markers and emphasis   (169's seventh)
  L3 + fold case                               (170's eighth)

CASES THAT MUST FAIL
  C-1 a citation naming a file that does not exist must be caught
  C-2 a citation naming a real file with text that is not there must be caught
  C-3 the layer report must be produced for every citation, so a layer doing
      nothing shows as a zero rather than being assumed dead
"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
D = os.path.abspath(os.path.join(HERE, '..', '..'))
WS = os.path.abspath(os.path.join(D, '..', '..', '..', '..'))

def L0(t): return t
def L1(t): return " ".join(t.split())
def L2(t):
    t = re.sub(r'^\s*>\s?', ' ', t, flags=re.M)
    t = t.replace('**', '').replace('*', '').replace('`', '')
    return L1(t)
def L3(t): return L2(t).casefold()
LAYERS = [("L0", L0), ("L1", L1), ("L2", L2), ("L3", L3)]

C = '173_leroy_the_canon_candidate_for_the_chain.md'
S = '174_mcsherry_signature_in_part.md'
Z = '60_stam_the_chain_derived_cold.md'
M = '167_rompf_the_chain_derived_cold.md'
N = '171_rompf_reply.md'
RULE = os.path.join(WS, '.claude', 'rules', 'what-you-can-observe-is-what-you-guaranteed.md')

CITES = [
    ("clause 2, the stretch's boundary function", C,
     "the design may select any realisation that induces the stretch's boundary function"),
    ("clause 3, placed under clause 2", C,
     "on an unbound edge it is free and placed under clause 2"),
    ("clause 3, two schedules compute different functions", C,
     "two schedules over the same operations compute different functions"),
    ("clause 4, bounded drift is a grade", C, "bounded drift"),
    ("clause 11, which ships is not settled", C, "Which ships is not settled here"),
    ("clause 8, the family sentence", C,
     "A sentence about reassociation that does not name the family and the reachability of the resolution is wrong for someone."),
    ("L3 end state", C,
     "**End state**: (P) at two instances"),
    ("L10 one expert second read owed", C, "the unit's most valuable unattacked claim"),
    ("O-7 the threads item", C,
     "so the compressor cannot drop it, because it is the unit's clearest successor"),
    ("R-k the static-length lever", C, "**R-k. The static-length lever**"),
    ("section 5 item 5, Q-C2 sharpened", C,
     "assumes the binding relation is decidable at com"),
    ("X-C, O-171-3 carried", C, "`171` O-171-3"),
    ("L3 step 5, step 2 is descriptive", C,
     "so step 2 is descriptive and the"),
    ("174 A3 either-way hedge", S,
     "if the convention is leading then clause 4's"),
    ("174 A5 the wording", S,
     'Write the end state as "(P) at one rule-free instance and one rule-dependent instance; (L) at\nzero"'),
    ("174 A4's cell counts", S,
     "unexecutable in 663 of my cells and promises a value unreachable in 17 of them"),
    ("174 A6 the stale strings", S, "Fix both strings."),
    ("174's own dropped anchor", S, "**And one of my own anchors is on the dropped list**"),
    ("60's statability disclaimer", Z,
     "That is a statability argument, not a benchmark"),
    ("167's founding sentence", M,
     "everything inside it is arvo's to choose, and everything at its edge is the consumer's contract"),
    ("171's (P) definition", N,
     "**(P) The partition.** A program divides into maximal stretches whose intermediates nothing binds."),
    ("171's four untested channels", N,
     "floating-point environment flags, `#[track_caller]` location data, symbol names in a backtrace"),
    ("the rule's thesis, only over", RULE,
     "A guarantee about a type holds only over the operations through which the type can be observed"),
]

BOGUS = [
    ("C-1 nonexistent file", os.path.join(D, '999_nope.md'), "anything"),
    ("C-2 real file, absent text", os.path.join(D, C),
     "clause 3 is hereby withdrawn by its author"),
]

counts = {k: 0 for k, _ in LAYERS}
missing = 0
print(f"{'citation':<46} {'first layer that finds it':>26}")
for label, path, needle in CITES:
    full = path if os.path.isabs(path) else os.path.join(D, path)
    if not os.path.exists(full):
        print(f"{label:<46} {'FILE MISSING':>26}  {path}")
        missing += 1
        continue
    text = open(full, encoding='utf-8').read()
    first = None
    for k, fn in LAYERS:
        if fn(needle) in fn(text):
            first = k
            break
    if first is None:
        missing += 1
        print(f"{label:<46} {'NOT FOUND':>26}")
    else:
        counts[first] += 1
        print(f"{label:<46} {first:>26}")

print()
caught = 0
for label, path, needle in BOGUS:
    if not os.path.exists(path):
        ok = True
    else:
        ok = L3(needle) not in L3(open(path, encoding='utf-8').read())
    print(f"{'caught' if ok else 'MISSED'}  {label}")
    caught += ok

print()
print(f"citations: {len(CITES)}, not found: {missing}")
print("layer report: " + ", ".join(f"{k} {counts[k]}" for k, _ in LAYERS))
print(f"C-3 every citation received a layer verdict: {missing == 0}")
print(f"negative controls caught: {caught} of {len(BOGUS)} (must be {len(BOGUS)})")
sys.exit(1 if (missing or caught != len(BOGUS)) else 0)
