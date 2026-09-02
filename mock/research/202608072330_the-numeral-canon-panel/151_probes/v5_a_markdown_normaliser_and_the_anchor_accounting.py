#!/usr/bin/env python3
# v5 (151): the anchor accounting for 151, on a normaliser written against markdown's inline
# grammar rather than against a list of tokens.
#
# 150 reports the SIXTH instance in this panel of markdown defeating a citation checker, arriving
# with a token none of the previous five used: blockquote markers, where collapsing whitespace
# leaves a stray `>` mid-sentence that stripping backticks and emphasis does not touch. Its reading
# is that the class is wider than 146 section 10 states, that each instance arrives with a different
# token, and that the fix is a normaliser written against a grammar rather than another token in a
# strip list. It says it did not write one and added the token instead.
#
# 146 section 10's own statement of the class is narrower than the evidence, and that is mine to
# repair. This probe writes the normaliser and controls it against every token the panel has
# observed, plus three the panel has not.
#
# Predictions, stated before running:
#
#   E1. A grammar-shaped normaliser recovers a phrase wrapped in every construct observed so far:
#       backticks, bold, italic, blockquote markers, headings, list markers.
#   E2. It also recovers one wrapped in three constructs nobody has hit yet: link syntax, nested
#       emphasis, and a hard line break. If any of these fails, the fix is still incomplete and I
#       say which.
#   E3. CONTROL, the one that matters: normalising must not manufacture a hit. A phrase absent from
#       the source must stay absent under every construct.
#   E4. CONTROL: a phrase present but at the wrong span must still fail, so the normaliser has not
#       become a substring search over the whole file.
#   E5. The anchor accounting: the stripper fires on 151's own accounting section.
import importlib.util
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

# ---------------------------------------------------------------------------
# The normaliser, written against markdown's inline grammar.
# ---------------------------------------------------------------------------
_BLOCK_PREFIX = re.compile(r"^[ \t]*(?:>[ \t]?)+|^[ \t]*#{1,6}[ \t]+|^[ \t]*(?:[-*+]|\d+\.)[ \t]+",
                           re.M)
_CODE_SPAN = re.compile(r"`+")
_LINK = re.compile(r"\[([^\]]*)\]\([^)]*\)")
_IMAGE = re.compile(r"!\[([^\]]*)\]\([^)]*\)")
_EMPH = re.compile(r"(\*{1,3}|_{1,3})")
_HARD_BREAK = re.compile(r"[ \t]{2,}\n|\\\n")

def normalise(text):
    """Reduce markdown to its text content.

    Order matters and is the whole of the grammar argument: block-level prefixes are stripped per
    line BEFORE whitespace collapses, because that is the only point at which a `>` is
    distinguishable from a `>` inside prose. Images before links, because an image is a link with a
    bang. Emphasis and code fences are inline runs of a delimiter and are removed as runs rather
    than as characters, so a literal asterisk in prose survives as one.
    """
    text = _HARD_BREAK.sub("\n", text)
    text = _BLOCK_PREFIX.sub("", text)
    text = _IMAGE.sub(r"\1", text)
    text = _LINK.sub(r"\1", text)
    text = _CODE_SPAN.sub("", text)
    text = _EMPH.sub("", text)
    return " ".join(text.split()).lower()

print("=" * 96)
print("E1/E2. Does the normaliser recover a phrase wrapped in each construct?")
print("=" * 96)
PHRASE = "the reduction is a congruence for the following operation"
WRAPS = {
    "plain": PHRASE,
    "backticks (instance 1-4)": f"the reduction is a `congruence` for the following operation",
    "bold (instance 1-4)": f"the reduction is a **congruence** for the following operation",
    "italic": f"the reduction is a *congruence* for the following operation",
    "blockquote (instance 6)": f"> the reduction is a congruence for the\n> following operation",
    "heading": f"### the reduction is a congruence for the following operation",
    "list item": f"- the reduction is a congruence for the following operation",
    "nested emphasis (untested)": f"the reduction is a ***congruence*** for the following operation",
    "link (untested)": f"the reduction is a [congruence](http://x) for the following operation",
    "hard break (untested)": f"the reduction is a congruence for the  \nfollowing operation",
    "blockquote + bold + tick": f"> the **reduction** is a `congruence` for the\n> following operation",
}
target = normalise(PHRASE)
ok, bad = 0, []
for name, s in WRAPS.items():
    hit = target in normalise(s)
    print(f"  {'ok  ' if hit else 'FAIL'}  {name}")
    ok += hit
    if not hit:
        bad.append(name)
e1 = not bad
print(f"\n  E1/E2: {'CONFIRMED' if e1 else 'REFUTED on ' + ', '.join(bad)} "
      f"({ok} of {len(WRAPS)})")

print()
print("=" * 96)
print("E3/E4. The controls that keep the normaliser from becoming a substring search")
print("=" * 96)
ABSENT = "the reduction is a homomorphism for the preceding operation"
e3 = all(normalise(ABSENT) not in normalise(s) for s in WRAPS.values())
print(f"  E3 a phrase nobody wrote stays absent under every construct: {e3} (must be True)")
doc = "para one.\n\n" + WRAPS["bold (instance 1-4)"] + "\n\npara three."
lines = doc.split("\n")
seg = "\n".join(lines[0:2])
e4 = target not in normalise(seg)
print(f"  E4 a real phrase read at the wrong span still fails: {e4} (must be True)")
print(f"  E3/E4: {'CONFIRMED' if e3 and e4 else 'REFUTED'}")

print()
print("=" * 96)
print("E5. The anchor accounting for 151")
print("=" * 96)

def load(rel, name):
    spec = importlib.util.spec_from_file_location(name, os.path.join(PANEL, rel))
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod

sys.argv = [sys.argv[0]]
r1 = load("119_probes/r1_the_anchor_inventory_and_what_the_candidate_carries.py", "r1mod")

_RE_FINDING = re.compile(r"\bF\d{1,3}(?:-\d{1,2})?(?:'(?!s))?\b")
_RE_OPTION = re.compile(r"\bO-1\d\d-[A-Z]\b")
_RE_PROBE = re.compile(
    r"\b\d+_probes/[a-z]\d+[a-z]?(?:_[A-Za-z0-9_]*)?(?:\.(?:py|rs|sh|txt|s))?\b"
    r"|\b[a-z]\d+[a-z]?_[A-Za-z0-9_]+\.(?:py|rs|sh|txt|s)\b")
_RE_LINE = re.compile(r"\b(1[0-5]\d):(\d+)(?:-\d+)?\b")

def stem(x):
    x = x.split("/")[-1]
    m = re.match(r"([a-z]\d+[a-z]?)", x)
    return m.group(1) if m else x

def anchors(text):
    t = text.replace("`", "").replace("**", "").replace("*", "")
    return {
        "finding": set(_RE_FINDING.findall(t)),
        "option": set(_RE_OPTION.findall(t)),
        "probe_stem": {stem(x) for x in _RE_PROBE.findall(t)},
        "line_panel": {f"{a}:{b}" for a, b in _RE_LINE.findall(t)},
    }

UNION = [
    "139_muratori_the_strategy_set_derived_cold.md",
    "140_mcsherry_the_strategy_set_derived_cold.md",
    "141_lamport_the_strategy_set_attacked.md",
    "142_muratori_reply_the_repair_was_dead_on_arrival.md",
    "143_mcsherry_reply_the_scope_the_wording_and_one_claim_i_keep.md",
    "144_fog_the_weighting_half_measured.md",
    "145_leroy_formalising_the_strategy_object.md",
    "146_leroy_the_canon_candidate_for_the_strategy_object.md",
    "147_muratori_signature_in_part_one_clause_is_false.md",
    "148_mcsherry_signature_in_part_on_the_strategy_object.md",
    "149_lamport_signature_in_part_on_the_strategy_object.md",
    "150_fog_signature_in_part_on_the_strategy_object.md",
]
CAND = "151_leroy_the_candidate_revised_against_four_signatures.md"

def read(n):
    p = os.path.join(PANEL, n)
    return open(p).read() if os.path.exists(p) else None

union = {}
for n in UNION:
    t = read(n)
    if t is None:
        continue
    for c, v in anchors(t).items():
        union.setdefault(c, set()).update(v)

raw = read(CAND)
if raw is None:
    print("  151 not present yet; union only.")
    for c in sorted(union):
        print(f"    {c:<16}{len(union[c]):>6}")
    sys.exit(0)

full = anchors(raw)
strip = anchors(r1.strip_accounting(raw))
print(f"  {'class':<16}{'in the union':>14}{'in 151':>9}{'incl. accounting':>18}{'not carried':>13}")
fired = []
for c in sorted(union):
    u, s, f = union[c], strip.get(c, set()), full.get(c, set())
    if len(f) > len(s):
        fired.append(f"{c} +{len(f) - len(s)}")
    print(f"  {c:<16}{len(u):>14}{len(s):>9}{len(f):>18}{len(u - s):>13}")
print(f"\n  E5, the stripper: {'FIRED  (' + ', '.join(fired) + ')' if fired else 'did not fire'}")
print()
for c in sorted(union):
    nc = sorted(union[c] - strip.get(c, set()))
    print(f"  {c}: {len(union[c]) - len(nc)} carried, {len(nc)} not")
    for i in range(0, len(nc), 6):
        print("      " + "  ".join(f"{str(x):<18}" for x in nc[i:i + 6]))
