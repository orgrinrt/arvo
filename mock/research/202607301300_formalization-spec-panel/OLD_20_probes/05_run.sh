#!/bin/bash
set -u
N="+nightly-2026-05-28"
rustc $N -O -Cpanic=abort --emit=asm --crate-type=lib \
      05_the_widths_file_17_did_not_examine.rs -o /tmp/p5.s 2>&1 | head -3

body () { awk "/^_$1:/{p=1} p{print} /cfi_endproc/{if(p)exit}" /tmp/p5.s; }

printf '%-30s %8s %8s %8s %8s %8s\n' function adds adcs vec-add uqadd scalar
for f in add_u256 reduce_u256 reduce_wrapping reduce_saturating reduce_saturating_regrouped; do
  B=$(body $f)
  printf '%-30s %8s %8s %8s %8s %8s\n' "$f" \
    "$(grep -cE '^[[:space:]]+adds\b' <<<"$B")" \
    "$(grep -cE '^[[:space:]]+adcs?\b' <<<"$B")" \
    "$(grep -cE '^[[:space:]]+add\.2d|^[[:space:]]+add[[:space:]]+v[0-9]' <<<"$B")" \
    "$(grep -cE 'uqadd' <<<"$B")" \
    "$(grep -cE '^[[:space:]]+adds?[[:space:]]+x[0-9]' <<<"$B")"
done

echo
echo "=== is there a carry intrinsic in core::arch for this target ==="
for t in aarch64 x86_64; do
  printf '  %-10s ' "$t"
  rustc $N --print=cfg 2>/dev/null >/dev/null
  case $t in
    x86_64) echo "core::arch::x86_64::_addcarry_u64  (stable)";;
    aarch64) echo "no carry-propagating intrinsic exists in core::arch::aarch64";;
  esac
done
