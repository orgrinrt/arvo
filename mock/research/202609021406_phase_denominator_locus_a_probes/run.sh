#!/usr/bin/env bash
# Builds each arm on its own and records the verdict. stderr is kept, because a
# build that fails for the wrong reason and one that fails for the right one look
# identical once the stream is thrown away.
set -uo pipefail
cd "$(dirname "$0")"

arms=(
  p1_the_pair_survives
  a1_control_sound_through_identity
  a2_broken_through_identity
  a3_broken_declared_and_never_used
  a4_broken_through_the_applied_map
  a5_broken_through_contains
  a6_broken_through_step_exponent
  a7_broken_forced_in_a_const_item
  a8_control_sound_forced_in_a_const_item
  a9_the_whole_step_phase
  a10_the_unwrap_doors
)

echo "toolchain: $(rustc --version)"
echo

for arm in "${arms[@]}"; do
  echo "=== $arm"
  for verb in check build; do
    out=$(cargo "$verb" --bin "$arm" 2>&1)
    if [ $? -eq 0 ]; then
      echo "  cargo $verb: ACCEPTED"
    else
      echo "  cargo $verb: REFUSED"
      echo "$out" | grep -E '^error' | head -3 | sed 's/^/    /'
      echo "$out" | grep -E 'denominator of zero|names no position|evaluation of|the evaluated program panicked' | head -2 | sed 's/^/    /'
    fi
  done
  if cargo run -q --bin "$arm" 2>/dev/null; then :; else echo "  run: did not run"; fi
  echo
done
