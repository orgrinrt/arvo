#!/usr/bin/env bash
# p4: 106 section 0.2 reports four distinct ways this corpus produces a
# meaningless green, and claims two of them as its own findings. Each is a
# claim about an instrument, and every count in this panel was taken with one
# of these instruments, so they are checked rather than accepted.
#
#  (1) cargo test --workspace from mock/benches reaches only the driver
#  (2) cargo test without --test-threads=1 hangs on one crate
#  (3) tail -4 reads the DOC-TEST result line, not the unit-test one
#  (4) running the suite creates variants/*/target/, after which a source
#      grep for outputs_may_differ returns 133 where it returned 1
set -u
B=/Users/orgrinrt/Dev/clause-dev/arvo/mock/benches

echo "=== (3) does tail -4 read the doc-test block? ==="
echo "-- structure of a real cargo test tail, from bitpack-shared --"
cd "$B/variants/bitpack-shared" && cargo test 2>/dev/null | tail -8
echo
echo "-- what tail -4 alone would have reported --"
cargo test 2>/dev/null | tail -4
echo
echo "-- exit status of the pipeline --"
cargo test 2>/dev/null | tail -4 >/dev/null; echo "exit=$?"

echo
echo "=== (4) the target/ contamination of a source grep ==="
cd "$B"
echo -n "grep -rl outputs_may_differ variants/            : "
grep -rl outputs_may_differ variants/ 2>/dev/null | wc -l
echo -n "grep -rl ... --exclude-dir=target                : "
grep -rl outputs_may_differ variants/ --exclude-dir=target 2>/dev/null | wc -l
echo
echo "-- where the contaminating hits live --"
grep -rl outputs_may_differ variants/ 2>/dev/null | sed 's|.*/target/|target/|' | grep '^target/' | sed 's|/[^/]*$||' | sort -u | head -5
echo
echo "-- and the same for the other counts 106 section 7 reports --"
for pat in score_output score_dimensions max_relative_error validate_output; do
  a=$(grep -rl "$pat" variants/ 2>/dev/null | wc -l | tr -d ' ')
  b=$(grep -rl "$pat" variants/ --exclude-dir=target 2>/dev/null | wc -l | tr -d ' ')
  printf "  %-22s contaminated=%-5s clean=%-5s\n" "$pat" "$a" "$b"
done
