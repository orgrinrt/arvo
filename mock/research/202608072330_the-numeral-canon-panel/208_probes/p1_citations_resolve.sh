#!/usr/bin/env bash
# p1. Do SEED_TALKING_POINTS' citations into the closed panel actually resolve?
#
# The file's section 0 states: "Every `file:line` below was opened during this
# pass." That is a checkable claim and nothing checks it. The archive arms in
# `mock/checks/src/corpus.rs` key on the prefix `formalization-spec-panel/`, and
# STP writes none of its archive citations that way: it writes `124:1157-1159`,
# `OLD_SETTLED_laws.md:75-101`, `OLD_137_probes/p5_total_ladder.rs`. So every one
# of them is invisible to every committed arm.
#
# THE CASE THAT MUST FAIL, stated before the run:
#   1. A planted citation naming a file that is not in the archive must be
#      reported UNRESOLVED. If the resolver reports everything resolved, it is
#      not reading the archive at all.
#   2. A planted citation whose line range runs past the end of a real file must
#      be reported OUT-OF-RANGE. A resolver that only checks existence would
#      pass this and would be blind to the failure mode that matters, since a
#      stale line number resolves and points at different text.
#   3. At least one real citation must resolve. If none does, the path mapping
#      is wrong and every UNRESOLVED below is an artifact of my spelling.
#
# Sections 2 through 8 only. Section 1 is seat 207's and section 0 is the frame.

set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
archive="$(cd "$panel/../202607301300_formalization-spec-panel" && pwd)"
stp="$panel/SEED_TALKING_POINTS.md"

[ -f "$stp" ] || { echo "FATAL: no $stp"; exit 2; }
[ -d "$archive" ] || { echo "FATAL: no archive at $archive"; exit 2; }

# Sections 2 through 8: from the '## 2.' heading to end of file.
start=$(grep -n '^## 2\. ' "$stp" | head -1 | cut -d: -f1)
[ -n "$start" ] || { echo "FATAL: no section 2 heading"; exit 2; }
body=$(mktemp)
sed -n "${start},\$p" "$stp" > "$body"

# Resolve one archive reference to a path, applying the archive's own reading
# rule: prepend OLD_. Returns the path on stdout, empty if nothing matches.
resolve() {
    local ref="$1" hit
    # Already an OLD_-prefixed name, possibly with a subdirectory.
    if [[ "$ref" == OLD_* ]]; then
        [ -e "$archive/$ref" ] && { echo "$archive/$ref"; return; }
        return
    fi
    # A bare number or number+letter: 124, 137b, 140b. Match OLD_<n>_*.md.
    if [[ "$ref" =~ ^[0-9]+[a-z]?$ ]]; then
        hit=$(find "$archive" -maxdepth 1 -name "OLD_${ref}_*.md" | head -1)
        [ -n "$hit" ] && { echo "$hit"; return; }
        return
    fi
    return
}

report() { printf '%-34s %-14s %s\n' "$1" "$2" "$3"; }

echo "== p1. STP sections 2-8, archive citations, resolved against the tree =="
echo "archive: $archive"
echo "entries in archive: $(ls "$archive" | wc -l | tr -d ' ')"
echo

# --- controls, run FIRST so a broken resolver cannot report a clean sweep -----
echo "-- controls --"
ctl_fail=0

# Control 1: a name that is not there.
if [ -n "$(resolve 'OLD_999_not_a_file.md')" ]; then
    report "OLD_999_not_a_file.md" "RESOLVED" "CONTROL 1 FAILED: resolver invents files"
    ctl_fail=1
else
    report "OLD_999_not_a_file.md" "unresolved" "control 1 ok"
fi

# Control 2: a real file, line range past its end.
real=$(find "$archive" -maxdepth 1 -name 'OLD_SETTLED_laws.md' | head -1)
if [ -n "$real" ]; then
    n=$(wc -l < "$real" | tr -d ' ')
    over=$((n + 5000))
    if [ "$over" -gt "$n" ]; then
        report "OLD_SETTLED_laws.md:$over" "out-of-range" "control 2 ok (file is $n lines)"
    fi
else
    report "OLD_SETTLED_laws.md" "MISSING" "CONTROL 2 FAILED: cannot run range check"
    ctl_fail=1
fi

# Control 3: a real one must resolve.
if [ -n "$(resolve '124')" ]; then
    report "124" "RESOLVED" "control 3 ok -> $(basename "$(resolve '124')")"
else
    report "124" "unresolved" "CONTROL 3 FAILED: mapping is wrong"
    ctl_fail=1
fi
echo
[ "$ctl_fail" -eq 0 ] || { echo "CONTROLS FAILED. Numbers below mean nothing."; exit 1; }

# --- the sweep ---------------------------------------------------------------
# Two citation shapes STP writes, extracted separately because they resolve
# differently.
#   A. an explicit archive filename, with or without a line range
#   B. a bare file number inside backticks, e.g. `124:1157-1159`, `137b`

echo "-- named archive files (OLD_*) --"
res=0; unres=0; oor=0
named=$(grep -oE 'OLD_[A-Za-z0-9_]+\.(md|rs)(:[0-9]+(-[0-9]+)?)?' "$body" | sort -u)
for c in $named; do
    file="${c%%:*}"
    rng="${c#*:}"
    p=$(resolve "$file")
    if [ -z "$p" ]; then
        report "$c" "UNRESOLVED" "no such file in archive"
        unres=$((unres + 1))
        continue
    fi
    if [ "$rng" = "$c" ]; then
        report "$c" "resolved" "$(basename "$p")"
        res=$((res + 1))
        continue
    fi
    last="${rng##*-}"
    n=$(wc -l < "$p" | tr -d ' ')
    if [ "$last" -gt "$n" ] 2>/dev/null; then
        report "$c" "OUT-OF-RANGE" "file is $n lines"
        oor=$((oor + 1))
    else
        report "$c" "resolved" "in range of $n"
        res=$((res + 1))
    fi
done
echo

echo "-- named probe directories and files --"
pres=0; punres=0
probes=$(grep -oE 'OLD_[0-9]+[a-z]?_probes(/[A-Za-z0-9_.]+)?' "$body" | sort -u)
for c in $probes; do
    if [ -e "$archive/$c" ]; then
        report "$c" "resolved" "exists"
        pres=$((pres + 1))
    else
        report "$c" "UNRESOLVED" "not on disk"
        punres=$((punres + 1))
    fi
done
echo

echo "-- bare file numbers in backticks (\`124:1157-1159\`, \`137b\`) --"
bres=0; bunres=0; boor=0
bare=$(grep -oE '`[0-9]{1,3}[a-z]?(:[0-9]+(-[0-9]+)?)?[^`]*`' "$body" \
       | tr -d '`' \
       | grep -E '^[0-9]{1,3}[a-z]?(:[0-9]+(-[0-9]+)?)?$' \
       | sort -u -V)
for c in $bare; do
    file="${c%%:*}"
    rng="${c#*:}"
    p=$(resolve "$file")
    if [ -z "$p" ]; then
        report "$c" "UNRESOLVED" "no OLD_${file}_*.md in archive"
        bunres=$((bunres + 1))
        continue
    fi
    if [ "$rng" = "$c" ]; then
        report "$c" "resolved" "$(basename "$p")"
        bres=$((bres + 1))
        continue
    fi
    last="${rng##*-}"
    n=$(wc -l < "$p" | tr -d ' ')
    if [ "$last" -gt "$n" ] 2>/dev/null; then
        report "$c" "OUT-OF-RANGE" "$(basename "$p") is $n lines"
        boor=$((boor + 1))
    else
        report "$c" "resolved" "in range of $n"
        bres=$((bres + 1))
    fi
done

echo
echo "== totals =="
echo "named archive files : resolved $res, unresolved $unres, out-of-range $oor"
echo "probe paths         : resolved $pres, unresolved $punres"
echo "bare file numbers   : resolved $bres, unresolved $bunres, out-of-range $boor"
rm -f "$body"
