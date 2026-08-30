#!/usr/bin/env python3
"""
s6: the whole overlap between the two vocabularies, measured rather than eyeballed.

Section 4 of the deliverable reports one word, "phase", meaning two things. It was noticed
by eye, and a collision found by eye is a lower bound. This enumerates both vocabularies'
terms from their own sources and reports, per term, how many times each side uses it. A
term with a non-zero count on both sides is a CANDIDATE collision and has to be read; the
list is the set to read, not the answer.

CONCEPT files : 02 03 06 07 08 18, the stretch that developed the format concept.
SURFACE files : 10 11 12 13 15 16 17, the surface and derivation stretches.
Checkpoints (09, 14, 19) and the inventory (23) are excluded from both, because a
checkpoint quotes both sides by construction and would manufacture overlap.

A first version of this probe was a shell script that reported zero for every term,
including `phase`, which the deliverable had already measured at 8 and 13. The cause was an
IFS change breaking glob expansion. It is recorded here because a zero from a broken
command reads exactly like a zero from a real one, which is the failure the panel's own
counting rule exists for.

Run:  python3 s6_vocabulary_overlap.py
"""

import glob
import re

CONCEPT_IDS = ["02", "03", "06", "07", "08", "18"]
SURFACE_IDS = ["10", "11", "12", "13", "15", "16", "17"]

# Vocabulary A terms, from 08:175-233 and 08:552-578.
A_TERMS = [
    "radix", "binade", "canonical exponent", "exponent form", "adjustment",
    "bias", "phase", "value set", "progression", "locus", "encoding", "denot",
]
# Vocabulary B terms, from 15:96-131 and 13.
B_TERMS = [
    "total width", "fraction width", "integer width", "container", "carrier",
    "stride", "rung", "extent", "ladder", "width pair",
]


def files_for(ids):
    out = []
    for i in ids:
        out.extend(sorted(glob.glob(f"../{i}_*.md")))
    return out


def count(term, files):
    pat = re.compile(re.escape(term), re.I)
    return sum(len(pat.findall(open(f, errors="replace").read())) for f in files)


def main():
    cf, sf = files_for(CONCEPT_IDS), files_for(SURFACE_IDS)
    assert len(cf) == len(CONCEPT_IDS), f"concept files: {cf}"
    assert len(sf) == len(SURFACE_IDS), f"surface files: {sf}"
    print(f"concept side : {len(cf)} files  {[f.split('/')[-1][:2] for f in cf]}")
    print(f"surface side : {len(sf)} files  {[f.split('/')[-1][:2] for f in sf]}")
    print()
    print(f"{'term':<22}{'concept':>9}{'surface':>9}   reading")
    print("-" * 72)

    both = []
    for label, terms in (("vocabulary A, the concept", A_TERMS),
                         ("vocabulary B, the widths", B_TERMS)):
        print(f"--- {label} ---")
        for t in terms:
            c, s = count(t, cf), count(t, sf)
            if c and s:
                reading = "IN BOTH, candidate collision"
                both.append((t, c, s))
            elif c:
                reading = "concept only"
            elif s:
                reading = "surface only"
            else:
                reading = "in neither"
            print(f"{t:<22}{c:>9}{s:>9}   {reading}")
        print()

    print("-" * 72)
    print(f"terms on both sides: {len(both)}  {[t for t, _, _ in both]}")
    print()
    print("Every one of these was read. The verdicts:")
    verdicts = {
        "phase": "COLLISION. Value-space offset within a binade (08:222-224, 08:306)"
                 " against bit offset of a packed element (16:178-179). s4 shows all four"
                 " combinations occur, so neither determines the other.",
        "encoding": "not a collision. 08:253-277 uses it for the datum-to-value map and"
                    " the surface side uses it in the ordinary sense. No sentence turns"
                    " on the difference.",
        "container": "not a collision. Both sides mean the storage the numeral lowers to."
                     " The concept side barely uses it and never load-bearingly.",
        "denot": "not a collision. One meaning throughout, and 08's denotation clause is"
                 " where it is load-bearing.",
        "bias": "not a collision, but it is thin. Load-bearing only on the concept side"
                " (the affine map's B term). Check any surface-side hit before reusing it.",
        "radix": "not a collision. Concept side only in any load-bearing sense.",
        "extent": "not a collision. Surface side only in any load-bearing sense.",
    }
    for t, c, s in both:
        v = verdicts.get(t, "NOT YET READ. This is a gap in this probe's coverage.")
        print(f"  {t} ({c} vs {s}): {v}")
    print()
    print("So the overlap is wide and the collision count is one. That is the useful")
    print("shape of the answer: most shared words are shared innocently, and the single")
    print("hazard sits inside 08's own membership predicate, which is the worst place")
    print("for it.")


if __name__ == "__main__":
    main()
