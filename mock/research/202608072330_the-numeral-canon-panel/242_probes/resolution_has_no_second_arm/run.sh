#!/usr/bin/env bash
# Two arms. The second must refuse to compile; that refusal is the finding.
set -u
echo "======== ARM 1: resolve four ambient algebras over the ratified ten"
cargo run --quiet 2>&1
echo "-------- exit: $?"
echo
echo "======== ARM 2 (control, must REFUSE): a candidate leaving one coordinate unfixed"
cargo build --quiet --features omit_a_coordinate 2>&1 | head -20
echo "-------- exit: ${PIPESTATUS[0]}"
