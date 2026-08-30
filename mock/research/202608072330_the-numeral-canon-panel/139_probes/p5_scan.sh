#!/bin/sh
# p5's asm scan. Answers T2 (the const-selected call keeps one arm) and T3
# (the runtime-selected control keeps both), by looking for the magic constant
# each arm carries.
#
# The scan is only meaningful if it can report "both". T3 is that control: if
# the runtime-selected function also reports one arm, the scan is measuring
# nothing and every T2 result is void.
set -e
cd "$(dirname "$0")"

rustc -O --emit asm -o p5.s p5_open_set.rs 2>/dev/null
rustc -O -o p5 p5_open_set.rs 2>/dev/null

# The magics as they appear in emitted constants, both endian spellings of the
# literal and the decimal form, since the assembler's rendering varies.
FAST_HEX=$(printf '%x' $((0x5BD1E995)))
SMALL_HEX=$(printf '%x' $((0x27D4EB2D)))

echo "magic for the fast arm : 0x$FAST_HEX"
echo "magic for the small arm: 0x$SMALL_HEX"
echo

# Each arm is #[inline(never)], so the reliable signal is which arm SYMBOL the
# monomorphised body branches to. The extern "C" entry points are only thunks
# that tail-call the monomorphisation, so the scan has to find the MANGLED
# monomorphised symbol, not the thunk. That mistake is why the first run of
# this scan reported zero arms for all three functions.
extract() {
  awk -v pat="$1" '
    $0 ~ /^_.*:$/ { inside = ($0 ~ pat) }
    inside { print }
  ' p5.s | awk '{print} /cfi_endproc/{exit}'
}

LIB_PAT="3mul.*13LibraryPreset"
CON_PAT="3mul.*10MyStrategy"
RT_PAT="20mul_runtime_selected"

for pair in "library-preset:$LIB_PAT" "consumer-strategy:$CON_PAT" "runtime-selected:$RT_PAT"; do
  label=${pair%%:*}
  pat=${pair#*:}
  body=$(extract "$pat")
  if [ -z "$body" ]; then
    echo "$label: SYMBOL NOT FOUND in p5.s (scan is broken)"
    continue
  fi
  fast=$(printf '%s\n' "$body" | grep -c 'arm_fast' || true)
  small=$(printf '%s\n' "$body" | grep -c 'arm_small' || true)
  branch=$(printf '%s\n' "$body" | grep -cE '\b(tbz|tbnz|cbz|cbnz|b\.[a-z]+)\b' || true)
  lines=$(printf '%s\n' "$body" | wc -l | tr -d ' ')
  echo "$label: body=${lines} lines, arm_fast=${fast}, arm_small=${small}, conditional branches=${branch}"
done

lp_fast=$(extract "$LIB_PAT" | grep -c 'arm_fast' || true)
lp_small=$(extract "$LIB_PAT" | grep -c 'arm_small' || true)
cs_fast=$(extract "$CON_PAT" | grep -c 'arm_fast' || true)
cs_small=$(extract "$CON_PAT" | grep -c 'arm_small' || true)
rt_fast=$(extract "$RT_PAT" | grep -c 'arm_fast' || true)
rt_small=$(extract "$RT_PAT" | grep -c 'arm_small' || true)

echo
echo "verdict:"

fail=0
if [ "$lp_fast" -ge 1 ] && [ "$lp_small" -eq 0 ]; then
  echo "  T2a library preset (time-weighted) keeps ONLY the fast arm: PASS"
else
  echo "  T2a library preset keeps fast=$lp_fast small=$lp_small: FAIL"
  fail=1
fi
if [ "$cs_small" -ge 1 ] && [ "$cs_fast" -eq 0 ]; then
  echo "  T2b consumer strategy (space-weighted) keeps ONLY the small arm: PASS"
else
  echo "  T2b consumer strategy keeps fast=$cs_fast small=$cs_small: FAIL"
  fail=1
fi
if [ "$rt_fast" -ge 1 ] && [ "$rt_small" -ge 1 ]; then
  echo "  T3 runtime-selected control keeps BOTH arms, so the scan can see a"
  echo "     surviving arm and the two PASSes above are not free: PASS"
else
  echo "  T3 runtime-selected control keeps fast=$rt_fast small=$rt_small: FAIL,"
  echo "     the scan cannot distinguish, so T2 is void"
  fail=1
fi
echo
echo "scan failures: $fail"
exit $fail
