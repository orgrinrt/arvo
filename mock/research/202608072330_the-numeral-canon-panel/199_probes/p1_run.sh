#!/usr/bin/env nutshell
# Builds and runs `p1_arm_e_compiles_and_does_not_compute.rs`, then compiles arm D
# separately because it must not build.
#
# ARM B is the negative control and the reason the run counts: if arm E gives the
# right answer at length 32 there is no gap between "compiles" and "computes" and
# the probe has established nothing.
# ARM D is the second: if the capacity-derived shape accepts an insufficient
# accumulator, the derivation is decorative.
set -uo pipefail
src="p1_arm_e_compiles_and_does_not_compute.rs"
[ -f "$src" ] || { echo "run me from 199_probes" >&2; exit 2; }
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
echo "### rustc: $(rustc --version)"
echo
echo "######## BASE: arms A, B, C   (must compile, then must print as required)"
if rustc --edition 2021 -o "$tmp/a" "$src" 2>&1; then
  "$tmp/a"
else
  echo "    *** BASE DID NOT COMPILE, nothing below counts ***"
fi
echo
echo "######## ARM D: capacity 64, 4-bit elements, 8-bit accumulator   (must be REFUSED)"
if out=$(rustc --edition 2021 -o "$tmp/d" "$src" --cfg arm_d 2>&1); then
  echo "    COMPILED  *** NOT AS REQUIRED: the derivation is decorative ***"
else
  echo "    REFUSED   as required"
  printf '%s\n' "$out" | grep -E "^error|accumulator too narrow" | head -4 | sed 's/^/      /'
fi
