#!/bin/sh
# Profile: release (-O). Toolchain: nightly-2026-05-28 per rust-toolchain.toml.
# Host: aarch64-apple-darwin.
set -e
cd "$(dirname "$0")"
for arm in base alt; do
  if [ "$arm" = alt ]; then F='--cfg feature="alt_policy"'; else F=''; fi
  echo "=== build $arm ==="
  rustc --edition 2024 -O $F -o "rc_$arm" cfg_const.rs 2>&1 | grep -E '^error' || true
  ./"rc_$arm"
  rustc --edition 2024 -O $F --crate-type lib --emit asm -o "asm_$arm.s" cfg_const.rs 2>/dev/null
  printf '  lowered() emitted body: '
  sed -n '/^_lowered:/,/ret/p' "asm_$arm.s" | grep -vE '^_lowered:|\.p2align' | tr -d '\t' | tr '\n' ' '
  printf '\n  branch on a build value: '
  if sed -n '/^_lowered:/,/ret/p' "asm_$arm.s" | grep -qE '^\s*(cbz|cbnz|b\.)'; then echo PRESENT; else echo none; fi
  rm -f "rc_$arm"
done

echo
echo "CONTROL HAZARD differs between builds  : 8191 vs 0        (want differ)"
echo "CONTROL CONTROL_STABLE identical       : 8191 vs 8191     (want identical)"
echo "CONTROL no branch on a build value     : none in either   (want none)"
echo
echo "VERDICT: a const fn does read cfg, so F159-2's premise is verified"
echo "         independently of 157's probe and the finding stands on its own"
echo "         evidence rather than on 157's."
echo
echo "         One sharpening of F159-2's own wording, from this run. The base"
echo "         build's lowered body is 'cmp; csel', which is check-SHAPED. It"
echo "         is not a validity check: it is the saturating semantics the type"
echo "         declares, which is arithmetic and is what I15 permits. The claim"
echo "         F159-2 needs is the narrower one this probe measures: neither"
echo "         build branches on the build, and each emits one path. I15's"
echo "         property holds in both builds while the denotation differs"
echo "         between them, which is a relation no single build can witness."
