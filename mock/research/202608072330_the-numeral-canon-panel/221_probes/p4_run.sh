#!/bin/sh
# Runner for p2. Resolves the tree from its own location, never from a checkout
# path: `a_probe_reads_the_tree_it_sits_in` is the arm that made that a rule.
# stderr is not discarded and the compile is checked before the run, because a
# probe that did not compile prints nothing and a pipeline reads that as zero.
set -u
here=$(cd "$(dirname "$0")" && pwd)
src="$here/p4_what_the_const_width_surface_can_carry.rs"
out="$here/p4_out"
rm -rf "$out"; mkdir -p "$out"
RUSTC="${RUSTC:-rustc}"
echo "### $($RUSTC +nightly-2026-05-28 --version 2>&1 || $RUSTC --version 2>&1)"
echo "### src: $(basename "$src")"
echo
if ! $RUSTC +nightly-2026-05-28 --edition 2021 -O -o "$out/p4" "$src" 2>"$out/build.log"; then
    echo "*** p4 DID NOT COMPILE, no number below means anything ***"
    cat "$out/build.log"
    exit 1
fi
echo "### compiled clean"
echo
"$out/p4"
rc=$?
echo
echo "### exit $rc"
exit $rc
