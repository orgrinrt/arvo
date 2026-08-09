#!/usr/bin/env bash
# Regenerates output.txt. Run from this directory.
#
# P13 needs the notko proc-macro crate built first:
#   (cd "$WS/notko" && cargo +nightly-2026-05-28 build -p notko-macros)
# where WS is the workspace root holding notko/ and arvo/ side by side.

set -u
TC="+nightly-2026-05-28"
WS="${WS:-$HOME/Dev/clause-dev}"
mkdir -p out

rustc "$TC" --version

for f in p1_fn_type_param_default \
         p2_trait_type_param_default \
         p3_assoc_type_cannot_be_overridden \
         p4_default_taken_without_context \
         p5_struct_default_is_syntactic \
         p6_alias_identity_across_scopes \
         p7_scoped_supply_and_precedence \
         p8_what_a_bound_determines \
         p8b_bound_determines_no_width \
         p9_default_type_parameter_fallback \
         p10_output_by_projection \
         p11_tier_agreement \
         p12_body_scope_does_not_reach_the_signature; do
  echo "=== $f ==="
  rustc "$TC" --edition 2021 --crate-type lib --out-dir out "$f.rs" 2>&1
  echo "--- rc=$? ---"
done

echo "=== p13_notko_attribute ==="
DYLIB=$(ls "$WS"/notko/target/debug/libnotko_macros.dylib 2>/dev/null \
     || ls "$WS"/notko/target/debug/libnotko_macros.so 2>/dev/null)
if [ -n "${DYLIB:-}" ]; then
  rustc "$TC" --edition 2021 --crate-type lib \
    --extern notko_macros="$DYLIB" --out-dir out \
    p13_notko_attribute/lib.rs 2>&1
  echo "--- rc=$? ---"
else
  echo "SKIPPED: notko-macros not built"
fi
