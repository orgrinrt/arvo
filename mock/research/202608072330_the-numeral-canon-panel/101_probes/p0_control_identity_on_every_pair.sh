#!/usr/bin/env bash
# Is the noise-floor control byte-identical to the arm it controls, in EVERY
# family that declares one?
#
# Why this runs before anything else in `101_probes/`. Sections of `100` and
# everything in my p4 use the control pair as a CALIBRATION: two arms the bench
# declares compile to the same machine code, so any measured difference between
# them is the instrument and nothing else. That reading is only as good as the
# byte-identity claim.
#
# `26_probes/control_identity.sh` checks exactly one pair, the carrier one, and
# `bitpack-carrier-d16-control/src/lib.rs:1-8` cites it. The other two control
# pairs assert byte-identity in their own module headers and neither names a
# check: `bitpack-contend-d16-control/src/lib.rs:1-5` ("byte-identical to
# `bitpack-contend-d16`") and `bitpack-wide-d16-control/src/lib.rs:1-3`. So two
# of the three pairs the four control-bearing families rest on were asserted and
# not verified. This verifies all three.
#
# Method is `26_probes/control_identity.sh`'s, generalised over pairs and with
# its normalisations restated rather than inherited: disassemble both dylibs,
# strip the path header otool prints, strip absolute addresses (the two bodies
# sit at different addresses), strip the exported symbol name, and strip the
# literal-pool comment that spells the variant's own name (the harness
# registration passes it, so it necessarily differs). Everything else must match
# instruction for instruction. One normalisation is mine rather than the
# predecessor's, and it is why this script's first run reported all three pairs
# DIFFERING: otool prints an `adrp` page-relative operand as a bare decimal, so
# the address normalisation misses it. Every one of the 16, 8 and 30 differing
# lines in that run was an `adrp` page number off by one, with instruction
# counts already equal at 50497, 55678 and 55513 per pair. That first output is
# kept at `p0_before_the_adrp_normalisation.out`, because the differences it
# shows ARE the evidence that nothing else differs.
#
# Build first (each variant crate is free-standing, so each has its own target):
#   for c in bitpack-carrier-d16 bitpack-carrier-d16-control \
#            bitpack-contend-d16 bitpack-contend-d16-control \
#            bitpack-wide-d16   bitpack-wide-d16-control; do
#     (cd mock/benches/variants/$c && cargo build --release); done
#
# Usage:  ./p0_control_identity_on_every_pair.sh
# Exit 0 = every pair identical. Exit 1 = at least one differs.

set -u

HERE="$(cd "$(dirname "$0")" && pwd)"
VAR="$(cd "$HERE/../../../benches/variants" && pwd)"

PAIRS=(
  "bitpack-carrier-d16:bitpack-carrier-d16-control"
  "bitpack-contend-d16:bitpack-contend-d16-control"
  "bitpack-wide-d16:bitpack-wide-d16-control"
)

norm() {
  # $1 = dylib path, $2 = crate underscore name
  otool -tV "$1" \
    | tail -n +2 \
    | sed -E \
        -e 's/^[0-9a-f]{8,16}[[:space:]]//' \
        -e 's/0x[0-9a-f]+/ADDR/g' \
        -e 's/#[[:space:]]*[0-9]+/#IMM/g' \
        -e 's/(adrp[[:space:]]+[a-z0-9]+,)[[:space:]]*-?[0-9]+/\1 PAGE/' \
        -e "s/${2}//g" \
        -e 's/;.*$//' \
    | sed -E 's/[[:space:]]+$//'
}

rc=0
for p in "${PAIRS[@]}"; do
  a="${p%%:*}"; b="${p##*:}"
  an="bench_$(echo "$a" | tr '-' '_')"
  bn="bench_$(echo "$b" | tr '-' '_')"
  A="$VAR/$a/target/release/lib$an.dylib"
  B="$VAR/$b/target/release/lib$bn.dylib"
  for f in "$A" "$B"; do
    if [ ! -f "$f" ]; then echo "MISSING $f" >&2; rc=2; continue 2; fi
  done
  # The two crates carry different symbol names, so normalise BOTH names out of
  # both files rather than each out of its own: a body may mention either.
  na=$(norm "$A" "$an" | sed -E "s/${bn}//g")
  nb=$(norm "$B" "$bn" | sed -E "s/${an}//g")
  ca=$(printf '%s\n' "$na" | grep -c .)
  cb=$(printf '%s\n' "$nb" | grep -c .)
  if [ "$na" = "$nb" ]; then
    printf '%-28s vs %-28s IDENTICAL   (%s instruction lines each)\n' "$a" "$b" "$ca"
  else
    d=$(diff <(printf '%s\n' "$na") <(printf '%s\n' "$nb") | grep -c '^[<>]')
    printf '%-28s vs %-28s DIFFER      (%s vs %s lines, %s differing)\n' "$a" "$b" "$ca" "$cb" "$d"
    diff <(printf '%s\n' "$na") <(printf '%s\n' "$nb") | head -20
    rc=1
  fi
done
exit $rc
