#!/bin/bash
cd "$(dirname "$0")"
echo "width_max,impls,run1_s,run2_s" > timings.csv
for n in 128 256 512 1024 2048 4096 8192 16384; do
  f=probe_8_width_table_$n.rs
  [ -f "$f" ] || continue
  a=$( { /usr/bin/time -p rustc --edition 2024 --crate-type=lib --out-dir out $f >/dev/null ; } 2>&1 | awk '/^real/{print $2}' )
  b=$( { /usr/bin/time -p rustc --edition 2024 --crate-type=lib --out-dir out $f >/dev/null ; } 2>&1 | awk '/^real/{print $2}' )
  echo "$n,$((2*n)),$a,$b" >> timings.csv
done
echo DONE >> timings.csv
