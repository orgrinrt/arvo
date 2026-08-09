#!/usr/bin/env bash
# Rebuild and rerun everything 51 cites, from source, on the pinned toolchain.
#
#   ./verify.sh
#
# Nothing here is a bench. No bench harness has run in this panel, so every
# magnitude in 51 is unpriced. What this produces is emitted assembly, counts
# read off it, and one correctness run that calls the emitted symbols.
set -u
cd "$(dirname "$0")"
PIN=nightly-2026-05-28

echo "== toolchain =="
rustc +$PIN --version
rustc +$PIN -vV | grep host
echo

echo "== feature gates anywhere in this probe directory (expected: 0) =="
grep -rn '#!\[feature' . 2>/dev/null | wc -l | tr -d ' '
echo

echo "== step 0: reproduce 17's t2, whose result 51 rests on =="
mkdir -p repro
rustc +$PIN --edition 2021 -O --emit asm --crate-type lib \
  ../17_probes/t2_aggregate_erasure.rs --out-dir repro 2>/dev/null
echo "  emitted $(wc -c < repro/t2_aggregate_erasure.s) bytes"
echo "  17_probes/asm/t2_aggregate_erasure.s is $(wc -c < ../17_probes/asm/t2_aggregate_erasure.s) bytes"
python3 oracle.py repro/t2_aggregate_erasure.s --pairs \
  t2_scalar_typed:t2_scalar_native,t2_typed_sum:t2_handwritten_sum,t2_typed_sum_cold_400:t2_typed_sum_aligned_access3 \
  --terse
echo

echo "== step 1: the rigged two-arm matrix, kept because the fold is the finding =="
echo "   (both arms are the same MIR, so they fold at every width and prove nothing)"
python3 run_width_matrix.py --opt 3 --elems 1000 | tail -6
echo

echo "== step 2: three arms, where the arms are genuinely different code =="
python3 run_v2.py --opt 3 --elems 1000 | tail -24
echo

echo "== step 3: cross-check, calling the emitted symbols =="
python3 gen_v3.py widths3 1000 > /dev/null
rm -rf link3 && mkdir -p link3 asm3
for f in widths3/w*.rs; do
  n=$(basename "$f" .rs)
  rustc +$PIN --edition 2024 -C opt-level=3 -C panic=abort --cfg link_check \
    --crate-type staticlib --crate-name "$n" "$f" --out-dir link3 2>/dev/null
  rustc +$PIN --edition 2024 -C opt-level=3 -C panic=abort --cfg link_check \
    --emit asm --crate-type lib "$f" --out-dir asm3 2>/dev/null
done
libtool -static -o link3/libwidthmatrix.a link3/w*.a 2>/dev/null
python3 gen_check3.py widths3 check3.rs > /dev/null
rustc +$PIN --edition 2021 -O check3.rs -L link3 -o /tmp/51_check3 2>/dev/null
/tmp/51_check3
rm -rf link3            # 400 MB of archives, rebuilt on demand, never committed
echo

echo "== step 4: the five-arm matrix and the two attacks =="
python3 run_v3.py | tail -30
echo

echo "== step 5: does it survive other opt levels, element counts and target-cpu =="
python3 robustness.py
echo
echo "done."
