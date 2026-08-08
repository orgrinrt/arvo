# p1. Does anything in the panel, in SETTLED.md, or in the repository's own agent rules
# actually DEFINE what a strategy is, as opposed to using the word?
#
# Method: a definition is a sentence that predicates something of the subject "a strategy" /
# "the strategy axis" / "strategies" with a copula or a defining verb. Using the word in a
# clause about something else is not a definition. So we look for the copular frames
# specifically, and print every hit for manual reading, because the classification is the
# judgement and the grep is only the sieve.
#
# Run from the panel directory.
import io, os, re, glob

PANEL = "/Users/orgrinrt/Dev/clause-dev/arvo/mock/research/202608072330_the-numeral-canon-panel"
ARVO = "/Users/orgrinrt/Dev/clause-dev/arvo"

# copular / defining frames with 'strategy' as the SUBJECT
FRAMES = [
    r"\ba strategy is\b",
    r"\bstrategies are\b",
    r"\bthe strategy (?:axis )?is\b",
    r"\ba strategy (?:means|denotes|names|selects|carries|is defined)\b",
    r"\bstrategy (?:is defined as|means)\b",
    r"\bwhat a strategy\b",
    r"\bstrategy marker is\b",
    r"\bmarkers are\b",
]
PAT = re.compile("|".join(FRAMES), re.I)

def scan(paths, label):
    print("=" * 72)
    print(label)
    print("=" * 72)
    n = 0
    for fp in sorted(paths):
        try:
            lines = io.open(fp, encoding="utf-8", errors="replace").readlines()
        except (IOError, OSError):
            continue
        for i, ln in enumerate(lines, 1):
            if PAT.search(ln):
                n += 1
                print("%-52s :%-5d %s" % (os.path.relpath(fp, ARVO), i, ln.strip()[:150]))
    print("-> %d hits\n" % n)
    return n

panel_md = [p for p in glob.glob(os.path.join(PANEL, "*.md"))]
scan(panel_md, "PANEL *.md (incl. SETTLED, RULES, CANON_CANDIDATE, MORNING)")

rules = glob.glob(os.path.join(ARVO, ".claude", "rules", "*.md")) + \
        glob.glob(os.path.join(ARVO, ".claude", "*.md"))
scan(rules, "arvo/.claude (the repository's own agent rules)")

ws = glob.glob("/Users/orgrinrt/Dev/clause-dev/.claude/rules/*.md")
scan(ws, "workspace .claude/rules")
