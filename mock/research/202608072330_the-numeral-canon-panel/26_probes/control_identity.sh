#!/usr/bin/env bash
# Does the noise-floor control compile byte-identical to the arm it controls?
#
# `bitpack-carrier-d16-control` calls the same `sum_d16` on the same region
# with the same arguments as `bitpack-carrier-d16`. If the two exported
# function bodies are the same machine code, then any measured gap between
# those two arms is the harness's own resolution on this workload and nothing
# else, which is what makes the control a control. This script checks that
# rather than assuming it.
#
# The comparison is on disassembled instruction text with four things
# normalised away, because each differs between the two dylibs for a reason
# that is not a code difference: the file path otool prints as its first line,
# absolute addresses (the two bodies sit at different addresses), the exported
# symbol name, and the literal-pool string holding the variant's own name,
# which the harness registration passes and which necessarily spells the
# variant. Everything else must match, instruction for instruction.
#
# Run before normalisation, the raw diff is exactly three lines: the path
# header and one `add x0, x0, #ADDR ; literal pool for: "..."` whose comment
# names the variant. Instruction counts were already equal at 50497 each.
#
# Usage:  ./control_identity.sh [path-to-mock-target-release]
# Exit 0 = identical. Exit 1 = they differ, and the diff is printed.

set -u

REL="${1:-$(cd "$(dirname "$0")/../../../.." && pwd)/mock/target/release}"

A="$REL/libbench_bitpack_carrier_d16.dylib"
B="$REL/libbench_bitpack_carrier_d16_control.dylib"

for f in "$A" "$B"; do
    if [ ! -f "$f" ]; then
        echo "missing: $f" >&2
        echo "build first: cargo build --release -p bench-bitpack-carrier-d16 \\" >&2
        echo "                              -p bench-bitpack-carrier-d16-control" >&2
        exit 2
    fi
done

# The exported entry point each variant crate's #[bench_variant] emits. Both
# dylibs export exactly one such symbol per monomorphised size; the disassembly
# below covers the whole text section, which for these two crates is that
# entry point plus the inlined transform and nothing else of substance.
strip_noise() {
    # drop the leading address column, the raw byte column where present, and
    # any absolute address operand, leaving mnemonics and register operands
    otool -tV "$1" 2>/dev/null \
        | tail -n +2 \
        | sed -E 's/^[0-9a-f]+[[:space:]]+//' \
        | sed -E 's/0x[0-9a-f]+/ADDR/g' \
        | sed -E 's/_bench_bitpack_carrier_d16(_control)?/SYM/g' \
        | sed -E 's/bitpack-carrier-d16(-control)?/NAME/g' \
        | grep -vE '^\(__TEXT|^$' \
        | sed -E 's/[[:space:]]+/ /g'
}

TA=$(mktemp); TB=$(mktemp)
trap 'rm -f "$TA" "$TB"' EXIT

strip_noise "$A" > "$TA"
strip_noise "$B" > "$TB"

echo "instructions (d16):         $(wc -l < "$TA")"
echo "instructions (d16-control): $(wc -l < "$TB")"

if diff -q "$TA" "$TB" > /dev/null; then
    echo "IDENTICAL: the control compiles to the same code as the arm it controls"
    exit 0
fi

echo "DIFFER: the control is not byte-identical to its arm, so the gap between"
echo "them is not purely measurement resolution. First 40 differing lines:"
diff "$TA" "$TB" | head -40
exit 1
