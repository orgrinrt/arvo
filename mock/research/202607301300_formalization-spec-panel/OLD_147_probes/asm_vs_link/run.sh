#!/bin/sh
# Does `--emit=asm` on a library agree with the linked binary, under LTO?
#
# One function, one source, three profiles. The left column is what a panel
# probe reads when it inspects emitted assembly. The right column is what the
# program actually runs.
#
# Pin: nightly-2026-05-28. Host: aarch64-apple-darwin.
set -e
cd "$(dirname "$0")"

count() { grep -cE '\bv[0-9]+\.[0-9]+[bhsd]\b|\.[0-9]+[bhsd][[:space:]]+v[0-9]+|\bq[0-9]+\b' || true; }

printf '%-14s %-18s %s\n' profile emit_asm linked_binary
for P in release release-thin release-fat; do
  rm -rf target/asmout && mkdir -p target/asmout
  cargo rustc -q --profile "$P" --lib -- --emit=asm -o target/asmout/lib.s 2>/dev/null
  S=$(ls -t target/asmout/lib-*.s | head -1)
  A=$(awk '/accumulate:$/,/cfi_endproc/' "$S" | count)
  cargo build -q --profile "$P"
  B=$(objdump -d "target/$P/driver" 2>/dev/null | awk '/accumulate>:/,/^$/' | count)
  printf '%-14s %-18s %s\n' "$P" "$A" "$B"
done
