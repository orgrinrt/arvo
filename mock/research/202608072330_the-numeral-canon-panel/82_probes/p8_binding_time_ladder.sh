#!/usr/bin/env bash
# When does a const predicate's refusal actually fire?
#
# Op's steer at 83 says the licensed category is whatever is available at const
# time. This script measures that const time is not one moment. Three
# constructions, each carrying a declaration the verdict must refuse, each
# compiled two ways: type check only (--emit=metadata, no codegen, no generic
# instantiation) and full codegen.
#
# Exit codes are captured directly rather than through a pipe, because piping
# through `head` reports the exit code of `head`.
set -u
cd "$(dirname "$0")"

run() {
  local label="$1"; shift
  "$@" >/tmp/bt_out.txt 2>&1
  local rc=$?
  if [ $rc -eq 0 ]; then
    printf '  %-46s ACCEPTED\n' "$label"
  else
    printf '  %-46s REFUSED  %s\n' "$label" "$(grep -m1 -oE '^error(\[E[0-9]+\])?' /tmp/bt_out.txt)"
    grep -m1 -oE 'evaluation panicked: [^"]*|the declared operand window [^`]*' /tmp/bt_out.txt \
      | head -1 | sed 's/^/       /'
  fi
}

echo "toolchain: $(rustc --version)"
echo "host:      $(rustc -vV | grep host | cut -d' ' -f2)"
echo

echo "RUNG 1. Structural trait bound. Sign uniformity is carried by which"
echo "        declaration shape was written; a straddling window simply does not"
echo "        implement the permission. (p3d)"
run "type check only, straddling in dead code" \
  rustc -O -C panic=abort --crate-type=staticlib --emit=metadata -o /tmp/bt1a.meta p3d_bad_straddling_dead_code.rs
run "full codegen, straddling in dead code" \
  rustc -O -C panic=abort --crate-type=staticlib -o /tmp/bt1b.a p3d_bad_straddling_dead_code.rs
run "type check only, licensed declarations" \
  rustc -O -C panic=abort --crate-type=staticlib --emit=metadata -o /tmp/bt1c.meta p3d_structural_permission.rs
echo

echo "RUNG 2. Inline const block in a NON-GENERIC function, reading a module"
echo "        const that lives outside any type. (p7)"
run "type check only, straddling const, dead code present" \
  rustc -O -C panic=abort --crate-type=staticlib --emit=metadata -o /tmp/bt2a.meta p7_bad_straddling_const.rs
run "full codegen, straddling const" \
  rustc -O -C panic=abort --crate-type=staticlib -o /tmp/bt2b.a p7_bad_straddling_const.rs
run "full codegen, licensed const" \
  rustc -O -C panic=abort --crate-type=staticlib -o /tmp/bt2c.a p7_window_from_outside_the_typestate.rs
echo

echo "RUNG 3. Const assert in an associated const of a GENERIC function, which"
echo "        is what both cold derivations built and what p3a reproduces."
run "type check only, straddling in dead code" \
  rustc -O -C panic=abort --crate-type=staticlib --emit=metadata -o /tmp/bt3a.meta p3b_straddling_declaration_refused.rs
run "full codegen, straddling in UNREACHED pub fn" \
  rustc -O -C panic=abort --crate-type=staticlib -o /tmp/bt3b.a p3b_straddling_declaration_refused.rs
run "full codegen, straddling in a REACHED no_mangle fn" \
  rustc -O -C panic=abort --crate-type=staticlib -o /tmp/bt3c.a p3b2_straddling_reachable.rs
echo

echo "RUNG 0, for comparison. The crate-level const carrying the model-band"
echo "        cross-check, which is not attached to any function at all."
run "type check only, perturbed closed form (p3d)" \
  rustc -O -C panic=abort --crate-type=staticlib --emit=metadata -o /tmp/bt0a.meta p3d_perturbed_verdict.rs
run "type check only, perturbed closed form (p7)" \
  rustc -O -C panic=abort --crate-type=staticlib --emit=metadata -o /tmp/bt0b.meta p7_perturbed_verdict.rs
