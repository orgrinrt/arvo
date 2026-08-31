#!/usr/bin/env bash
# Two arms beyond the matrix.
#
# Arm A: one `const fn` body called at both evaluation sites with the same
# arguments, which is arvo's own shape rather than a free-standing const.
# Arm B: whether a const expression can observe the build profile, which bounds
# the independence the matrix reports.
#
# Controls, outcomes written before the run:
#   N1  arm A's runtime half must compile and print at both profiles, so a
#       refusal on the const half is attributable to the site rather than to
#       the file.
#   N2  arm A with arguments that do not overflow must produce a value at both
#       sites, or the arm is measuring the file.
#   N3  arm B must print two lines that agree with each other, whatever they
#       say. Disagreement would mean the two halves are not reading the same
#       flag and the arm proves nothing either way.
set -uo pipefail
cd "$(dirname "$0")"
out=second_out; rm -rf "$out"; mkdir -p "$out"

echo "### Arm A: one const fn body, two call sites"
printf '%-8s %-18s %-6s %s\n' args site 'da' outcome
for prof in on off; do
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
        --cfg 'feature="const_site"' -o "$out/a_const_$prof" one_fn.rs \
        > /dev/null 2> "$out/a_const_$prof.stderr"
  if [ $? -ne 0 ]; then
    k=$(grep -oE 'error\[E[0-9]+\]' "$out/a_const_$prof.stderr" | head -1)
    printf '%-8s %-18s %-6s %s\n' '200,100' 'const context' "$prof" "REFUSED ${k:-error}"
  else
    printf '%-8s %-18s %-6s %s\n' '200,100' 'const context' "$prof" "value $("$out/a_const_$prof")"
  fi
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
        -o "$out/a_run_$prof" one_fn.rs > /dev/null 2> "$out/a_run_$prof.stderr"
  v=$("$out/a_run_$prof" 2> "$out/a_run_$prof.runerr" || true)
  if [ -z "$v" ]; then
    printf '%-8s %-18s %-6s %s\n' '200,100' 'call at runtime' "$prof" \
      "PANIC $(grep -m1 panicked -A1 "$out/a_run_$prof.runerr" | tail -1)"
  else
    printf '%-8s %-18s %-6s %s\n' '200,100' 'call at runtime' "$prof" "$v"
  fi
done

echo
echo "### N1, the runtime half compiles at both profiles"
n1=0
for prof in on off; do [ -x "$out/a_run_$prof" ] || n1=1; done
[ "$n1" = 0 ] && echo "  PASS" || echo "  FAIL, the file does not build"
echo "### N2, the same body with arguments that do not overflow"
sed 's/200, 100/200, 55/; s/black_box(100u8)/black_box(55u8)/' one_fn.rs > "$out/ctl.rs"
n2=0
for prof in on off; do
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
        --cfg 'feature="const_site"' -o "$out/ctl_c_$prof" "$out/ctl.rs" \
        > /dev/null 2> "$out/ctl_c_$prof.stderr" || n2=1
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
        -o "$out/ctl_r_$prof" "$out/ctl.rs" > /dev/null 2> "$out/ctl_r_$prof.stderr" || n2=1
  [ -x "$out/ctl_c_$prof" ] && printf '  const   da=%-4s %s\n' "$prof" "$("$out/ctl_c_$prof")"
  [ -x "$out/ctl_r_$prof" ] && printf '  runtime da=%-4s %s\n' "$prof" "$("$out/ctl_r_$prof")"
done
[ "$n2" = 0 ] && echo "  PASS, both sites produce a value at both profiles" \
              || echo "  FAIL, the control refuses somewhere"

echo
echo "### Arm B: can a const expression observe the build profile?"
for prof in on off; do
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$prof" \
        -o "$out/b_$prof" profile_visible.rs > /dev/null 2> "$out/b_$prof.stderr"
  echo "  debug-assertions=$prof:"
  "$out/b_$prof" | sed 's/^/    /'
done
echo "### N3, the two halves of arm B must agree with each other"
n3=0
for prof in on off; do
  c=$("$out/b_$prof" | sed -n '1s/.*= //p'); r=$("$out/b_$prof" | sed -n '2s/.*= //p')
  [ "$c" = "$r" ] || n3=1
done
[ "$n3" = 0 ] && echo "  PASS, they agree at both profiles" \
              || echo "  FAIL, the two halves read different flags"

echo
echo "### stderr sizes"
wc -c "$out"/*.stderr | tail -12

echo
echo "### Arm C: which flag actually moves arm A, debug-assertions or"
echo "###        overflow-checks. The first sets the second by default, so arm A"
echo "###        cannot tell them apart. dimension::build_profile's grammar names"
echo "###        debug-assertions and opt level and not overflow-checks, so which"
echo "###        one acts decides whether the declared axis names the thing that"
echo "###        moves or a proxy for it."
printf '%-22s %-22s %s\n' 'debug-assertions' 'overflow-checks' 'const-context outcome'
for da in on off; do
  for oc in on off; do
    rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$da" -C "overflow-checks=$oc" \
          --cfg 'feature="const_site"' -o "$out/c_${da}_${oc}" one_fn.rs \
          > /dev/null 2> "$out/c_${da}_${oc}.stderr"
    if [ $? -ne 0 ]; then
      k=$(grep -oE 'error\[E[0-9]+\]' "$out/c_${da}_${oc}.stderr" | head -1)
      printf '%-22s %-22s %s\n' "$da" "$oc" "REFUSED ${k:-error}"
    else
      printf '%-22s %-22s %s\n' "$da" "$oc" "value $("$out/c_${da}_${oc}")"
    fi
  done
done
echo "### N4, the four cells must not all be the same, or the arm distinguishes"
echo "###     nothing and cannot say which flag acts."
u=$(for da in on off; do for oc in on off; do
      if [ -x "$out/c_${da}_${oc}" ]; then "$out/c_${da}_${oc}"; else echo REFUSED; fi
    done; done | sort -u | grep -c .)
[ "$u" -gt 1 ] && echo "  PASS, $u distinct outcomes across the four cells" \
               || echo "  FAIL, one outcome everywhere"

echo
echo "### Arm D: the same 2x2 on the literal-in-a-const shape the matrix used,"
echo "###        so the two shapes can be compared under one flag pair. If the"
echo "###        literal refuses in all four cells and the const fn call refuses"
echo "###        in two, the two arms are measuring different mechanisms and the"
echo "###        matrix's result does not generalise to a const fn."
printf '%-22s %-22s %s\n' 'debug-assertions' 'overflow-checks' 'const A: u8 = 200 + 100'
for da in on off; do
  for oc in on off; do
    rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$da" -C "overflow-checks=$oc" \
          -o "$out/d_${da}_${oc}" const_add.rs > /dev/null 2> "$out/d_${da}_${oc}.stderr"
    if [ $? -ne 0 ]; then
      k=$(grep -oE 'error\[E[0-9]+\]' "$out/d_${da}_${oc}.stderr" | head -1)
      printf '%-22s %-22s %s\n' "$da" "$oc" "REFUSED ${k:-error}"
    else
      printf '%-22s %-22s %s\n' "$da" "$oc" "value $("$out/d_${da}_${oc}")"
    fi
  done
done
echo "### N5, arm D must differ from arm C somewhere, or the two shapes behave"
echo "###     the same and there is nothing to report."
diff <(for da in on off; do for oc in on off; do
         [ -x "$out/c_${da}_${oc}" ] && echo V || echo R; done; done) \
     <(for da in on off; do for oc in on off; do
         [ -x "$out/d_${da}_${oc}" ] && echo V || echo R; done; done) > /dev/null \
  && echo "  FAIL, the two shapes agree in all four cells" \
  || echo "  PASS, the two shapes disagree, so they are different mechanisms"

echo
echo "### Arm E: the same operation written so the refusal is a value the body"
echo "###        inspects rather than a check the backend may not have emitted."
echo "###        If this refuses in all four cells, the guarantee arm C loses is"
echo "###        recoverable by construction and the finding is constructive"
echo "###        rather than a complaint."
printf '%-22s %-22s %s\n' 'debug-assertions' 'overflow-checks' 'const-context outcome'
for da in on off; do
  for oc in on off; do
    rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$da" -C "overflow-checks=$oc" \
          --cfg 'feature="const_site"' -o "$out/e_${da}_${oc}" checked_fn.rs \
          > /dev/null 2> "$out/e_${da}_${oc}.stderr"
    if [ $? -ne 0 ]; then
      k=$(grep -oE 'error\[E[0-9]+\]' "$out/e_${da}_${oc}.stderr" | head -1)
      printf '%-22s %-22s %s\n' "$da" "$oc" "REFUSED ${k:-error}"
    else
      printf '%-22s %-22s %s\n' "$da" "$oc" "value $("$out/e_${da}_${oc}")"
    fi
  done
done
echo "### N6, arm E's control: the same body with operands that fit must produce"
echo "###     a value in all four cells, or arm E refuses for the wrong reason."
sed 's/checked_add(200, 100)/checked_add(200, 55)/; s/black_box(100u8)/black_box(55u8)/' \
    checked_fn.rs > "$out/e_ctl.rs"
n6=0
for da in on off; do for oc in on off; do
  rustc --edition 2021 -C opt-level=3 -C "debug-assertions=$da" -C "overflow-checks=$oc" \
        --cfg 'feature="const_site"' -o "$out/ectl_${da}_${oc}" "$out/e_ctl.rs" \
        > /dev/null 2> "$out/ectl_${da}_${oc}.stderr" || n6=1
  [ -x "$out/ectl_${da}_${oc}" ] && printf '  da=%-4s oc=%-4s %s\n' "$da" "$oc" "$("$out/ectl_${da}_${oc}")"
done; done
[ "$n6" = 0 ] && echo "  PASS" || echo "  FAIL, the control refuses too"
