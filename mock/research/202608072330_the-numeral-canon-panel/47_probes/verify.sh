#!/usr/bin/env bash
# Rebuilds and reruns every probe in 47_probes from source on the pinned toolchain.
#
#   ./verify.sh
#
# Three files are EXPECTED TO FAIL TO COMPILE. Their committed .err is the result, and this
# script reports their refusal counts rather than treating them as breakage.

set -u
cd "$(dirname "$0")" || exit 1
TC=nightly-2026-05-28
mkdir -p bin

echo "toolchain: $(rustc +$TC --version)"
echo "feature gates in this directory: $(grep -c '^#!\[feature' ./*.rs | grep -v ':0$' | wc -l | tr -d ' ') files with any"
echo

run_ok() {
  echo "=== $1 (must compile and run)"
  rustc +$TC --edition 2021 -O "$1.rs" -o "bin/$2" || { echo "UNEXPECTED: failed to compile"; return; }
  "./bin/$2" > "$1.out" && echo "ok, output in $1.out"
}

run_refuse() {
  echo "=== $1 (must FAIL to compile)"
  if rustc +$TC --edition 2021 --crate-type lib "$1.rs" 2> "$1.err"; then
    echo "UNEXPECTED: it compiled"
  else
    echo "refused as expected; '^error' lines: $(grep -c '^error' "$1.err")"
    echo "(the count includes the 'aborting due to N previous errors' line; subtract one)"
  fi
}

run_ok p1_single_type_output p1
run_refuse p1b_negctl_distinctness
run_refuse p2_scalar_single_output_refused
run_ok p2b_kind_asymmetry_positive p2b
run_refuse p3_access_type_from_const_refused
run_ok p3b_access_type_two_routes_that_work p3b
run_ok p4_stating_injectivity_needs_one_subject p4
run_refuse p4b_negctl_joint_distinctness
run_ok p5_one_output_against_all_three_forcings p5
run_refuse p5b_negctl_forcings
run_ok p6_two_ladders_not_one p6

echo
echo "=== feature-gate check (must be zero)"
grep -c '^#!\[feature' ./*.rs
