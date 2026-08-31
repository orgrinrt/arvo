#!/bin/sh
# Runner for p3. Every arm states its required verdict and the script says
# whether it got it, so a reader does not have to compare two lists.
#
# NOT `-o /dev/null`. 191 records that as the defect that made every arm of its
# own p1 fail for the wrong reason, and the only thing that caught it was the
# base arm having to compile. Same discipline: A1/A2 must compile, and if they
# do not, nothing else in the run means anything.
set -u
here=$(cd "$(dirname "$0")" && pwd)
src="$here/p3_composing_p7_and_p8.rs"
out="$here/p3_out"
rm -rf "$out"; mkdir -p "$out"
RUSTC="${RUSTC:-rustc}"
echo "### $($RUSTC +nightly-2026-05-28 --version 2>/dev/null || $RUSTC --version)"
echo "### src: $(basename "$src")"
echo

run() { # $1 label  $2 required (COMPILE|REFUSE)  $3.. cfgs
  lbl=$1; req=$2; shift 2
  args=""
  for c in "$@"; do args="$args --cfg $c"; done
  # shellcheck disable=SC2086
  if $RUSTC +nightly-2026-05-28 --edition 2021 --crate-type lib \
       --out-dir "$(mktemp -d)" $args "$src" > "$out/$lbl.log" 2>&1; then
    got=COMPILE
  else
    got=REFUSE
  fi
  if [ "$got" = "$req" ]; then verdict="as required"; else verdict="*** NOT AS REQUIRED ***"; fi
  printf "%-6s %-16s required=%-8s got=%-8s %s\n" "$lbl" "${*:-<none>}" "$req" "$got" "$verdict"
  if [ "$got" = REFUSE ]; then
    grep -m2 -E '^error(\[E[0-9]+\])?:' "$out/$lbl.log" | sed 's/^/         /'
  fi
}

run A1A2 COMPILE
run A2m  REFUSE  mutate
run B1   REFUSE  arm_b1
run B2   REFUSE  arm_b2
run C1   COMPILE arm_c1
run C2   COMPILE arm_c2
echo
echo "### logs under $(basename "$out")/"
