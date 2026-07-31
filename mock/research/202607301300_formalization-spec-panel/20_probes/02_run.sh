#!/bin/bash
set -u
N="+nightly-2026-05-28"
RS=02_what_survives_monomorphisation.rs
NM=/opt/homebrew/opt/llvm/bin/llvm-nm   # Apple's nm cannot read LLVM 22 bitcode

build () { # $1 = extra flags, $2 = out
  rustc $N -O -Cpanic=abort $1 --emit=obj --crate-type=lib $RS -o "$2" 2>/dev/null
}

echo "=== the symbols four compositions leave in the object file, default mangling ==="
build "" /tmp/p2def.o
$NM /tmp/p2def.o | grep -E 'sum4' | sed 's/^[0-9a-f]* T /  /'

echo
echo "=== same, with -Csymbol-mangling-version=v0 stated explicitly ==="
build "-Csymbol-mangling-version=v0" /tmp/p2v0.o
$NM /tmp/p2v0.o | grep -E 'sum4' | sed 's/^[0-9a-f]* T /  /'
echo "  (identical: v0 is the DEFAULT on this toolchain. State it anyway, as one pins a toolchain.)"

echo
echo "=== decoding: Kt<hex>_ is a u16 const, <len><name> is a marker type ==="
echo "  Kt17_ = 0x17 = 23 = I     Kt29_ = 0x29 = 41 = F     6Strict / 9Bitpacked = the markers"
echo "  Run 03_the_build_layer_reader.py over this output to see it decoded."

echo
echo "=== and what inlining does to the same channel ==="
sed 's/#\[inline(never)\]/#[inline]/' $RS > /tmp/p2_inline.rs
rustc $N -O -Cpanic=abort --emit=obj --crate-type=lib /tmp/p2_inline.rs -o /tmp/p2_inline.o 2>/dev/null
printf '  sum4 symbols with #[inline(never)]: %s\n' "$($NM /tmp/p2def.o    | grep -c sum4)"
printf '  sum4 symbols with #[inline]:        %s\n' "$($NM /tmp/p2_inline.o | grep -c sum4)"
echo "  the intent is legible exactly where the operation survives as a function."
