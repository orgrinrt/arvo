#!/usr/bin/env bash
# The driver for the composition-contracts probes.
#
# One command, from anywhere: `./run.sh` in this directory. It builds both arms
# at release and writes each one's whole stdout beside it, so the committed
# output is the run rather than a transcription of it.
#
# `cargo build --release` and then the binary, never `cargo run` alone, so the
# profile the numbers were taken at is stated by the command rather than assumed.
set -uo pipefail
cd "$(dirname "$0")"

TOOL=$(rustc --version)

for arm in \
  p1_the_end_to_end_min_plus_run_rebuilt_from_the_mechanism \
  p2_equality_transports_as_a_congruence_and_not_as_an_adequacy
do
  OUT="output_${arm}.txt"
  {
    echo "tool:    ${TOOL}"
    echo "profile: release, opt-level 3, no feature gates"
    echo "driver:  run.sh in this directory"
    echo
    cargo build --release --bin "${arm}" 2>&1 | sed 's/^/    build: /'
    ./target/release/"${arm}"
  } | tee "${OUT}"
  echo
done
