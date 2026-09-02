#!/usr/bin/env bash
# Recompiles every source here and prints one line each. Output committed as
# out/verify.txt so a later reader compares against a record rather than rerunning
# to find out whether they agree.
#
# EXPECT is the intended outcome: PASS means it must compile, FAIL means the
# refusal IS the result and a clean compile would be the surprise.
set -uo pipefail
cd "$(dirname "$0")"
RS="rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib"
mkdir -p out

run() { # $1 expect  $2 file  $3.. extra flags
  local expect="$1" file="$2"
  shift 2
  local stem="${file%%_*}"
  if $RS "$@" --emit=metadata -o "out/${stem}.meta" "$file" > /dev/null 2> "out/${stem}.log"; then
    got=PASS
  else
    got=FAIL
  fi
  if [ "$got" = "$expect" ]; then mark="ok "; else mark="BAD"; fi
  printf '%s  %-6s %-6s %s\n' "$mark" "$expect" "$got" "$file"
}

echo "toolchain: $(rustc +nightly-2026-05-28 --version)"
echo "target:    $(rustc +nightly-2026-05-28 -vV | sed -n 's/^host: //p')"
echo
printf '%s  %-6s %-6s %s\n' "   " EXPECT GOT FILE
printf '%s  %-6s %-6s %s\n' "---" ------ ------ ----

run PASS p01_nat_canonicity.rs -O
run PASS p02_const_door_alias.rs -O
run PASS p03_hybrid_door_closed_algebra.rs -O
run PASS p04_five_spellings.rs -O
run FAIL p05_diag_mismatch.rs
run FAIL p06_default_param_elision.rs
run FAIL p07a_pin_const_to_nat_nogate.rs
run FAIL p07b_pin_const_to_nat_min_gca.rs
run FAIL p07c_pin_const_to_nat_type_const.rs
run FAIL p07d_pin_const_block.rs
run PASS p08_does_p06_shape_keep_the_ceiling.rs -O
run PASS p09_decimal_ladder.rs -O
run PASS p10_decimal_container.rs -O
run FAIL p11_diag_battery.rs
run FAIL p12_first_day_errors.rs
run FAIL p13_where_the_door_error_lands.rs
run FAIL p14_lazy_type_alias.rs
run PASS p15_markers_do_not_partition.rs -O
run FAIL p14b_lazy_over_full_ladder.rs
