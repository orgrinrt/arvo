# p4. Section 4.3 says arvo's own preset table cannot establish that the arithmetic axes vary
# independently of the storage axes, because four presets fill a 2x2 with no freedom left.
# That is true of the TABLE. It is not true of the repository.
#
# The committed `warm-clamp-arity-*` family in mock/benches/ holds one strategy (Warm), one
# overflow policy (clamp), one layout, and varies the accumulator. Its own title says the arms
# are "the shipped doubled container against minimum storage, against minimum storage with the
# fold lane-split, and against minimum storage with the accumulator sized by the design's own
# interior-safety rule". So it varies headroom and intermediate precision against each other,
# which is exactly the independence the table cannot show.
#
# This probe only reads committed findings files and reports which variant wins in each. It
# proves independence and variability; it does not price anything, and the numbers it prints
# belong to the harness run that produced them, not to this probe.
import io, os, re, glob, subprocess

BENCH = "/Users/orgrinrt/Dev/clause-dev/arvo/mock/benches"

def tracked(path):
    r = subprocess.run(["git", "ls-files", "--error-unmatch", os.path.relpath(path, "/Users/orgrinrt/Dev/clause-dev/arvo")],
                       cwd="/Users/orgrinrt/Dev/clause-dev/arvo",
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return r.returncode == 0

# what each arm varies, read off the family's own title rather than assigned by me
ARM = {
    "warm-clamp-acc64":     "headroom=doubled (the shipped container)",
    "warm-clamp-minimum":   "headroom=minimum",
    "warm-clamp-min-lanes": "headroom=minimum + fold lane-split",
    "warm-clamp-accfit":    "headroom=minimum, accumulator by interior-safety rule",
    "warm-clamp-accfit-dyn":"headroom=minimum, accumulator sized dynamically",
    "warm-clamp-head":      "headroom rule as shipped",
}

files = sorted(glob.glob(os.path.join(BENCH, "warm-clamp-arity-*_findings.md")))
rows, skipped = [], []
for fp in files:
    if not tracked(fp):
        skipped.append(os.path.basename(fp))
        continue
    s = io.open(fp, encoding="utf-8", errors="replace").read()
    m = re.search(r"^- \*\*Fastest: (\S+)\*\* at ([\d.]+) ns median \(([-+][\d.]+)% vs baseline\)", s, re.M)
    if not m:
        continue
    rows.append((os.path.basename(fp).replace("_findings.md", ""), m.group(1), m.group(2), m.group(3)))

print("committed findings files read : %d" % len(rows))
print("uncommitted, SKIPPED (a claim on one of these would be void): %d" % len(skipped))
for b in skipped:
    print("    %s" % b)
print()

print("%-38s %-24s %14s %9s" % ("run", "fastest arm", "median ns", "vs base"))
for r in rows:
    print("%-38s %-24s %14s %9s" % r)
print()

winners = {}
for _, w, _, _ in rows:
    winners[w] = winners.get(w, 0) + 1
print("winner tally across committed runs:")
for w, c in sorted(winners.items(), key=lambda x: -x[1]):
    print("   %-24s %2d runs   [%s]" % (w, c, ARM.get(w, "?")))
print()

if len(winners) > 1:
    print("RESULT: the winning arm is NOT constant across the family.")
    print("  -> %d distinct arms win somewhere." % len(winners))
    print("  -> the best value on these axes is a function of the run's conditions, not a constant.")
else:
    print("RESULT: one arm wins everywhere; no flip in the committed set.")
