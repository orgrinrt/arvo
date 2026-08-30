#!/usr/bin/env bash
# Runs every probe in 24_probes and writes each output beside its source.
# Run from this directory. Requires the repository's pinned nightly, which resolves
# because this path is inside the arvo tree; a bare `rustc` outside it is stable.
set -u
cd "$(dirname "$0")"

echo "toolchain: $(rustc --version)"
echo

echo "=== s1: grid-and-reach agreement ==="
python3 s1_grid_reach_agreement.py | tee s1.out
echo

echo "=== s2: which numerals the width keying misses ==="
python3 s2_which_numerals_the_wf_keying_misses.py | tee s2.out
echo

echo "=== s3: one definition, three projections ==="
mkdir -p build
rustc --edition 2021 --crate-type lib --crate-name s3 \
      s3_one_definition_two_projections.rs --out-dir build || exit 1
echo "no_std library built clean, no feature gates"
rustc --edition 2021 s3_run.rs --extern s3=build/libs3.rlib -L build --out-dir build || exit 1
./build/s3_run | tee s3.out
echo

echo "=== s4: the two phases ==="
python3 s4_two_phases.py | tee s4.out
echo

echo "=== s5: the knee without an enumeration ==="
python3 s5_knee_without_enumeration.py | tee s5.out
echo

echo "=== s6: the whole vocabulary overlap ==="
python3 s6_vocabulary_overlap.py | tee s6.out
echo

echo "=== feature-gate audit of every probe source ==="
if grep -n 'feature(' ./*.rs; then
  echo "FAIL: a probe carries a feature gate"
  exit 1
else
  echo "no #![feature(...)] in any probe source"
fi
