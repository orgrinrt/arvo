#!/bin/sh
# Rebuild and rerun every probe in this directory, and check each against the
# outcome it is supposed to have. Plain sh rather than nutshell, to match the
# runner convention the other probe directories in this panel already use.
#
# Six files. Four must exit 0 and two must refuse to compile. A run where a
# must-refuse file compiles is a failed verification even if everything else is
# green, because those two are what make the other four mean anything.
set -u
cd "$(dirname "$0")" || exit 1
BIN=$(mktemp -d) || exit 1
trap 'rm -rf "$BIN"' EXIT
RUSTC="rustc --edition 2024"
fail=0

echo "toolchain: $(rustc --version)"
echo "expected:  $(cat toolchain.txt)"
echo

run() { # name, source, expected exit
  printf '%-46s' "$2"
  $RUSTC -O --out-dir "$BIN" "$2" >"$BIN/build.log" 2>&1
  if [ $? -ne 0 ]; then echo "BUILD FAILED"; cat "$BIN/build.log"; fail=1; return; fi
  "$BIN/$1" >"$BIN/run.log" 2>&1
  got=$?
  if [ "$got" -eq "$3" ]; then echo "exit $got  ok"; else echo "exit $got  EXPECTED $3"; fail=1; fi
}

refuse() { # source
  printf '%-46s' "$1"
  $RUSTC --crate-type=lib --out-dir "$BIN" "$1" >"$BIN/err.log" 2>&1
  if [ $? -ne 0 ]; then
    echo "refused  ok ($(grep -c '^error' "$BIN/err.log") errors)"
  else
    echo "COMPILED, and it must not"; fail=1
  fi
}

compiles() { # source
  printf '%-46s' "$1"
  $RUSTC --crate-type=lib --out-dir "$BIN" "$1" >"$BIN/ok.log" 2>&1
  if [ $? -eq 0 ]; then echo "compiled ok"; else echo "REFUSED, and it must not"; cat "$BIN/ok.log"; fail=1; fi
}

echo "-- the two ladders --"
run p1_two_ladders_not_one p1_two_ladders_not_one.rs 1
run p1b_access_is_not_a_function_of_the_carrier p1b_access_is_not_a_function_of_the_carrier.rs 0

echo
echo "-- the curry orders, and the two files that must refuse --"
compiles p2_curry_orders_agree.rs
refuse   p2b_negctl_one_cell_disagrees.rs
refuse   p2c_negctl_false_distinguishability.rs

echo
echo "-- the operation set --"
run p3_the_count_is_constant_in_the_operation_set p3_the_count_is_constant_in_the_operation_set.rs 0

echo
echo "-- the placement axes --"
run p4_the_placement_reads_only_the_total_width p4_the_placement_reads_only_the_total_width.rs 0

echo
echo "-- how far the chain runs --"
run p5_how_far_a_chain_runs_before_the_carrier_shows p5_how_far_a_chain_runs_before_the_carrier_shows.rs 0

echo
echo "-- the fused result --"
run p6_the_fused_result_is_reachable_by_composition p6_the_fused_result_is_reachable_by_composition.rs 0

echo
if [ "$fail" -eq 0 ]; then echo "VERIFIED"; else echo "VERIFICATION FAILED"; fi
exit $fail
