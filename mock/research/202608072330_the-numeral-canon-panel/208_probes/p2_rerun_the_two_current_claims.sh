#!/usr/bin/env bash
# p2. The two claims SEED_TALKING_POINTS calls current evidence, re-run today.
#
# Section 0 exception 1 names exactly two results as current: T1's total ladder
# and T2's structural array, recompiled 2026-08-09 under the pin, logs committed
# at `SEED_TALKING_POINTS_probes/`. Nothing has re-run them since, and a compile
# fact is exactly the kind of claim that stops being true without announcing it.
#
# This is the third independent instance of each: the archive's own run, the
# 2026-08-09 re-run, and this one.
#
# THE CASE THAT MUST FAIL, stated before the run. A sweep in which every arm
# compiles proves nothing about whether the compiler was invoked at all, so the
# archive's own negative controls are arms here:
#   b1b  one byte of padding in the odd-arity node. MUST REFUSE. If it compiles,
#        every layout assertion in b1 is vacuous and b1's `unsafe` undischarged.
#   b1c  the same corruption reached through an unnamed capacity. MUST REFUSE.
# A run where b1 compiles and b1b does not refuse is a broken run reporting a
# clean result, which is the shape this probe exists to make impossible.

set -uo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
archive="$(cd "$here/../../202607301300_formalization-spec-panel" && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

echo "== p2. re-running the two current-evidence claims, plus the archive's own controls =="
echo "date    : $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
echo "rustc   : $(rustc --version)"
echo "pin     : $(grep channel "$here/../../../../rust-toolchain.toml" | tr -d ' ')"
echo "archive : $archive"
echo

# arm <label> <path> <expect: compiles|refuses> <extra rustc args...>
fails=0
arm() {
    local label="$1" src="$2" expect="$3"; shift 3
    if [ ! -f "$src" ]; then
        printf '%-14s %-9s %s\n' "$label" "MISSING" "no such file: $src"
        fails=$((fails + 1)); return
    fi
    local log="$work/$label.log" rc
    ( cd "$work" && rustc --edition 2024 "$@" "$src" ) > "$log" 2>&1
    rc=$?
    local got; [ "$rc" -eq 0 ] && got="compiles" || got="refuses"
    if [ "$got" = "$expect" ]; then
        printf '%-14s %-9s exit %-3s as required\n' "$label" "$got" "$rc"
    else
        printf '%-14s %-9s exit %-3s EXPECTED %s -- first diagnostic:\n' \
            "$label" "$got" "$rc" "$expect"
        grep -m2 -E '^(error|warning)' "$log" | sed 's/^/                 /'
        fails=$((fails + 1))
    fi
}

echo "-- controls, run first --"
arm "b1b" "$archive/OLD_76_probes/b1b_layout_law_negative_control.rs" refuses --crate-type=lib
arm "b1c" "$archive/OLD_76_probes/b1c_perimeter_control.rs"           refuses --crate-type=lib
echo
echo "-- the two claims --"
arm "T2 b1"  "$archive/OLD_76_probes/b1_structural_array.rs" compiles --crate-type=lib
arm "T1 p5"  "$archive/OLD_137_probes/p5_total_ladder.rs"    compiles

echo
if [ "$fails" -eq 0 ]; then
    echo "RESULT: all four arms behaved as the archive recorded. Both compile facts"
    echo "reproduce on the pin today, and the two negative controls still refuse,"
    echo "so the compiler was doing work and the positives are not vacuous."
else
    echo "RESULT: $fails arm(s) disagreed with the archive. Read the diagnostics above"
    echo "before quoting either claim."
fi
exit 0
