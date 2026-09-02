#!/usr/bin/env bash
# Build each arm on its own and say which built.
#
# The two controls must build and the three arms must not. Any other outcome
# invalidates the run rather than producing a weaker result: if a control fails
# the setup is broken, and if an arm builds the claim being tested is false.
#
# stderr is folded in rather than discarded, because the diagnostic is the
# evidence and a silent failure would read as a pass here.
set -u
cd "$(dirname "$0")"

declare -a MUST_BUILD=(
  a0_control_the_shipped_shape_builds
  a4_control_bool_does_carry_the_signedness
)
declare -a MUST_FAIL=(
  a1_width_cannot_carry_a_negative_slot
  a2_width_cannot_carry_the_widest_slot_max
  a3_width_cannot_carry_a_fixed_point_exponent
)

verdict=0

for arm in "${MUST_BUILD[@]}"; do
  echo "=== $arm  (must build) ==="
  if out=$(cargo build --quiet --example "$arm" 2>&1); then
    echo "    built."
  else
    echo "    DID NOT BUILD. The setup is broken and nothing below is evidence."
    echo "$out" | sed 's/^/    /'
    verdict=1
  fi
  echo
done

for arm in "${MUST_FAIL[@]}"; do
  echo "=== $arm  (must not build) ==="
  if out=$(cargo build --quiet --example "$arm" 2>&1); then
    echo "    IT BUILT. The claim under test is false."
    verdict=1
  else
    echo "    refused, as follows:"
    echo "$out" | grep -E '^(error|  -->|  \||[0-9]+ \|)' | head -12 | sed 's/^/    /'
  fi
  echo
done

echo "=== verdict ==="
if [ "$verdict" -eq 0 ]; then
  echo "Both controls built and all three arms were refused."
  echo "\`Width\` carries neither a negative coordinate nor one above 2^32,"
  echo "and \`Bool\` does carry the one truth value among the ten."
else
  echo "RUN INVALID: an expectation was not met. See above."
fi
exit "$verdict"
