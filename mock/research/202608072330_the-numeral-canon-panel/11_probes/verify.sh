#!/bin/sh
# Recompiles every probe in this directory and prints one line each.
# Run from 11_probes/. Expected output is committed as out/verify.txt.
set -u
cd "$(dirname "$0")"
R="rustc +nightly-2026-05-28 --edition 2024 -O --crate-type=lib --emit=metadata"
say() { printf "%-40s %s\n" "$1" "$2"; }
clang++ -std=c++20 -O2 -c a01_cpp_nttp_derivation.cpp -o out/a01.o 2>/dev/null \
  && say a01_cpp COMPILES || say a01_cpp FAILS
zig build-obj a02_zig_comptime_derivation.zig -O ReleaseFast -femit-bin=out/a02.o 2>/dev/null \
  && say a02_zig COMPILES || say a02_zig FAILS
for f in b01_table_caps_the_algebra b02_the_table_chases_its_tail \
         b03_the_ceiling_is_the_const_surface d01_bare_parameter_carrier \
         d02_postmono_check_fires e01_enumeration_free_bridge \
         e02_closing_the_overshoot e03_overshoot_under_min_gca \
         e04_overshoot_const_block f01_const_param_default_from_siblings; do
  if $R -o "out/$f.meta" "$f.rs" >"out/$f.log" 2>&1
  then say "$f" COMPILES
  else say "$f" "REFUSES: $(head -1 "out/$f.log" | cut -c1-72)"; fi
done
cd c_orphan || exit 1
rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib arvo_min.rs -o libarvo_min.rlib 2>/dev/null \
  && say arvo_min COMPILES
for f in consumer_ok consumer_bad consumer_partition consumer_partition_nored; do
  if rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib \
       --extern arvo_min=libarvo_min.rlib --emit=metadata -o "../out/$f.meta" "$f.rs" \
       >"../out/$f.log" 2>&1
  then say "$f" COMPILES
  else say "$f" "REFUSES: $(head -1 "../out/$f.log" | cut -c1-66)"; fi
done
