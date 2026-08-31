#!/bin/sh
# Seat 216, probe 4. Compiles each arm on its own and records what the compiler said.
#
# An arm named `must compile` that fails, or `must FAIL` that compiles, is the finding.
# The expected outcome is the first line of each arm's own module documentation, read
# out of the source here rather than restated, so this script cannot relabel an arm to
# match what happened.
set -u
cd "$(dirname "$0")/p4_arms" || exit 1
mkdir -p ../.build/p4
for f in *.rs; do
    name="${f%.rs}"
    printf '=== %s ===\n' "$name"
    head -1 "$f" | sed 's|^//! *|    declares: |'
    err=$(rustc --edition 2021 --crate-type bin -o "../.build/p4/$name" "$f" 2>&1)
    if [ -z "$err" ]; then
        printf '    OUTCOME: compiled\n\n'
    else
        printf '    OUTCOME: refused\n'
        printf '%s\n' "$err" | grep -E '^(error|help:)' | head -4 | sed 's/^/      /'
        printf '\n'
    fi
done
