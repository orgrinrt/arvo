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
  if $RUSTC +nightly-2026-05-28 --edition 2021 --crate-type lib --out-dir "$out" \
       $args "$src" > "$out/$lbl.log" 2>&1; then got=COMPILE; else got=REFUSE; fi
  if [ "$got" = "$req" ]; then v="as required"; else v="*** NOT AS REQUIRED ***"; fi
  printf "%-8s %-12s required=%-8s got=%-8s %s\n" "$lbl" "${*:-<none>}" "$req" "$got" "$v"
  if [ "$got" = REFUSE ]; then grep -m3 -E '^error' "$out/$lbl.log" | sed 's/^/           /'; fi
}
run S1S2S3 COMPILE
run S2m    REFUSE  mutate
run S4     REFUSE  arm_s4
