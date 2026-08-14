#!/usr/bin/env python3
# x3 (136): the two attribution errors, and the impossible figure, checked at source.
#
# Both errors are the coordinator's relays and one is mine to own in the candidate's wording.
# I check them at source rather than accept 133's account, and I reproduce the impossibility by
# a different route from 133's s1: s1 searched for a representation 2^b(2^k - 1) at every width
# up to 2^60. This probe instead pins the width from the denominator the figure itself names,
# 32,768 = 2^15, and enumerates the complete set of counts a sweep of that shape can produce.
#
# Predictions, stated before running:
#
#   Q1. The closed form: at width W and fraction width F, bit-drop differs from toward-zero on
#       exactly 2^(W-1) - 2^(W-1-F) values, those being the negatives with a nonzero dropped
#       bit. Verified against my own quantiser implementations by direct sweep, with a control
#       that an off-by-one form fails at every cell.
#   Q2. If the denominator 32,768 is the swept domain size, W = 15, and the complete set of
#       possible differing counts is {2^14 - 2^(14-F) : F = 0..14}, sixteen numbers. Prediction:
#       21,204 is not among them, and moreover every member is below 2^14 = 16,384 while 21,204
#       exceeds it, so no F at that width can produce it. That is a stronger statement than
#       s1's: the figure is not merely unrepresentable, it exceeds the maximum the sweep can
#       report at the only width its own denominator admits.
#   Q3. The figure appears nowhere in 125 or in 125_probes. Grep.
#   Q4. 125 section 10 records twelve of thirteen crates at 108 tests with the thirteenth
#       non-terminating, not 123 across 13. Grep, and grep 122's u0 for the completed 123.
#   Q5. The sentence "no member carries more than one of the first three" appears in neither
#       125 nor 126 and enters at 131. Grep for its distinguishing phrase.
#   Q6. 125's T9 has no definition in 125. Grep for a definition line versus the one reference.
import os
import re
import subprocess

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

def q_floor(n, d):
    return n // d

def q_toward_zero(n, d):
    return n // d if n >= 0 else -((-n) // d)

print("=" * 88)
print("Q1. The closed form for the bit-drop / toward-zero difference, on my own sweep")
print("=" * 88)
print(f"  {'W':>3} {'F':>3} {'measured':>10} {'2^(W-1)-2^(W-1-F)':>20} {'match':>7} {'off-by-one ctrl':>16}")
q1 = True
ctrl_ever_matched = False
for W in (4, 6, 8, 10):
    for F in range(1, min(W, 6)):
        d = 1 << F
        diff = sum(1 for raw in range(-(1 << (W - 1)), 1 << (W - 1))
                   if q_floor(raw, d) != q_toward_zero(raw, d))
        closed = (1 << (W - 1)) - (1 << (W - 1 - F))
        ctrl = closed + 1
        q1 &= (diff == closed)
        ctrl_ever_matched |= (diff == ctrl)
        print(f"  {W:>3} {F:>3} {diff:>10} {closed:>20} {str(diff == closed):>7} "
              f"{str(diff == ctrl):>16}")
print(f"\n  Q1 closed form holds at every cell: {q1}")
print(f"  control (off-by-one form) matched anywhere: {ctrl_ever_matched} (must be False)")

print()
print("=" * 88)
print("Q2. Can any sweep over 32,768 values report 21,204 differences?")
print("=" * 88)
W = 15
possible = sorted({(1 << (W - 1)) - (1 << (W - 1 - F)) for F in range(0, W)})
print(f"  32,768 = 2^15, so W = {W} and the complete set of reportable counts is:")
print(f"    {possible}")
print(f"  maximum reportable at this width: {max(possible)} = 2^14 - 1")
print(f"  21204 in the set: {21204 in possible}")
print(f"  21204 exceeds the maximum: {21204 > max(possible)}")
print(f"\n  Q2: the figure is not merely absent from the set, it is larger than any count the")
print(f"      sweep can produce at the only width its own denominator admits. It cannot be a")
print(f"      cell of the sweep it was reported as, under any fraction width.")
# and the second denominator the sweep could mean: the signed half-range
alt = sorted({(1 << 15) - (1 << (15 - F)) for F in range(0, 16)})
print(f"\n  control on the reading of '32,768': if it names the half-range rather than the whole")
print(f"      domain, W = 16 and the set is {alt[:5]} ... {alt[-2:]}")
print(f"      21204 in that set: {21204 in alt} (so the figure fails under both readings)")

print()
print("=" * 88)
print("Q3 through Q6. Provenance, at source")
print("=" * 88)

def grep(pattern, *paths):
    try:
        out = subprocess.run(["grep", "-rn", "-E", pattern, *paths],
                             capture_output=True, text=True, cwd=PANEL)
        return [l for l in out.stdout.strip().split("\n") if l]
    except Exception as e:
        return [f"ERROR {e}"]

hits = grep(r"21,?204", "125_knuth_rounding_cold_derivation.md", "125_probes")
print(f"  Q3 '21204' in 125 or 125_probes: {len(hits)} hit(s) (predicted 0)")
for h in hits[:3]:
    print(f"     {h}")

hits = grep(r"108 tests|Twelve of the thirteen", "125_knuth_rounding_cold_derivation.md")
print(f"\n  Q4a 125's own recorded gate result: {len(hits)} hit(s) (predicted >= 1)")
for h in hits[:3]:
    print(f"     {h[:120]}")
hits = grep(r"123 across 13", "125_knuth_rounding_cold_derivation.md")
print(f"  Q4b '123 across 13' in 125: {len(hits)} hit(s) (predicted 0)")
hits = grep(r"TOTAL PASSED", "122_probes/u0_test_gate_run.txt")
print(f"  Q4c the completed count, in 122's own artifact: {len(hits)} hit(s) (predicted 1)")
for h in hits[:2]:
    print(f"     {h}")
hits = grep(r"123 across 13", "131_leroy_formalising_the_rounding_axis.md",
            "132_leroy_the_canon_candidate_for_the_rounding_axis.md",
            "134_wronski_cosign_with_one_addendum_on_the_dither.md",
            "135_dolan_signature_in_part.md")
print(f"  Q4d files carrying the wrong attribution forward: {len(hits)} hit(s)")
for h in hits:
    print(f"     {h.split(':')[0]}:{h.split(':')[1]}")

hits = grep(r"no member carries more than one", "125_knuth_rounding_cold_derivation.md",
            "126_wronski_rounding_character_cold_derivation.md")
print(f"\n  Q5a the exclusivity sentence in either source file: {len(hits)} hit(s) (predicted 0)")
hits = grep(r"no member carries more than one", "131_leroy_formalising_the_rounding_axis.md",
            "132_leroy_the_canon_candidate_for_the_rounding_axis.md")
print(f"  Q5b where it does appear: {len(hits)} hit(s) (predicted >= 1, both mine)")
for h in hits:
    print(f"     {h.split(':')[0]}:{h.split(':')[1]}")

hits = grep(r"\bT9\b", "125_knuth_rounding_cold_derivation.md")
defs = [h for h in hits if re.search(r"\*\*T9\b|^.*:T9\.", h)]
print(f"\n  Q6 'T9' in 125: {len(hits)} reference(s), {len(defs)} of them a definition")
for h in hits:
    print(f"     {h[:110]}")
print(f"      (every other theorem in 125 is introduced as '**Tn (...)**'; T9 is not, so the")
print(f"       label is a dangling forward reference and 132's citation of it resolves to nothing)")

print()
print("=" * 88)
print("VERDICT")
print("=" * 88)
print("  The figure cannot be a cell of the sweep it was attributed to, by a route independent")
print("  of 133's: it exceeds the maximum count that sweep can report at either reading of its")
print("  own denominator. It appears nowhere in 125 or its probes. 125 records a gate result")
print("  that contradicts what three later files attribute to it, and the completed count those")
print("  files mean is in 122's committed artifact. The exclusivity sentence entered at 131.")
print("  Every one of these is a coordinator relay, and 131 and 132 are mine.")
