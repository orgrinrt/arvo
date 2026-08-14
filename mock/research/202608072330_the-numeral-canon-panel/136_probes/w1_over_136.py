#!/usr/bin/env python3
# 136: the anchor accounting, on 132_probes/w1's patterns, reused rather than rebuilt.
#
# w1 itself extends 119_probes/r1 with the theorem and probe-stem classes. Neither is edited here;
# both belong to other files. This runner only changes what is measured: 136 against the union of
# this topic's five preceding files and the three signatures.
#
# Predictions, stated before running:
#   S1. The accounting-section stripper fires, because section 11 names anchors deliberately.
#       If it does not fire, this section carries no anchors of its own and that is worth knowing.
#       *** THIS PREDICTION IS SELF-REFERENTIAL AND WAS REFUTED ONCE ON THE WAY. *** On the run
#       taken before 136 section 11 listed its dropped anchors, the stripper did not fire,
#       correctly, because there was nothing in that section to strip. Listing them to account
#       for them honestly is what makes it fire. Both runs are real and 136 section 11 states
#       both, because the pair is the point: the honest sentence is what disables the naive
#       count, and the stripper is what restores it.
#   S2. `not carried` is non-empty and every entry in it belongs to a clause 136 does not change.
#       A revision cites a standing clause by section number rather than repeating its anchors, so
#       a zero here would mean 136 had restated 132 rather than superseded parts of it.
#   S3. Every anchor a CHANGED clause rests on is carried. This is the check that matters and it
#       is the one the count cannot make: it is the hand check in section 11, and this probe's job
#       is to produce the list it runs against.
import importlib.util
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
PANEL = os.path.dirname(HERE)

def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod

sys.argv = [sys.argv[0]]
w1 = load(os.path.join(PANEL, "132_probes",
                       "w1_the_anchor_count_and_the_blind_commit_ordering.py"), "w1mod")
r1 = load(os.path.join(PANEL, "119_probes",
                       "r1_the_anchor_inventory_and_what_the_candidate_carries.py"), "r1mod") if os.path.exists(
    os.path.join(PANEL, "119_probes", "r1_the_anchor_inventory_and_what_the_candidate_carries.py")) else w1.r1

UNION = [
    "125_knuth_rounding_cold_derivation.md",
    "126_wronski_what_rounding_actually_selects.md",
    "127_dolan_attacking_the_convergence.md",
    "128_knuth_reply_the_boundary_moves_to_the_coupling.md",
    "129_wronski_reply_the_fork_is_decorrelation_not_determinism.md",
    "130_dolan_reply_one_axis_two_keyings.md",
    "131_leroy_formalising_the_rounding_axis.md",
    "132_leroy_the_canon_candidate_for_the_rounding_axis.md",
    "133_knuth_signature_in_part_with_two_corrections.md",
    "134_wronski_cosign_with_one_addendum_on_the_dither.md",
    "135_dolan_signature_in_part.md",
]
CAND = "136_leroy_the_candidate_revised_against_three_signatures.md"

def read(name):
    p = os.path.join(PANEL, name)
    return open(p).read() if os.path.exists(p) else None

union = {}
missing = []
for name in UNION:
    t = read(name)
    if t is None:
        missing.append(name)
        continue
    for cls, vals in w1.anchors(t).items():
        union.setdefault(cls, set()).update(vals)

raw = read(CAND)
full = w1.anchors(raw)
stripped = w1.anchors(r1.strip_accounting(raw))

print("=" * 96)
print("Anchor accounting for 136, on 132_probes/w1's patterns")
print("=" * 96)
if missing:
    print(f"  union files not present and therefore not counted: {missing}")
print(f"  union built from {len(UNION) - len(missing)} files")
print()
print(f"  {'class':<20}{'in the union':>14}{'in 136':>9}{'incl. accounting':>18}{'not carried':>13}")
fired = []
for cls in sorted(union):
    u = union[cls]
    s = stripped.get(cls, set())
    f = full.get(cls, set())
    nc = sorted(u - s)
    if len(f) > len(s):
        fired.append(f"{cls} +{len(f) - len(s)}")
    print(f"  {cls:<20}{len(u):>14}{len(s):>9}{len(f):>18}{len(nc):>13}")

print()
print(f"  S1, the stripper: {'FIRED  (' + ', '.join(fired) + ')' if fired else 'did not fire'}")

print()
for cls in sorted(union):
    nc = sorted(union[cls] - stripped.get(cls, set()))
    carried = len(union[cls]) - len(nc)
    print(f"  {cls}: {carried} carried, {len(nc)} not")
    if nc:
        for i in range(0, len(nc), 6):
            print("      " + "  ".join(f"{str(x):<20}" for x in nc[i:i + 6]))

print()
print("=" * 96)
print("VERDICT")
print("=" * 96)
print(f"  S1 stripper fired: {bool(fired)}")
tot_nc = sum(len(union[c] - stripped.get(c, set())) for c in union)
print(f"  S2 not-carried is non-empty: {tot_nc > 0} ({tot_nc} across all classes)")
print("  S3 is not answerable by this probe. Whether every CHANGED clause carries its own")
print("     anchors is a hand check against the list above, run in 136 section 11. A count")
print("     cannot make it, because a revision legitimately drops the anchors of clauses it")
print("     leaves standing, and that is indistinguishable from dropping the ones it moved.")
