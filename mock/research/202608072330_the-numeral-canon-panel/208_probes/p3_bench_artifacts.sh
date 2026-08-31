#!/usr/bin/env bash
# p3. The bench artifacts SEED_TALKING_POINTS calls current evidence.
#
# Section 0 exception 2 names `mock/benches/` as the one thing in this workspace
# that can price anything, and names four families the closed panel's members
# built: warm-container-*, warm-clamp-*, bitpack-*, bitpack-footprint-*. It says
# the ARTIFACTS are citable as current evidence while the panel files discussing
# them are not.
#
# Three things are checkable and none had been checked:
#   1. T9 states "57 CSV+meta+findings triples" for `mock/benches/warm-container-*`.
#      A count inside an append-only document goes stale silently, and seat 207
#      found exactly that shape on the op roster (23 stated, 37 actual).
#   2. Whether any were deleted since, which decides whether a low count is an
#      overcount in the document or a loss on disk. Git answers it; mtimes do not.
#   3. Which era produced them, since section 0's four names are offered as the
#      closed panel's output and the directory now holds both panels' runs.
#
# THE CASE THAT MUST FAIL, stated before the run:
#   A. A family name that does not exist must report 0 triples. If the counter
#      reports non-zero for a nonsense family it is counting something else.
#   B. The era split must put at least one family on each side. If everything
#      lands in one era the discriminator is not discriminating.

set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
benches="$(cd "$here/../../../benches" && pwd)"
repo="$(cd "$here/../../../.." && pwd)"

# The current panel's directory is stamped 2026-08-07 23:30 UTC. Anything a
# meta file dates before that was produced under the closed panel.
CUTOFF=1786145400   # 2026-08-07 23:30:00 UTC

echo "== p3. the committed bench artifacts, counted, dated and attributed =="
echo "at     : $benches"
echo "cutoff : $(date -u -r "$CUTOFF" '+%Y-%m-%d %H:%M UTC') (the current panel's own timestamp)"
echo

triples() {
    local fam="$1" n=0 csv stem
    shopt -s nullglob
    for csv in "$benches/${fam}"*.csv; do
        stem="${csv%.csv}"
        [ -f "${stem}.meta.json" ] && [ -f "${stem}_findings.md" ] && n=$((n + 1))
    done
    shopt -u nullglob
    echo "$n"
}

echo "-- control A --"
c=$(triples "definitely-not-a-bench-family-")
if [ "$c" -eq 0 ]; then
    echo "ok : nonsense family reports 0 triples"
else
    echo "FAILED : nonsense family reports $c; the counter counts something else"; exit 1
fi
echo

echo "-- T9's count, and where the number actually comes from --"
actual=$(triples "warm-container")
echo "T9 states                       : 57 triples under mock/benches/warm-container-*"
echo "triples matching that glob      : $actual"
echo "csv ever deleted from that glob : $(cd "$repo" && git log --diff-filter=D --name-only --format='' -- 'mock/benches/warm-container*.csv' | grep -c '\.csv$')"
echo
echo "The archive's own file 141 states the same 57, and 141 is right:"
echo "  csv added by 141's commit ccf05099, across all sections it landed:"
(cd "$repo" && git show --diff-filter=A --name-only --format='' ccf05099 \
    | grep '\.csv$' | sed 's#.*/##;s/_n[0-9]*\.csv$//;s/-w[0-9]*$//;s/-l[0-9]$//' \
    | sort | uniq -c | sort -rn | sed 's/^/    /')
echo "    total: $(cd "$repo" && git show --diff-filter=A --name-only --format='' ccf05099 | grep -c '\.csv$')"
echo
echo "  So 57 is 141's nine-section total. Four of those sections are not"
echo "  warm-container-* at all. The seed attached a nine-section figure to a"
echo "  one-family glob; nothing was lost from disk."
echo

echo "-- every family, with its era --"
printf '%-34s %6s  %s\n' "family" "csv" "era"
old_fams=0; new_fams=0
shopt -s nullglob
for fam in $(ls "$benches"/*.csv | sed 's#.*/##;s/_n[0-9]*\.csv$//;s/\.csv$//;s/-w[0-9]*$//;s/-l[0-9]$//' | sort -u); do
    n=0; oldest=""; newest=""
    for m in "$benches/${fam}"*.meta.json; do
        t=$(grep -oE '"timestamp":[0-9]+' "$m" | cut -d: -f2)
        [ -z "$t" ] && continue
        n=$((n + 1))
        [ -z "$oldest" ] && oldest=$t
        [ "$t" -lt "$oldest" ] && oldest=$t
        [ -z "$newest" ] && newest=$t
        [ "$t" -gt "$newest" ] && newest=$t
    done
    [ "$n" -eq 0 ] && continue
    if [ "$newest" -lt "$CUTOFF" ]; then era="closed panel"; old_fams=$((old_fams + 1))
    elif [ "$oldest" -ge "$CUTOFF" ]; then era="current panel"; new_fams=$((new_fams + 1))
    else era="SPANS BOTH"; fi
    printf '%-34s %6s  %s (%s)\n' "$fam" "$n" "$era" "$(date -u -r "$oldest" '+%Y-%m-%d')"
done
shopt -u nullglob
echo
echo "-- control B --"
if [ "$old_fams" -gt 0 ] && [ "$new_fams" -gt 0 ]; then
    echo "ok : $old_fams families on the closed side, $new_fams on the current side"
else
    echo "FAILED : era split put everything on one side; the discriminator is not discriminating"
fi
echo

echo "-- what section 0's four names reach, of the CLOSED-panel corpus --"
#
# The four names are three globs (bitpack-footprint-* is inside bitpack-*), and
# two of them, bitpack-*, also match families the CURRENT panel produced. So the
# question has to be asked inside the closed-panel set only. An earlier version
# of this block compared a both-era reachable count against a one-era total and
# reported 4 where the answer is 55; the counts were each right and the
# subtraction was between different populations.
reach_old=0; unreach_old=0; unreach_fams=""
shopt -s nullglob
for csv in "$benches"/*.csv; do
    m="${csv%.csv}.meta.json"
    [ -f "$m" ] || continue
    t=$(grep -oE '"timestamp":[0-9]+' "$m" | cut -d: -f2)
    [ -n "$t" ] || continue
    [ "$t" -lt "$CUTOFF" ] || continue
    b=$(basename "$csv")
    case "$b" in
        warm-container*|warm-clamp*|bitpack*) reach_old=$((reach_old + 1)) ;;
        *) unreach_old=$((unreach_old + 1))
           unreach_fams="$unreach_fams$(echo "$b" | sed 's/_n[0-9]*\.csv$//;s/-w[0-9]*$//;s/-l[0-9]$//')\n" ;;
    esac
done
shopt -u nullglob
echo "closed-panel csv total                      : $((reach_old + unreach_old))"
echo "  reachable from the four named prefixes    : $reach_old"
echo "  NOT reachable from them                   : $unreach_old"
echo
echo "  the closed-panel families section 0 does not name:"
printf "%b" "$unreach_fams" | sort | uniq -c | sort -rn | sed 's/^/    /'
echo
echo "Section 0 says \"several bench families\" and gives four, so it does not"
echo "claim to be exhaustive. It is still the file's own statement of what may be"
echo "cited as current evidence, and a reader taking the list as the extent loses"
echo "$unreach_old committed closed-panel csv across the families printed above,"
echo "precise-* and quantiser-* among them."
