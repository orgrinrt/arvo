#!/usr/bin/env bash
# Rebuilds and reruns every probe in this directory from scratch.
#   ./verify.sh
# Pin: nightly-2026-05-28, rustc 1.98.0-nightly (57d06900f 2026-05-27).
# A bare `rustc` outside the repo tree resolves to stable, so the toolchain is explicit.
set -u
cd "$(dirname "$0")"
RS="rustc +nightly-2026-05-28 --edition 2021 -O"
mkdir -p bin
fail=0

for p in p1_fibre_count p2_stride_is_not_size_of p3_blind_suite p4_access_width \
         p5_recovery_direction p6_trait_form_recovers_both p7_alignment_is_not_a_third; do
  echo "=== $p ==="
  if $RS "$p.rs" -o "bin/${p%%_*}" 2>/dev/null; then
    "bin/${p%%_*}" | tee "$p.out"
  else
    echo "COMPILE FAILED: $p"; fail=1
  fi
  echo
done

# p5b does NOT compile, on purpose. A refusal is the result, so success here is a non-zero exit.
echo "=== p5b_const_to_type (expected to be REFUSED) ==="
if rustc +nightly-2026-05-28 --edition 2021 --crate-type lib p5b_const_to_type.rs \
     > /dev/null 2> p5b_const_to_type.err; then
  echo "UNEXPECTED: p5b compiled. the claim in section 6 is void and must be redone."; fail=1
else
  echo "refused as expected. diagnostic count:"
  grep -c "^error" p5b_const_to_type.err
  grep -m1 "generic_const_exprs" p5b_const_to_type.err
fi

echo
echo "feature gates enabled across all probes (expect none):"
grep -n '^#!\[feature' ./*.rs || echo "  none"
exit $fail
