#!/bin/sh
# Re-runs every probe in 13_probes and prints the expected outcome beside the
# actual one. Nine must compile clean, twelve must refuse.
set -u
RUSTC="rustc +nightly-2026-05-28 --edition 2021 --crate-type lib"
fail=0
for f in p06_nat_algebra p07_matrix p10_full p13_surface_bounded \
         p14_multiply_past_the_table p17_alias_only_no_error p21_named_widths \
         p25_declared_output p29_extension_without_towers; do
  $RUSTC "$f.rs" >/dev/null 2>&1
  rc=$?; [ $rc -eq 0 ] || fail=$((fail+1))
  echo "expect-clean  $f  exit=$rc"
done
for f in p01_the_wall p02_mgca_assoc_const_path p03_mgca_type_const \
         p04_mgca_type_const_block p05_const_default_from_param \
         p08_negative_control p16_unbridged_width p19_lazy_type_alias \
         p20_width_mismatch p23_named_ceiling p24_reverse_wall \
         p26_declared_too_narrow; do
  $RUSTC "$f.rs" >/dev/null 2>&1
  rc=$?; [ $rc -ne 0 ] || fail=$((fail+1))
  echo "expect-refuse $f  exit=$rc"
done
rustc +nightly-2026-05-28 --edition 2021 -O p11_buf_sizes.rs -o /tmp/p11 >/dev/null 2>&1 && /tmp/p11
rustc +nightly-2026-05-28 --edition 2021 -O p15_sizes.rs -o /tmp/p15 >/dev/null 2>&1 && /tmp/p15
echo "unexpected outcomes: $fail"
