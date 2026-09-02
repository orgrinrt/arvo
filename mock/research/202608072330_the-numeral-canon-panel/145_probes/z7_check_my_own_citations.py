#!/usr/bin/env python3
# z7 (145): every citation in 145 opened and its CONTENT tested, not merely resolved.
#
# A citation checker that only confirms a file exists at a line number passes for a citation
# pointing at the wrong text, which is the class 136 had to correct twice (a dead T9 label and a
# misdirected F131-6). So each entry below carries a phrase that must be present at or near the
# cited location, and the checker is mutation-tested three ways before its pass is worth anything.
#
# Predictions, stated before running:
#   H1. Every citation resolves and its content matches.
#   H2. CONTROL: a phrase nobody wrote must fail.
#   H3. CONTROL: a real phrase looked for in the wrong file must fail.
#   H4. CONTROL: a real phrase looked for at a line span it does not occupy must fail.
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)
ARVO = os.path.abspath(os.path.join(HERE, "..", "..", "..", ".."))

def norm(s):
    """Whitespace-normalised, blockquote markers stripped, and markdown emphasis and code ticks
    removed on both sides. The tick and asterisk stripping was added after two citations failed
    that were verbatim correct: `**not** part of what was ratified` and `the `O2` partition
    refines the `O1` partition`. A quotation wrapped in the source's own markup is still verbatim,
    and neither stripping can make an absent phrase appear, which is what the mutants below check.
    """
    s = re.sub(r"^\s*>\s?", "", s, flags=re.M)
    s = s.replace("`", "").replace("*", "").replace("_", "")
    return " ".join(s.split()).lower()

def read(path):
    p = path if os.path.isabs(path) else os.path.join(PANEL, path)
    if not os.path.exists(p):
        p2 = os.path.join(ARVO, path)
        if os.path.exists(p2):
            p = p2
    return open(p).read() if os.path.exists(p) else None

# (label, path, line span or None, phrase that must be present)
CITES = [
    ("I1 demoted", "INTENTS.md", (51, 61), "DEMOTED TO OPEN"),
    ("I13 ratified", "INTENTS.md", (214, 235), "predicated arms composed"),
    ("I13 scope limit", "INTENTS.md", (263, 267), "is not part of what was ratified"),
    ("op on hedge tokens", "INTENTS.md", (241, 246), "No adding \"unsure\" into the predicate"),
    ("I16 law shape", "INTENTS.md", (317, 331), "does not police what shape a law takes"),
    ("Q51 span", "OPTIONS.md", (2425, 2461), "What a strategy is, after the pair was attacked"),
    ("Q51 denoted repair", "OPTIONS.md", (2425, 2461),
     "component one fixes the denoted answer, not the computed one"),
    ("140 closure asymmetry", "140_mcsherry_the_strategy_set_derived_cold.md", (160, 169),
     "closed, finite, and arvo's to enumerate"),
    ("140 permanence test", "140_mcsherry_the_strategy_set_derived_cold.md", (186, 191),
     "fails permanence the moment anyone adds a fifth"),
    ("139 firewall text", "139_muratori_the_strategy_set_derived_cold.md", (264, 270),
     "Every difference in an answer traces to the policy"),
    ("139 concedes the chain granularity",
     "139_muratori_the_strategy_set_derived_cold.md", (769, 777),
     "the procedure should be stated over chains"),
    ("141 monotonicity theorem", "141_lamport_the_strategy_set_attacked.md", (510, 524),
     "the O2 partition refines the O1 partition"),
    ("141 F3 absorption", "141_lamport_the_strategy_set_attacked.md", (287, 308),
     "R(R(x) + c) = R(x + c)"),
    ("141 F7 identity", "141_lamport_the_strategy_set_attacked.md", (705, 716),
     "bit-identical to the plain lowering"),
    ("142 F142-1", "142_muratori_reply_the_repair_was_dead_on_arrival.md", (453, 464),
     "are the same two\nfunctions"),
    ("142 q1 counts", "142_muratori_reply_the_repair_was_dead_on_arrival.md", (88, 100),
     "6356992"),
    ("142 F142-2 partition", "142_muratori_reply_the_repair_was_dead_on_arrival.md", (466, 475),
     "Translation equivariance partitions the rounding axis"),
    ("142 F142-6 schedule", "142_muratori_reply_the_repair_was_dead_on_arrival.md", (515, 527),
     "depends on the accumulation schedule"),
    ("142 necessary not sufficient",
     "142_muratori_reply_the_repair_was_dead_on_arrival.md", (264, 270),
     "Naming the six modes is\nnecessary and not sufficient"),
    ("143 intersection lesson",
     "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md", (424, 431),
     "the intersection of their dimensions rather than the union"),
    ("143 anti-monotone control",
     "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md", (218, 226),
     "control fires, so the real comparator's zero is a real zero"),
    ("143 two-argument count",
     "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md", (296, 313),
     "is not currently writable"),
    ("144 F144-15", "144_fog_the_weighting_half_measured.md", (787, 803),
     "answer-invisible with conforming arms"),
    ("144 F144-16", "144_fog_the_weighting_half_measured.md", (805, 823),
     "if and only if that axis is\nanswer-visible"),
    ("144 F144-18", "144_fog_the_weighting_half_measured.md", (837, 852),
     "No arm in arvo's committed bench corpus is established"),
    ("144 F144-13", "144_fog_the_weighting_half_measured.md", (764, 775),
     "changes which arm a\nfixed weighting selects at 24.6%"),
    ("144 F144-2 exact LP", "144_fog_the_weighting_half_measured.md", (640, 649),
     "loses by at least one unit at every point of the simplex"),
    ("144 F144-4 dead coordinate", "144_fog_the_weighting_half_measured.md", (662, 669),
     "makes every arm weakly selectable"),
    ("144 F144-11 arm regret", "144_fog_the_weighting_half_measured.md", (739, 748),
     "carrying a weighting\ncosts exactly zero"),
    ("144 4.3 rescale", "144_fog_the_weighting_half_measured.md", (379, 392),
     "Normalised, it is exactly 0.0% at every pair"),
    ("144 section 6 repair", "144_fog_the_weighting_half_measured.md", (491, 501),
     "the comparison is made after the weighting, on the weighted scalar"),
    ("144 withdrawal", "144_fog_the_weighting_half_measured.md", (992, 1010),
     "3.8 nanoseconds against a 79.2 nanosecond"),
    ("144 pairwise gate", "144_fog_the_weighting_half_measured.md", (1004, 1010),
     "the gate has to be pairwise"),
    ("137 gate run", "137_probes/g0_test_gate.out", None, "123 passed across 13 crates, 0 failed"),
]

print("=" * 96)
print("H1. Every citation, opened and its content tested")
print("=" * 96)
ok = bad = 0
for label, path, span, phrase in CITES:
    text = read(path)
    if text is None:
        print(f"  FAIL  {label:<40} file not found: {path}")
        bad += 1
        continue
    if span:
        lines = text.split("\n")
        seg = "\n".join(lines[span[0] - 1: span[1]])
    else:
        seg = text
    hit = norm(phrase) in norm(seg)
    print(f"  {'ok  ' if hit else 'FAIL'}  {label:<40} {path}"
          f"{'' if not span else f':{span[0]}-{span[1]}'}")
    ok += hit
    bad += not hit
print(f"\n  citations checked: {len(CITES)}   ok: {ok}   failed: {bad}")
print(f"  H1: {'CONFIRMED' if bad == 0 else 'REFUTED'}")

print()
print("=" * 96)
print("H2/H3/H4. The mutants, each of which must be caught")
print("=" * 96)

def check(path, span, phrase):
    text = read(path)
    if text is None:
        return False
    if span:
        lines = text.split("\n")
        seg = "\n".join(lines[span[0] - 1: span[1]])
    else:
        seg = text
    return norm(phrase) in norm(seg)

m2 = check("144_fog_the_weighting_half_measured.md", (787, 803),
           "the selector is provably optimal in every case")
m3 = check("139_muratori_the_strategy_set_derived_cold.md", (787, 803),
           "answer-invisible with conforming arms")
m4 = check("144_fog_the_weighting_half_measured.md", (1, 40),
           "answer-invisible with conforming arms")
print(f"  H2 a phrase nobody wrote, in the right file and span: {m2} (must be False)")
print(f"  H3 a real phrase in the wrong file:                   {m3} (must be False)")
print(f"  H4 a real phrase at the wrong span:                   {m4} (must be False)")
caught = (not m2) and (not m3) and (not m4)
print(f"\n  mutants all caught: {caught}")
print(f"  H2/H3/H4: {'CONFIRMED' if caught else 'REFUTED, and H1 above is then worthless'}")
