#!/bin/bash
set -u
N="+nightly-2026-05-28"
OPT=/opt/homebrew/opt/llvm/bin/opt
LLC=/opt/homebrew/opt/llvm/bin/llc

# -Cno-prepopulate-passes so we get rustc's IR BEFORE the optimisation pipeline,
# which is where a build-layer pass registered at a pipeline extension point
# would see it.
rustc $N -O -Cpanic=abort -Cno-prepopulate-passes --emit=llvm-ir --crate-type=lib \
      04_the_pass_closes_the_residue.rs -o /tmp/p4_raw.ll 2>/dev/null

echo "=== 1. the fadd rustc emits for the reduction, before any LLVM pass ==="
grep -E "= fadd" /tmp/p4_raw.ll | sed 's/^/  /'

run () { # $1 = ll, $2 = label
  $OPT -passes='default<O3>' -S "$1" -o /tmp/p4_$2_opt.ll 2>/dev/null
  $LLC -O3 -o /tmp/p4_$2.s /tmp/p4_$2_opt.ll 2>/dev/null
  BODY=$(awk '/^_residue_relaxed_reduce:/{p=1} p{print} /cfi_endproc/{if(p)exit}' /tmp/p4_$2.s)
  printf '  %-26s vec-fadd=%-3s scalar-fadd=%-3s  vec-width-in-IR=%s\n' "$2" \
    "$(grep -cE 'fadd(\.2d)?[[:space:]]+v[0-9]' <<<"$BODY")" \
    "$(grep -cE 'fadd[[:space:]]+d[0-9]' <<<"$BODY")" \
    "$(grep -oE 'fadd[^,]*<[0-9]+ x double>' /tmp/p4_$2_opt.ll | head -1 | grep -oE '<[0-9]+ x double>' || echo none)"
}

echo
echo "=== 2. same IR, same pipeline, one flag on one instruction ==="
run /tmp/p4_raw.ll as-emitted
sed -E 's/= fadd (double)/= fadd reassoc \1/' /tmp/p4_raw.ll > /tmp/p4_re.ll
run /tmp/p4_re.ll plus-reassoc
sed -E 's/= fadd (double)/= fadd fast \1/'    /tmp/p4_raw.ll > /tmp/p4_fa.ll
run /tmp/p4_fa.ll plus-fast

echo
echo "=== 3. cost of the un-inlined shim, after the pass marks it alwaysinline ==="
python3 - <<'PYEOF'
import re
src = open('/tmp/p4_re.ll').read()
# Exactly the three edits a FunctionPass makes after rewriting the flags:
#   F.removeFnAttr(NoInline); F.addFnAttr(AlwaysInline); CB->removeFnAttr(NoInline)
m   = re.search(r'define\s+[^\n]*@residue_relaxed_reduce[^\n]*?#(\d+)', src)
grp = m.group(1)
src = re.sub(rf'(attributes #{grp} = \{{[^}}]*?)\bnoinline\b\s*', r'\1alwaysinline ', src)
src = re.sub(r'(call [^\n]*@residue_relaxed_reduce\([^\n]*\)) #\d+', r'\1', src)
open('/tmp/p4_ai.ll','w').write(src)
PYEOF
$OPT -passes='default<O3>' -S /tmp/p4_ai.ll -o /tmp/p4_ai_out.ll 2>/dev/null
printf '  calls to the shim left in @caller: %s\n' \
  "$(awk '/^define.*@caller/{p=1} p{print} /^}/{if(p)exit}' /tmp/p4_ai_out.ll | grep -c 'call .*residue_relaxed_reduce')"
printf '  vector fadd inside @caller:        %s\n' \
  "$(awk '/^define.*@caller/{p=1} p{print} /^}/{if(p)exit}' /tmp/p4_ai_out.ll | grep -cE 'fadd[^,]*<[0-9]+ x double>')"
