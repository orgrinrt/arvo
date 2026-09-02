#!/usr/bin/env bash
# a5. Which of Q9's routes the pinned nightly actually supports, with no feature gates.
#
# `221` puts this at item 4 of what it would attack next and says it did not run it. It is
# the compile question that decides Q9 between option 6 and option 7, and between option 5
# and option 6 through t3 against t4.
#
# Every arm states its expectation before the run. Two are controls: t0 must compile or the
# driver is measuring itself, and t8 must be refused or t7's acceptance means nothing.
#
# stderr is folded in rather than discarded, because a refusal is the result here.
#
# The first run of this script had `-o /dev/null`, and t0 caught it: rustc creates its
# temporary directory beside the output path, cannot create one inside /dev, and every arm
# that would have compiled reported "couldn't create a temp dir" instead. Six of nine arms
# were then measuring the driver. The output of that run is kept beside this file as
# a5_v1_the_positive_control_caught_the_driver.txt.

set -u
EDITION=2024
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT

echo "toolchain: $(rustc --version)"
echo "edition:   $EDITION, no feature gates anywhere"
echo

declare -a EXPECT=(
  "t0_positive_control:compiles:the driver works"
  "t1_arithmetic_on_a_generic_const:refused:retirement dl_width_arithmetic_as_a_const_generic"
  "t2_width_comparison_in_a_where_clause:refused:retirement dl_const_generic_width_comparison_in_a_where_clause"
  "t3_const_predicate_over_a_structural_nat:compiles:option 5 can cross back"
  "t4_const_predicate_over_a_const_generic:compiles:option 6 needs no crossing"
  "t5_const_generic_used_as_a_const_argument_downstream:compiles:passing a width on is not arithmetic"
  "t6_bridge_from_a_const_to_a_nat_by_table:compiles:the bridge works and costs one impl per width"
  "t7_type_level_comparison_without_const_ops:compiles:option 7 needs no const operation"
  "t8_negative_control_the_comparison_must_refuse_the_wrong_direction:refused:t7 is a comparison rather than a free bound"
)

fail=0
for row in "${EXPECT[@]}"; do
  name="${row%%:*}"; rest="${row#*:}"; want="${rest%%:*}"; why="${rest#*:}"
  out=$(rustc --edition "$EDITION" --crate-type bin -o "$OUT/$name" "$name.rs" 2>&1)
  rc=$?
  if [ $rc -eq 0 ]; then got=compiles; else got=refused; fi
  if [ "$got" = "$want" ]; then verdict="as predicted"; else verdict="AGAINST PREDICTION"; fail=1; fi
  printf '%-62s want %-8s got %-8s  %s\n' "$name" "$want" "$got" "$verdict"
  printf '    why it is here: %s\n' "$why"
  if [ "$got" = refused ]; then
    printf '%s\n' "$out" | grep -E '^error' | head -3 | sed 's/^/    /'
  fi
done
echo
if [ $fail -eq 0 ]; then echo "every arm landed where it was predicted to"; else echo "at least one arm went against its prediction, which is the result"; fi
