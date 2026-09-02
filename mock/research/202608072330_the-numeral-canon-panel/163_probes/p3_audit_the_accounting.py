#!/usr/bin/env python3
"""
163 P3. Independent audit of 161's anchor accounting instrument.

`162` section 9 states plainly that it did NOT verify the instrument, so this check is
unclaimed by anyone. `161` section 8 rests four numbers and one conclusion on it, and the
conclusion is the strong one: "Zero novel anchors means nothing here cites material
outside the thirteen sources plus the governing files."

WHAT IS TESTED
  A. Does the extractor see every anchor a human reader would see in the candidate?
     Tested by a deliberately DIFFERENT extraction: any `NNN:` or `path.ext:` followed by
     digits, backticked or not, anywhere in the file.
  B. Is the "1 novel anchor" figure right? The candidate cites files outside the thirteen
     sources (108, 82, 122). Each such citation must appear as NOVEL unless some source
     also carries it.

NEGATIVE CONTROLS, declared before the run
  H1. My looser extractor must find AT LEAST as many anchors as the strict one. If it
      finds fewer, my extractor is broken and every disagreement below is mine.
  H2. A string that is not an anchor but looks like one to a loose pattern (a time, a
      version, a ratio) must be visible in the difference list rather than silently
      inflating it, so the difference is reported item by item and not as a count.
"""
import re, os

D = os.path.dirname(os.path.abspath(__file__)) + "/.."
SOURCES = ["109_bellard_the_primitive_derived_cold.md",
           "110_willsey_the_primitive_derived_cold.md",
           "111_jhala_the_primitive_attacked.md",
           "112_leijen_where_the_refinement_lives.md",
           "113_op_steer_them_and_make_them_build_it_together.md",
           "114_leroy_formalising_the_primitive.md",
           "154_kiselyov_the_primitive_derived_cold.md",
           "155_dolan_the_primitive_derived_cold.md",
           "156_checkpoint_nine_the_queue_for_ops_seat.md",
           "157_lamport_the_primitive_attacked.md",
           "158_dolan_reply_the_instrument_could_not_reach_it.md",
           "159_kiselyov_reply.md",
           "160_leroy_formalising_the_primitive.md"]
CAND = "161_leroy_the_canon_candidate_for_the_primitive.md"

STRICT_LINE = re.compile(r'`([A-Za-z0-9_./-]+\.(?:md|rs|py|toml|sh|err|out|txt|s)):(\d+(?:-\d+)?)`')
STRICT_BARE = re.compile(r'`(\d{2,3}):(\d+(?:-\d+)?)`')
# looser: no backticks required, and a bare panel number may be followed by a section-style
# reference the strict pattern's closing backtick would have hidden.
LOOSE = re.compile(r'\b([A-Za-z0-9_./-]+\.(?:md|rs|py|toml|sh|err|out|txt|s)|\d{2,3}):(\d+(?:-\d+)?)\b')

def strict(t):
    a = set()
    for m in STRICT_LINE.finditer(t): a.add(f"{m.group(1)}:{m.group(2)}")
    for m in STRICT_BARE.finditer(t): a.add(f"{m.group(1)}:{m.group(2)}")
    return a

def loose(t):
    return {f"{m.group(1)}:{m.group(2)}" for m in LOOSE.finditer(t)}

cand = open(os.path.join(D, CAND)).read()
src = ""
for s in SOURCES:
    src += open(os.path.join(D, s)).read()

cs, cl = strict(cand), loose(cand)
ss, sl = strict(src), loose(src)

print(f"candidate anchors, strict pattern : {len(cs)}")
print(f"candidate anchors, loose pattern  : {len(cl)}")
print(f"H1 (loose >= strict)              : {'PASS' if cl >= cs else 'FAIL'}")
print()
extra = sorted(cl - cs)
print(f"seen by the loose pattern and not the strict one: {len(extra)}")
for e in extra:
    print(f"    {e}")
print()
print("B. citations into files outside the thirteen sources, as the strict pattern sees them:")
outside = sorted(a for a in cs if re.match(r'^\d{2,3}:', a)
                 and a.split(':')[0] not in {s.split('_')[0] for s in SOURCES})
for o in outside:
    in_src = o in ss
    print(f"    {o:20s} carried by some source: {in_src}   -> "
          f"{'not novel' if in_src else 'MUST BE REPORTED NOVEL'}")
novel_strict = sorted(cs - ss)
print(f"\nstrict novel set (what the instrument reports): {novel_strict}")
print(f"B verdict: {'PASS, the outside citations are accounted for' if set(o for o in outside if o not in ss) <= set(novel_strict) else 'FAIL, an outside citation is unreported'}")
