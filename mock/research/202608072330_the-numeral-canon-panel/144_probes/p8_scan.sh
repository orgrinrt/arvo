#!/usr/bin/env bash
# Scan the emitted assembly for p8. V2 and W1 are answered here rather than by
# reading, so that a later reader re-runs a command instead of trusting a
# transcript.
#
# 139's p5 scan first reported zero arms for every function because it looked at
# the `extern "C"` entry points, which were thunks tail-calling the
# monomorphised symbol one hop down. This probe removes that hazard at the
# source rather than by chasing the hop: `dispatch` is `#[inline(always)]`, so
# each entry point's own body IS the dispatch and there is nothing below it to
# miss. The runtime-selected control is what proves the scan can still see an
# arm when one is there.
set -u
cd "$(dirname "$0")"

rustc -O -C opt-level=3 --emit asm -o p8.s \
    p8_a_nonlinear_selector_is_const_and_lowers_to_one_branch.rs || exit 1

fails=0
printf '%-26s %6s %6s %6s %6s %14s\n' function lines arm_a arm_b arm_c cond-branches
for f in entry_linear_first entry_linear_balanced entry_cheb_compromise entry_runtime_selected; do
    body=$(awk -v f="_$f:" '
        $0 == f {inside=1; next}
        inside && /cfi_endproc/ {inside=0}
        inside {print}
    ' p8.s)
    lines=$(printf '%s\n' "$body" | grep -c '[^[:space:]]')
    a=$(printf '%s\n' "$body" | grep -c '5arm_a')
    b=$(printf '%s\n' "$body" | grep -c '5arm_b')
    c=$(printf '%s\n' "$body" | grep -c '5arm_c')
    br=$(printf '%s\n' "$body" | grep -cE '^[[:space:]]*(cb[nz]|tb[nz]|b\.[a-z]+|cmp)')
    printf '%-26s %6s %6s %6s %6s %14s\n' "$f" "$lines" "$a" "$b" "$c" "$br"
    total=$((a + b + c))
    case "$f" in
      entry_runtime_selected)
        # W1: the control must keep every arm and at least one conditional branch
        if [ "$total" -lt 3 ] || [ "$br" -lt 1 ]; then
            echo "  W1 FAIL: the control lost arms or branches, so the scan is blind"
            fails=$((fails + 1))
        fi
        ;;
      *)
        # V2: exactly one arm, no conditional branch
        if [ "$total" -ne 1 ] || [ "$br" -ne 0 ]; then
            echo "  V2 FAIL on $f: $total arms, $br conditional branches"
            fails=$((fails + 1))
        fi
        ;;
    esac
done

echo
echo "V2: each const-selected entry is one unconditional branch to one arm."
echo "W1: the runtime-selected control keeps all three arms and two conditional"
echo "    branches, so the absence of arms above is a result rather than a"
echo "    limitation of the scan."
echo
echo "scan failures: $fails"
exit $((fails > 0))
