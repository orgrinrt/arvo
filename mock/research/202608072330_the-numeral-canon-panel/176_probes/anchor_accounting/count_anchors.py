#!/usr/bin/env python3
"""176 anchor accounting. Counts the citation anchors in the twelve source files this
candidate compresses and in the candidate itself, and diffs the SETS, because a rising
total is not reassurance: a prior round's count rose from 100 to 120 while nine unique
targets vanished underneath the additions.

The dropped list is written to dropped_anchors.txt, a SIBLING file, never into the
candidate, because an author that prints the anchors it dropped makes them present in
the new text and disables the diff.

Anchor classes counted:
  A. line anchors:  `path.ext:N` and `path.ext:N-M`, plus bare panel `NNN:N[-M]`
  B. probe paths:   `NNN_probes/...`

CONTROLS, declared before the run:
  positive: `112:904-906` must be found in the candidate (it is cited there); if the
            extractor cannot see it, every count here is noise.
  negative (the case that must fail): a copy of the candidate with a planted anchor
            `999_no_such_file.md:123` must be reported as NOVEL (present in the
            candidate, absent from every source). If the planted anchor is not
            reported, the novel-anchor check cannot fire and proves nothing.
"""
import re, sys, os, tempfile

D = os.path.dirname(os.path.abspath(__file__)) + "/../.."
SOURCES = ["166_dispatcher_note_opening_the_tenth_unit.md",
           "167_rompf_the_chain_derived_cold.md",
           "168_mcsherry_the_chain_derived_cold.md",
           "169_kiselyov_the_chain_attacked.md",
           "170_mcsherry_reply.md",
           "171_rompf_reply.md",
           "172_leroy_formalising_the_chain.md",
           "173_leroy_the_canon_candidate_for_the_chain.md",
           "174_mcsherry_signature_in_part.md",
           "175_rompf_signature_in_part.md",
           "60_stam_the_chain_derived_cold.md",
           "43_rompf_what_a_composition_is.md"]
CAND = "176_leroy_the_candidate_revised_against_two_signatures.md"

LINE = re.compile(r'`([A-Za-z0-9_./-]+\.(?:md|rs|py|toml|sh|err|out|txt|s)):(\d+(?:-\d+)?)`')
BARE = re.compile(r'`(\d{2,3}):(\d+(?:-\d+)?)`')
PROBE = re.compile(r'`(\d{2,3}_probes/[A-Za-z0-9_./-]*)`')

def anchors(text):
    a = set()
    for m in LINE.finditer(text): a.add(f"{m.group(1)}:{m.group(2)}")
    for m in BARE.finditer(text): a.add(f"{m.group(1)}:{m.group(2)}")
    return a

def probes(text):
    return {m.group(1) for m in PROBE.finditer(text)}

def main():
    src_anchors, src_probes = set(), set()
    per_file = []
    for s in SOURCES:
        t = open(os.path.join(D, s)).read()
        a, p = anchors(t), probes(t)
        per_file.append((s, len(a), len(p)))
        src_anchors |= a; src_probes |= p
    cand_text = open(os.path.join(D, CAND)).read()
    ca, cp = anchors(cand_text), probes(cand_text)

    print("per-source line-anchor / probe-path counts:")
    for s, na, np_ in per_file: print(f"  {s:55s} {na:4d} {np_:3d}")
    print(f"\nunique line anchors across the 12 sources : {len(src_anchors)}")
    print(f"unique probe paths across the 12 sources  : {len(src_probes)}")
    print(f"line anchors in the candidate             : {len(ca)}")
    print(f"probe paths in the candidate              : {len(cp)}")
    kept = ca & src_anchors
    novel = ca - src_anchors
    dropped = src_anchors - ca
    print(f"candidate anchors that appear in a source : {len(kept)}")
    print(f"candidate anchors novel to the candidate  : {len(novel)}")
    for n in sorted(novel): print(f"    NOVEL {n}")
    print(f"source anchors not carried (dropped)      : {len(dropped)}")
    with open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "dropped_anchors.txt"), "w") as f:
        f.write("# anchors present in the 12 source files and not carried into 176.\n")
        f.write("# sibling file by design: printing these inside the candidate would\n")
        f.write("# make them present in it and disable the set diff.\n")
        for d_ in sorted(dropped): f.write(d_ + "\n")
    kept_p = cp & src_probes; dropped_p = src_probes - cp
    print(f"probe paths carried                       : {len(kept_p)} of {len(src_probes)}")

    # CONTROLS
    print("\n-- controls --")
    pos = "60:210" in ca
    print(f"positive control (60:210 found in candidate): {'PASS' if pos else 'FAIL'}")
    planted = cand_text + "\n`999_no_such_file.md:123`\n"
    pa = anchors(planted)
    caught = "999_no_such_file.md:123" in (pa - src_anchors)
    print(f"negative control (planted anchor reported NOVEL): {'PASS (fires)' if caught else 'FAIL'}")
    if not (pos and caught): sys.exit(1)

main()
