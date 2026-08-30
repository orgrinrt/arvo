#!/bin/sh
# 164: reruns every instrument the two signatures rest on and diffs against the
# committed outputs. The case that must fail: a deliberately corrupted diff target
# (the CONTROL at the bottom compares 162's p1 output against 163's p1 output,
# which must differ; if it reports REPRODUCES the differ is broken).
cd "$(dirname "$0")/.."
S="${TMPDIR:-/tmp}/164repro.$$"; mkdir -p "$S"
r() { # name, generated, committed
  if diff -q "$2" "$3" >/dev/null 2>&1; then echo "$1 : REPRODUCES"; else echo "$1 : DIFFERS"; fi
}
python3 162_probes/p1_how_far_does_the_premise_reach.py > "$S/a" 2>&1
r "162 p1 (premise reach)" "$S/a" 162_probes/p1_how_far_does_the_premise_reach.out
python3 163_probes/p1_clause9_satisfiability.py > "$S/b" 2>&1
r "163 p1 (clause 9 satisfiability)" "$S/b" 163_probes/p1_output.txt
python3 163_probes/p3_audit_the_accounting.py > "$S/c" 2>&1
r "163 p3 (accounting audit)" "$S/c" 163_probes/p3_output.txt
sh 162_probes/p2_cfg_in_const_fn/run.sh > "$S/d" 2>&1
r "162 p2 (cfg in const fn, both builds)" "$S/d" 162_probes/p2_cfg_in_const_fn/run.out
rustc --edition 2021 -O 163_probes/p2_offset_is_not_the_discriminator/offset.rs -o "$S/off" 2>/dev/null && "$S/off" > "$S/e"
r "163 p2 run (offset)" "$S/e" 163_probes/p2_offset_is_not_the_discriminator/offset_run.out
( cd 163_probes/p2_offset_is_not_the_discriminator && rustc --edition 2021 -O --cfg oob offset.rs -o "$S/offc" 2>&1 | head -4 > "$S/f" )
head -4 163_probes/p2_offset_is_not_the_discriminator/offset_control.err > "$S/g"
r "163 p2 control (must show line-shift)" "$S/f" "$S/g"
grep -n "I15" 109_bellard_the_primitive_derived_cold.md | cut -d: -f1 | tr '\n' ' ' > "$S/h"; echo >> "$S/h"
printf '310 320 452 454 656 \n' > "$S/i"
r "162 L15 grep (109 I15 occurrences)" "$S/h" "$S/i"
echo "CONTROL (162 p1 vs 163 p1, must DIFFER):"
r "  control" "$S/a" "$S/b"
rm -rf "$S"
