#!/usr/bin/env nutshell
# Compiles `p1_capacity_against_id_width.rs` arm by arm and records, for each,
# whether rustc accepted it.
#
# DEFECT IN VERSION ONE, kept because the base arm is exactly what caught it.
# Version one passed `-o /dev/null` and rustc creates a temporary beside its
# output, so every arm died with "couldn't create a temp dir ... /dev/rmeta...".
# Three arms reported REFUSED and looked correct. The only thing that exposed it
# was the base arm failing to compile when it must. A probe whose arms all
# expect refusal cannot tell a refusal from a broken invocation, which is why
# the base arm is first and is not optional. Transcript: `p1_v1_devnull.out`.
#
# THE G ARMS ARE THE ONES THAT DECIDE THE RESULT. An inherent associated const
# is evaluated where it is used. So arms C-bad and F may be measuring the call
# site rather than the type, and the mechanism would then be a landmine rather
# than a refusal. G names the bad instantiation without touching the const,
# three ways: returning one, aliasing one, holding one in a field.
set -uo pipefail
src="p1_capacity_against_id_width.rs"
[ -f "$src" ] || { echo "run me from 191_probes" >&2; exit 2; }
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
echo "### rustc: $(rustc --version)"
echo

run() {
  name="$1"; expect="$2"; shift 2
  out=$(rustc --edition 2021 -o "$tmp/a.out" "$src" "$@" 2>&1)
  rc=$?
  if [ $rc -eq 0 ]; then got="COMPILED"; else got="REFUSED "; fi
  if [ "$got" = "$expect" ]; then v="as required"; else v="*** NOT AS REQUIRED ***"; fi
  echo "######## $name"
  echo "         required=$expect  got=$got   $v"
  [ $rc -eq 0 ] || printf '%s\n' "$out" | grep -E "^(error|warning)" | head -4 | sed 's/^/    /'
  echo
}

run "BASE   A, C, E: fitting capacities"          "COMPILED"
run "B      monomorphic, 300 into 8 bits"         "REFUSED " --cfg arm_b
run "C-bad  generic, called at 300 / 8"           "REFUSED " --cfg arm_c_bad
run "D      the where-clause spelling"            "REFUSED " --cfg arm_d
run "F      associated-const gate, called"        "REFUSED " --cfg arm_f
run "G1     bad type returned, const untouched"   "REFUSED " --cfg arm_g_construct
run "G2     bad type aliased, const untouched"    "REFUSED " --cfg arm_g_typedef
run "G3     bad type in a field, untouched"       "REFUSED " --cfg arm_g_field
