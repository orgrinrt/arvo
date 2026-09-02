#!/bin/sh
# Probe 0: closes file 32's open item 4 (the vectorisation anomaly).
#
# Root cause: the codegen-inspection command in 32_probes/OUTCOMES.md:21-23
# carries `-C lto=fat` on an unlinked `--emit=asm --crate-type lib` build.
# Under LTO, rustc/LLVM runs the PRE-LINK optimisation pipeline on the module
# and defers the loop vectoriser (with the rest of the late loop pipeline) to
# the LTO backend at link time. `--emit=asm` on a lib never runs that step, so
# the vectoriser never runs at all, on ANY function in the crate, including a
# verbatim copy of a control that vectorises when compiled without the flag.
# The standalone control was therefore not built "under identical flags" in
# effect; the flag that differed was the one the methodology inherited from
# the symbol-visibility check-build discipline, where it is correct.
#
# Run from 34_probes/. Expected output:
#   shape A (no lto, --emit=asm):      control vectorises, identity path vectorises
#   shape B (lto=fat, --emit=asm):     nothing vectorises (pre-link module)
#   shape C (lto=fat, staticlib):      LTO backend runs; the identity-contract
#                                      path and the verbatim control fold to ONE
#                                      symbol address with a vectorised body
set -e
IM=../32_probes/identity_model/src/lib.rs
COMMON="--edition 2021 -C opt-level=3 -C codegen-units=1 -C panic=abort"

echo "== shape A: no LTO, --emit=asm =="
rustc $COMMON --crate-type lib --emit=asm "$IM" -o /tmp/p34_nolto.s 2>/dev/null
for sym in probe_vectorises_verbatim_control probe_elementwise_add_fixed_no_assert; do
  n=$(awk -v s="_${sym}:" '$0 ~ s{f=1} f{print} f&&/^\tret$/{exit}' /tmp/p34_nolto.s | grep -cE '\.2d' || true)
  echo "  $sym: NEON .2d lines = $n"
done

echo "== shape B: lto=fat, --emit=asm (file 32's inspection command) =="
rustc $COMMON -C lto=fat --crate-type lib --emit=asm "$IM" -o /tmp/p34_lto.s 2>/dev/null
for sym in probe_vectorises_verbatim_control probe_elementwise_add_fixed_no_assert; do
  n=$(awk -v s="_${sym}:" '$0 ~ s{f=1} f{print} f&&/^\tret$/{exit}' /tmp/p34_lto.s | grep -cE '\.2d' || true)
  echo "  $sym: NEON .2d lines = $n"
done

echo "== shape C: lto=fat, staticlib (the LTO backend actually runs) =="
rustc $COMMON -C lto=fat --crate-type staticlib "$IM" -o /tmp/p34_lto.a 2>/dev/null
rm -rf /tmp/p34_ar && mkdir /tmp/p34_ar && (cd /tmp/p34_ar && ar x /tmp/p34_lto.a)
OBJ=$(for o in /tmp/p34_ar/*.o; do nm "$o" 2>/dev/null | grep -q probe_vectorises && echo "$o" && break; done)
echo "  symbol addresses (identical address = LLVM folded identical bodies):"
nm "$OBJ" | grep -E "probe_vectorises_verbatim_control|probe_elementwise_add_fixed_no_assert" | sed 's/^/    /'
echo "  vector loads in the shared body:"
objdump -d --no-show-raw-insn "$OBJ" | awk '/probe_vectorises_verbatim_control/{f=1;next} f&&/^[0-9a-f]+ </{exit} f{print}' | grep -cE 'ldp\tq|add\.2d' | sed 's/^/    ldp q \/ add.2d lines: /'
