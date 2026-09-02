#!/usr/bin/env python3
"""
177 P2. Independent anchor check on 176, both directions, with the accounting section
excluded, as the standing discipline requires.

WHY EXCLUDED: an author that prints the anchors it dropped makes them present in the new
text and disables the set diff. 176 prints counts and its one novel anchor inside section
7. This rerun strips section 7 entirely and asks whether the numbers survive, which tests
whether any carried anchor exists only inside the accounting block.

NEGATIVE CONTROLS, declared before the run
  L1. A planted anchor absent from every source must be reported novel. If not, the novel
      test cannot fire.
  L2. Stripping section 7 must not INCREASE the carried set. If it does, the stripper is
      removing the wrong region.
  L3. The extractor must find at least one anchor in the full text. A zero makes every
      difference below vacuous.
"""
import re, os

D = os.path.dirname(os.path.abspath(__file__)) + "/.."
SOURCES = ["166_dispatcher_note_opening_the_tenth_unit.md",
           "167_rompf_the_chain_derived_cold.md",
           "168_mcsherry_the_chain_derived_cold.md",
           "169_kiselyov_the_chain_attacked.md",
           "170_mcsherry_reply.md", "171_rompf_reply.md",
           "172_leroy_formalising_the_chain.md",
           "173_leroy_the_canon_candidate_for_the_chain.md",
           "174_mcsherry_signature_in_part.md",
           "175_rompf_signature_in_part.md",
           "60_stam_the_chain_derived_cold.md",
           "43_rompf_what_a_composition_is.md"]
CAND = "176_leroy_the_candidate_revised_against_two_signatures.md"

LINE  = re.compile(r'`([A-Za-z0-9_./-]+\.(?:md|rs|py|toml|sh|err|out|txt|s)):(\d+(?:-\d+)?)`')
BARE  = re.compile(r'`(\d{2,3}):(\d+(?:-\d+)?)`')
PROBE = re.compile(r'`(\d{2,3}_probes/[A-Za-z0-9_./-]*)`')
# a deliberately looser pass: backticks not required
LOOSE = re.compile(r'\b([A-Za-z0-9_./-]+\.(?:md|rs|py|toml|sh|err|out|txt|s)|\d{2,3}):(\d+(?:-\d+)?)\b')

def anchors(t):
    a = {f"{m.group(1)}:{m.group(2)}" for m in LINE.finditer(t)}
    a |= {f"{m.group(1)}:{m.group(2)}" for m in BARE.finditer(t)}
    return a
def loose(t):  return {f"{m.group(1)}:{m.group(2)}" for m in LOOSE.finditer(t)}
def probes(t): return {m.group(1) for m in PROBE.finditer(t)}

cand_full = open(os.path.join(D, CAND)).read()
# strip section 7 (the accounting) up to section 8
lines = cand_full.split("\n")
s7 = next(i for i, l in enumerate(lines) if l.startswith("## 7."))
s8 = next(i for i, l in enumerate(lines) if l.startswith("## 8."))
cand_nosec7 = "\n".join(lines[:s7] + lines[s8:])

src = "".join(open(os.path.join(D, s)).read() for s in SOURCES)
sa, sp = anchors(src), probes(src)
fa, fp = anchors(cand_full), probes(cand_full)
na, np_ = anchors(cand_nosec7), probes(cand_nosec7)

print(f"source unique line anchors        : {len(sa)}")
print(f"source unique probe paths         : {len(sp)}")
print(f"candidate anchors, full text      : {len(fa)}   {sorted(fa)}")
print(f"candidate anchors, section 7 cut  : {len(na)}   {sorted(na)}")
print(f"candidate probe paths, full text  : {len(fp)}")
print(f"candidate probe paths, sec 7 cut  : {len(np_)}")
print()
print(f"carried (candidate & source), full : {len(fa & sa)}  {sorted(fa & sa)}")
print(f"novel   (candidate - source), full : {len(fa - sa)}  {sorted(fa - sa)}")
print(f"dropped (source - candidate), full : {len(sa - fa)}")
print()
print(f"anchors present ONLY inside section 7: {sorted(fa - na)}")
print(f"probe paths present ONLY inside sec 7: {sorted(fp - np_)}")
print()
print("probe paths the candidate carries:")
for p in sorted(fp):
    print(f"    {p:52s} in a source: {p in sp}")
print()
lf = loose(cand_full)
print(f"loose extraction over the candidate: {len(lf)}  {sorted(lf)}")
print(f"seen loose and not strict          : {sorted(lf - fa)}")
print()
print("CONTROLS")
planted = cand_full + "\n`999_no_such_file.md:123`\n"
l1 = "999_no_such_file.md:123" in (anchors(planted) - sa)
l2 = na <= fa
l3 = len(fa) > 0
print(f"  L1 planted anchor reported novel      : {'PASS' if l1 else 'FAIL'}")
print(f"  L2 stripping does not grow the set    : {'PASS' if l2 else 'FAIL'}")
print(f"  L3 extractor finds something          : {'PASS' if l3 else 'FAIL'}")
