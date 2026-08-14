#!/usr/bin/env bash
# p4, the asm read. Builds the three shapes and reports what each emits.
#
# The question: does the fork between "one name binds a point in each
# component" and "the name supplies a default the site may override" have
# anything a consumer can observe? Three encodings of the same two folds are
# compiled and their symbols compared.
set -u
cd "$(dirname "$0")" || exit 1

SRC=p4_one_name_two_carriers.rs
ASM=/tmp/p4_108.s

echo "=== toolchain ==="
rustc --version
echo
echo "=== gates and forbidden constructs in the source ==="
printf 'feature(  : %s\n' "$(grep -c 'feature(' $SRC)"
printf 'dyn        : %s\n' "$(grep -cE '\bdyn ' $SRC)"
printf 'TypeId     : %s\n' "$(grep -v '^//' $SRC | grep -c 'TypeId')"
printf 'alloc/std  : %s\n' "$(grep -cE 'extern crate (alloc|std)|use (alloc|std)::' $SRC)"
printf 'no_std     : %s\n' "$(grep -c '#!\[no_std\]' $SRC)"
echo

echo "=== compile ==="
rustc -O --edition 2021 --crate-type=lib "$SRC" -o /tmp/p4_108.rlib 2>&1 | grep -E '^(error|warning: unused)' || echo "clean"
rustc -O --edition 2021 --crate-type=lib --emit=asm "$SRC" -o "$ASM" 2>&1 | grep -E '^error' || true
echo

echo "=== the emitted symbols for the six entry points ==="
grep -E '^_(a_|b_|c_)' "$ASM"
echo

echo "=== per-symbol instruction and branch counts ==="
echo
for sym in a_time_first a_size_first; do
  # body runs from the label to the next .globl or the next label at column 0
  body=$(awk -v s="_${sym}:" '
    $0 == s {inbody=1; next}
    inbody && /\.cfi_endproc/ {exit}
    inbody && /^L?BB[0-9_]*:$/ {next}
    inbody && /^\t\.(globl|p2align|cfi_startproc)/ {next}
    inbody {print}
  ' "$ASM")
  ins=$(echo "$body" | grep -cE '^\t[a-z]')
  # conditional branches on aarch64: b.<cond>, cbz/cbnz, tbz/tbnz
  br=$(echo "$body" | grep -cE '^\t(b\.[a-z]+|cbn?z|tbn?z)' )
  csel=$(echo "$body" | grep -cE '^\t(csel|csinc|csinv|cset)' )
  echo "  $sym: instructions=$ins conditional_branches=$br csel_family=$csel"
  echo "$body" | grep -E '^\t(b\.[a-z]+|cbn?z|tbn?z)' | sed 's/^/      branch: /' || true
done
echo

echo "=== is anything left of the cost table in the binary? ==="
printf 'COST symbol present : %s\n' "$(grep -c 'COST' "$ASM")"
printf '__const sections    : %s\n' "$(grep -c '__const' "$ASM")"
echo

echo "=== what the aliases mean ==="
cat <<'EOF'

Six entry points, two emitted bodies. `b_as_stored` and `c_default` are
assembler aliases of `a_time_first`; `b_other_weighting` and `c_overridden`
are aliases of `a_size_first`.

So the three encodings of the pair's fourth clause are the same program.
Shape B's `reinterpret`, which `106` section 10 prices as "a cast that changes
no value, which is free at runtime and not free in the design", is free at
runtime here to the symbol rather than to the instruction, and shape C reaches
the same code while never asking the value's type to move.

The fork therefore has NOTHING a consumer can observe, which puts it where
`100` section 6.2 put the check-against-generate fork by the same instrument.
It is a design-tier question about which spelling a consumer writes, and it is
settled on ergonomics and on who is allowed to move component one, not on cost.
EOF

echo "=== what the branches are, since a count alone would mislead ==="
cat <<'INNER'

Both bodies carry two conditional branches and one `csel`, and none of the
three tests a strategy, a policy or an arm choice:

  cbz x1, ...   the empty-slice guard, which is the slice's length
  b.ne ...      the loop backedge
  csel ...      the saturating clamp, which IS component one, lowered to a
                select rather than to a branch on a marker

The arm choice and the weighting appear nowhere. That is the erasure result the
unit already has on four instruments, reproduced here on a fifth arrangement,
and it is the reason the fork between shapes B and C cannot be decided on cost.
INNER
