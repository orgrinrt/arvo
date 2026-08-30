#!/usr/bin/env bash
# Did the four axes 182 section 5.1 named as blockers get declared by the axis pass?
#
# The brief for 188 asserts: "It found four axes nothing declared. Two of them,
# and two more, have since been declared (ambient_domain, radix,
# accumulator_width, toolchain)." This tests that sentence.
#
# CASE THAT MUST FAIL: the control below asks the same question of four ids that
# ARE declared. If the "declared" column reads no for those, the grep is broken
# and every no above it is a fact about the grep.
set -u
D=../../../registry/dimension.toml

echo "=== the four axes 182 section 5.1 says nothing declares ==="
printf '%-34s %s\n' AXIS DECLARED
for a in declared_operand_window representable_range_geometry encoding constant_embedding_convention; do
  if grep -qE "^id = \"$a\"\$" "$D"; then r=yes; else r=no; fi
  printf '%-34s %s\n' "$a" "$r"
done

echo
echo "=== CONTROL: four ids the axis pass did declare, which must all read yes ==="
printf '%-34s %s\n' AXIS DECLARED
for a in ambient_domain radix accumulator_width toolchain; do
  if grep -qE "^id = \"$a\"\$" "$D"; then r=yes; else r=no; fi
  printf '%-34s %s\n' "$a" "$r"
done

echo
echo "=== every declared axis, for the record ==="
grep -oE '^id = "[a-z_]+"' "$D" | sed 's/id = //' | tr -d '"' | sort | tr '\n' ' '; echo
echo "count: $(grep -c '^\[\[dimension\]\]' "$D")"
