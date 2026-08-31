#!/bin/sh
set -u
here=$(cd "$(dirname "$0")" && pwd)
out="$here/p3_out"; mkdir -p "$out"
RUSTC="${RUSTC:-rustc}"
echo "### $($RUSTC +nightly-2026-05-28 --version)"
run() {
  lbl=$1; req=$2; shift 2
  args=""; for c in "$@"; do args="$args --cfg $c"; done
  # shellcheck disable=SC2086
  if $RUSTC +nightly-2026-05-28 --edition 2021 --crate-type lib --out-dir "$(mktemp -d)" \
       $args "$here/p3b_const_to_type_bridge.rs" > "$out/$lbl.log" 2>&1; then got=COMPILE; else got=REFUSE; fi
  if [ "$got" = "$req" ]; then v="as required"; else v="*** NOT AS REQUIRED ***"; fi
  printf "%-6s %-14s required=%-8s got=%-8s %s\n" "$lbl" "${*:-<none>}" "$req" "$got" "$v"
  sed -n '1,3p' "$out/$lbl.log" | sed 's/^/         /'
}
run C2a REFUSE
run C2b REFUSE with_feature
