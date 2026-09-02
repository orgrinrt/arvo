#!/usr/bin/env bash
# Rebuild and rerun everything 51 cites, from source, on the pinned toolchain.
#
#   ./verify.sh
#
# Nothing here is a bench. No bench harness has run in this panel, so every
# magnitude in 51 is unpriced. What this produces is emitted assembly, counts
# read off it, and two correctness runs.
#
# Roughly 400 MB of static archives are built and deleted inside step 3 rather
# than committed. Everything else in this directory is committed.
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

echo "== step 0: reproduce 17's t2, which 51 rests on =="
mkdir -p repro
rustc +$PIN --edition 2021 -O --emit asm --crate-type lib \
  ../17_probes/t2_aggregate_erasure.rs --out-dir repro 2>/dev/null
echo "  this run:      $(wc -c < repro/t2_aggregate_erasure.s) bytes"
echo "  17's committed: $(wc -c < ../17_probes/asm/t2_aggregate_erasure.s) bytes"
python3 oracle.py repro/t2_aggregate_erasure.s --pairs \
  t2_scalar_typed:t2_scalar_native,t2_typed_sum:t2_handwritten_sum,t2_typed_sum_cold_400:t2_typed_sum_aligned_access3 \
  --terse
echo

echo "== step 1: the rigged two-arm matrix, kept because the fold is the finding about it =="
echo "   (both arms are the same MIR, so they fold at every width and nothing could fail)"
python3 run_width_matrix.py --opt 3 --elems 1000 | tail -5
echo

echo "== step 2: three arms, where the arms are genuinely different code =="
python3 run_v2.py --opt 3 --elems 1000 | tail -23
echo

echo "== step 3: cross-check, linking and calling the emitted symbols =="
python3 gen_v3.py widths3 1000 > /dev/null
rm -rf link3 && mkdir -p link3 asm3
for f in widths3/w*.rs; do
  n=$(basename "$f" .rs)
  rustc +$PIN --edition 2024 -C opt-level=3 -C panic=abort --cfg link_check \
    --crate-type staticlib --crate-name "$n" "$f" --out-dir link3 2>/dev/null
  rustc +$PIN --edition 2024 -C opt-level=3 -C panic=abort --cfg link_check \
    --emit asm --crate-type lib "$f" --out-dir asm3 2>/dev/null
done
libtool -static -o link3/libwidthmatrix.a link3/libw[0-9]*.a 2>/dev/null
python3 gen_check3.py widths3 check3.rs > /dev/null
rustc +$PIN --edition 2021 -O check3.rs -L link3 -o /tmp/51_check3 2>/dev/null
/tmp/51_check3
rm -rf link3
echo

echo "== step 4: the five-arm matrix and the two attacks =="
python3 run_v3.py | tail -28
echo

echo "== step 5a: control that FAILED, width fixed and access swept =="
echo "   (extra bytes are dead under the mask, so the access count never really varied)"
python3 gen_access_control.py acc_ctl > /dev/null
mkdir -p asm_acc
for ab in 3 4 8; do
  rustc +$PIN --edition 2024 -C opt-level=3 -C panic=abort --cfg link_check \
    --emit asm --crate-type lib "acc_ctl/a$ab.rs" --out-dir asm_acc 2>/dev/null
  printf '  ACCESS=%s  ' "$ab"
  python3 oracle.py "asm_acc/a$ab.s" "a${ab}_loop" "a${ab}_flat" --terse
done
echo

echo "== step 5b: control that WORKS, gather shape fixed and width swept =="
python3 gen_fixed_gather.py fixgather 1000 > /dev/null
mkdir -p asm_fix
printf '  %-4s %-22s %-22s %-22s\n' W loop8 flat8 wide
for w in 13 17 18 19 23 31 47; do
  f=$(printf 'fixgather/w%02d.rs' "$w")
  rustc +$PIN --edition 2024 -C opt-level=3 -C panic=abort --cfg link_check \
    --emit asm --crate-type lib "$f" --out-dir asm_fix 2>/dev/null
  s=$(printf 'asm_fix/w%02d.s' "$w")
  row=""
  for a in loop8 flat8 wide; do
    r=$(python3 loopshape.py "$s" "w${w}_${a}" --width "$w" | tail -1 \
        | awk '{print $2"i/"$3"ld/"$5"acc"}')
    row="$row$(printf '%-22s ' "$r")"
  done
  printf '  %-4s %s\n' "$w" "$row"
done
echo

echo "== step 6: other optimisation levels, element counts and target-cpu =="
echo "   (one config per invocation: the whole sweep in one process exhausts memory)"
for cfg in "2 1000 -" "3 1000 -" "3 1000 native" "3 4096 -"; do
  # shellcheck disable=SC2086
  python3 -u robustness.py $cfg
done
echo "   -Os last, and degenerate: it unrolls nothing, so no arm can be distinguished"
python3 -u robustness.py s 1000 -
echo

echo "== step 7: open every citation in 51 and print what is there =="
python3 check_citations.py | tail -3
echo
echo "done."
