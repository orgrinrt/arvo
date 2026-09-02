#!/bin/sh
set -e
cd "$(dirname "$0")/../../../../mock/benches" 2>/dev/null || cd mock/benches
TOTAL=0
for c in bitpack-carrier-shared bitpack-contend-shared bitpack-footprint-shared bitpack-plan-shared bitpack-shared bitpack-wide-shared quantiser-fadd-shared quantiser-radix-shared satfold-shared warm-clamp-shared warm-container-shared wide-rung-shared; do
  OUT=$(cargo test --offline --release --manifest-path "variants/$c/Cargo.toml" 2>&1)
  N=$(echo "$OUT" | grep -oE '[0-9]+ passed' | head -1 | grep -oE '^[0-9]+')
  if [ -z "$N" ]; then echo "$c: MISSING OR ZERO"; else echo "$c: $N passed"; TOTAL=$((TOTAL+N)); fi
done
OUT=$(cargo test --offline --release --manifest-path "variants/bitpack-write-contend-shared/Cargo.toml" -- --test-threads=1 2>&1)
N=$(echo "$OUT" | grep -oE '[0-9]+ passed' | head -1 | grep -oE '^[0-9]+')
if [ -z "$N" ]; then echo "bitpack-write-contend-shared: MISSING OR ZERO"; else echo "bitpack-write-contend-shared (serial): $N passed"; TOTAL=$((TOTAL+N)); fi
echo "TOTAL: $TOTAL"
echo "MISSING OR ZERO" # control: nonexistent-crate style line, informational
