#!/usr/bin/env bash
# Rebuild and rerun everything 17 cites, from source, on the pinned toolchain.
#
#   ./verify.sh
#
# Nothing here is a bench. No bench harness has run in this panel, so every magnitude in
# 17 is unpriced. What this script produces is compile results, counts, emitted assembly
# and correctness runs.
set -u
cd "$(dirname "$0")"
PIN=nightly-2026-05-28
mkdir -p bin asm json ctl/json alias/json

echo "== toolchain =="
rustc +$PIN --version
echo

echo "== feature gates in this probe directory (expected: none) =="
n=$(grep -rn '#!\[feature' ./*.rs ctl/*.rs alias/*.rs 2>/dev/null | wc -l | tr -d ' ')
echo "  $n"
echo

echo "== t1: which test procedure observes which packed-layout defect =="
rustc +$PIN --edition 2021 -O t1_defect_matrix.rs -o bin/t1 2>/dev/null
./bin/t1 > t1_defect_matrix.out
tail -8 t1_defect_matrix.out
echo

echo "== t2 + t3: erasure at the aggregate, two oracles =="
rustc +$PIN --edition 2021 -O --emit asm --crate-type lib \
  t2_aggregate_erasure.rs --out-dir asm 2>/dev/null
python3 t3_opcode_oracle.py asm/t2_aggregate_erasure.s > t3_opcode_oracle.out
grep -E 'ERASED|IDENTICAL|DIFFERS|instructions' t3_opcode_oracle.out | head -8
echo

echo "== the panel's instrument across optimisation levels =="
for o in 0 1 2 3 s z; do
  d=/tmp/17_q12_O$o; mkdir -p "$d"
  rustc +$PIN --edition 2024 -C opt-level=$o --emit asm --crate-type lib \
    ../15_probes/q12_erasure_asm.rs --out-dir "$d" 2>/dev/null
  f=$(ls "$d"/*.s 2>/dev/null | head -1)
  folded=$(grep -cE '^_q12_[a-z0-9_]+ = _' "$f" 2>/dev/null || echo 0)
  echo "  opt-level=$o  folded symbols: $folded"
done
echo

echo "== the panel's instrument across three toolchains =="
for tc in nightly-2026-03-28 nightly-2026-05-28 nightly-2026-06-18; do
  d=/tmp/17_q12_$tc; mkdir -p "$d"
  if rustc +$tc --edition 2024 -O --emit asm --crate-type lib \
       ../15_probes/q12_erasure_asm.rs --out-dir "$d" 2>/dev/null; then
    f=$(ls "$d"/*.s | head -1)
    echo "  $tc  folded symbols: $(grep -cE '^_q12_[a-z0-9_]+ = _' "$f")"
  else
    echo "  $tc  BUILD FAILED"
  fi
done
echo

echo "== t4: the two directions a validation claim is quantified over =="
python3 t4_validation_directions.py > t4_validation_directions.out
sed -n '/^defect/,/^over-refusal: refuses W = 33..35/p' t4_validation_directions.out
echo

echo "== t5: the C1 perimeter, three variants =="
rustdoc +$PIN --edition 2021 -Z unstable-options --output-format json \
  --out-dir json t5_perimeter.rs 2>/dev/null
rustdoc +$PIN --edition 2021 -Z unstable-options --output-format json \
  --out-dir ctl/json ctl/t5_perimeter_control.rs 2>/dev/null
rustdoc +$PIN --edition 2021 -Z unstable-options --output-format json \
  --out-dir alias/json alias/t5_perimeter_alias.rs 2>/dev/null
{
  echo "### variant: leaky (three seeded leaks)"
  python3 t5_perimeter.py json/t5_perimeter.json
  echo
  echo "### variant: control (no leaks)"
  python3 t5_perimeter.py ctl/json/t5_perimeter_control.json
  echo
  echo "### variant: alias (one leak behind a type alias)"
  python3 t5_perimeter.py alias/json/t5_perimeter_alias.json
} > t5_perimeter.out 2>&1
grep -E '^###|HOLES|no public position|via alias' t5_perimeter.out
echo

echo "== reproduced from 16, whose premises 17 rests on =="
rustc +$PIN --edition 2021 -O ../16_probes/p3_blind_suite.rs -o bin/p3_16 2>/dev/null
./bin/p3_16 | grep -E 'PASS|FAIL|green'
echo
echo "done."
