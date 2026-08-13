#!/usr/bin/env bash
# driver for probe p2 (file 89). Pins the toolchain explicitly: rustup resolves
# from the CWD, and a probe run outside the repo picks up a different one.
set -u
RC="rustup run nightly-2026-05-28 rustc"
SRC="$(dirname "$0")/p2_const_gate.rs"
OUT=$(mktemp -d)
echo "p2: the const gate for the saturating verdict"
$RC --version
$RC -vV | grep host
echo
run() {
  local label="$1"; shift
  local t0 t1
  t0=$(python3 -c 'import time;print(time.time())')
  local out rc
  out=$($RC --edition 2021 -O --crate-type=lib --out-dir "$OUT" "$@" "$SRC" 2>&1); rc=$?
  t1=$(python3 -c 'import time;print(time.time())')
  local el; el=$(python3 -c "print(f'{$t1-$t0:.2f}')")
  echo "=== $label ==="
  if [ $rc -eq 0 ]; then echo "ACCEPT (rustc exit 0) in ${el}s"; else echo "REFUSE (rustc exit $rc) in ${el}s"; fi
  echo "$out" | grep -E "^error" | head -3
  echo
}
run "default: law = E_64 (x^64 == x^65), true at width 64"
run "--cfg use_e63: law = E_63, false at width 64" --cfg use_e63
run "--cfg perturb: criterion samples one point short" --cfg perturb
run "--cfg nonfragment: law carries sat_sub, fragment check must catch it" --cfg nonfragment
run "--cfg unchecked_nonfragment: fragment check removed (negative control)" --cfg unchecked_nonfragment
cat <<'EOT'
Reading of the last one. It ACCEPTS, and accepting is the finding: with the
fragment check removed the criterion returns TRUE for a law that is FALSE at
width 64 with witness x = 101, and the file asserts both facts, so a clean
compile is the compiled demonstration that fragment membership is load-bearing
rather than decorative.

Seconds are an ad-hoc quick spike with no substance. Accept and refuse are the
results.
EOT
rm -rf "$OUT"
