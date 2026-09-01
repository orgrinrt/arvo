#!/usr/bin/env bash
# The full 2x2: {const evaluation, execution} x {debug-assertions on, off}, on
# three arithmetic expressions, each written identically on both sides.
#
# What it is for. `dimension::build_profile` is the only declared axis that
# mentions const evaluation at all, and it welds two channels into one sentence:
# "the overflow panic plus a const-eval refusal ... neither exists in a release
# artifact". If the table has four distinct cells rather than two, the two
# channels are independent and no declared axis separates const evaluation from
# execution.
#
# Controls, outcomes written before the run:
#   M1  a negative control per operation: the same expression with operands that
#       do not overflow must produce a value in all four cells. An arm where the
#       control also refuses is measuring the file rather than the arithmetic.
#   M2  the runtime binaries must actually run and print, so a cell recorded as
#       `wraps` rests on an observed value rather than on a successful compile.
#   M3  every stderr is kept and its size printed. A refusal is a stderr fact.
#   M4  the const and runtime halves must be the same expression at the same
#       types. Checked by printing both sources into the output.
set -uo pipefail
cd "$(dirname "$0")"
out=matrix_out; rm -rf "$out"; mkdir -p "$out"

echo "### M4, the two halves of each pair, printed so they can be compared by eye"
for op in add sub shl; do echo "--- const_$op.rs"; sed 's/^/    /' "const_$op.rs"; done
echo "--- matrix.rs (the runtime half, one binary, op chosen by argv)"
sed 's/^/    /' matrix.rs
echo

printf '%-6s %-18s %-20s %s\n' op site 'debug-assertions' outcome
for op in add sub shl; do
  for prof in on off; do
    rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
          -o "$out/c_${op}_$prof" "const_$op.rs" > /dev/null 2> "$out/c_${op}_$prof.stderr"
    if [ $? -ne 0 ]; then
      k=$(grep -oE 'error\[E[0-9]+\]' "$out/c_${op}_$prof.stderr" | head -1)
      printf '%-6s %-18s %-20s %s\n' "$op" "const evaluation" "$prof" "REFUSED ${k:-error}"
    else
      printf '%-6s %-18s %-20s %s\n' "$op" "const evaluation" "$prof" "value $("$out/c_${op}_$prof")"
    fi
  done
done
for prof in on off; do
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
        -o "$out/rt_$prof" matrix.rs > /dev/null 2> "$out/rt_$prof.stderr"
  for op in add sub shl; do
    v=$("$out/rt_$prof" "$op" 2> "$out/rt_${op}_$prof.runerr")
    if [ -z "$v" ]; then
      printf '%-6s %-18s %-20s %s\n' "$op" "execution" "$prof" \
        "PANIC $(grep -m1 panicked -A1 "$out/rt_${op}_$prof.runerr" | tail -1)"
    else
      printf '%-6s %-18s %-20s %s\n' "$op" "execution" "$prof" "value $v"
    fi
  done
done

echo
echo "### M1, negative controls: operands that do not overflow, all four cells"
for op in add sub shl; do
  case $op in
    add) e="200u8 + 55u8" ;;
    sub) e="1u8 - 1u8" ;;
    shl) e="1u8 << 7u32" ;;
  esac
  printf 'const C: u8 = %s;\nfn main() { println!("{}", C); }\n' "$e" > "$out/ctl_$op.rs"
  for prof in on off; do
    rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
          -o "$out/ctlbin_${op}_$prof" "$out/ctl_$op.rs" > /dev/null 2> "$out/ctl_${op}_$prof.stderr"
    if [ $? -ne 0 ]; then
      printf '  %-5s const  da=%-4s FAIL, the control refuses too\n' "$op" "$prof"
    else
      printf '  %-5s const  da=%-4s value %s\n' "$op" "$prof" "$("$out/ctlbin_${op}_$prof")"
    fi
  done
done

echo
echo "### M3, stderr sizes"
wc -c "$out"/*.stderr | tail -20
