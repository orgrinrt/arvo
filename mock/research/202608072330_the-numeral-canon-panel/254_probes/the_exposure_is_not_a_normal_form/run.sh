#!/usr/bin/env bash
# Seat 253. One compilation, four pairs. Pairs 1 and 2 are the cases that must
# fail for "the exposed tuple is a normal form for identity"; pairs 3 and 4 are
# the controls that say the comparator can return both answers.
set -u
echo "======== rustc"
rustc --version
echo
cargo run --quiet 2>&1
echo "-------- exit: $?"
