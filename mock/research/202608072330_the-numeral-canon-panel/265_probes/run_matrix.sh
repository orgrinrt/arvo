#!/bin/sh
# Builds one probe crate for every pointer width this host can reach, and writes
# one output file per target under the probe's `out/`, exit code first.
#
#   sh run_matrix.sh <probe-dir>
#
# 16-bit has no precompiled core, so it is built with `-Zbuild-std=core` from the
# pinned toolchain's `rust-src`. The pointer width of each target is recorded
# from `rustc --print cfg` so the file says what it was built for rather than
# asking the reader to know.
set -u
probe="$1"
cd "$probe" || exit 2
mkdir -p out
for target in aarch64-apple-darwin x86_64-unknown-linux-gnu i686-unknown-linux-gnu thumbv6m-none-eabi wasm32-unknown-unknown msp430-none-elf; do
  out="out/$target.txt"
  pw=$(rustc --print cfg --target "$target" 2>/dev/null | grep target_pointer_width)
  extra=""
  case "$target" in msp430-none-elf) extra="-Zbuild-std=core" ;; esac
  # a clean build every time, so a cached artifact never stands in for a result
  cargo clean --target "$target" -p "$(basename "$probe")" >/dev/null 2>&1 || true
  cargo build --target "$target" $extra > "$out.tmp" 2>&1
  code=$?
  {
    echo "exit=$code"
    echo "target=$target $pw"
    echo "toolchain=$(rustc -vV | sed -n 's/^release: //p') $(rustc -vV | sed -n 's/^commit-hash: //p' | cut -c1-9)"
    echo "---"
    cat "$out.tmp"
  } > "$out"
  rm -f "$out.tmp"
  echo "$target -> exit=$code $pw"
done
