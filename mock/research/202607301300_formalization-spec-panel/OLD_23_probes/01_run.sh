#!/bin/bash
# PROBE 1 runner. Counts nameable compositions in three artifacts.
set -u
cd "$(dirname "$0")"
N="+nightly-2026-05-28"
RS=01_where_the_channel_is_complete.rs
NM=/opt/homebrew/opt/llvm/bin/llvm-nm
OUT=/tmp/23p1
mkdir -p $OUT

echo "24 monomorphisations exist in the source: 12 compositions x 2 operations."
echo "No inline attribute appears anywhere in the file."
echo

rustc $N -O -Cpanic=abort --emit=obj --crate-type=lib $RS -o $OUT/o.o 2>/dev/null
n_obj_add=$($NM $OUT/o.o | grep -c '3add')
n_obj_red=$($NM $OUT/o.o | grep -c '6reduce')
echo "=== 1. optimised object file, the artifact that ships ==="
printf '  add    monomorphisations nameable: %2s / 12\n' "$n_obj_add"
printf '  reduce monomorphisations nameable: %2s / 12\n' "$n_obj_red"

rustc $N -O -Cpanic=abort --emit=llvm-ir --crate-type=lib $RS -o $OUT/opt.ll 2>/dev/null
n_ir_add=$(grep -c '^define.*3add' $OUT/opt.ll)
n_ir_red=$(grep -c '^define.*6reduce' $OUT/opt.ll)
echo
echo "=== 2. optimised LLVM IR, same pipeline, more readable ==="
printf '  add    defines: %2s / 12\n' "$n_ir_add"
printf '  reduce defines: %2s / 12\n' "$n_ir_red"

rustc $N -O -Cpanic=abort -Cno-prepopulate-passes --emit=llvm-ir --crate-type=lib $RS -o $OUT/raw.ll 2>/dev/null
r_add=$(grep -c '^define.*3add' $OUT/raw.ll)
r_red=$(grep -c '^define.*6reduce' $OUT/raw.ll)
echo
echo "=== 3. IR as rustc emits it, before any LLVM pass runs (-Cno-prepopulate-passes) ==="
printf '  add    defines: %2s / 12\n' "$r_add"
printf '  reduce defines: %2s / 12\n' "$r_red"

echo
echo "=== 4. the same three, with lto=fat, since that is what a release build does ==="
rustc $N -O -Cpanic=abort -Clto=fat --emit=obj --crate-type=lib $RS -o $OUT/lto.o 2>/dev/null
printf '  add    symbols under fat LTO: %2s / 12\n' "$($NM $OUT/lto.o | grep -c '3add')"
printf '  reduce symbols under fat LTO: %2s / 12\n' "$($NM $OUT/lto.o | grep -c '6reduce')"

echo
echo "=== 5. sizes, because a verifier has to read whichever of these it picks ==="
for f in $OUT/o.o $OUT/opt.ll $OUT/raw.ll; do
  printf '  %-14s %8s bytes\n' "$(basename $f)" "$(wc -c < $f | tr -d ' ')"
done

echo
echo "=== 6. one raw define, to show the body is present and the name is intact ==="
grep -m1 '^define.*6reduce' $OUT/raw.ll | cut -c1-140
