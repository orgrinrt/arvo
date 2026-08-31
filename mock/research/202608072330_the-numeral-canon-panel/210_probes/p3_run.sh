#!/bin/sh
# Runner for p2. Compiles at -O because the binary arms sweep 8192 x 8192.
#
# NOT `-o /dev/null` and stderr is not discarded: `191` records the first as the
# defect that made every arm of its own p1 fail for the wrong reason, and the
# workspace rule records the second. If the probe does not compile, nothing it
# would have printed means anything, so the compile is checked first and the run
# is skipped on failure.
set -u
here=$(cd "$(dirname "$0")" && pwd)
src="$here/p3_clause9_witness_quantifier.rs"
out="$here/p3_out"
rm -rf "$out"
mkdir -p "$out"
RUSTC="${RUSTC:-rustc}"
echo "### $($RUSTC +nightly-2026-05-28 --version 2>&1 || $RUSTC --version 2>&1)"
echo "### src: $(basename "$src")"
echo

if ! $RUSTC +nightly-2026-05-28 --edition 2021 -O -o "$out/p3" "$src" 2>"$out/build.log"; then
    echo "*** p3 DID NOT COMPILE, no number below means anything ***"
    cat "$out/build.log"
    exit 1
fi
echo "### compiled clean"
echo

"$out/p3"
rc=$?
echo
echo "### exit $rc"
exit $rc
