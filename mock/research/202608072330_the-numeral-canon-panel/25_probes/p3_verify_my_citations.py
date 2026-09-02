# p3. Every file:line I cite in 25, opened and printed, so a reader checks the target rather
# than trusting me. The panel has caught rung-off-a-summary errors five times; this is the
# thirty seconds that prevents a sixth.
#
# Each entry is (path, line-or-range, the word I expect to find there). The expectation is
# the real test: a citation that resolves to the wrong content still "resolves".
import io, os

# Resolved from this file's own location. All three were absolute, naming a
# checkout that still exists on this host, so they did not fail when the arc
# moved: they resolved against a different tree and said nothing.
PANEL = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PRIOR = os.path.normpath(os.path.join(PANEL, "../202607301300_formalization-spec-panel"))
ARVO = os.path.normpath(os.path.join(PANEL, "../../.."))

F = {
    "06": PANEL + "/06_kiselyov_where_a_numeral_is_inferred.md",
    "10": PANEL + "/10_lattner_fresh_eyes_on_the_container_derivation.md",
    "11": PANEL + "/11_chlipala_prior_art_on_typed_widths.md",
    "15": PANEL + "/15_giesen_the_axes_the_ladders_left_out.md",
    "23": PANEL + "/23_spj_the_sentences_a_canon_could_carry.md",
    "01": PANEL + "/01_op_answers.md",
    "RULES": PANEL + "/RULES.md",
    # Archived and prefixed after this probe was written; the content is
    # unchanged, so pointing at where it went is what keeps the check honest.
    "SETTLED": PANEL + "/archive/OLD_SETTLED.md",
    "MORNING": PANEL + "/archive/OLD_MORNING.md",
    "CANDIDATE": PANEL + "/archive/OLD_CANON_CANDIDATE.md",
    # The prior panel is archived in full, every file `OLD_` prefixed, which
    # happened after this probe was written. The reading rule is to prepend it.
    "142c": PRIOR + "/OLD_142c_op_checkpoint_thirtyfive.md",
    "143b": PRIOR + "/OLD_143b_op_checkpoint_thirtysix.md",
    "144b": PRIOR + "/OLD_144b_op_checkpoint_thirtyseven.md",
    "impl": ARVO + "/.claude/rules/implementation.md",
    "toolbox": os.path.join(PANEL, "../../../../.claude/rules/arvo-toolbox-not-policer.md"),
}

CITES = [
    ("06", 306, 306, "breaks that tie"),
    ("10", 193, 193, "does not know what a strategy is"),
    ("11", 544, 545, "does not know what a strategy is"),
    ("15", 426, 426, "granularly"),
    ("23", 701, 710, "strategy-blind"),
    ("23", 813, 813, "footprint"),
    ("23", 825, 828, "wrap"),
    ("23", 838, 846, "headroom rule"),
    ("23", 918, 926, "rung read off a summary"),
    ("23", 1023, 1029, "eight rows mention"),
    ("23", 1096, 1096, "footprint"),
    ("01", 96, 98, "confident enough"),
    ("RULES", 62, 66, "Converge before escalating"),
    ("RULES", 72, 78, "Forbidden"),
    ("RULES", 103, 104, "did not cover"),
    ("RULES", 106, 109, "never happened"),
    ("RULES", 110, 114, "Every probe is a spike"),
    ("RULES", 116, 118, "One instance of evidence"),
    ("RULES", 175, 197, "cost eighteen files"),
    ("RULES", 153, 157, "61 of 78 citations"),
    ("RULES", 124, 126, "Counts are measurements"),
    ("SETTLED", 78, 87, "strategy and profile"),
    ("SETTLED", 73, 74, "erases on lowering"),
    ("SETTLED", 93, 93, "never written by a consumer"),
    ("MORNING", 108, 108, "No row anywhere says what a strategy is"),
    ("CANDIDATE", 3163, 3163, "Layout"),
    ("142c", 59, 59, "function of the active profile"),
    ("142c", 58, 60, "one mechanism with two knobs"),
    ("143b", 10, 12, "constant"),
    ("143b", 16, 17, "It is wrong"),
    ("143b", 24, 27, "special case"),
    ("143b", 81, 83, "incomplete by a lot"),
    ("143b", 105, 107, "additive"),
    ("144b", 10, 16, "synergy"),
    ("144b", 33, 34, "separation working"),
    ("144b", 52, 52, "select a strategy"),
    ("impl", 31, 31, "load-bearing"),
    ("impl", 34, 36, "SIMD"),
    ("impl", 52, 58, "bitpacked"),
    ("impl", 60, 64, "MUST carry"),
    ("toolbox", 0, 0, "not buckets the substrate picks"),
]

bad = 0
for key, a, b, expect in CITES:
    fp = F[key]
    lines = io.open(fp, encoding="utf-8", errors="replace").readlines()
    if a == 0:                              # whole-file expectation
        blob = " ".join(" ".join(lines).split())
        ok = expect in blob
        print("%-10s %-9s %-34s %s" % (key, "(file)", expect[:34], "OK" if ok else "*** MISSING ***"))
        bad += 0 if ok else 1
        continue
    if b > len(lines):
        print("%-10s %-9s %-34s *** OUT OF RANGE, file has %d lines ***"
              % (key, "%d-%d" % (a, b), expect[:34], len(lines)))
        bad += 1
        continue
    blob = " ".join(" ".join(lines[a - 1:b]).split())
    ok = (expect == "") or (expect.lower() in blob.lower())
    print("%-10s %-9s %-34s %s" % (key, "%d-%d" % (a, b), expect[:34], "OK" if ok else "*** CONTENT MISMATCH ***"))
    if not ok:
        print("      got: %s" % blob.strip().replace("\n", " ")[:150])
        bad += 1

print()
print("citations checked: %d, failures: %d" % (len(CITES), bad))
