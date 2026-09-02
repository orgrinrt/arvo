#!/bin/sh
# p5: which of the two interior projections in a chained reduction blocks
# vectorisation, at both codegen profiles.
#
# METHOD, and why each choice was forced.
#
# * `--crate-type cdylib`, which is what the bench harness builds every variant
#   as. A staticlib was tried first: every object file in it starts at address
#   zero, so the identical-code-folding check compared addresses across objects
#   and reported a hundred unrelated symbols as folded together.
#
# * Disassembled from the LINKED image, not `--emit asm`. Under `-Clto=fat`,
#   `--emit asm` reports the PRE-LTO module, in which nothing is vectorised,
#   including the positive control. An earlier version used it and concluded
#   the documented profile suppresses vectorisation entirely, which is a
#   statement about the instrument rather than about the profile.
#
# * Vector registers matched as bare `v0` / `q4` operands. ARM's syntax puts the
#   element form on the register (`v0.8h`) and Apple's puts it on the mnemonic
#   (`add.8h v0, v4, v0`), so a pattern requiring `v0.` matches nothing here and
#   reports a fully vectorised function as scalar. That is what the first
#   version did.
#
# CONTROLS. `control_plain_sum` and `control_indexed_sum` carry no projection
# and must vectorise; if they do not, the setup is what suppresses
# vectorisation and nothing below may be read as a fact about the arms. All
# three instrument defects above were found by adding this control, and none of
# them was visible without it.
set -e
SRC=p5_which_mask_blocks_the_vectoriser.rs
SYMS="control_plain_sum control_indexed_sum both_deferred value_eager acc_eager both_eager"

for spec in "default:-Copt-level=3 -Clto=off -Ccodegen-units=16" \
            "documented:-Copt-level=3 -Clto=fat -Ccodegen-units=1"; do
  name=${spec%%:*}; flags=${spec#*:}
  rustc --crate-type cdylib $flags -o "libp5_$name.dylib" "$SRC" 2>/dev/null
  otool -tV "libp5_$name.dylib" > "p5_${name}_linked.s"
  echo "profile=$name  ($flags)  [cdylib, post-LTO disassembly]"
  for f in $SYMS; do
    addr=$(nm -g "libp5_$name.dylib" 2>/dev/null | awk -v s="_$f" '$3==s {print $1}' | head -1)
    fold=$(nm -g "libp5_$name.dylib" 2>/dev/null | awk -v a="$addr" -v s="_$f" '$1==a && $3!=s && $3 ~ /^_[a-z]/ {printf "%s ", $3}')
    body=$(awk -v F="_$f" '$0 ~ "^" F ":" {inf=1; next} inf && /^_[a-zA-Z]/ {inf=0} inf {print}' "p5_${name}_linked.s")
    lines=$(printf '%s\n' "$body" | grep -c . || true)
    vec=$(printf '%s\n' "$body" | grep -cE '[ \t,][vq][0-9]+([ \t,]|$)' || true)
    if [ "$vec" -gt 0 ]; then v=YES; else v=no; fi
    printf "  %-20s addr=%s lines=%-4s vectorised=%-4s vec_operands=%-5s%s\n" \
           "$f" "${addr:-ABSENT}" "$lines" "$v" "$vec" "${fold:+folded_with: $fold}"
  done
  ctrl=$(awk -v F="_control_plain_sum" '$0 ~ "^" F ":" {inf=1; next} inf && /^_[a-zA-Z]/ {inf=0} inf {print}' "p5_${name}_linked.s" | grep -cE '[ \t,][vq][0-9]+([ \t,]|$)' || true)
  if [ "$ctrl" -eq 0 ]; then
    echo "  *** CONTROL FAILED: the projection-free reduction did not vectorise here,"
    echo "  *** so nothing above is a fact about the arms. Do not read this run."
  else
    echo "  control fired: the projection-free reduction vectorises here, so a"
    echo "  non-vectorising arm above is a fact about that arm."
  fi
  echo
done
