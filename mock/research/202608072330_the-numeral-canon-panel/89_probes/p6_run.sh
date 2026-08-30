#!/usr/bin/env bash
# driver for probe p6 (file 89). Toolchain pinned explicitly.
set -u
RC="rustup run nightly-2026-05-28 rustc"
SRC="$(dirname "$0")/p6_signed_window_gate.rs"
OUT=$(mktemp -d)
echo "p6: the const gate for a signed saturating law over a declared window"
$RC --version; $RC -vV | grep host; echo
run() {
  local label="$1"; shift
  local out rc t0 t1
  t0=$(python3 -c 'import time;print(time.time())')
  out=$($RC --edition 2021 -O --crate-type=lib --out-dir "$OUT" "$@" "$SRC" 2>&1); rc=$?
  t1=$(python3 -c 'import time;print(time.time())')
  echo "=== $label ==="
  if [ $rc -eq 0 ]; then echo "ACCEPT (rustc exit 0) in $(python3 -c "print(f'{$t1-$t0:.2f}')")s";
  else echo "REFUSE (rustc exit $rc) in $(python3 -c "print(f'{$t1-$t0:.2f}')")s"; fi
  echo "$out" | grep -E "^error" | head -2
  echo
}
run "default: declared window [0, MAX], admissible, n = 8"
run "--cfg straddle: declared window [MIN, MAX]" --cfg straddle
run "--cfg unchecked_straddle: admissibility check removed (negative control)" --cfg unchecked_straddle
cat <<'EOT'
Reading of the last one. It ACCEPTS, and accepting is the finding: with the
window hypothesis removed, the box criterion LICENSES fold reassociation on a
straddling window, and the same file asserts at compile time that the law is
false at width 64 with witness (MAX, MAX, MIN). That is 82's straddling
divergence reproduced as a compiled consequence of dropping the hypothesis.

The rung-0 check additionally asserts, at compile time, that on a straddling
window at widths 2 to 4 brute force says FALSE while the criterion says TRUE,
so the control cannot silently stop controlling.

Seconds are an ad-hoc quick spike with no substance. Accept and refuse are the
results.
EOT
rm -rf "$OUT"
