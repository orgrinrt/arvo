#!/usr/bin/env bash
# Builds p3 and its mutant and prints the whole result, so the .out file is
# reproducible from one command rather than from a transcript.
set -u
cd "$(dirname "$0")"

echo "toolchain: $(rustc --version)"
echo "target:    $(rustc -vV | sed -n 's/^host: //p')"
echo

echo "=== p3_three_encodings.rs: must compile ==="
if rustc --edition 2024 -O -C panic=abort --emit asm \
        -o p3_three_encodings.s p3_three_encodings.rs 2>p3_err.txt; then
    echo "compiles, zero feature gates"
else
    echo "FAILED"
    cat p3_err.txt
fi
echo
echo "feature gates in the source: $(grep -c '#!\[feature' p3_three_encodings.rs)"
echo "dyn / TypeId / generic_const_exprs in CODE (comments stripped): \
$(grep -v '^\s*//' p3_three_encodings.rs | grep -cE '\bdyn\b|TypeId|generic_const_exprs')"
echo

echo "=== the four entry bodies, verbatim from the emitted assembly ==="
sed -n '/^_e1_named:/,/subsections_via_symbols/p' p3_three_encodings.s
echo

echo "=== which arm each entry reaches ==="
for f in e1_named e2_weighted e3_direct e4_consumer; do
    line=$(grep -A2 "^_${f}:" p3_three_encodings.s | grep -o 'arm[0-9]' | head -1)
    alias=$(grep "^_${f} = " p3_three_encodings.s || true)
    if [ -n "$alias" ]; then
        echo "  $f -> $alias   (identical body, merged by the linker)"
    else
        echo "  $f -> $line"
    fi
done
echo

echo "=== p3_mutant_generator_bug.rs: must NOT compile ==="
if rustc --edition 2024 -O -C panic=abort --emit asm \
        -o /dev/null p3_mutant_generator_bug.rs 2>p3_mutant_err.txt; then
    echo "MUTANT COMPILED. The differential cannot fail and proves nothing."
else
    echo "refused, as required:"
    head -3 p3_mutant_err.txt
fi
rm -f p3_err.txt
