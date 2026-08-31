#!/bin/sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
src="$here/p4_storage_from_the_capacity_type.rs"
out="$here/p4_out"; rm -rf "$out"; mkdir -p "$out"
RUSTC="${RUSTC:-rustc}"
echo "### $($RUSTC +nightly-2026-05-28 --version)"
echo "### src: $(basename "$src")"
echo
run() {
  lbl=$1; req=$2; shift 2
  args=""; for c in "$@"; do args="$args --cfg $c"; done
  # shellcheck disable=SC2086
  if $RUSTC +nightly-2026-05-28 --edition 2021 --crate-type lib --out-dir "$(mktemp -d)" \
       $args "$src" > "$out/$lbl.log" 2>&1; then got=COMPILE; else got=REFUSE; fi
  if [ "$got" = "$req" ]; then v="as required"; else v="*** NOT AS REQUIRED ***"; fi
  printf "%-8s %-12s required=%-8s got=%-8s %s\n" "$lbl" "${*:-<none>}" "$req" "$got" "$v"
  if [ "$got" = REFUSE ]; then grep -m3 -E '^error' "$out/$lbl.log" | sed 's/^/           /'; fi
}
run S1S2S3 COMPILE
run S2m    REFUSE  mutate
run S4       REFUSE  arm_s4
run S6b      REFUSE  arm_s6b

echo
echo "--- S7: built as a binary and executed ---"
bin=$(mktemp -d)
if $RUSTC +nightly-2026-05-28 --edition 2021 --crate-type bin --cfg arm_run \
     -o "$bin/s7" "$src" > "$out/S7.log" 2>&1; then
  if "$bin/s7" >> "$out/S7.log" 2>&1; then
    printf "%-8s %-12s required=%-8s got=%-8s %s\n" S7 arm_run "RUN-OK" "RUN-OK" "as required"
    tail -2 "$out/S7.log" | sed 's/^/           /'
  else
    printf "%-8s %-12s required=%-8s got=%-8s %s\n" S7 arm_run "RUN-OK" "FAILED" "*** NOT AS REQUIRED ***"
    tail -4 "$out/S7.log" | sed 's/^/           /'
  fi
else
  printf "%-8s %-12s required=%-8s got=%-8s %s\n" S7 arm_run "RUN-OK" "NO-BUILD" "*** NOT AS REQUIRED ***"
  grep -m2 -E '^error' "$out/S7.log" | sed 's/^/           /'
fi
if $RUSTC +nightly-2026-05-28 --edition 2021 --crate-type bin --cfg arm_run --cfg mutate \
     -o "$bin/s7m" "$src" > "$out/S7m.log" 2>&1; then
  printf "%-8s %-12s required=%-8s got=%-8s %s\n" S7m "arm_run,mutate" "NO-BUILD" "BUILT" "*** NOT AS REQUIRED ***"
else
  printf "%-8s %-12s required=%-8s got=%-8s %s\n" S7m "arm_run,mutate" "NO-BUILD" "NO-BUILD" "as required"
  grep -m1 -E '^error' "$out/S7m.log" | sed 's/^/           /'
fi
