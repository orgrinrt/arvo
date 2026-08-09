#!/bin/bash
set -u
rm -f *.rlib
R() { rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib "$@"; }
R base.rs -o libbase.rlib && echo "base                          exit 0"
for f in pB_keyed_on_numeral pC_per_kind_markers pD_keyed_on_form pG_consumer pK_chosen pI_braces; do
  if R --extern base=libbase.rlib $f.rs -o out.rlib >/dev/null 2>&1; then echo "$f  EXPECT 0 -> exit 0"; else echo "$f  EXPECT 0 -> FAILED"; fi
done
for f in pA_current_defect pE1_overkey pE2_mispair pF2 pH_precision_collision pL_rowassert pM_supertrait; do
  c=$(R --extern base=libbase.rlib $f.rs -o out.rlib 2>&1 | grep -oE '^error\[E[0-9]+\]' | head -1)
  echo "$f  EXPECT refusal -> ${c:-NONE}"
done
echo "pJ_collision  EXPECT refusal -> $(R pJ_collision.rs -o out.rlib 2>&1 | grep -oE '^error\[E[0-9]+\]' | head -1)"
echo "pN_direction  EXPECT refusal -> $(R pN_direction.rs -o out.rlib 2>&1 | grep -oE '^error\[E[0-9]+\]' | head -1)"
R pN2.rs -o out.rlib >/dev/null 2>&1 && echo "pN2 Carrier:Lowering leak     EXPECT 0 -> exit 0"
