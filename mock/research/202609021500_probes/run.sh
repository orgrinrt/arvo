#!/usr/bin/env bash
# Every Q31 arm, in one command, from any directory.
#
#   ./run.sh
#
# It encodes no expectation anywhere. What each arm was supposed to do is in that
# arm's own header comment and in its file name; what is printed here is the exit
# code that actually came back, and the full compiler output goes to `out/`.
#
# `cargo build` and never `cargo check`: `Slots::ADMITTED` is a const, a const is
# evaluated at codegen, and `check` skips it. `arvo-format/src/slots.rs` says so
# in its own doc comment, and without this line the first five arms would report
# five silent passes.
set -uo pipefail
cd "$(dirname "$0")"
mkdir -p out
: > out/SUMMARY.txt
say() { printf '%s\n' "$*" | tee -a out/SUMMARY.txt; }

say "toolchain: $(rustc --version)"
say ""
say "--- the admission arms, built one at a time so a refusal does not hide the rest"
for f in arms/*.rs; do
  name=$(basename "$f" .rs)
  log="out/${name}.txt"
  { echo "=== $name"; echo "--- cargo build --bin $name"; } > "$log"
  cargo build --bin "$name" >> "$log" 2>&1
  code=$?
  echo "--- exit: $code" >> "$log"
  say "$(printf '%-58s exit=%s' "$name" "$code")"
done

say ""
say "--- the carrier mutation: two copies of the shipped slots.rs differing in one type"
for what in lib c1_mutant_shipped_admits_63 c2_mutant_mutated_refuses_63; do
  if [ "$what" = lib ]; then arg=(--lib); log=out/carrier_lib.txt; else arg=(--bin "$what"); log="out/${what}.txt"; fi
  { echo "=== carrier $what"; } > "$log"
  ( cd carrier && cargo build "${arg[@]}" ) >> "$log" 2>&1
  say "$(printf '%-58s exit=%s' "carrier/$what" "$?")"
done

say ""
say "--- target invariance, and its mutant, per installed target"
TARGETS="aarch64-apple-darwin x86_64-unknown-linux-gnu i686-unknown-linux-gnu thumbv6m-none-eabi"
# msp430 is 16-bit and the pin ships no prebuilt artifacts for it, so core is built
# from source. It needs the `rust-src` component, which the pin already lists.
BUILD_STD_TARGETS="msp430-none-elf"
for t in $TARGETS; do
  for d in invariance mutant; do
    log="out/cross_${d}_${t}.txt"
    { echo "=== $d @ $t"; } > "$log"
    ( cd "cross/$d" && cargo build --target "$t" ) >> "$log" 2>&1
    say "$(printf '%-24s %-28s exit=%s' "$d" "$t" "$?")"
  done
done

say ""
say "--- and the 16-bit target, with core built from source"
for t in $BUILD_STD_TARGETS; do
  for d in invariance mutant; do
    log="out/cross_${d}_${t}.txt"
    { echo "=== $d @ $t (-Z build-std=core)"; } > "$log"
    ( cd "cross/$d" && cargo build -Z build-std=core --target "$t" ) >> "$log" 2>&1
    say "$(printf '%-24s %-28s exit=%s' "$d" "$t" "$?")"
  done
done
