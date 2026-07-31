#!/bin/bash
set -u
RS=01_contract_is_not_contract.rs
N="+nightly-2026-05-28"

echo "=== A0. is the source form available in arvo's environment (no_std) ==="
rustc $N -O --cfg nostd --emit=metadata $RS -o /tmp/p1.meta 2>&1 \
  | grep -E "^error" | head -2 | sed 's/^/  /'
echo "  (arvo: all 16 crate roots carry #![no_std]; crates/arvo/src/lib.rs:14)"
echo "  the core route is core::f64::math::mul_add behind #![feature(core_float_math)],"
echo "  which is on no table in unstable-features.md, so it is unvetted."

echo
echo "=== A. which LLVM intrinsic does it emit (with std) ==="
rustc $N -O --emit=llvm-ir $RS -o /tmp/p1.ll 2>/dev/null
grep -oE "@llvm\.(fma|fmuladd)\.f64" /tmp/p1.ll | sort -u | sed 's/^/  /'
echo "  and the intrinsic that gives the LICENCE semantics:"
cat > /tmp/p1_fmuladd.rs <<'RS2'
#![crate_type="lib"]
#![no_std]
#![feature(core_intrinsics)]
#[no_mangle] pub fn f(a:f64,b:f64,c:f64)->f64 { unsafe { core::intrinsics::fmuladdf64(a,b,c) } }
RS2
rustc $N -O --emit=llvm-ir /tmp/p1_fmuladd.rs -o /tmp/p1_fmuladd.ll 2>/dev/null
grep -oE "@llvm\.(fma|fmuladd)\.f64" /tmp/p1_fmuladd.ll | sort -u \
  | sed 's/^/  core::intrinsics::fmuladdf64 -> /'
echo "  which needs #![feature(core_intrinsics)], FORBIDDEN by unstable-features.md."

echo
echo "=== B to D. what it costs, by target ==="
printf '  %-42s %-28s %s\n' "target / features" "via_mul_add" "via_source"
read_body () { # $1 = asm file, $2 = symbol
  awk "/^_?$2:/{p=1} p{print} /(cfi_endproc|\.size)/{if(p)exit}" "$1" \
    | grep -oE '^[[:space:]]+[a-z][a-z0-9._]*' | tr -d ' \t' \
    | grep -vE '^(cfi|p2align|section|globl|type|size|file|loc|text)' | paste -sd' ' -
}
rustc $N -O --emit=asm $RS -o /tmp/p1_arm.s 2>/dev/null
printf '  %-42s %-28s %s\n' "aarch64-apple-darwin (host, has FMA)" \
  "$(read_body /tmp/p1_arm.s via_mul_add)" "$(read_body /tmp/p1_arm.s via_source)"

# +stable because the x86_64 std is installed for that toolchain on this machine.
# llvm.fma's lowering is not toolchain dependent; what changes per row is FMA support.
rustc +stable -O --target=x86_64-unknown-linux-gnu --emit=asm $RS -o /tmp/p1_x86.s 2>/dev/null
printf '  %-42s %-28s %s\n' "x86_64-linux, baseline (no FMA unit)" \
  "$(read_body /tmp/p1_x86.s via_mul_add)" "$(read_body /tmp/p1_x86.s via_source)"
rustc +stable -O --target=x86_64-unknown-linux-gnu -Ctarget-feature=+fma,+avx \
      --emit=asm $RS -o /tmp/p1_x86f.s 2>/dev/null
printf '  %-42s %-28s %s\n' "x86_64-linux, +fma,+avx" \
  "$(read_body /tmp/p1_x86f.s via_mul_add)" "$(read_body /tmp/p1_x86f.s via_source)"
echo
echo "  the jmpq on the baseline row is a tail call into libm's software fma."
echo "  a liberty that can only help has become two instructions into a call."
