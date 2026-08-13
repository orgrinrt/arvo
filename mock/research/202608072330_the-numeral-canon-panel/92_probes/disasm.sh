#!/bin/sh
# Dump `bench_entry` from every arm dylib and count the instructions the
# findings reason about.
#
# Two questions this answers, both of which a timing number alone cannot.
#
# Whether the kernel inlined into the timed region. The variant crates are
# built without fat LTO, matching every sibling bench family in this
# directory, so a kernel reaching the timed region only through a cross-crate
# call would be measured with that call in it. `calls` below counts `bl`
# instructions inside `bench_entry`; the only ones that should appear are into
# core's panic paths.
#
# Which arms the vector saturating add reaches. `80` section 5.3's one
# qualitative claim is that `uqadd` appears only in the arms the law licensed.
# Each dylib holds every monomorphisation the bench declares, saturating and
# wrapping, so these are per-dylib totals rather than per-row counts, which is
# enough for a presence claim and not enough for a density one.
#
# Run from `mock/target/release`. Output committed beside this file as
# `disasm.txt`.

set -e

printf '%-24s %8s %8s %8s %8s %8s\n' arm instrs uqadd addv ldr calls
printf '%-24s %8s %8s %8s %8s %8s\n' ------------------------ -------- -------- -------- -------- --------

for a in seq iterfold nolaw lanes4_idx lanes16 lanes16_3 lanes16_constl lanes64 neon neon8 gate_true gate_false; do
    f="libbench_satfold_$a.dylib"
    [ -f "$f" ] || continue
    d=$(objdump -d --disassemble-symbols=_bench_entry "$f" 2>/dev/null)
    instrs=$(printf '%s\n' "$d" | grep -cE '^[[:space:]]*[0-9a-f]+:')
    uqadd=$(printf '%s\n' "$d" | grep -c 'uqadd' || true)
    addv=$(printf '%s\n' "$d" | grep -cE 'add\.16b|add\.8b' || true)
    ldr=$(printf '%s\n' "$d" | grep -cE '[[:space:]]ldr[[:space:]]' || true)
    calls=$(printf '%s\n' "$d" | grep -cE '[[:space:]]bl[[:space:]]' || true)
    printf '%-24s %8s %8s %8s %8s %8s\n' "$a" "$instrs" "$uqadd" "$addv" "$ldr" "$calls"
done

echo
echo "non-panic call targets inside bench_entry, per arm (empty is the wanted result):"
for a in seq iterfold nolaw lanes4_idx lanes16 lanes16_3 lanes16_constl lanes64 neon neon8 gate_true gate_false; do
    f="libbench_satfold_$a.dylib"
    [ -f "$f" ] || continue
    t=$(objdump -d --disassemble-symbols=_bench_entry "$f" 2>/dev/null \
        | grep -E '[[:space:]]bl[[:space:]]' \
        | grep -v panic || true)
    [ -n "$t" ] && printf '  %s:\n%s\n' "$a" "$t"
done

echo
echo "the const-gate erasure question, as instruction-stream hashes:"
echo "  addresses, encodings and objdump's own filename banner stripped, because"
echo "  the banner prints the dylib's name and would make every arm differ."
echo "  lanes16_3 is the size-matched control: the same kernel with no gate,"
echo "  declaring exactly the three sizes the gated arm declares, so the"
echo "  comparison is not about how many monomorphisations each dylib carries."
for a in lanes16 lanes16_3 gate_true gate_false seq; do
    f="libbench_satfold_$a.dylib"
    [ -f "$f" ] || continue
    h=$(objdump -d --disassemble-symbols=_bench_entry "$f" 2>/dev/null \
        | grep -vF 'file format' \
        | sed -E 's/^[[:space:]]*[0-9a-f]+:[[:space:]]*[0-9a-f ]+//' \
        | shasum | cut -d' ' -f1)
    printf '  %-16s %s\n' "$a" "$h"
done
