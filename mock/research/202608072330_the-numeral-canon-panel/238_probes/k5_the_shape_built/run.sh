#!/usr/bin/env bash
# Build the tree, run its assertions, then run the real lint over both crates.
#
# Three questions in order, because a later yes means nothing without the
# earlier ones: does the shape build, does it still compute the right answers,
# and does the crate outside the door clear the rule.
#
# The mutation below is what says the assertions can fail at all. It flips the
# phase the biased format declares, which every negative assertion in
# `the_answers_are_the_ones_the_shipped_laws_assert` turns on, runs the suite,
# and puts the file back. A run where the mutant passes is a run where the
# suite proves nothing, and it is reported as that rather than as a pass.
set -u
cd "$(dirname "$0")"

echo "=== 1. does the shape build ==="
if cargo build --quiet 2>&1; then echo "    built."; else echo "    DID NOT BUILD."; exit 1; fi
echo

echo "=== 2. do the assertions hold ==="
cargo test --quiet 2>&1 | grep -E '^test |test result' | sed 's/^/    /'
echo

echo "=== 3. the mutation: can they fail ==="
cp outside/src/lib.rs outside/src/lib.rs.orig
sed -i '' 's|const PHASE: Phase = Phase::of(1, 2);|const PHASE: Phase = Phase::NONE;|' outside/src/lib.rs
if cargo test --quiet 2>&1 | grep -q 'test result: FAILED'; then
    echo "    the mutant is refused, so the assertions are load-bearing:"
    cargo test --quiet 2>&1 | grep -E "panicked|assertion|the_answers" | head -4 | sed 's/^/      /'
    mutant_died=1
else
    echo "    THE MUTANT PASSED. The suite does not constrain the phase and"
    echo "    nothing above is evidence."
    mutant_died=0
fi
mv outside/src/lib.rs.orig outside/src/lib.rs
cargo build --quiet 2>&1 >/dev/null
echo

echo "=== 4. the lint, over both crates, no exemption anywhere ==="
# Captured rather than piped. `cmd | sed` reports `sed`'s status, which is
# always zero, and an earlier version of this file read that as the gate's and
# printed a pass over a failing gate.
gate_out=$( cd gate && cargo run --quiet 2>&1 )
gate_ok=$?
echo "$gate_out" | sed 's/^/    /'
echo

echo "=== verdict ==="
if [ "$mutant_died" -eq 1 ] && [ "$gate_ok" -eq 0 ]; then
    echo "The shape builds, computes the shipped answers, refuses the mutant, and"
    echo "leaves the crate outside the door with nothing for the lint to report."
    exit 0
fi
[ "$mutant_died" -eq 1 ] || echo "RUN INVALID: the mutant survived."
[ "$gate_ok" -eq 0 ] || echo "RUN INVALID: the lint gate refused."
exit 1
